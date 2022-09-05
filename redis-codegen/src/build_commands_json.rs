use crate::commands::{
    AclCategory, ArgType, Arity, CommandArgument, CommandDefinition, CommandFlag,
};
use anyhow::{bail, Context, Result};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;

/// Internal rough struct representation of a doc entry.
/// Used to avoid deserializing into a HashMap<String, serde_json::Value>
#[derive(Deserialize, Debug)]
struct DocEntry {
    summary: String,
    since: String,
    group: String,
    deprecated_since: Option<String>,
    replaced_by: Option<String>,
    complexity: Option<String>,
    #[serde(default)]
    subcommands: HashMap<String, Box<DocEntry>>,
    #[serde(default)]
    arguments: Vec<serde_json::Value>,
    #[serde(default)]
    history: Vec<Vec<String>>,
    #[serde(default)]
    doc_flags: Vec<String>,
}

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
///
/// The commands output is parsed into a Vec<serde_json::Value> as it mostly consists of list of lists.
/// The docs are parsed into a HashMap<String, DocEntry> for ease of use.
pub fn built_commands_json(
    commands: Vec<u8>,
    docs: Vec<u8>,
) -> Result<HashMap<String, CommandDefinition>> {
    let commands: Vec<serde_json::Value> = serde_json::from_slice(&commands)?;
    let docs: HashMap<String, DocEntry> = serde_json::from_slice(&docs)?;

    let mut commands_json = HashMap::<String, CommandDefinition>::new();
    for entry in commands {
        let cmd = entry.as_array().expect("json array");
        let command_name = cmd[0].as_str().expect("command name");
        let docs = docs.get(command_name);

        let commands = map_command_doc_entries(cmd, docs.expect("docs for command"))
            .with_context(|| format!("generate json for cmd: {}", command_name))?;
        commands_json.extend(commands);
    }

    Ok(commands_json)
}

/// Maps cmd `serde_json::Value` and respective docs into command name and `CommandDefinition`
fn map_command_doc_entries(
    cmd: &[serde_json::Value],
    docs: &DocEntry,
) -> Result<Vec<(String, CommandDefinition)>> {
    // Extract basic fields from the array based response based on the index.
    let name = cmd[0].as_str().expect("name").to_uppercase();

    let arity: Arity = serde_json::from_value(cmd[1].clone()).context("arity")?;
    let command_flags: Vec<CommandFlag> =
        serde_json::from_value(cmd[2].clone()).context("command_flags")?;
    let acl_categories: Vec<AclCategory> =
        serde_json::from_value(cmd[6].clone()).context("acl_categories")?;
    let hints = serde_json::from_value(cmd[7].clone()).context("hints")?;
    let subcommands = if cmd.len() > 9 {
        Some(cmd[9].clone())
    } else {
        None
    };
    let key = name.replace('|', " ");

    let obj = CommandDefinition {
        summary: docs.summary.clone(),
        since: docs.since.clone().into(),
        group: FromStr::from_str(&docs.group).context("parsing group")?,
        complexity: docs
            .complexity
            .as_ref()
            .map(|x| FromStr::from_str(x))
            .transpose()
            .context("parsing complexity")?,
        deprecated_since: docs.deprecated_since.as_ref().map(|x| x.to_owned().into()),
        replaced_by: docs
            .replaced_by
            .as_ref()
            .map(|x| FromStr::from_str(x))
            .transpose()
            .context("parsing replaced_by")?,
        history: docs
            .history
            .iter()
            .map(|x| {
                assert!(x.len() == 2);
                (x[0].to_owned(), x[1].to_owned()).into()
            })
            .collect(),
        acl_categories,
        arity,
        arguments: docs
            .arguments
            .iter()
            .map(convert_argument)
            .collect::<Result<Vec<_>>>()
            .context("parsing arguments")?,
        command_flags,
        doc_flags: docs
            .doc_flags
            .iter()
            .map(|x| FromStr::from_str(x))
            .collect::<Result<Vec<_>>>()
            .context("parsing doc_flags")?,
        hints,
    };
    let mut result = vec![(key, obj)];

    // Process subcommands
    if let Some(subcommands) = subcommands {
        for subcommand in subcommands
            .as_array()
            .expect("subcommands must be an array")
        {
            let subcommand = subcommand.as_array().expect("Subcommand must be an array");
            let subcommand_name = subcommand[0]
                .as_str()
                .expect("subcommand name must be a string");
            let docs = docs
                .subcommands
                .get(subcommand_name)
                .expect("docs for command");

            let commands = map_command_doc_entries(subcommand, docs.as_ref())
                .with_context(|| format!("generate json for cmd: {}", subcommand_name))?;
            result.extend(commands)
        }
    }

    Ok(result)
}

