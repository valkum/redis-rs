use std::collections::HashMap;

use itertools::Itertools;

use crate::ident::to_snake;

use super::Token;

#[derive(Debug)]
pub(crate) struct TypeRegistryEntry {
    pub(crate) fully_qualified_path_prefix: String,
    pub(crate) fully_qualified_path: String,
    pub(crate) type_name: String,
    pub(crate) token: Token,
}

impl TypeRegistryEntry {
    pub(crate) fn fully_qualified_path(&self) -> String {
        [
            &self.fully_qualified_path_prefix,
            &self.fully_qualified_path,
            &self.type_name,
        ]
        .iter()
        .filter(|x| !x.is_empty())
        .join("::")
    }
}

/// A type registry
///
/// This will handle resolving fully qualified paths to types.
/// When inserting it checks if the token is already registered.
/// If it is, the fqtn is added to the index.
/// If not, the fqtn is added to the index and the token is added to the registry.
/// While adding: When the fully qualified path is alread taken, create a different one based on the fqtn.
#[derive(Debug)]
pub(crate) struct TypeRegistry {
    pub(crate) fully_qualified_path_prefix: String,
    pub(crate) registry: Vec<TypeRegistryEntry>,
    pub(crate) index: HashMap<String, usize>,
}

impl TypeRegistry {
    pub(crate) fn new(fully_qualified_path_prefix: String) -> Self {
        Self {
            fully_qualified_path_prefix,
            registry: Vec::new(),
            index: HashMap::new(),
        }
    }

    pub(crate) fn insert_token(&mut self, token: &Token) -> bool {
        eprintln!("insert {}", token.fqtn());
        // First check if we already have this token present.
        // If this is the case we add the fqtn to the index pointing to this registry entry.
        if let Some(index) = self
            .registry
            .iter()
            .find_position(|element| &element.token == token)
        {
            self.index.insert(token.fqtn(), index.0);
            return false;
        }

        // Crates a fully qualified path based which should resemble fully_qualified_path_prefix::<command>::<...>
        // This assumes the generated types here are exposed as types in the super module.
        // We first try to generate a fully_qualified_path that is short and add more parts from the fqtn when this fully_qualified name is already taken.

        // TODO: The logic here can be improved. I think this can create weird fully qualified paths on usage when two completely different commands have the same argument but the short `fqp`
        // is already taken. In this case the second command will use a type with a completely unrelated module name in its `fqp`
        let iter = token.fqtn.iter();
        let mut length = 0;
        while length <= iter.len() {
            let fully_qualified_path = iter
                .clone()
                .take(length)
                .map(|ident| to_snake(ident))
                .join("::");

            // This fully qualified path is already taken. Continue
            if self.registry.iter().any(|element| {
                element.fully_qualified_path == fully_qualified_path
                    && element.type_name == token.name
            }) {
                length += 1;
                continue;
            }

            self.registry.push(TypeRegistryEntry {
                fully_qualified_path_prefix: self.fully_qualified_path_prefix.clone(),
                fully_qualified_path,
                type_name: token.name.clone(),
                token: token.to_owned(),
            });
            let position = self.registry.len() - 1;
            self.index.insert(token.fqtn(), position);
            return true;
        }
        // We should never end here.
        unreachable!()
    }

    pub(crate) fn resolve(&self, ident: &[&str]) -> Option<String> {
        if let Some(x) = self.index.get(&ident.join("::")) {
            return self
                .registry
                .get(*x)
                .map(|entry| entry.fully_qualified_path());
        }
        None
    }
}
