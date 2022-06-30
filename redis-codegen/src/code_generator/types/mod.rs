use self::type_registry::TypeRegistryEntry;
use super::constants::{append_constant_module_docs, TOKEN_DOCS};
use crate::{
    commands::{ArgType, CommandArgument, CommandDefinition},
    ident::to_camel,
    ident::to_snake,
};
use itertools::Itertools;
use std::collections::HashMap;

mod type_registry;

pub(crate) use type_registry::TypeRegistry;

struct Module<'a> {
    submodules: HashMap<&'a str, Module<'a>>,
    entries: Vec<&'a TypeRegistryEntry>,
}

impl Module<'_> {
    fn new() -> Self {
        Self {
            submodules: HashMap::new(),
            entries: Vec::new(),
        }
    }
}

pub(crate) struct TypeGenerator {}

impl TypeGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

impl TypeGenerator {
    pub(crate) fn generate(
        &self,
        generator: &mut super::CodeGenerator,
        commands: &[(&str, &CommandDefinition)],
        fully_qualified_path_prefix: String,
    ) -> TypeRegistry {
        generator.push_line("#![cfg_attr(rustfmt, rustfmt_skip)]");
        append_constant_module_docs(TOKEN_DOCS, generator);
        generator.push_line("use crate::types::{ToRedisArgs, RedisWrite};");

        // First we flatten all top-level arguments of each command into a iterator.
        // We then fold that iterator into a vec of Tokens.
        // A token can be a NewType, Enum or struct, depending on the type that the argument needed.
        let enums_structs = commands
            .iter()
            .flat_map(|(command_name, definition)| definition.arguments.iter().map(|x| (*command_name, x)))
            // .fold(vec![], fold_to_token)
            .fold(vec![], |acc, (command_name, definition)| fold_to_token(acc, (*command_name).to_owned(), definition));

        // At this point we created a type (Token) for each oneof, block or argument with a token.
        // The identifiers of those types are not unique. Each token holds a fqtn (fully qualified token name),
        // which consists of the command name and, if originates at a deeper level, possible multiple argument identifiers.
        // We need to generate code for each Token that is different (ignoring the fqtn). Therefore, we can dedup based on the implemented PartialEq trait (which ignores the fqtn).

        // TypeRegistry will only insert a new entry when the token is not already in its registry.
        let mut registry = TypeRegistry::new(fully_qualified_path_prefix);

        for token in &enums_structs {
            registry.insert_token(token);
        }

        let mut groups = Vec::new();
        // Now we group based on the fully_qualified_path_prefix
        for (key, entries) in &registry
            .registry
            .iter()
            .group_by(|a| &a.fully_qualified_path)
        {
            groups.push((key, entries.collect::<Vec<_>>()))
        }

        groups.sort_by(|a, b| a.0.cmp(b.0));
        let mut module = Module::new();
        for (group, entries) in groups.into_iter() {
            if group.is_empty() {
                module.entries.extend(entries.into_iter());
            } else {
                let mut parts = group.split("::");

                if let Some(ident) = parts.next() {
                    debug_assert!(!ident.is_empty());
                    let mut sub_module = module.submodules.entry(ident).or_insert_with(Module::new);

                    for ident in parts {
                        debug_assert!(!ident.is_empty());
                        sub_module = sub_module
                            .submodules
                            .entry(ident)
                            .or_insert_with(Module::new);
                    }
                    sub_module.entries.extend(entries.into_iter());
                }
            }
        }

        append_modules_recursive(module, generator, &registry);

        registry
    }
}

fn append_modules_recursive(
    module: Module,
    generator: &mut super::CodeGenerator,
    registry: &TypeRegistry,
) {
    for entry in module.entries {
        entry.token.append(generator, registry)
    }
    for (module_name, module) in module.submodules {
        generator.push_line(&format!("pub mod {} {{", module_name));
        generator.depth += 1;
        append_modules_recursive(module, generator, registry);
        generator.depth -= 1;
        generator.push_line("}");
    }
}

type TokenQueue<'a> = Vec<(Vec<String>, &'a CommandArgument)>;

