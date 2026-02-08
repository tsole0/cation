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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_ir::pauli::{Pauli, PauliString};
    use crate::core_ir::symb::Symbol;

    #[test]
    fn identical_scalars_are_equal() {
        let a = Expr::Scalar(1.0);
        let b = Expr::Scalar(1.0);

        assert_eq!(a, b);
    }

    #[test]
    fn identical_expessions_are_equal() {
        let s = Symbol::new("phi");
        
        let a = Expr::Symbol(s.clone());
        let b = Expr::Symbol(s);

        assert_eq!(a, b);
    }

    #[test]
    fn structurally_identical_sums_are_equal() {
        let x = Expr::pauli(PauliString::new(vec![(0, Pauli::X)]));
        let y = Expr::pauli(PauliString::new(vec![(1, Pauli::Y)]));

        let a = Expr::sum(vec![x.clone(), y.clone()]);
        let b = Expr::sum(vec![x, y]);

        assert_eq!(a, b);
    }

    #[test]
    fn algebraicly_identical_structurally_different_sums_not_equal() {
        let x = Expr::pauli(PauliString::new(vec![(0, Pauli::X)]));
        let y = Expr::pauli(PauliString::new(vec![(1, Pauli::Y)]));

        let a = Expr::sum(vec![x.clone(), y.clone()]);
        let b = Expr::sum(vec![y, x]);

        assert_ne!(a, b);
    }

    #[test]
    fn structurally_identical_sums_and_products_not_equal() {
        let x = Expr::pauli(PauliString::new(vec![(0, Pauli::X)]));
        let y = Expr::pauli(PauliString::new(vec![(1, Pauli::Y)]));

        let sum = Expr::sum(vec![x.clone(), y.clone()]);
        let product = Expr::product(vec![x, y]);

        assert_ne!(sum, product);
    }

}