/// Converts `serde_json::Value` of an argument into `CommandArgument`
fn convert_argument(value: &serde_json::Value) -> Result<CommandArgument> {
    let map = value.as_object().context("argument object")?;
    let flags: Vec<&str> = map
        .get("flags")
        .and_then(|x| {
            x.as_array().map(|x| {
                x.iter()
                    .map(|x| x.as_str().expect("flags must be strings"))
                    .collect()
            })
        })
        .unwrap_or_default();

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
        multiple: flags.contains(&"multiple"),
        optional: flags.contains(&"optional"),
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

#[cfg(test)]
mod tests {
    use crate::{
        build_commands_json::DocEntry,
        commands::{
            AclCategory, ArgType, Arity, CommandArgument, CommandDefinition, CommandFlag,
            CommandGroup, DocFlag, History, Version,
        },
    };

    use super::map_command_doc_entries;

    const SADD_JSON: &str = r#"[
        "sadd",
        -3,
        [
          "write",
          "denyoom",
          "fast"
        ],
        1,
        1,
        1,
        [
          "@write",
          "@set",
          "@fast"
        ],
        [],
        [
          {
            "flags": [
              "RW",
              "insert"
            ],
            "begin_search": {
              "type": "index",
              "spec": {
                "index": 1
              }
            },
            "find_keys": {
              "type": "range",
              "spec": {
                "lastkey": 0,
                "keystep": 1,
                "limit": 0
              }
            }
          }
        ],
        []
      ]"#;

    const SADD_JSON_DOCS: &str = r#"{
        "summary": "Add one or more members to a set",
        "since": "1.0.0",
        "group": "set",
        "complexity": "O(1) for each element added, so O(N) to add N elements when the command is called with multiple arguments.",
        "history": [
          [
            "2.4.0",
            "Accepts multiple `member` arguments."
          ]
        ],
        "arguments": [
          {
            "name": "key",
            "type": "key",
            "key_spec_index": 0
          },
          {
            "name": "member",
            "type": "string",
            "flags": [
              "multiple"
            ]
          }
        ]
      }"#;

    #[test]
    fn redis_command_sadd() {
        let input: Vec<serde_json::Value> = serde_json::from_str(SADD_JSON).unwrap();
        let input_docs: DocEntry = serde_json::from_str(SADD_JSON_DOCS).unwrap();

        let result = map_command_doc_entries(&input, &input_docs);
        let target = vec![(
            "SADD".to_owned(),
            CommandDefinition {
                summary: "Add one or more members to a set".to_owned(),
                since: Version::from("1.0.0".to_owned()),
                group: CommandGroup::Set,
                complexity: Some("O(1) for each element added, so O(N) to add N elements when the command is called with multiple arguments.".to_owned()),
                deprecated_since: None,
                replaced_by: None,
                history: vec![History::from(("2.4.0".to_owned(), "Accepts multiple `member` arguments.".to_owned()))],
                acl_categories: vec![AclCategory::Write, AclCategory::Set, AclCategory::Fast],
                arity: Arity::from(-3),
                arguments: vec![
                    CommandArgument{ name: "key".to_owned(), r#type: ArgType::Key, token: None, multiple: false, optional: false },
                    CommandArgument{ name: "member".to_owned(), r#type: ArgType::String, token: None, multiple: true, optional: false }],
                command_flags: vec![CommandFlag::Write, CommandFlag::Denyoom, CommandFlag::Fast],
                doc_flags: vec![],
                hints: vec![]
            }
        )];

        assert_eq!(result.unwrap(), target);
    }

    const XINFO_JSON: &str = r#"[
        "xinfo",
        -2,
        [],
        0,
        0,
        0,
        [
          "@slow"
        ],
        [],
        [],
        [
          [
            "xinfo|help",
            2,
            [
              "loading",
              "stale"
            ],
            0,
            0,
            0,
            [
              "@stream",
              "@slow"
            ],
            [],
            [],
            []
          ],
          [
            "xinfo|groups",
            3,
            [
              "readonly"
            ],
            2,
            2,
            1,
            [
              "@read",
              "@stream",
              "@slow"
            ],
            [],
            [
              {
                "flags": [
                  "RO",
                  "access"
                ],
                "begin_search": {
                  "type": "index",
                  "spec": {
                    "index": 2
                  }
                },
                "find_keys": {
                  "type": "range",
                  "spec": {
                    "lastkey": 0,
                    "keystep": 1,
                    "limit": 0
                  }
                }
              }
            ],
            []
          ],
          [
            "xinfo|stream",
            -3,
            [
              "readonly"
            ],
            2,
            2,
            1,
            [
              "@read",
              "@stream",
              "@slow"
            ],
            [],
            [
              {
                "flags": [
                  "RO",
                  "access"
                ],
                "begin_search": {
                  "type": "index",
                  "spec": {
                    "index": 2
                  }
                },
                "find_keys": {
                  "type": "range",
                  "spec": {
                    "lastkey": 0,
                    "keystep": 1,
                    "limit": 0
                  }
                }
              }
            ],
            []
          ],
          [
            "xinfo|consumers",
            4,
            [
              "readonly"
            ],
            2,
            2,
            1,
            [
              "@read",
              "@stream",
              "@slow"
            ],
            [
              "nondeterministic_output"
            ],
            [
              {
                "flags": [
                  "RO",
                  "access"
                ],
                "begin_search": {
                  "type": "index",
                  "spec": {
                    "index": 2
                  }
                },
                "find_keys": {
                  "type": "range",
                  "spec": {
                    "lastkey": 0,
                    "keystep": 1,
                    "limit": 0
                  }
                }
              }
            ],
            []
          ]
        ]
      ]"#;

    const XINFO_JSON_DOCS: &str = r#"{
        "summary": "A container for stream introspection commands",
        "since": "5.0.0",
        "group": "stream",
        "complexity": "Depends on subcommand.",
        "subcommands": {
          "xinfo|help": {
            "summary": "Show helpful text about the different subcommands",
            "since": "5.0.0",
            "group": "stream",
            "complexity": "O(1)"
          },
          "xinfo|groups": {
            "summary": "List the consumer groups of a stream",
            "since": "5.0.0",
            "group": "stream",
            "complexity": "O(1)",
            "history": [
              [
                "7.0.0",
                "Added the `entries-read` and `lag` fields"
              ]
            ],
            "arguments": [
              {
                "name": "key",
                "type": "key",
                "key_spec_index": 0
              }
            ]
          },
          "xinfo|stream": {
            "summary": "Get information about a stream",
            "since": "5.0.0",
            "group": "stream",
            "complexity": "O(1)",
            "history": [
              [
                "6.0.0",
                "Added the `FULL` modifier."
              ],
              [
                "7.0.0",
                "Added the `max-deleted-entry-id`, `entries-added`, `recorded-first-entry-id`, `entries-read` and `lag` fields"
              ]
            ],
            "arguments": [
              {
                "name": "key",
                "type": "key",
                "key_spec_index": 0
              },
              {
                "name": "full",
                "type": "block",
                "token": "FULL",
                "flags": [
                  "optional"
                ],
                "arguments": [
                  {
                    "name": "count",
                    "type": "integer",
                    "token": "COUNT",
                    "flags": [
                      "optional"
                    ]
                  }
                ]
              }
            ]
          },
          "xinfo|consumers": {
            "summary": "List the consumers in a consumer group",
            "since": "5.0.0",
            "group": "stream",
            "complexity": "O(1)",
            "arguments": [
              {
                "name": "key",
                "type": "key",
                "key_spec_index": 0
              },
              {
                "name": "groupname",
                "type": "string"
              }
            ]
          }
        }
      }"#;

    #[test]
    fn redis_command_xinfo() {
        let input: serde_json::Value = serde_json::from_str(XINFO_JSON).unwrap();
        let input_docs: DocEntry = serde_json::from_str(XINFO_JSON_DOCS).unwrap();

        let result = map_command_doc_entries(input.as_array().unwrap(), &input_docs).unwrap();
        let target = vec![(
            "XINFO".to_owned(),
            CommandDefinition {
                summary: "A container for stream introspection commands".to_owned(),
                since: Version::from("5.0.0".to_owned()),
                group: CommandGroup::Stream,
                complexity: Some("Depends on subcommand.".to_owned()),
                deprecated_since: None,
                replaced_by: None,
                history: vec![],
                acl_categories: vec![AclCategory::Slow],
                arity: Arity::from(-2),
                arguments: vec![],
                command_flags: vec![],
                doc_flags: vec![],
                hints: vec![]
            }
        ),
        (
            "XINFO HELP".to_owned(),
            CommandDefinition {
                summary: "Show helpful text about the different subcommands".to_owned(),
                since: Version::from("5.0.0".to_owned()),
                group: CommandGroup::Stream,
                complexity: Some("O(1)".to_owned()),
                deprecated_since: None,
                replaced_by: None,
                history: vec![],
                acl_categories: vec![AclCategory::Stream, AclCategory::Slow],
                arity: Arity::from(2),
                arguments: vec![],
                command_flags: vec![CommandFlag::Loading, CommandFlag::Stale],
                doc_flags: vec![],
                hints: vec![]
            }
        ),
        (
            "XINFO GROUPS".to_owned(),
            CommandDefinition {
                summary: "List the consumer groups of a stream".to_owned(),
                since: Version::from("5.0.0".to_owned()),
                group: CommandGroup::Stream,
                complexity: Some("O(1)".to_owned()),
                deprecated_since: None,
                replaced_by: None,
                history: vec![History::from(("7.0.0".to_owned(), "Added the `entries-read` and `lag` fields".to_owned()))],
                acl_categories: vec![AclCategory::Read, AclCategory::Stream, AclCategory::Slow],
                arity: Arity::from(3),
                arguments: vec![
                    CommandArgument{ name: "key".to_owned(), r#type: ArgType::Key, token: None, multiple: false, optional: false }
                ],
                command_flags: vec![CommandFlag::Readonly],
                doc_flags: vec![],
                hints: vec![]
            }
        ),
        (
          "XINFO STREAM".to_owned(),
          CommandDefinition {
              summary: "Get information about a stream".to_owned(),
              since: Version::from("5.0.0".to_owned()),
              group: CommandGroup::Stream,
              complexity: Some("O(1)".to_owned()),
              deprecated_since: None,
              replaced_by: None,
              history: vec![History::from(("6.0.0".to_owned(), "Added the `FULL` modifier.".to_owned())), History::from(("7.0.0".to_owned(), "Added the `max-deleted-entry-id`, `entries-added`, `recorded-first-entry-id`, `entries-read` and `lag` fields".to_owned()))],
              acl_categories: vec![AclCategory::Read, AclCategory::Stream, AclCategory::Slow],
              arity: Arity::from(-3),
              arguments: vec![
                CommandArgument{ name: "key".to_owned(), r#type: ArgType::Key, token: None, multiple: false, optional: false },
                CommandArgument{ name: "full".to_owned(),
                  r#type: ArgType::Block{
                    arguments: vec![CommandArgument{ name: "count".to_owned(), r#type: ArgType::Integer, token: Some("COUNT".to_owned()), multiple: false, optional: true }]
                  },
                  token: Some("FULL".to_owned()), multiple: false, optional: true }
              ],
              command_flags: vec![CommandFlag::Readonly],
              doc_flags: vec![],
              hints: vec![]
          }
      ),
      (
        "XINFO CONSUMERS".to_owned(),
        CommandDefinition {
            summary: "List the consumers in a consumer group".to_owned(),
            since: Version::from("5.0.0".to_owned()),
            group: CommandGroup::Stream,
            complexity: Some("O(1)".to_owned()),
            deprecated_since: None,
            replaced_by: None,
            history: vec![],
            acl_categories: vec![AclCategory::Read, AclCategory::Stream, AclCategory::Slow],
            arity: Arity::from(4),
            arguments: vec![
              CommandArgument{ name: "key".to_owned(), r#type: ArgType::Key, token: None, multiple: false, optional: false },
              CommandArgument{ name: "groupname".to_owned(),
                r#type: ArgType::String,
                token: None, multiple: false, optional: false }
            ],
            command_flags: vec![CommandFlag::Readonly],
            doc_flags: vec![],
            hints: vec!["nondeterministic_output".to_owned()]
        }
    )];

        assert_eq!(result, target);
    }
}
