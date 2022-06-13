use super::arguments::Argument;
use super::GenerationKind;
use super::COMMAND_NAME_OVERWRITE;
use crate::commands::ArgType;
use crate::commands::{CommandArgument, CommandDefinition, CommandGroup};
use crate::ident::to_snake;

/// An abstract type representing a code generation unit for a command
pub(crate) struct Command {
    name: String,
    command: String,
    docs: Vec<String>,
    group: CommandGroup,
    args: Vec<Argument>,
}

impl Command {
    pub(crate) fn new(mut name: String, definition: &CommandDefinition, kind: GenerationKind) -> Self {
        let command = name.clone();

        let mut kv_index: (u8, u8) = (0, 0);
        let args = definition
            .arguments
            .iter()
            .filter_map(|x| map_argument(&mut kv_index, x, kind))
            .collect::<Vec<_>>();

        let docs = build_docs(&name, definition, kind);

        name = if let Some(&(_, name)) = COMMAND_NAME_OVERWRITE
            .iter()
            .find(|(name, _)| name == name)
        {
            name.to_owned()
        } else {
            to_snake(&name)
        };

        if kind == GenerationKind::IgnoreMultiple {
            name = name + "_single";
        }

        Self {
            name,
            command,
            docs,
            group: definition.group,
            args,
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
fn map_argument(
    (key_id, value_id): &mut (u8, u8),
    arg: &CommandArgument,
    kind: GenerationKind,
) -> Option<Argument> {
    // TODO Ignore argument until we have proper token handling.
    // We propably want to generate a type for each token that implements ToRedisArgs and expands to the wrapped type and the Token
    if arg.token.is_some() {
        return None;
    }

    let accecpts_multple = arg.multiple && (kind == GenerationKind::Full);

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
                accecpts_multple,
            ))
        }
        ArgType::String | ArgType::Integer | ArgType::Double => {
            let idx = *value_id;
            *value_id += 1;

            let name = to_snake(&arg.name);

            // ToRedis is implemented for Vec this it currently does not make much sense to specialize the trait bound for multiple.
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
                accecpts_multple,
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
                accecpts_multple,
            ))
        }
        // These should be tuples. ToRedisArgs should take care of it
        ArgType::Block { arguments: _ } => {
            let idx = *value_id;
            *value_id += 1;

            let name = to_snake(&arg.name);

            let r#trait = "ToRedisArgs".to_string();

            Some(Argument::new_generic(
                name,
                format!("T{}", idx),
                r#trait,
                arg.optional,
                accecpts_multple,
            ))
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
