use crate::commands::{CommandDefinition, CommandSet};
use crate::feature_gates::FeatureGate;
use crate::GenerationType;
use async_commands_generator::AsyncCommandsTrait;
use cluster_pipeline_generator::ClusterPipelineImpl;
use command_generator::CommandImpl;
use commands::Command;
use commands_generator::CommandsTrait;
use comment::Comment;
use itertools::Itertools;
use pipeline_generator::PipelineImpl;
use types::TypeGenerator;

mod arguments;
mod async_commands_generator;
mod cluster_pipeline_generator;
mod command_generator;
mod commands;
mod commands_generator;
mod comment;
mod constants;
mod pipeline_generator;
mod types;

pub static BLACKLIST: &[&str] = &["SCAN", "HSCAN", "SSCAN", "ZSCAN", "CLIENT KILL", "OBJECT"];
pub static COMMAND_NAME_OVERWRITE: &[(&str, &str)] = &[("MOVE", "move_key")];
pub static COMMAND_COMPATIBILITY: &[(&str, &str)] = &[
    ("GETDEL", "get_del"),
    ("GETEX", "get_ex"),
    ("ZREMRANGEBYLEX", "zrembylex"),
];

pub struct CodeGenerator<'a> {
    depth: u8,
    buf: &'a mut String,
}

fn push_indent(buf: &mut String, depth: u8) {
    for _ in 0..depth {
        buf.push_str("    ");
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenerationKind {
    Full,
    IgnoreMultiple,
}

#[derive(Debug)]
pub(crate) struct GenerationConfig<'a> {
    pub(crate) explicit_lifetime: bool,
    pub(crate) kind: GenerationKind,
    pub(crate) type_registry: &'a types::TypeRegistry,
}

pub(crate) trait Generator {
    fn generate(&self, generator: &mut CodeGenerator, commands: &[(&str, &CommandDefinition)]);
}

impl<'a> CodeGenerator<'a> {
    pub(crate) fn generate_types(
        commands: &CommandSet,
        buf: &mut String,
        fully_qualified_path_prefix: String,
    ) -> types::TypeRegistry {
        let mut code_gen = CodeGenerator { depth: 0, buf };

        let commands = commands
            .iter()
            .sorted_by(|x, y| Ord::cmp(&x.1.group, &y.1.group).then(Ord::cmp(&x.0, &y.0)))
            .map(|(name, def)| (name.as_str(), def))
            .collect::<Vec<_>>();
        let generator = TypeGenerator::new();
        generator.generate(&mut code_gen, &commands, fully_qualified_path_prefix)
    }

    pub(crate) fn generate(
        generation_type: GenerationType,
        commands: &CommandSet,
        buf: &mut String,
        type_registry: &types::TypeRegistry,
    ) {
        let mut code_gen = CodeGenerator { depth: 0, buf };

        let config = GenerationConfig {
            explicit_lifetime: false,
            kind: GenerationKind::Full,
            type_registry,
        };

        let generation_unit: Box<dyn Generator> = match generation_type {
            GenerationType::CommandsTrait => Box::new(CommandsTrait::new(&config)),
            GenerationType::CommandImpl => Box::new(CommandImpl::new(&config)),
            GenerationType::AsyncCommandsTrait => Box::new(AsyncCommandsTrait::new(&config)),
            GenerationType::Pipeline => Box::new(PipelineImpl::new(&config)),
            GenerationType::ClusterPipeline => Box::new(ClusterPipelineImpl::new(&config)),
        };

        let commands = commands
            .iter()
            .sorted_by(|x, y| Ord::cmp(&x.1.group, &y.1.group).then(Ord::cmp(&x.0, &y.0)))
            .map(|(name, def)| (name.as_str(), def))
            .collect::<Vec<_>>();

        generation_unit.generate(&mut code_gen, &commands);
    }

    pub fn push_indent(&mut self) {
        push_indent(self.buf, self.depth);
    }

    pub(crate) fn push_line(&mut self, line: &str) {
        self.push_indent();
        self.buf.push_str(line);
        self.buf.push('\n')
    }

    fn append_doc(&mut self, command: &Command) {
        let docs = command.docs().to_owned();
        let doc_comment = Comment(docs);
        doc_comment.append_with_indent(self.depth, self.buf, Default::default());
    }
    fn append_fn_attributes(&mut self, command: &Command) {
        self.append_feature_gate(command);
        if command.deprecated {
            if let Some(since) = &command.deprecated_since {
                self.push_line(&format!(
                    "#[deprecated = \"Deprecated in redis since redis version {}.\"]",
                    since
                ));
            } else {
                self.push_line("#[deprecated = \"Deprecated in redis itself.\"]");
            }
        }
    }

    fn append_feature_gate(&mut self, command: &Command) {
        let group = command.group();
        let command = command.command();

        if let Some(feature) = group.to_feature().or_else(|| command.to_feature()) {
            self.push_indent();
            self.buf
                .push_str(&format!("#[cfg(feature = \"{}\")]\n", feature));
            self.push_indent();
            self.buf.push_str(&format!(
                "#[cfg_attr(docsrs, doc(cfg(feature = \"{}\")))]\n",
                feature
            ));
        }
    }
}
