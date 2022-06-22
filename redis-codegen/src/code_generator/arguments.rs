use super::GenerationConfig;
use itertools::Itertools;
use std::fmt;

#[derive(Debug, Clone)]
pub enum TypeKind {
    Concrete(String),
    Trait { generic_ident: String, name: String },
}

#[derive(Debug, Clone)]
pub enum Type {
    Tuple(Vec<Type>),
    Single(TypeKind),
}

/// An abstract type representing a code generation unit for an argument for a command
///
/// This bundles the name with the assigned trait ident and trait
/// This is overly generic, to be able to generate more than the default ToRedisArgs trait bounds
#[derive(Debug, Clone)]
pub(crate) struct Argument<'a> {
    pub name: String,
    pub r#type: Type,
    pub optional: bool,
    pub multiple: bool,
    pub config: &'a GenerationConfig,
}

impl<'a> Argument<'a> {
    pub(crate) fn new_concrete(
        name: String,
        r#type: String,
        optional: bool,
        multiple: bool,
        config: &'a GenerationConfig,
    ) -> Self {
        Self {
            name,
            r#type: Type::Single(TypeKind::Concrete(r#type)),
            optional,
            multiple,
            config,
        }
    }

    pub(crate) fn new_generic(
        name: String,
        generic_ident: String,
        r#trait: String,
        optional: bool,
        multiple: bool,
        config: &'a GenerationConfig,
    ) -> Self {
        Self {
            name,
            r#type: Type::Single(TypeKind::Trait {
                generic_ident,
                name: r#trait,
            }),
            optional,
            multiple,
            config,
        }
    }

    pub(crate) fn new_block(
        name: String,
        args: &[Argument],
        optional: bool,
        multiple: bool,
        config: &'a GenerationConfig,
    ) -> Self {
        let sub_args = args
            .iter()
            .map(|arg| arg.r#type.to_owned())
            .collect::<Vec<_>>();
        Self {
            name,
            r#type: Type::Tuple(sub_args),
            optional,
            multiple,
            config,
        }
    }

    // Returns the Trait Bound notation for this argument
    pub(crate) fn trait_bound(&self) -> Option<String> {
        map_traits(&self.r#type)
    }
}

impl<'a> fmt::Display for Argument<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (name, r#type) = flatten_arguments(&self.name, &self.r#type);

        match (self.optional, self.multiple) {
            (true, false) => write!(f, "{}: Option<{}>", name, r#type),
            (true, true) => {
                if self.config.explicit_lifetime {
                    write!(f, "{}: Option<&'a [{}]>", name, r#type)
                } else {
                    write!(f, "{}: Option<&[{}]>", name, r#type)
                }
            }
            (false, true) => {
                if self.config.explicit_lifetime {
                    write!(f, "{}: &'a [{}]", name, r#type)
                } else {
                    write!(f, "{}: &[{}]", name, r#type)
                }
            }
            (false, false) => write!(f, "{}: {}", name, r#type),
        }
    }
}

fn flatten_arguments<'a>(arg_name: &'a str, r#type: &Type) -> (&'a str, String) {
    match r#type {
        Type::Single(kind) => match kind {
            TypeKind::Concrete(name) => (arg_name, name.to_owned()),
            TypeKind::Trait {
                generic_ident,
                name: _,
            } => (arg_name, generic_ident.to_owned()),
        },
        Type::Tuple(kinds) => {
            let mut iter = kinds
                .iter()
                .map(|kind| flatten_arguments(arg_name, kind))
                .map(|arg| arg.1);

            let needs_parentheses = iter.len() >= 2;

            let buf = iter.join(", ");

            if needs_parentheses {
                (arg_name, format!("({})", buf))
            } else {
                (arg_name, buf)
            }
        }
    }
}

/// Gets called recursively to return a String
fn map_traits(r#type: &Type) -> Option<String> {
    match r#type {
        // `T: Trait`
        Type::Single(kind) => match kind {
            TypeKind::Concrete(_) => None,
            TypeKind::Trait {
                generic_ident,
                name,
            } => Some(format!("{}: {}", generic_ident, name)),
        },
        // `T0: Trait1, T1: Trait2`
        Type::Tuple(kinds) => {
            let buf = kinds.iter().filter_map(map_traits).join(", ");
            if buf.is_empty() {
                None
            } else {
                Some(buf)
            }
        }
    }
}
