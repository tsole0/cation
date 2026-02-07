//! Canonicalizations of semantically identical expressions.
//! 
//! Provides a single deterministic output that is identical for all
//! semantically equivalent expressions. Importantly, does not provide
//! deterministically single output for all ALGEBRAICLY equivalent expressions.
use std::sync::Arc;

use crate::core_ir::expr::Expr;

trait Flatten {
    fn flatten(&self) -> Arc<Expr>;
}

trait Canonical {
    fn canonical(&self) -> Arc<Expr>;
}

/// Flatten the abstract syntax tree of an Expression
/// Does not simplify algebra
impl Flatten for Expr {

    fn flatten(&self) -> Arc<Expr> {
        match self {
            Expr::Sum(summands) => {
                let mut output_terms = vec![];
                for term in summands.iter() {
                    let flat_term = term.flatten();
                    match &*flat_term {
                        Expr::Sum(inner_terms) => {
                            for inner_term in inner_terms.iter() {
                                output_terms.push(inner_term.clone());
                            }
                        },
                        _ => output_terms.push(flat_term.clone())
                    }
                }
                Expr::sum(output_terms)
            },

            Expr::Product(factors) => {
                let mut output_terms = vec![];
                for term in factors.iter() {
                    let flat_term = term.flatten();
                    match &*flat_term {
                        Expr::Product(inner_terms) => {
                            for inner_term in inner_terms.iter() {
                                output_terms.push(inner_term.clone());
                            }
                        },
                        _ => output_terms.push(flat_term.clone())
                    }
                }
                Expr::product(output_terms)
            },

            _ => {
                Arc::new(self.clone())
            }

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_ir::expr::Expr;
    use crate::core_ir::pauli::{Pauli, PauliString};
    use crate::core_ir::symb::Symbol;

    #[test]
    fn test_flatten_simple_sum() {
        // (a + (b + c)) → (a + b + c)

        let a = Expr::symbol(Symbol::new("a"));
        let b = Expr::symbol(Symbol::new("b"));
        let c = Expr::symbol(Symbol::new("c"));

        let inner_sum = Expr::sum(vec![b.clone(), c.clone()]);
        let nested_sum = Expr::sum(vec![a.clone(), inner_sum]);

        let flattened = nested_sum.flatten();

        // Expect flattened sum to have exactly 3 terms: a, b, c
        if let Expr::Sum(terms) = flattened.as_ref() {
            assert_eq!(terms.len(), 3);
            assert!(terms.contains(&a));
            assert!(terms.contains(&b));
            assert!(terms.contains(&c));
        } else {
            panic!("Flatten did not return a Sum");
        }
    }

    #[test]
    fn test_flatten_nested_product() {
        // (x * (y * z)) → (x * y * z)

        let x = Expr::pauli(PauliString::new(vec![(0, Pauli::X)]));
        let y = Expr::pauli(PauliString::new(vec![(1, Pauli::Y)]));
        let z = Expr::pauli(PauliString::new(vec![(2, Pauli::Z)]));

        let inner_prod = Expr::product(vec![y.clone(), z.clone()]);
        let nested_prod = Expr::product(vec![x.clone(), inner_prod]);

        let flattened = nested_prod.flatten();

        if let Expr::Product(factors) = flattened.as_ref() {
            assert_eq!(factors.len(), 3);
            assert!(factors.contains(&x));
            assert!(factors.contains(&y));
            assert!(factors.contains(&z));
        } else {
            panic!("Flatten did not return a Product");
        }
    }

    #[test]
    fn test_flatten_leaf_node() {
        let s = Expr::symbol(Symbol::new("phi"));
        let flattened = s.flatten();
        assert_eq!(&*s, &*flattened); // should be the same Arc
    }
}