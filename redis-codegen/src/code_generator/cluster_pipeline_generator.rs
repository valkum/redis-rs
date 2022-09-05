use super::{
    commands::Command,
    constants::{append_constant_docs, CLUSTER_PIPELINE_DOCS},
    GenerationConfig, Generator,
};
use crate::commands::CommandDefinition;
use itertools::Itertools;

pub(crate) struct ClusterPipelineImpl<'a> {
    pub(crate) config: &'a GenerationConfig<'a>,
}
impl<'a> ClusterPipelineImpl<'a> {
    pub fn new(config: &'a GenerationConfig) -> Self {
        Self { config }
    }
}

impl Generator for ClusterPipelineImpl<'_> {
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
        }
        generator.depth -= 1;
        generator.push_line("}")
    }
}

// $(#[$attr])*
// #[inline]
// #[allow(clippy::extra_unused_lifetimes, clippy::needless_lifetimes)]
// pub fn $name<$lifetime, $($tyargs: $ty),*>(
//     &mut self $(, $argname: $argty)*
// ) -> &mut Self {
//     self.add_command(::std::mem::replace($body, Cmd::new()))
// }

impl ClusterPipelineImpl<'_> {
    fn append_imports(&self, generator: &mut super::CodeGenerator) {
        generator.push_line("#![cfg_attr(rustfmt, rustfmt_skip)]");
        generator.push_line("#[cfg(feature = \"cluster\")]");
        generator.push_line("use crate::cluster_pipeline::ClusterPipeline;");
        generator.push_line("use crate::cmd::Cmd;");
        generator.push_line("use crate::types::ToRedisArgs;");
    }

    fn append_preface(&self, generator: &mut super::CodeGenerator) {
        append_constant_docs(CLUSTER_PIPELINE_DOCS, generator);
        generator.push_line("#[cfg(feature = \"cluster\")]");
        generator.push_line("impl ClusterPipeline {");
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

    // Generates:
    // ```

    // pub fn $name<$lifetime, $($tyargs: $ty),*>(
    //     &mut self $(, $argname: $argty)*
    // ) -> &mut Self {
    // ```
    fn append_fn_decl(&self, generator: &mut super::CodeGenerator, command: &Command) {
        let mut trait_bounds = vec![];
        let mut args = vec!["&mut self".to_owned()];

        for arg in command.arguments() {
            trait_bounds.push(arg.trait_bound());
            args.push(arg.to_string())
        }

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
            "pub fn {command_name}{trait_bounds}({}) -> &mut Self {{",
            args.join(", ")
        ));
    }

    /// Appends the function body. Generates:
    /// ```
    /// self.add_command(::std::mem::replace($body, Cmd::new()))
    /// ```
    fn append_fn_body(&self, generator: &mut super::CodeGenerator, command: &Command) {
        generator.push_line(&format!(
            "self.add_command(Cmd::{}({}))",
            command.fn_name(),
            command.arguments().map(|arg| &arg.name).join(", ")
        ));
    }
}
