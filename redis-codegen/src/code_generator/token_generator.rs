use super::{
    constants::{append_constant_docs, TOKEN_DOCS, append_constant_module_docs},
    GenerationConfig, Generator,
};
use crate::{
    commands::{ArgType, CommandArgument, CommandDefinition},
    ident::to_camel,
    ident::to_snake,
};
use itertools::Itertools;

pub(crate) struct TokenImpl {}

impl TokenImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Generator for TokenImpl {
    fn generate(
        &self,
        generator: &mut super::CodeGenerator,
        commands: &[(&str, &CommandDefinition)],
    ) {
        generator.push_line("#![cfg_attr(rustfmt, rustfmt_skip)]");
        append_constant_module_docs(TOKEN_DOCS, generator);
        generator.push_line("use crate::types::{ToRedisArgs, RedisWrite};");

        let all_oneof_definitions = commands
            .iter()
            .flat_map(|(_, definition)| definition.arguments.iter())
            .fold(vec![], fold_to_token);

        for token in all_oneof_definitions {
            token.append(generator);
        }
    }
}

fn fold_to_token(mut acc: Vec<Token>, arg: &CommandArgument) -> Vec<Token> {
    let mut queue = vec![arg];
    let mut cur = queue.pop();
    while let Some(arg) = cur {
        let token_name = arg.token.clone();
        match &arg.r#type {
            ArgType::Oneof { arguments } => {
                if let Some(name) = token_name {
                    let token_name = to_camel(&name);
                    if acc.iter().all(|x| x.name != token_name) {
                        acc.push(Token::new_oneof(token_name, arguments, &mut queue))
                    }
                } else {
                    // Use the field name for this purpose. We need an enum for each Oneof
                    let token_name = to_camel(&arg.name);
                    if acc.iter().all(|x| x.name != token_name) {
                        acc.push(Token::new_oneof(token_name, arguments, &mut queue))
                    }
                }
            }
            ArgType::Block { arguments } => {
                if let Some(name) = token_name {
                    let token_name = to_camel(&name);
                    if acc.iter().all(|x| x.name != token_name) {
                        acc.push(Token::new_block(token_name, arguments, &mut queue))
                    }
                }
            }
            // If these have token set, generate a new Token for these
            ArgType::String => {
                if let Some(name) = token_name {
                    let token_name = to_camel(&name);
                    if acc.iter().all(|x| x.name != token_name) {
                        acc.push(Token::new_wrapper(
                            token_name,
                            Some(name),
                            "String".to_owned(),
                        ))
                    }
                }
            }
            ArgType::Integer => {
                if let Some(name) = token_name {
                    let token_name = to_camel(&name);
                    if acc.iter().all(|x| x.name != token_name) {
                        acc.push(Token::new_wrapper(token_name, Some(name), "i64".to_owned()))
                    }
                }
            }
            ArgType::Double => {
                if let Some(name) = token_name {
                    let token_name = to_camel(&name);
                    if acc.iter().all(|x| x.name != token_name) {
                        acc.push(Token::new_wrapper(token_name, Some(name), "f64".to_owned()))
                    }
                }
            }
            // Wo do not support the other types for now
            _ => {}
        }

        cur = queue.pop();
    }
    acc
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
enum TokenType {
    NewType(Option<String>, String),
    Struct(Vec<StructFieldDefinition>),
    Enum(Vec<(String, VariantType)>),
}

#[derive(Debug)]
struct Token {
    name: String,
    kind: TokenType,
}

impl Token {
    pub fn new_wrapper(name: String, redis_token: Option<String>, wrapper_type: String) -> Token {
        Token {
            name,
            kind: TokenType::NewType(redis_token, wrapper_type),
        }
    }

