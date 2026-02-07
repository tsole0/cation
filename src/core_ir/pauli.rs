//! Pauli operators and Pauli strings.
//!
//! This module defines the atomic operators used to construct
//! Hamiltonians and observables. All Pauli strings are stored in a
//! canonical form.

use std::fmt;

/// Single qubit Pauli operators.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pauli {
    I,
    X,
    Y,
    Z,
}

/// A tensor product of Pauli operators acting on specific qubit indices.
///
/// Invariants:
/// - Indices are unique
/// - Operators are stored in ascending index order
/// - Identity operators are omitted
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PauliString {
    ops: Vec<(usize, Pauli)>
}

impl PauliString {
    /// Construct a new PauliString from an indexed list of Pauli operators.
    /// The input is canonicalized: identities are removed and indices
    /// are sorted.
    pub fn new(mut ops: Vec<(usize, Pauli)>) -> Self {
        ops.retain(|(_, p)| *p != Pauli::I);
        ops.sort_by_key(|(i, _)| *i);

        // NOTE: duplicate indices are currently not allowed
        // and should be validated later.

        Self { ops }
    }
}

impl fmt::Display for PauliString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.ops.is_empty() {
            // Return the identity operator if there are no non-identity terms
            return write!(f, "I");
        }

        for (i, (idx, p)) in self.ops.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{:?}{}", p, idx)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn identity_operators_are_removed() {
        let ps = PauliString::new(
            vec![
            (0, Pauli::X),
            (1, Pauli::I),
            (2, Pauli::Z),
            ]
        );
        assert_eq!(ps.ops, &[(0, Pauli::X,), (2, Pauli::Z)]);
    }

    #[test]
    fn pauli_operators_are_sorted() {
        let ps = PauliString::new(
            vec![
                (2, Pauli::X),
                (1, Pauli::Z),
                (0, Pauli::Y),
            ]
        );

        assert_eq!(ps.ops,
            &[
                (0, Pauli::Y),
                (1, Pauli::Z),
                (2, Pauli:: X),
            ]
        )
    }

    #[test]
    fn empty_pauli_equivalent_to_identity() {
        let ps = PauliString::new(vec![]);

        assert!(ps.ops.is_empty());

    }

}