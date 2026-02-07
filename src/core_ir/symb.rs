//! Symbolic parameters used in quantum expressions.
//!
//! This module defines immutable symbolic values that may appear in
//! operator expressions. Symbols are not variables and carry no
//! evaluation semantics by themselves.

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
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