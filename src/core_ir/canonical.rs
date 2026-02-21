//! Canonicalizations of semantically identical expressions.
//! 
//! Provides a single deterministic output that is identical for all
//! semantically equivalent expressions. Importantly, does not provide
//! deterministically single output for all ALGEBRAICLY equivalent expressions.
use std::sync::Arc;

use crate::core_ir::expr::Expr;

/// Wrapper showing that internal expression is canonicalized.
/// Functions may require arguments of canonical form
/// to guarantee that they work as intended
#[derive(Clone, Debug, PartialEq)]
pub struct Canonicalized<T> {
    inner: T
}

trait Flatten {
    fn flatten(&self) -> Arc<Expr>;
}

impl Canonicalized<Expr> {
    pub fn get(&self) -> &Expr {&self.inner}

    fn new(expr: Expr) -> Self {
        Canonicalized { inner: expr }
    }
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

trait Canonical {
    fn canonical(&self) -> Arc<Canonicalized<Expr>>;
}

impl Canonical for Expr {
    fn canonical(&self) -> Arc<Canonicalized<Expr>> {
        Arc::new(Canonicalized::new(self.canonical_inner()))
    }
}

/// Flatten and sort tree.
impl Expr {
    fn canonical_inner(&self) -> Expr {
        // Flatten
        let flat = self.flatten();
        // Canonicalize
        match flat.as_ref() {
            Expr::Sum(terms) => {
                let mut out: Vec<Arc<Expr>> = Vec::new();
                for term in terms.iter() {
                    let term_canonical = Arc::new(term.canonical_inner());
                    out.push(term_canonical);
                }
                out.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                Arc::unwrap_or_clone(Expr::sum(out))
            }
            Expr::Product(factors) => {
                let mut out: Vec<Arc<Expr>> = Vec::new();
                for factor in factors.iter() {
                    let factor_canonical = Arc::new(factor.canonical_inner());
                    out.push(factor_canonical);
                }
                // Do NOT sort since we assume products do not commute.
                Arc::unwrap_or_clone(Expr::product(out))
            }
            _ => Arc::unwrap_or_clone(flat) // Leaf node case
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
        assert_eq!(&*s, &*flattened); // should be the same value
    }

    #[test]
    fn test_canonical_same_order() {
        let a = Expr::symbol(Symbol::new("a"));
        let b = Expr::symbol(Symbol::new("b"));
        let c = Expr::symbol(Symbol::new("c"));

        let sum1 = Expr::sum(vec![a.clone(), b.clone(), c.clone()]);
        let sum2 = Expr::sum(vec![a.clone(), b.clone(), c.clone()]);

        assert_eq!(sum1.canonical(), sum2.canonical());
    }

    #[test]
    fn test_canonical_different_order() {
        let a = Expr::symbol(Symbol::new("a"));
        let b = Expr::symbol(Symbol::new("b"));
        let c = Expr::symbol(Symbol::new("c"));

        let sum1 = Expr::sum(vec![a.clone(), b.clone(), c.clone()]);
        let sum2 = Expr::sum(vec![c.clone(), a.clone(), b.clone()]);

        assert_eq!(sum1.canonical(), sum2.canonical());
    }

    #[test]
    fn test_canonical_nested() {
        let a = Expr::symbol(Symbol::new("a"));
        let b = Expr::symbol(Symbol::new("b"));
        let c = Expr::symbol(Symbol::new("c"));

        let inner_sum1 = Expr::sum(vec![c.clone(), b.clone()]);
        let nested1 = Expr::sum(vec![a.clone(), inner_sum1]);

        let inner_sum2 = Expr::sum(vec![b.clone(), c.clone()]);
        let nested2 = Expr::sum(vec![a.clone(), inner_sum2]);

        assert_eq!(nested1.canonical(), nested2.canonical());
    }

    #[test]
    fn test_canonicalized_nested_products_different_order_not_equal() {
        let x = Expr::pauli(PauliString::new(vec![(0, Pauli::X)]));
        let y = Expr::pauli(PauliString::new(vec![(0, Pauli::Y)]));

        let s = Expr::symbol(Symbol::new("phi"));
        let a = Expr::sum(
            vec![
                Expr::scalar(6.0),
                s.clone(),
                x.clone()
            ]
        );

        let combined1 = Expr::product(
            vec![
                a.clone(),
                x.clone(),
                s.clone(),
                y.clone()
            ]
        );

        let combined2: Arc<Expr> = Expr::product(
            vec![
                x,
                y,
                s,
                a
            ]
        );

        assert_ne!(combined1.canonical(), combined2.canonical());
    }

        #[test]
    fn test_canonicalized_products_with_changed_internal_sums_equal() {
        let x = Expr::pauli(PauliString::new(vec![(0, Pauli::X)]));
        let y = Expr::pauli(PauliString::new(vec![(0, Pauli::Y)]));

        let s = Expr::symbol(Symbol::new("phi"));
        let a = Expr::sum(
            vec![
                Expr::scalar(6.0),
                s.clone(),
                x.clone()
            ]
        );
        let a_changed = Expr::sum(
            vec![
                Expr::scalar(6.0),
                x.clone(),
                s.clone()
            ]
        );

        let combined1 = Expr::product(
            vec![
                a.clone(),
                x.clone(),
                s.clone(),
                y.clone()
            ]
        );

        let combined2: Arc<Expr> = Expr::product(
            vec![
                a_changed,
                x,
                s,
                y
            ]
        );

        assert_eq!(combined1.canonical(), combined2.canonical());

    }

}