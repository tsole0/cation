//! Symbolic parameters used in quantum expressions.
//!
//! This module defines immutable symbolic values that may appear in
//! operator expressions. Symbols are not variables and carry no
//! evaluation semantics by themselves.

use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Symbol {
    /// A symbol representing a parameter in an operator expression.
    Named(String),
    /// A symbol that has been bound to a numeric value.
    ///
    /// Bound symbols still retain symbolic identity; binding does not
    /// imply evaluation.
    Bound {
        name: String,
        value: f64,
    },
}

impl Symbol {
    /// Create new named symbolic parameter
    pub fn new(name: impl Into<String>) -> Self {
        Symbol::Named(name.into())
    }

    /// Returns the name of symbol, ignoring any bound value
    pub fn name(&self) -> &str {
        match self {
            Symbol::Named(name) => name,
            Symbol::Bound { name, .. } => name,
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Named(name) => write!(f, "{}", name),
            Symbol::Bound { name, value } => write!(f, "{}={}", name, value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_symbols_with_same_name_are_equal() {
        let a = Symbol::new("theta");
        let b = Symbol::new("theta");

        assert_eq!(a, b);
    }

    #[test]
    fn bound_and_unbound_symbols_are_not_equal() {
        let a = Symbol::new("theta");
        let b = Symbol::Bound {
            name: ("theta".to_string()),
            value: (1.0)
        };
        assert_ne!(a, b);
    }

    #[test]
    fn symbol_name_preserved_when_bound() {
        let s = Symbol::Bound {
            name: ("phi".to_string()),
            value: (1.0)
        };
        assert_eq!(s.name(), "phi");
    }
}