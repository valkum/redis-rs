use super::arguments::Argument;
use super::GenerationConfig;
use super::GenerationKind;
use super::COMMAND_NAME_OVERWRITE;
use crate::commands::ArgType;
use crate::commands::DocFlag;
use crate::commands::{CommandArgument, CommandDefinition, CommandGroup};
use crate::ident::to_snake;

/// An abstract type representing a code generation unit for a command
#[derive(Debug, Clone)]
pub(crate) struct Command<'a> {
    name: String,
    command: String,
    docs: Vec<String>,
    group: CommandGroup,
    args: Vec<Argument<'a>>,
    pub(crate) deprecated: bool,
    pub(crate) deprecated_since: Option<String>,
}

impl<'a> Command<'a> {
    pub(crate) fn new(
        mut name: String,
        definition: &CommandDefinition,
        config: &'a GenerationConfig,
    ) -> Self {
        let command = name.clone();

        let mut kv_index: (u8, u8) = (0, 0);
        let args = definition
            .arguments
            .iter()
            .filter_map(|arg| map_argument(&mut kv_index, arg, config))
            .collect::<Vec<_>>();

        let docs = build_docs(&name, definition, config.kind);

        name = if let Some(&(_, name)) = COMMAND_NAME_OVERWRITE
            .iter()
            .find(|(ow_name, _)| ow_name == &name)
        {
            name.to_owned()
        } else {
            to_snake(&name)
        };

        Self {
            name,
            command,
            docs,
            group: definition.group,
            args,
            deprecated: definition.doc_flags.contains(&DocFlag::Deprecated),
            deprecated_since: definition
                .deprecated_since
                .as_ref()
                .map(ToString::to_string),
        }
    }

    pub(crate) fn fn_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn command(&self) -> &str {
        &self.command
    }

    pub(crate) fn arguments(&self) -> impl Iterator<Item = &Argument> + ExactSizeIterator {
        self.args.iter()
    }

    pub(crate) fn group(&self) -> CommandGroup {
        self.group
    }

    pub(crate) fn docs(&self) -> &[String] {
        &self.docs
    }
}

// Todo handle key_specs correctly
fn map_argument<'a>(
    indices: &mut (u8, u8),
    arg: &CommandArgument,
    config: &'a GenerationConfig,
) -> Option<Argument<'a>> {
    let (ref mut key_id, ref mut value_id) = indices;

    // TODO Ignore token arguments until we have proper token handling.
    // We probably want to generate a type for each token that implements ToRedisArgs and expands to the wrapped type and the Token
    if arg.token.is_some() {
        return None;
    }

    let accepts_multiple = arg.multiple && (config.kind == GenerationKind::Full);

    match arg.r#type {
        ArgType::Key { key_spec_index: _ } => {
            let idx = *key_id;
            *key_id += 1;

            let name = to_snake(&arg.name);

            let r#trait = "ToRedisArgs".to_string();

            Some(Argument::new_generic(
                name,
                format!("K{}", idx),
                r#trait,
                arg.optional,
                accepts_multiple,
                config,
            ))
        }
        ArgType::Integer => {
            let name = to_snake(&arg.name);

            Some(Argument::new_concrete(
                name,
                "i64".to_owned(),
                arg.optional,
                accepts_multiple,
                config,
            ))
        }
        ArgType::Double => {
            let name = to_snake(&arg.name);

            Some(Argument::new_concrete(
                name,
                "f64".to_owned(),
                arg.optional,
                accepts_multiple,
                config,
            ))
        }
        ArgType::String => {
            let idx = *value_id;
            *value_id += 1;

            let name = to_snake(&arg.name);

            // ToRedis is implemented for Vec thus it currently does not make much sense to specialize the trait bound for multiple.
            // Else something like this could be useful?
            // let r#trait = if arg.multiple {
            //     "Iterator<Item = impl ToRedisArgs>".to_string()
            // } else {
            //     "ToRedisArgs".to_string()
            // };
            let r#trait = "ToRedisArgs".to_string();

            Some(Argument::new_generic(
                name,
                format!("T{}", idx),
                r#trait,
                arg.optional,
                accepts_multiple,
                config,
            ))
        }
        ArgType::Pattern => {
            let idx = *key_id;
            *key_id += 1;

            let name = to_snake(&arg.name);

            let r#trait = "ToRedisArgs".to_string();

            Some(Argument::new_generic(
                name,
                format!("K{}", idx),
                r#trait,
                arg.optional,
                accepts_multiple,
                config,
            ))
        }
        // Creates Tuple arguments
        // TODO: For now we do not allow nested block arguments
        ArgType::Block {
            arguments: ref sub_arguments,
        } => {
            let idx = *value_id;
            *value_id += 1;

            let name = to_snake(&arg.name);

            // Generate Arguments for each sub_argument
            // Returns None if there are nested block arguments, for now.
            let sub_arguments = sub_arguments
                .iter()
                .map(|arg| {
                    if matches!(&arg.r#type, &ArgType::Block { arguments: _ }) {
                        return None;
                    }
                    map_argument(indices, arg, config)
                })
                .collect::<Option<Vec<_>>>();

            if let Some(sub_arguments) = sub_arguments {
                // Create traits for the tuples
                Some(Argument::new_block(
                    name,
                    &sub_arguments,
                    arg.optional,
                    accepts_multiple,
                    config,
                ))
            } else {
                let r#trait = "ToRedisArgs".to_string();

                Some(Argument::new_generic(
                    name,
                    format!("T{}", idx),
                    r#trait,
                    arg.optional,
                    accepts_multiple,
                    config,
                ))
            }
        }

        _ => None,
    }
}

fn build_docs(command: &str, definition: &CommandDefinition, kind: GenerationKind) -> Vec<String> {
    let mut docs = vec![
        command.to_string(),
        String::new(),
        definition.summary.clone(),
        String::new(),
        format!("Since: Redis {}", definition.since),
        format!("Group: {}", definition.group),
    ];
    if kind == GenerationKind::IgnoreMultiple {
        // TODO improve wording here
        docs[0].push_str(" (Sliceless caller)")
    }

    if let Some(replaced_by) = &definition.replaced_by {
        docs.push(format!("Replaced By: {}", replaced_by))
    }

    if let Some(complexity) = &definition.complexity {
        docs.push(format!("Complexity: {}", complexity))
    }

    if let Some(replaced_by) = &definition.replaced_by {
        docs.push(format!("Replaced By: {}", replaced_by))
    }

    if !definition.command_flags.is_empty() {
        docs.push("CommandFlags:".to_string());
        for command_flag in &definition.command_flags {
            docs.push(format!("* {}", command_flag));
        }
    }

    if !definition.acl_categories.is_empty() {
        docs.push("ACL Categories:".to_string());
        for acl_category in &definition.acl_categories {
            docs.push(format!("* {}", acl_category));
        }
    }

    docs
}