    pub fn new_oneof<'a>(
        name: String,
        args: &'a [CommandArgument],
        queue: &mut Vec<&'a CommandArgument>,
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
                    queue.push(arg);

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
                            ArgType::String => Some("String".to_owned()),
                            ArgType::Integer => Some("i64".to_owned()),
                            ArgType::Double => Some("f64".to_owned()),
                            ArgType::Oneof { arguments: _ } => arg.token.as_ref().map(to_camel),
                            ArgType::Block { arguments: _ } => arg.token.as_ref().map(to_camel),
                            // We do not support the other types yet.
                            _ => continue,
                        };
                        if let Some(r#type) = r#type {
                            fields.push((to_snake(&arg.name), r#type));
                        }
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
            name,
            kind: TokenType::Enum(variants),
        }
    }

    pub fn new_block<'a>(
        name: String,
        args: &'a [CommandArgument],
        queue: &mut Vec<&'a CommandArgument>,
    ) -> Token {
        let mut fields = vec![];

        for arg in args {
            let type_name = arg
                .token
                .as_ref()
                .and_then(|s| if s.is_empty() { None } else { Some(s) })
                .map(to_camel);
            if let Some(type_name) = type_name {
                // Map optional pure-tokens in blocks to booleans
                if matches!(arg.r#type, ArgType::PureToken) && arg.optional {
                    if let Some(redis_token) = &arg.token {
                        fields.push(StructFieldDefinition::new_bool(
                            to_snake(&arg.name),
                            redis_token.clone(),
                        ));
                    }
                } else {
                    queue.push(arg);
                    fields.push(StructFieldDefinition::new(to_snake(&arg.name), type_name));
                }
                continue;
            }
            let r#type = match &arg.r#type {
                ArgType::String => Some("String".to_owned()),
                ArgType::Integer => Some("i64".to_owned()),
                ArgType::Double => Some("f64".to_owned()),
                ArgType::Oneof { arguments: _ } => arg.token.as_ref().map(to_camel),
                ArgType::Block { arguments: _ } => arg.token.as_ref().map(to_camel),
                // We do not support the other types yet.
                _ => continue,
            };
            if let Some(r#type) = r#type {
                fields.push(StructFieldDefinition::new(to_snake(&arg.name), r#type));
            }
        }

        Token {
            name,

            kind: TokenType::Struct(fields),
        }
    }
}

impl Token {
    fn append(&self, generator: &mut super::CodeGenerator) {
        match &self.kind {
            TokenType::NewType(redis_token, type_name) => {
                generator.push_line(&format!(
                    "/// Redis Type: {}",
                    redis_token.as_ref().unwrap_or(&self.name)
                ));
                generator.push_line(&format!("pub struct {}({});", self.name, type_name));
            }
            TokenType::Struct(fields) => {
                generator.push_line(&format!("/// Redis Block: {}", self.name));
                generator.push_line(&format!("pub struct {} {{", self.name));
                generator.depth += 1;
                for field in fields {
                    generator.push_line(&format!("/// {}", field.field_name));
                    generator
                        .push_line(&format!("pub {}: {},", field.field_name, field.field_type));
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
                            generator.push_line(&format!("{}({}),", variant, wrapped_type));
                        }
                        VariantType::Struct {
                            redis_token,
                            fields,
                        } => {
                            let fields = fields
                                .iter()
                                .map(|field| format!("{}: {}", field.0, field.1))
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

fn append_to_redis_args_impl(generator: &mut super::CodeGenerator, token: &Token) {
    generator.push_line(&format!("impl ToRedisArgs for {} {{", token.name));
    generator.depth += 1;

    generator.push_line("fn write_redis_args<W>(&self, out: &mut W)");
    generator.push_line("where");
    generator.depth += 1;
    generator.push_line("W: ?Sized + RedisWrite,");
    generator.depth -= 1;
    generator.push_line("{");
    generator.depth += 1;

    match &token.kind {
        TokenType::NewType(redis_token, _type_name) => {
            if let Some(redis_token) = redis_token {
                generator.push_line(&format!("\"{}\".write_redis_args(out);", redis_token));
            }
            generator.push_line("self.0.write_redis_args(out);");
        }
        TokenType::Struct(fields) => {
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
