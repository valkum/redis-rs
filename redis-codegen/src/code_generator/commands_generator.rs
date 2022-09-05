use super::{
    commands::Command,
    comment::Comment,
    constants::{append_constant_docs, COMMAND_TRAIT_DOCS},
    GenerationConfig, Generator,
};
use crate::commands::CommandDefinition;
use itertools::Itertools;
pub(crate) struct CommandsTrait<'a> {
    pub(crate) config: &'a GenerationConfig<'a>,
}

impl<'a> CommandsTrait<'a> {
    pub fn new(config: &'a GenerationConfig) -> Self {
        Self { config }
    }
}

impl Generator for CommandsTrait<'_> {
    fn generate(
        &self,
        generator: &mut super::CodeGenerator,
        commands: &[(&str, &CommandDefinition)],
    ) {
        generator.append_generated_file_header();
        self.append_imports(generator);
        generator.buf.push('\n');
        self.append_preface(generator);

        generator.depth += 1;
        for &(command_name, definition) in commands {
            let command = Command::new(command_name.to_owned(), definition, self.config);
            if !super::BLACKLIST.contains(&command_name) {
                self.append_command(generator, &command);
                generator.buf.push('\n')
            }

            if let Some(backwarts_compatible_name) = super::COMMAND_COMPATIBILITY
                .iter()
                .find(|(name, _)| *name == command_name)
            {
                self.append_alias_command(generator, &command, backwarts_compatible_name.1);
                generator.buf.push('\n')
            }
        }
        generator.depth -= 1;
        generator.push_line("}")
    }
}

impl CommandsTrait<'_> {
    fn append_imports(&self, generator: &mut super::CodeGenerator) {
        generator.push_line("#![cfg_attr(rustfmt, rustfmt_skip)]");
        generator.push_line("#[allow(deprecated)]");
        generator.push_line("use crate::connection::ConnectionLike;");
        generator.push_line("use crate::cmd::Cmd;");
        generator.push_line("use crate::types::{FromRedisValue, RedisResult, ToRedisArgs};");
        generator.push_line("use crate::Iter;");
    }

    fn append_preface(&self, generator: &mut super::CodeGenerator) {
        append_constant_docs(COMMAND_TRAIT_DOCS, generator);
        generator.push_line("pub trait Commands : ConnectionLike + Sized {");
    }

    fn append_command(&self, generator: &mut super::CodeGenerator, command: &Command) {
        log::debug!("Command: {:?}", command.fn_name());
        // Use the generic default one.
        generator.append_doc(command);
        generator.append_fn_attributes(command);

        self.append_fn_decl(generator, command, None);
        generator.depth += 1;

        self.append_fn_body(generator, command);

        generator.depth -= 1;
        generator.push_line("}");
    }

    fn append_alias_command(
        &self,
        generator: &mut super::CodeGenerator,
        command: &Command,
        alias: &str,
    ) {
        let alias_docs = vec![format!("This is an alias for [`{}`]", command.fn_name())];

        let doc_comment = Comment(alias_docs);
        // TODO: Insert redis-rs version when this gets merged
        generator.push_line("#[deprecated(since = \"0.22.0\", note = \"With version 0.22.0 redis crate switched to a generated api. This is a deprecated old handwritten function that now aliases to the generated one and will be removed in a future update. \")]");
        doc_comment.append_with_indent(generator.depth, generator.buf, Default::default());
        self.append_fn_decl(generator, command, Some(alias));

        generator.depth += 1;
        generator.push_line(&format!(
            "self.{}({})",
            command.fn_name(),
            command.arguments().map(|arg| &arg.name).join(", ")
        ));
        generator.depth -= 1;
        generator.push_line("}");
    }

    // Generates:
    // ```
    /// fn $name<$lifetime, $($tyargs: $ty, )* RV: FromRedisValue>(
    //     &mut self $(, $argname: $argty)*) -> RedisResult<RV> {
    // ```
    fn append_fn_decl(
        &self,
        generator: &mut super::CodeGenerator,
        command: &Command,
        name: Option<&str>,
    ) {
        let mut trait_bounds = vec![];
        let mut args = vec!["&mut self".to_owned()];

        for arg in command.arguments() {
            trait_bounds.push(arg.trait_bound());
            args.push(arg.to_string())
        }

        trait_bounds.push(Some("RV: FromRedisValue".to_owned()));
        let trait_bounds = trait_bounds
            .iter()
            .filter_map(|x| x.as_ref())
            .map(|x| x.as_str())
            .collect::<Vec<_>>();

        let command_name = name.unwrap_or_else(|| command.fn_name());
        let trait_bounds = if trait_bounds.is_empty() {
            String::new()
        } else {
            format!("<{}>", trait_bounds.join(", "))
        };

        if command.cursor {
            generator.push_line(&format!(
                "fn {command_name}{trait_bounds}({}) -> RedisResult<Iter<'_, RV>> {{",
                args.join(", ")
            ));
        } else {
            generator.push_line(&format!(
                "fn {command_name}{trait_bounds}({}) -> RedisResult<RV> {{",
                args.join(", ")
            ));
        }
    }

    /// Appends the function body. Generates:
    /// ```
    /// Cmd::$name($($argname),*).query(self)
    /// ```
    fn append_fn_body(&self, generator: &mut super::CodeGenerator, command: &Command) {
        if command.cursor {
            generator.push_line(&format!(
                "Cmd::{}({}).iter(self)",
                command.fn_name(),
                command.arguments().map(|arg| &arg.name).join(", ")
            ));
        } else {
            generator.push_line(&format!(
                "Cmd::{}({}).query(self)",
                command.fn_name(),
                command.arguments().map(|arg| &arg.name).join(", ")
            ));
        }
    }
}
