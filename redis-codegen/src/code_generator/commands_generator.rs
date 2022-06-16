
use itertools::Itertools;

use super::{
    commands::Command,
    constants::{append_constant_docs, COMMAND_TRAIT_DOCS},
    Generator,
};
pub(crate) struct CommandsTrait {
    lifetimes: Vec<String>,
}

impl Default for CommandsTrait {
    fn default() -> Self {
        Self {
            lifetimes: vec!["\'a".to_owned()],
        }
    }
}

impl Generator for CommandsTrait {
    fn append_imports(&self, generator: &mut super::CodeGenerator) {
        generator.buf.push_str("use crate::cmd::{Cmd, Iter};\n")
    }

    fn append_preface(&self, generator: &mut super::CodeGenerator) {
        append_constant_docs(COMMAND_TRAIT_DOCS, generator);
        generator.push_line("pub trait Commands : ConnectionLike + Sized {");
    }

    fn append_appendix(&self, generator: &mut super::CodeGenerator) {
        generator.push_line("}")
    }

    fn append_command(&self, generator: &mut super::CodeGenerator, command: &Command) {
        log::debug!("Command: {:?}", command.fn_name());
        // Use the generic default one.
        generator.append_doc(command);
        generator.append_fn_attributes(command);

        self.append_fn_decl(generator, command);
        generator.depth += 1;

        self.append_fn_body(generator, command);

        generator.depth -= 1;
        generator.push_line("}");
    }
}

impl CommandsTrait {
    // Generates:
    // ```
    /// fn $name<$lifetime, $($tyargs: $ty, )* RV: FromRedisValue>(
    //     &mut self $(, $argname: $argty)*) -> RedisResult<RV> {
    // ```
    fn append_fn_decl(&self, generator: &mut super::CodeGenerator, command: &Command) {
        let mut trait_bounds = vec![];
        let mut args = vec!["&mut self".to_owned()];
        let mut needs_lifetime = false;

        for arg in command.arguments() {
            needs_lifetime |= arg.multiple;
            trait_bounds.push(arg.trait_bound());
            args.push(arg.to_string())
        }

        if needs_lifetime {
            trait_bounds.insert(0, Some("\'a".to_owned()));
        }

        trait_bounds.push(Some("RV: FromRedisValue".to_owned()));
        let trait_bounds = trait_bounds
            .iter()
            .filter_map(|x| x.as_ref())
            .map(|x| x.as_str())
            .collect::<Vec<_>>();

        let command_name = command.fn_name();
        let trait_bounds = if trait_bounds.is_empty() {
            String::new()
        } else {
            format!("<{}>", trait_bounds.join(", "))
        };

        generator.push_line(&format!(
            "fn {command_name}{trait_bounds}({}) -> RedisResult<RV> {{",
            args.join(", ")
        ));
    }

    /// Appends the function body. Generates:
    /// ```
    /// Cmd::$name($($argname),*).query(self)
    /// ```
    fn append_fn_body(&self, generator: &mut super::CodeGenerator, command: &Command) {
        generator.push_line(&format!(
            "Cmd::{}({}).query(self)",
            command.fn_name(),
            command.arguments().map(|arg| &arg.name).join(", ")
        ));
    }
}
