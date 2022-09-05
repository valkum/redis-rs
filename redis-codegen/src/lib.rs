use anyhow::Result;
use code_generator::CodeGenerator;
use commands::{CommandDefinitionOverwrite, CommandSet};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufReader},
    path::{Path, PathBuf},
};

mod build_commands_json;
mod code_generator;
mod commands;
mod feature_gates;
mod ident;

pub use build_commands_json::{built_commands_json, retrieve_json};

pub fn generate_commands(
    spec: impl AsRef<Path>,
    overwrite_spec: impl AsRef<Path>,
    out_dir: Option<impl AsRef<Path>>,
    fully_qualified_mount_path: String,
) -> Result<()> {
    let out_dir = if let Some(out_dir) = out_dir.as_ref() {
        out_dir.as_ref().to_owned()
    } else {
        PathBuf::from(std::env::var("OUT_DIR").unwrap())
    };

    compile(spec, overwrite_spec, out_dir, fully_qualified_mount_path)?;
    Ok(())
}

fn deserialize<'a, T>(path: &Path) -> Result<T>
where
    T: Deserialize<'a>,
{
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let deserializer = &mut serde_json::Deserializer::from_reader(reader);

    let deserialized: T = serde_path_to_error::deserialize(deserializer)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

    return Ok(deserialized);
}

fn merge_overwrites(set: &mut CommandSet, overwrites: HashMap<String, CommandDefinitionOverwrite>) {
    for (name, overwrites) in overwrites.iter() {
        if let Some(command) = set.get_mut(name) {
            command.summary = overwrites
                .summary
                .clone()
                .unwrap_or_else(|| command.summary.clone());
            command.since = overwrites
                .since
                .clone()
                .unwrap_or_else(|| command.since.clone());
            command.group = overwrites.group.unwrap_or(command.group);
            command.complexity = overwrites
                .complexity
                .clone()
                .or_else(|| command.complexity.clone());
            command.deprecated_since = overwrites
                .deprecated_since
                .clone()
                .or_else(|| command.deprecated_since.clone());
            command.replaced_by = overwrites
                .replaced_by
                .clone()
                .or_else(|| command.replaced_by.clone());
            command.history = overwrites
                .history
                .clone()
                .unwrap_or_else(|| command.history.clone());
            command.acl_categories = overwrites
                .acl_categories
                .clone()
                .unwrap_or_else(|| command.acl_categories.clone());
            command.arity = overwrites.arity.unwrap_or(command.arity);
            command.arguments = overwrites
                .arguments
                .clone()
                .unwrap_or_else(|| command.arguments.clone());
            command.command_flags = overwrites
                .command_flags
                .clone()
                .unwrap_or_else(|| command.command_flags.clone());
            command.doc_flags = overwrites
                .doc_flags
                .clone()
                .unwrap_or_else(|| command.doc_flags.clone());
            command.hints = overwrites
                .hints
                .clone()
                .unwrap_or_else(|| command.hints.clone());
        }
    }
}

fn compile(
    spec: impl AsRef<Path>,
    overwrite_spec: impl AsRef<Path>,
    out_dir: PathBuf,
    fully_qualified_mount_path: String,
) -> Result<()> {
    let mut command_set: CommandSet = deserialize(spec.as_ref())?;
    let command_overwrites: HashMap<String, CommandDefinitionOverwrite> =
        deserialize(overwrite_spec.as_ref())?;

    merge_overwrites(&mut command_set, command_overwrites);

    let modules = generate_impls(command_set, fully_qualified_mount_path)?;
    for (module, content) in modules {
        let file_name = format!("{}.rs", module.name);

        let output_path = out_dir.join(&file_name);

        let previous_content = fs::read(&output_path);

        if previous_content
            .map(|previous_content| previous_content == content.as_bytes())
            .unwrap_or(false)
        {
            log::trace!("unchanged: {:?}", file_name);
        } else {
            log::trace!("writing: {:?}", file_name);
            fs::write(output_path, content)?;
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub name: String,
    pub feature_flag: Option<String>,
}

impl AsRef<str> for Module {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GenerationType {
    CommandsTrait,
    CommandImpl,
    AsyncCommandsTrait,
    Pipeline,
    ClusterPipeline,
}

fn generate_impls(
    command_set: CommandSet,
    fully_qualified_mount_path: String,
) -> Result<HashMap<Module, String>> {
    let mut modules = HashMap::new();

    let module = Module {
        name: "types".to_owned(),
        feature_flag: None,
    };
    let buf = modules.entry(module).or_insert_with(String::new);
    let type_registry =
        CodeGenerator::generate_types(&command_set, buf, fully_qualified_mount_path);

    // For each GenerationType call the appropiate generator
    for module_type in [
        GenerationType::CommandsTrait,
        GenerationType::CommandImpl,
        GenerationType::AsyncCommandsTrait,
        GenerationType::Pipeline,
        GenerationType::ClusterPipeline,
    ] {
        let module = match module_type {
            GenerationType::CommandsTrait => Module {
                name: "commands".to_owned(),
                feature_flag: None,
            },
            GenerationType::CommandImpl => Module {
                name: "command".to_owned(),
                feature_flag: None,
            },
            GenerationType::AsyncCommandsTrait => Module {
                name: "async_commands".to_owned(),
                feature_flag: Some("aio".to_owned()),
            },
            GenerationType::Pipeline => Module {
                name: "pipeline".to_owned(),
                feature_flag: None,
            },
            GenerationType::ClusterPipeline => Module {
                name: "cluster_pipeline".to_owned(),
                feature_flag: Some("cluster".to_owned()),
            },
        };
        let buf = modules.entry(module).or_insert_with(String::new);
        CodeGenerator::generate(module_type, &command_set, buf, &type_registry);
    }
    Ok(modules)
}
