use anyhow::Result;
use code_generator::CodeGenerator;
use commands::CommandSet;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufReader},
    path::{Path, PathBuf},
};

mod code_generator;
mod commands;
mod feature_gates;
mod ident;

pub fn generate_commands(spec: impl AsRef<Path>, out_dir: Option<impl AsRef<Path>>) -> Result<()> {
    let out_dir = if let Some(out_dir) = out_dir.as_ref() {
        out_dir.as_ref().to_owned()
    } else {
        PathBuf::from(std::env::var("OUT_DIR").unwrap())
    };

    compile(spec, out_dir)?;
    Ok(())
}

fn compile(spec: impl AsRef<Path>, out_dir: PathBuf) -> Result<()> {
    let f = File::open(spec)?;
    let reader = BufReader::new(f);

    let deserializer = &mut serde_json::Deserializer::from_reader(reader);

    let command_set: CommandSet = serde_path_to_error::deserialize(deserializer)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

    let modules = generate_impls(command_set)?;
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
    Tokens,
}

fn generate_impls(command_set: CommandSet) -> Result<HashMap<Module, String>> {
    let mut modules = HashMap::new();

    // For each GenerationType call the appropiate generator
    for module_type in [
        GenerationType::CommandsTrait,
        GenerationType::CommandImpl,
        GenerationType::AsyncCommandsTrait,
        GenerationType::Pipeline,
        GenerationType::ClusterPipeline,
        GenerationType::Tokens,
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
            GenerationType::Tokens => Module {
                name: "tokens".to_owned(),
                feature_flag: None,
            },
        };
        let buf = modules.entry(module).or_insert_with(String::new);
        CodeGenerator::generate(module_type, &command_set, buf);
    }
    Ok(modules)
}
