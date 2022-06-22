use super::{
    commands::Command,
    constants::{append_constant_docs, ASYNC_COMMAND_TRAIT_DOCS},
    GenerationConfig, Generator,
};
use crate::commands::CommandDefinition;

pub(crate) struct AsyncCommandsTrait {
    lifetime: String,
    pub(crate) config: GenerationConfig,
}

impl AsyncCommandsTrait {
    pub fn new(config: GenerationConfig) -> Self {
        Self {
            lifetime: "\'a".to_owned(),
            config,
        }
    }
}

impl Generator for AsyncCommandsTrait {
    fn generate(
        &self,
        generator: &mut super::CodeGenerator,
        commands: &[(&str, &CommandDefinition)],
    ) {
        self.append_imports(generator);
        generator.buf.push('\n');
        self.append_preface(generator);

        generator.depth += 1;
        for &(command_name, definition) in commands {
            let command = Command::new(command_name.to_owned(), definition, &self.config);
            if !super::BLACKLIST.contains(&command_name) {
                self.append_command(generator, &command);
                generator.buf.push('\n')
            }
        }
        generator.depth -= 1;
        generator.push_line("}")
    }
}

impl AsyncCommandsTrait {
    fn append_imports(&self, generator: &mut super::CodeGenerator) {
        generator.push_line("#![cfg_attr(rustfmt, rustfmt_skip)]");
        generator.push_line("use crate::cmd::{Cmd, Iter};");
        generator.push_line("use crate::types::ToRedisArgs;");
    }

    fn append_preface(&self, generator: &mut super::CodeGenerator) {
        append_constant_docs(ASYNC_COMMAND_TRAIT_DOCS, generator);
        generator.push_line("#[cfg(feature = \"aio\")]");
        generator
            .push_line("pub trait AsyncCommands : crate::aio::ConnectionLike + Send + Sized {");
    }

    fn append_appendix(&self, generator: &mut super::CodeGenerator) {}

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

    // Generates:
    // ```

    // fn $name<$lifetime, $($tyargs: $ty + Send + Sync + $lifetime,)* RV>(
    //     & $lifetime mut self
    //     $(, $argname: $argty)*
    // ) -> crate::types::RedisFuture<'a, RV>
    // where
    //     RV: FromRedisValue,
    // {
    // ```
    fn append_fn_decl(&self, generator: &mut super::CodeGenerator, command: &Command) {
        let mut trait_bounds = vec![];
        let mut args = vec![];

        for arg in command.arguments() {
            trait_bounds.push(arg.trait_bound());
            args.push(arg.to_string())
        }

        let additional_traits = format!(" + Send + Sync + {}", self.lifetime);
        let mut trait_bounds = trait_bounds
            .iter()
            .filter_map(|x| x.as_ref())
            .map(|x| {
                let mut x = x.clone();
                x.push_str(&additional_traits);
                x
            })
            .collect::<Vec<_>>();
        trait_bounds.insert(0, self.lifetime.clone());

        let command_name = command.fn_name();
        let trait_bounds = if trait_bounds.is_empty() {
            String::new()
        } else {
            format!("<{}>", trait_bounds.join(", "))
        };

        generator.push_line(&format!(
            "fn {command_name}{trait_bounds}({}) -> Self {{",
            args.join(", ")
        ));
    }

    /// Appends the function body. Generates:
    /// ```
    /// Box::pin(async move { ($body).query_async(self).await })
    /// ```
    fn append_fn_body(&self, generator: &mut super::CodeGenerator, command: &Command) {
        generator.push_line("Box::pin(async move {");

        generator.depth += 1;
        generator.push_line("let mut rv = Cmd::new();");
        generator.push_line(&format!("rv.arg(\"{}\");", command.command()));
        for arg in command.arguments() {
            generator.push_line(&format!("rv.arg({});", arg.name));
        }
        generator.push_line("rv.query_async(self).await");
        generator.depth -= 1;

        generator.push_line("})");
    }
}
