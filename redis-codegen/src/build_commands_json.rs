use crate::commands::{
    AclCategory, ArgType, Arity, CommandArgument, CommandDefinition, CommandFlag,
};
use anyhow::{bail, Context, Result};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// Retrieves the redis commands and docs json in redis server format from the given redis instance
pub fn retrieve_json(cli: String, host: String, port: String) -> Result<(Vec<u8>, Vec<u8>)> {
    let json_output = std::process::Command::new(cli.clone())
        .args(["-h", &host, "-p", &port, "--json", "command"])
        .output()?;
    let json_docs_output = std::process::Command::new(cli)
        .args(["-h", &host, "-p", &port, "--json", "command", "docs"])
        .output()?;

    if json_output.status.success() && json_docs_output.status.success() {
        return Ok((json_output.stdout, json_docs_output.stdout));
    }

    bail!("Failed to get json output from redis-cli")
}

/// Builds a map of command name to `CommandDefinition` like redis `generate-commands-json.py`
pub fn built_commands_json(
    commands: Vec<u8>,
    docs: Vec<u8>,
) -> Result<HashMap<String, CommandDefinition>> {
    let commands: Vec<serde_json::Value> = serde_json::from_slice(&commands)?;
    let docs: HashMap<String, serde_json::Value> = serde_json::from_slice(&docs)?;

    let mut commands_json = HashMap::<String, CommandDefinition>::new();
    for entry in commands {
        let cmd = entry.as_array().expect("json array");
        let command_name = cmd[0].as_str().expect("command name");
        let (name, obj) =
            map_command_doc_entry(cmd, docs.get(command_name).expect("docs for command"))
                .with_context(|| format!("generate json for cmd: {}", command_name))?;
        commands_json.insert(name, obj);
    }

    Ok(commands_json)
}

/// Maps cmd `serde_json::Value` and respective docs into command name and `CommandDefinition`
fn map_command_doc_entry(
    cmd: &[serde_json::Value],
    docs: &serde_json::Value,
) -> Result<(String, CommandDefinition)> {
    let name = cmd[0].as_str().expect("name").to_uppercase();
    let mut owned_docs = docs.to_owned();
    let docs = owned_docs.as_object_mut().context("docs object")?;

    let arity: Arity = serde_json::from_value(cmd[1].clone()).context("arity")?;
    let command_flags: Vec<CommandFlag> =
        serde_json::from_value(cmd[2].clone()).context("command_flags")?;
    let acl_categories: Vec<AclCategory> =
        serde_json::from_value(cmd[6].clone()).context("acl_categories")?;
    let hints = serde_json::from_value(cmd[7].clone()).context("hints")?;

    // TODO handling subcommands
    let _subcommands = if cmd.len() > 9 {
        Some(cmd[9].clone())
    } else {
        None
    };
    let key = name.replace('|', " ");

    let (_, _subcommand_docs) = docs
        .remove_entry("subcommands")
        .unwrap_or_else(|| (String::new(), serde_json::Value::Array(vec![])));

    let arguments = docs
        .remove_entry("arguments")
        .map(|x| x.1)
        .and_then(|x| x.as_array().cloned())
        .unwrap_or_default();

    let obj = CommandDefinition {
        summary: extract("summary", docs)?,
        since: extract("since", docs)?,
        group: extract("group", docs)?,
        complexity: extract_or_none("complexity", docs)?,
        deprecated_since: extract_or_none("deprecated_since", docs)?,
        replaced_by: extract_or_none("replaced_by", docs)?,
        history: extract_or_none("history", docs)?.unwrap_or_default(),
        acl_categories,
        arity,
        arguments: arguments
            .iter()
            .map(convert_argument)
            .collect::<Result<Vec<_>>>()?,
        command_flags,
        doc_flags: extract_or_none("doc_flags", docs)?.unwrap_or_default(),
        hints,
    };

    Ok((key, obj))
}

/// Extracts and converts `field` rom `serde_json::Value` into `T`s
fn extract<T: DeserializeOwned>(
    field: &str,
    docs: &mut serde_json::Map<String, serde_json::Value>,
) -> Result<T> {
    let entry = docs
        .remove_entry(field)
        .with_context(|| format!("docs entry for {}", field))?;
    serde_json::from_value(entry.1).with_context(|| format!("docs entry for {}", field))
}

/// Extracts and converts `field` rom `serde_json::Value` into `Some(T)` or `None` if `field` is not present in `docs`
fn extract_or_none<T: DeserializeOwned>(
    field: &str,
    docs: &mut serde_json::Map<String, serde_json::Value>,
) -> Result<Option<T>> {
    if let Some(entry) = docs.remove_entry(field) {
        return serde_json::from_value(entry.1)
            .with_context(|| format!("docs entry for {}", field))
            .map(|value| Some(value));
    }
    Ok(None)
}

/// Converts `serde_json::Value` of an argument into `CommandArgument`
fn convert_argument(value: &serde_json::Value) -> Result<CommandArgument> {
    let map = value.as_object().context("argument object")?;

    let arguments = map
        .get("arguments")
        .and_then(|x| x.as_array())
        .map(|x| x.iter().map(convert_argument).collect::<Result<Vec<_>>>())
        .transpose()?;

    let arg = CommandArgument {
        name: map
            .get("name")
            .and_then(|x| x.as_str())
            .map(|x| x.to_owned())
            .context("no name for argument")?,
        r#type: (map.get("type").context("argument name")?, arguments).try_into()?,
        token: map
            .get("token")
            .and_then(|x| x.as_str())
            .map(|x| x.to_owned()),
        multiple: map
            .get("multiple")
            .and_then(|x| x.as_bool())
            .unwrap_or_default(),
        optional: map
            .get("optional")
            .and_then(|x| x.as_bool())
            .unwrap_or_default(),
    };

    Ok(arg)
}

impl TryFrom<(&serde_json::Value, Option<Vec<CommandArgument>>)> for ArgType {
    type Error = anyhow::Error;

    fn try_from(
        (value, args): (&serde_json::Value, Option<Vec<CommandArgument>>),
    ) -> Result<Self, Self::Error> {
        if let Some(value) = value.as_str() {
            let r#type = match value {
                "string" => Self::String,
                "integer" => Self::Integer,
                "double" => Self::Double,
                "key" => Self::Key,
                "pattern" => Self::Pattern,
                "unix-time" => Self::UnixTime,
                "pure-token" => Self::PureToken,
                "oneof" => Self::Oneof {
                    arguments: args.context("Missing arguments for oneof")?,
                },
                "block" => Self::Block {
                    arguments: args.context("Missing arguments for block")?,
                },
                _ => bail!("unknown argument type"),
            };
            return Ok(r#type);
        }
        bail!("argument type not a string")
    }
}
