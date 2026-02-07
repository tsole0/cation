//! Semantic graph IR for quantum programs
//! Does not implement any evaluation semantics, but provides a structured
//! representation of quantum expressions and operators.

use std::sync::Arc;

use crate::core_ir::pauli::PauliString;
use crate::core_ir::symb::Symbol;


/// An algebraic expression over quantum operators.
///
/// Expressions form a purely symbolic representation and carry no
/// evaluation semantics. All expressions are immutable.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// Real scalar constant
    Scalar(f64),

    /// Symbolic character
    Symbol(Symbol),

    /// Pauli string operator
    Pauli(PauliString),

    /// A sum of expressions
    /// in which order is not meaningful
    Sum(Vec<Arc<Expr>>),

    /// Products of expressions
    /// Not generably invertable
    Product(Vec<Arc<Expr>>)
}

impl Expr {
    pub fn scalar(val: f64) -> Arc<Self> {
        Arc::new(Expr::Scalar(val))
    }

    pub fn symbol(sym: Symbol) -> Arc<Self> {
        Arc::new(Expr::Symbol(sym))
    }

    pub fn pauli(p: PauliString) -> Arc<Self> {
        Arc::new(Expr::Pauli(p))
    }

    pub fn sum(s: Vec<Arc<Expr>>) -> Arc<Self> {
        Arc::new(Expr::Sum(s))
    }

    pub fn product(factors: Vec<Arc<Expr>>) -> Arc<Self> {
        Arc::new(Expr::Product(factors))
    }
}