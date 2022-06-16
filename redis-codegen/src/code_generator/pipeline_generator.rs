use itertools::Itertools;

use super::{
    commands::Command,
    constants::{append_constant_docs, PIPELINE_DOCS},
    Generator,
};

pub(crate) struct PipelineImpl;

impl Generator for PipelineImpl {
    fn append_imports(&self, generator: &mut super::CodeGenerator) {
        generator.push_line("use crate::pipeline::Pipeline;");
        generator.push_line("use crate::cmd::Cmd;");
    }

    fn append_preface(&self, generator: &mut super::CodeGenerator) {
        append_constant_docs(PIPELINE_DOCS, generator);
        generator.push_line("impl Pipeline {");
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

impl PipelineImpl {
    // Generates:
    // ```

    // pub fn $name<$lifetime, $($tyargs: $ty),*>(
    //     &mut self $(, $argname: $argty)*
    // ) -> &mut Self {
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
        // generator.push_line("let mut rv = Cmd::new();");
        // generator.push_line(&format!("rv.arg(\"{}\");", command.command()));
        // for arg in command.arguments() {
        //     generator.push_line(&format!("rv.arg({});", arg.name));
        // }
        // generator.push_line("rv");
        // // TODO does this work without the replace?
        // // generator.push_line(&format!("let rv = ::std::mem::replace(rv, Cmd::new())"));
        // generator.push_line("self.add_command(rv)");
        // ::std::mem::replace($body, Cmd::new()) is basically Cmd::$fn_name
        generator.push_line(&format!(
            "self.add_command(Cmd::{}({}))",
            command.fn_name(),
            command.arguments().map(|arg| &arg.name).join(", ")
        ));
    }
}