fn fold_to_token(mut acc: Vec<Token>, command_name: String, arg: &CommandArgument) -> Vec<Token> {
    if command_name == "XTRIM" {
        println!("XTRIM");
    }
    let fqtn = vec![command_name];
    let mut queue = vec![(fqtn, arg)];
    let mut cur = queue.pop();
    while let Some((fqtn, arg)) = cur {
        let token_name = arg.token.clone();

        match &arg.r#type {
            ArgType::Oneof { arguments } => acc.push(Token::new_oneof(
                arg.name.clone(),
                token_name,
                arguments,
                &mut queue,
                fqtn,
            )),
            ArgType::Block { arguments } => acc.push(Token::new_block(
                arg.name.clone(),
                token_name,
                arguments,
                &mut queue,
                fqtn.clone(),
            )),
            // If these have token set, generate a new Token for these
            ArgType::String => acc.push(Token::new_wrapper(
                arg.name.clone(),
                token_name,
                "String".to_owned(),
                fqtn,
            )),
            ArgType::Integer => acc.push(Token::new_wrapper(
                arg.name.clone(),
                token_name,
                "i64".to_owned(),
                fqtn,
            )),
            ArgType::Double => acc.push(Token::new_wrapper(
                arg.name.clone(),
                token_name,
                "f64".to_owned(),
                fqtn,
            )),
            ArgType::PureToken => acc.push(Token::new_pure(arg.name.clone(), token_name, fqtn)),
            // Wo do not support the other types for now
            _ => {}
        }
        cur = queue.pop();
    }
    acc
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum VariantType {
    Variant {
        redis_token: Option<String>,
    },
    Wrapper {
        redis_token: Option<String>,
        wrapped_type: String,
    },
    Struct {
        redis_token: Option<String>,
        fields: Vec<(String, String)>,
    },
}
impl VariantType {
    fn new_wrapper(redis_token: Option<String>, wrapped_type: String) -> Self {
        Self::Wrapper {
            redis_token,
            wrapped_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct StructFieldDefinition {
    field_name: String,
    bool_redis_token: Option<String>,
    field_type: String,
}
impl StructFieldDefinition {
    fn new(field_name: String, field_type: String) -> Self {
        Self {
            field_name,
            bool_redis_token: None,
            field_type,
        }
    }
    fn new_bool(field_name: String, redis_token: String) -> Self {
        Self {
            field_name,
            bool_redis_token: Some(redis_token),
            field_type: "bool".to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
enum TokenType {
    NewType(String),
    Struct(Vec<StructFieldDefinition>),
    Enum(Vec<(String, VariantType)>),
}

#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
pub(crate) struct Token {
    fqtn: Vec<String>,
    name: String,
    redis_token: Option<String>,
    kind: TokenType,
}

impl Token {
    pub fn new_pure(name: String, redis_token: Option<String>, fqtn: Vec<String>) -> Token {
        Token {
            name: to_camel(redis_token.clone().unwrap_or(name)),
            fqtn,
            redis_token,
            kind: TokenType::Struct(vec![]),
        }
    }

    pub fn new_wrapper(
        name: String,
        redis_token: Option<String>,
        wrapper_type: String,
        fqtn: Vec<String>,
    ) -> Token {
        Token {
            name: to_camel(redis_token.clone().unwrap_or(name)),
            redis_token,
            fqtn,
            kind: TokenType::NewType(wrapper_type),
        }
    }

    pub fn new_oneof<'a>(
        name: String,
        redis_token: Option<String>,
        args: &'a [CommandArgument],
        queue: &mut TokenQueue<'a>,
        fqtn: Vec<String>,
    ) -> Token {
        let mut variants = vec![];

        for arg in args {
            let type_name = arg
                .token
                .as_ref()
                .and_then(|s| if s.is_empty() { None } else { Some(s) })
                .map(to_camel);

            let variant_name = type_name.clone().unwrap_or_else(|| to_camel(&arg.name));
            let redis_token = arg.token.clone();
            match &arg.r#type {
                ArgType::String => variants.push((
                    variant_name,
                    VariantType::new_wrapper(redis_token, "String".to_owned()),
                )),
                ArgType::Integer => variants.push((
                    variant_name,
                    VariantType::new_wrapper(redis_token, "i64".to_owned()),
                )),
                ArgType::Double => variants.push((
                    variant_name,
                    VariantType::new_wrapper(redis_token, "f64".to_owned()),
                )),
                ArgType::Key { key_spec_index: _ } => variants.push((
                    variant_name,
                    VariantType::new_wrapper(redis_token, "String".to_owned()),
                )),
                ArgType::PureToken => {
                    variants.push((variant_name, VariantType::Variant { redis_token }))
                }
                ArgType::Oneof { arguments: _ } => {
                    // We want to have at least one enum per oneof, thus we need to queue this arg in any case.
                    let mut sub_fqtn = fqtn.clone();
                    sub_fqtn.push(name.clone());
                    queue.push((sub_fqtn, arg));
                    let type_name = type_name.unwrap_or_else(|| to_camel(&arg.name));
                    variants.push((
                        variant_name,
                        VariantType::new_wrapper(redis_token, type_name),
                    ))
                }
                ArgType::Block { arguments } => {
                    let mut fields = vec![];

                    for arg in arguments {
                        let r#type = match &arg.r#type {
                            ArgType::String => "String".to_owned(),
                            ArgType::Integer => "i64".to_owned(),
                            ArgType::Double => "f64".to_owned(),
                            ArgType::Oneof { arguments: _ } => {
                                let mut sub_fqtn = fqtn.clone();
                                sub_fqtn.push(name.clone());
                                queue.push((sub_fqtn, arg));

                                let type_name = to_camel(arg.token.as_ref().unwrap_or(&arg.name));
                                format!("{}::{}", name.clone(), type_name)
                            }
                            ArgType::Block { arguments: _ } => {
                                let mut sub_fqtn = fqtn.clone();
                                sub_fqtn.push(name.clone());
                                queue.push((sub_fqtn, arg));

                                let type_name = to_camel(arg.token.as_ref().unwrap_or(&arg.name));
                                format!("{}::{}", name.clone(), type_name)
                            }
                            // We do not support the other types yet.
                            _ => continue,
                        };
                        fields.push((to_snake(&arg.name), r#type));
                    }
                    variants.push((
                        variant_name,
                        VariantType::Struct {
                            redis_token,
                            fields,
                        },
                    ))
                }
                // We do not support any other types currently.
                _ => {}
            }
        }
        Token {
            name: to_camel(redis_token.clone().unwrap_or(name)),
            fqtn,
            redis_token,
            kind: TokenType::Enum(variants),
        }
    }

    pub fn new_block<'a>(
        name: String,
        redis_token: Option<String>,
        args: &'a [CommandArgument],
        queue: &mut TokenQueue<'a>,
        fqtn: Vec<String>,
    ) -> Token {
        let mut fields = vec![];

        for arg in args {
            let type_name = arg
                .token
                .as_ref()
                .and_then(|s| if s.is_empty() { None } else { Some(s) })
                .map(to_camel);
            // We will need a seperate type for each argument that has a token set. This is a requirement to be able to send the token along the argument value.
            if let Some(type_name) = type_name {
                // Map optional pure-tokens in blocks to booleans
                // The pure-token will then be printed when set to true during ToRedisArgs::write_redis_args
                if matches!(arg.r#type, ArgType::PureToken) && arg.optional {
                    if let Some(redis_token) = &arg.token {
                        fields.push(StructFieldDefinition::new_bool(
                            to_snake(&arg.name),
                            redis_token.clone(),
                        ));
                    }
                } else {
                    let mut sub_fqtn = fqtn.clone();
                    sub_fqtn.push(name.clone());
                    queue.push((sub_fqtn, arg));

                    fields.push(StructFieldDefinition::new(
                        to_snake(&arg.name),
                        format!("{}::{}", name.clone(), type_name),
                    ));
                }
                continue;
            }

            let r#type = match &arg.r#type {
                ArgType::String => "String".to_owned(),
                ArgType::Integer => "i64".to_owned(),
                ArgType::Double => "f64".to_owned(),
                ArgType::Oneof { arguments: _ } => {
                    let mut sub_fqtn = fqtn.clone();
                    sub_fqtn.push(name.clone());
                    queue.push((sub_fqtn.clone(), arg));
                    format!("{}::{}", name.clone(), to_camel(&arg.name))
                }
                ArgType::Block { arguments: _ } => {
                    let mut sub_fqtn = fqtn.clone();
                    sub_fqtn.push(name.clone());
                    queue.push((sub_fqtn.clone(), arg));
                    format!("{}::{}", name.clone(), to_camel(&arg.name))
                }
                // We do not support the other types yet.
                _ => continue,
            };
            fields.push(StructFieldDefinition::new(to_snake(&arg.name), r#type));
        }

        Token {
            name: to_camel(redis_token.clone().unwrap_or(name)),
            fqtn,
            redis_token,
            kind: TokenType::Struct(fields),
        }
    }

    fn fqtn(&self) -> String {
        let mut prefix = self.fqtn.iter().join("::");
        prefix.push_str("::");
        prefix.push_str(&self.name);
        prefix
    }

    /// Resolves an internal referenced type
    fn resolve(&self, registry: &TypeRegistry, type_name: &str) -> Option<String> {
        let mut fqtn: Vec<&str> = self.fqtn.iter().map(AsRef::as_ref).collect::<Vec<_>>();
        fqtn.push(type_name);
        registry.resolve(&fqtn)
    }

    fn append(&self, generator: &mut super::CodeGenerator, registry: &TypeRegistry) {
        match &self.kind {
            TokenType::NewType(type_name) => {
                generator.push_line(&format!(
                    "/// Redis Type: {} {}",
                    self.redis_token.as_ref().unwrap_or(&self.name),
                    self.fqtn()
                ));

                let resolved_type = self.resolve(registry, type_name);
                let type_name = resolved_type.as_ref().unwrap_or(type_name);
                generator.push_line(&format!("pub struct {}({});", self.name, type_name));
            }
            TokenType::Struct(fields) => {
                generator.push_line(&format!("/// Redis Block: {}", self.name));
                generator.push_line(&format!("pub struct {} {{", self.name));
                generator.depth += 1;
                for field in fields {
                    generator.push_line(&format!("/// {}", field.field_name));
                    let resolved_type = self.resolve(registry, &field.field_type);
                    let field_type = resolved_type.as_ref().unwrap_or(&field.field_type);
                    generator.push_line(&format!("pub {}: {},", field.field_name, field_type));
                }
                generator.depth -= 1;
                generator.push_line("}");
            }
            TokenType::Enum(variants) => {
                generator.push_line(&format!("/// Redis Type: {}", self.name));
                generator.push_line(&format!("pub enum {} {{", self.name));
                generator.depth += 1;

                for (variant, variant_type) in variants {
                    match variant_type {
                        VariantType::Variant { redis_token } => {
                            generator.push_line(&format!(
                                "/// {}",
                                redis_token.as_ref().map(AsRef::as_ref).unwrap_or("Unknown")
                            ));
                            generator.push_line(&format!("{},", variant))
                        }
                        VariantType::Wrapper {
                            redis_token,
                            wrapped_type,
                        } => {
                            generator.push_line(&format!(
                                "/// {}",
                                redis_token.as_ref().map(AsRef::as_ref).unwrap_or("Unknown")
                            ));
                            let resolved_type = self.resolve(registry, wrapped_type);
                            let wrapped_type = resolved_type.as_ref().unwrap_or(wrapped_type);
                            generator.push_line(&format!("{}({}),", variant, wrapped_type));
                        }
                        VariantType::Struct {
                            redis_token,
                            fields,
                        } => {
                            let fields = fields
                                .iter()
                                .map(|field| {
                                    let resolved_type = self.resolve(registry, &field.1);
                                    let r#type = resolved_type.as_ref().unwrap_or(&field.1);
                                    format!("{}: {}", field.0, r#type)
                                })
                                .join(", ");
                            let buf = format!("{} {{{}}},", variant, fields);

                            generator.push_line(&format!(
                                "/// {}",
                                redis_token.as_ref().map(AsRef::as_ref).unwrap_or("Unknown")
                            ));
                            generator.push_line(&buf);
                        }
                    }
                }
                generator.depth -= 1;
                generator.push_line("}");
            }
        }

        generator.buf.push('\n');

        append_to_redis_args_impl(generator, self);
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.redis_token == other.redis_token && self.kind == other.kind
    }
}

fn append_to_redis_args_impl(generator: &mut super::CodeGenerator, token: &Token) {
    generator.push_line(&format!(
        "impl crate::types::ToRedisArgs for {} {{",
        token.name
    ));
    generator.depth += 1;

    generator.push_line("fn write_redis_args<W>(&self, out: &mut W)");
    generator.push_line("where");
    generator.depth += 1;
    generator.push_line("W: ?Sized + crate::types::RedisWrite,");
    generator.depth -= 1;
    generator.push_line("{");
    generator.depth += 1;

    match &token.kind {
        TokenType::NewType(_type_name) => {
            if let Some(redis_token) = &token.redis_token {
                generator.push_line(&format!("\"{}\".write_redis_args(out);", redis_token));
            }
            generator.push_line("self.0.write_redis_args(out);");
        }
        TokenType::Struct(fields) => {
            if let Some(redis_token) = &token.redis_token {
                generator.push_line(&format!("\"{}\".write_redis_args(out);", redis_token));
            }
            for field in fields {
                if let Some(redis_token) = &field.bool_redis_token {
                    generator.push_line(&format!("if self.{} {{", field.field_name));
                    generator.depth += 1;
                    generator.push_line(&format!("\"{}\".write_redis_args(out);", redis_token));
                    generator.depth -= 1;
                    generator.push_line("}");
                } else {
                    generator
                        .push_line(&format!("self.{}.write_redis_args(out);", field.field_name));
                }
            }
        }
        TokenType::Enum(variants) => {
            if let Some(redis_token) = &token.redis_token {
                generator.push_line(&format!("\"{}\".write_redis_args(out);", redis_token));
            }

            generator.push_line("match self {");
            generator.depth += 1;
            for (variant, variant_type) in variants {
                match variant_type {
                    VariantType::Variant { redis_token } => {
                        if let Some(redis_token) = redis_token {
                            generator.push_line(&format!(
                                "{}::{} => \"{}\".write_redis_args(out),",
                                token.name, variant, redis_token
                            ))
                        }
                    }
                    VariantType::Wrapper {
                        redis_token,
                        wrapped_type: _,
                    } => {
                        generator.push_line(&format!("{}::{}(inner) => {{", token.name, variant));
                        generator.depth += 1;
                        if let Some(redis_token) = redis_token {
                            generator
                                .push_line(&format!("\"{}\".write_redis_args(out);", redis_token));
                        }
                        generator.push_line("inner.write_redis_args(out);");
                        generator.depth -= 1;
                        generator.push_line("},")
                    }
                    VariantType::Struct {
                        redis_token,
                        fields,
                    } => {
                        generator.push_line(&format!(
                            "{}::{}{{{}}} => {{",
                            token.name,
                            variant,
                            fields.iter().map(|(field, _)| field).join(", ")
                        ));
                        generator.depth += 1;
                        if let Some(redis_token) = redis_token {
                            generator
                                .push_line(&format!("\"{}\".write_redis_args(out);", redis_token));
                        }
                        for field in fields {
                            generator.push_line(&format!("{}.write_redis_args(out);", field.0));
                        }
                        generator.depth -= 1;
                        generator.push_line("},")
                    }
                }
            }
            generator.depth -= 1;
            generator.push_line("}");
        }
    }

    generator.depth -= 1;
    generator.push_line("}");

    generator.depth -= 1;
    generator.push_line("}");
}
