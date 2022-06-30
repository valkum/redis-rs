use super::arguments::Argument;
use super::GenerationConfig;
use super::GenerationKind;
use super::COMMAND_NAME_OVERWRITE;
use crate::commands::ArgType;
use crate::commands::DocFlag;
use crate::commands::{CommandArgument, CommandDefinition, CommandGroup};
use crate::ident::to_camel;
use crate::ident::to_snake;
use std::collections::HashMap;

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
        // Collect arguments based on the command definition
        let mut args = definition
            .arguments
            .iter()
            .filter_map(|arg| map_argument(&command, &mut kv_index, arg, config))
            .collect::<Vec<_>>();

        // The previous step can yield arguments with the same identifier. We need to distinguish them thus we refine the collection of arguments.
        let mut dedup_map: HashMap<String, i32> = HashMap::new();
        for arg in args.iter_mut() {
            match dedup_map.entry(arg.name.clone()) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    // This argument ident was already used, lets add a number to the end
                    *entry.get_mut() += 1;
                    arg.name.push_str(&entry.get().to_string());
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(0);
                }
            }
        }

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
    command_name: &str,
    indices: &mut (u8, u8),
    arg: &CommandArgument,
    config: &'a GenerationConfig,
) -> Option<Argument<'a>> {
    let (ref mut key_id, ref mut value_id) = indices;

    let accepts_multiple = arg.multiple && (config.kind == GenerationKind::Full);

    if let Some(token_name) = &arg.token {
        let name = to_snake(&arg.name);
        let token_type_name = to_camel(token_name);
        if let Some(type_name) = config
            .type_registry
            .resolve(&[command_name, &token_type_name])
        {
            return Some(Argument::new_concrete(
                name,
                type_name,
                arg.optional,
                accepts_multiple,
                config,
            ));
        } else {
            eprintln!("Missing type for {command_name}.{name} falling back to generic ToRedisArgs");
        }
    }

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
        ArgType::Block { arguments: _ } => {
            let name = to_snake(&arg.name);
            let type_name = to_camel(&arg.name);
            config
                .type_registry
                .resolve(&[command_name, &type_name])
                .map(|type_name| {
                    Argument::new_concrete(name, type_name, arg.optional, accepts_multiple, config)
                })
        }
        ArgType::Oneof { arguments: _ } => {
            let name = to_snake(&arg.name);
            let type_name = to_camel(&arg.name);
            config
                .type_registry
                .resolve(&[command_name, &type_name])
                .map(|type_name| {
                    Argument::new_concrete(name, type_name, arg.optional, accepts_multiple, config)
                })
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
