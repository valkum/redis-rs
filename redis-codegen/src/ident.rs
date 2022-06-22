//! Utility functions for working with identifiers, taken from prost

use heck::{ToSnakeCase, ToUpperCamelCase};

/// Converts a `camelCase` or `SCREAMING_SNAKE_CASE` identifier to a `lower_snake` case Rust field
/// identifier.
pub fn to_snake(s: &str) -> String {
    let mut ident = s.to_snake_case();

    // Use a raw identifier if the identifier matches a Rust keyword:
    // https://doc.rust-lang.org/reference/keywords.html.
    match ident.as_str() {
        // 2015 strict keywords.
        | "as" | "break" | "const" | "continue" | "else" | "enum" | "false"
        | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut"
        | "pub" | "ref" | "return" | "static" | "struct" | "trait" | "true"
        | "type" | "unsafe" | "use" | "where" | "while"
        // 2018 strict keywords.
        | "dyn"
        // 2015 reserved keywords.
        | "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" | "priv" | "typeof"
        | "unsized" | "virtual" | "yield"
        // 2018 reserved keywords.
        | "async" | "await" | "try" => ident.insert_str(0, "r#"),
        _ => (),
    }
    ident
}

/// Converts a `lower_snake`, `camelCase` or `SCREAMING_SNAKE_CASE` identifier to a `CamelCase` case Rust type or variant
/// identifier.
pub fn to_camel<T>(s: T) -> String
where
    T: AsRef<str>,
{
    let ident = s.as_ref();
    match ident {
        "*" => "Star".to_owned(),
        "=" => "Equals".to_owned(),
        "~" => "Approx".to_owned(),
        "$" => "LastId".to_owned(),
        _ => ident.to_upper_camel_case(),
    }
}
