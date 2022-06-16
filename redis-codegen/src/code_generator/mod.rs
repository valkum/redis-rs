use self::constants::{
    ASYNC_COMMAND_TRAIT_DOCS, CLUSTER_PIPELINE_DOCS, COMMAND_TRAIT_DOCS, PIPELINE_DOCS,
};
use crate::commands::DocFlag;
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

mod arguments;
mod async_commands_generator;
mod cluster_pipeline_generator;
mod command_generator;
mod commands;
mod commands_generator;
mod comment;
mod constants;
mod pipeline_generator;

pub static BLACKLIST: &[&str] = &["SCAN", "HSCAN", "SSCAN", "ZSCAN", "CLIENT KILL", "OBJECT"];
pub static COMMAND_NAME_OVERWRITE: &[(&str, &str)] = &[("MOVE", "move_key")];
pub static COMMAND_COMPATIBILITY: &[(&str, &str)] =
    &[("GETDEL", "get_del"), ("ZREMRANGEBYLEX", "zrembylex")];

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

pub(crate) trait Generator {
    fn append_imports(&self, generator: &mut CodeGenerator);
    fn append_preface(&self, generator: &mut CodeGenerator);
    fn append_appendix(&self, generator: &mut CodeGenerator);
    fn append_command(&self, generator: &mut super::CodeGenerator, command: &Command);
}

impl<'a> CodeGenerator<'a> {
    pub fn generate(generation_type: GenerationType, commands: &CommandSet, buf: &mut String) {
        let mut code_gen = CodeGenerator { depth: 0, buf };

        let generation_unit: Box<dyn Generator> = match generation_type {
            GenerationType::CommandsTrait => Box::new(CommandsTrait::default()),
            GenerationType::CommandImpl => Box::new(CommandImpl::default()),
            GenerationType::AsyncCommandsTrait => Box::new(AsyncCommandsTrait::default()),
            GenerationType::Pipeline => Box::new(PipelineImpl {}),
            GenerationType::ClusterPipeline => Box::new(ClusterPipelineImpl {}),
            // This GenerationType is special, as it won't generate commands but the needed tokens for the commands
            GenerationType::Tokens => {
                code_gen.append_oneof_tokens(commands);
                return;
            }
        };

        code_gen.append_general_imports();
        generation_unit.append_imports(&mut code_gen);
        code_gen.buf.push('\n');
        generation_unit.append_preface(&mut code_gen);
        code_gen.depth += 1;

        // Group commands by group and then by command name
        for (command_name, definition) in commands
            .iter()
            .sorted_by(|x, y| Ord::cmp(&x.1.group, &y.1.group).then(Ord::cmp(&x.0, &y.0)))
        {
            let command = Command::new(command_name.clone(), definition, GenerationKind::Full);
            if !BLACKLIST.contains(&command_name.as_str()) {
                generation_unit.append_command(&mut code_gen, &command);
                code_gen.buf.push('\n')
            }

            // We would be better of instead of generating a whole implementation for this by just calling to the full generation, transforming in the fn body.
            // This needs to wait until we get rid of the macro that currently creates the trait impls.
            // if definition.accepts_multiple() {
            //     let command = Command::new(command_name.clone(), definition, GenerationKind::Full);
            //     generation_unit.append_single_alt_command(&mut code_gen,
            //         command.clone(),
            //         definition,
            //     );
            // }

            if let Some(backwarts_compatible_name) = COMMAND_COMPATIBILITY
                .iter()
                .find(|(name, _)| name == command_name)
            {
                generation_unit.append_command(
                    &mut code_gen,
                    &command.backwards_compatibiity(backwarts_compatible_name.1.to_string()),
                );
                code_gen.buf.push('\n')
            }
        }
        code_gen.depth -= 1;
        generation_unit.append_appendix(&mut code_gen);
    }

    pub fn push_indent(&mut self) {
        push_indent(self.buf, self.depth);
    }

    pub(crate) fn push_line(&mut self, line: &str) {
        self.push_indent();
        self.buf.push_str(line);
        self.buf.push('\n')
    }

    fn append_general_imports(&mut self) {
        // General imports
        self.push_line("use crate::types::{FromRedisValue, NumericBehavior, RedisResult, ToRedisArgs, RedisWrite, Expiry};");
        self.push_line("use crate::connection::{Connection, ConnectionLike, Msg};");
    }

    // Todo Improve docs. Add complexity group etc.
    fn append_doc(&mut self, command: &Command) {
        let docs = command.docs().to_owned();
        let doc_comment = Comment(docs);
        doc_comment.append_with_indent(self.depth, self.buf);
    }
    fn append_fn_attributes(&mut self, command: &Command) {
        self.append_feature_gate(command);
        if command.deprecated {
            // Including the version might be hard here, as we want to put the crate version here in which this got deprecated.
            self.push_line("#[deprecated]");
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

    fn append_oneof_tokens(&mut self, commands: &CommandSet) {
        let all_oneof_definitions = commands
            .iter()
            .filter(|(_, definition)| definition.takes_token())
            .map(|(_, definition)| {
                definition
                    .arguments
                    .iter()
                    .filter(|arg| arg.r#type.is_oneof())
            })
            .flatten()
            .collect::<Vec<_>>();
        dbg!(all_oneof_definitions);
    }
}
