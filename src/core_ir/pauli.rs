//! Pauli operators and Pauli strings.
//!
//! This module defines the atomic operators used to construct
//! Hamiltonians and observables. All Pauli strings are stored in a
//! canonical form.

use std::fmt;
use std::cmp::Ordering;
use std::ops::Mul;

/// Helper function to multiply two Pauli matrices,
/// returning (phase, pauli)
pub fn multiply_paulis(a: Pauli, b: Pauli) -> (u8, Pauli) {
    match (a, b) {
        (Pauli::I, p) | (p, Pauli::I) => (0, p),
        (Pauli::X, Pauli::X) => (0, Pauli::I),
        (Pauli::Y, Pauli::Y) => (0, Pauli::I),
        (Pauli::Z, Pauli::Z) => (0, Pauli::I),
        (Pauli::X, Pauli::Y) => (1, Pauli::Z),  // iZ
        (Pauli::Y, Pauli::Z) => (1, Pauli::X),  // iX
        (Pauli::Z, Pauli::X) => (1, Pauli::Y),  // iY
        (Pauli::Y, Pauli::X) => (3, Pauli::Z),  // -iZ
        (Pauli::Z, Pauli::Y) => (3, Pauli::X),  // -iX
        (Pauli::X, Pauli::Z) => (3, Pauli::Y),  // -iY
    }
}

/// Single qubit Pauli operators.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    pub ops: Vec<(usize, Pauli)>,
    pub phase: u8
}

impl TryFrom<char> for Pauli {

    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        
        match c {
            'I' => Ok(Self::I),
            'X' => Ok(Self::X),
            'Y' => Ok(Self::Y),
            'Z' => Ok(Self::Z),
            _ => Err(format!("Invalid Pauli character: '{}'", c))
        }
    }
}

impl From<Pauli> for char {
    fn from(p: Pauli) -> Self {
        match p {
            Pauli::I => 'I',
            Pauli::X => 'X',
            Pauli::Y => 'Y',
            Pauli::Z => 'Z',
        }
    }
}


/// Order PauliStrings by their index
impl Ord for PauliString {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ops.iter().map(|(idx, _)| idx)
            .cmp(other.ops.iter().map(|(idx, _)| idx))
    }
}

/// PartialOrd required by Ord
impl PartialOrd for PauliString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PauliString {
    /// Construct a new PauliString from an indexed list of Pauli operators.
    /// The input is canonicalized: identities are removed and indices
    /// are sorted.
    pub fn new(mut ops: Vec<(usize, Pauli)>, phase: u8) -> Self {
        ops.retain(|(_, p)| *p != Pauli::I);
        ops.sort_by_key(|(i, _)| *i);

        // NOTE: duplicate indices are currently not allowed
        // and should be validated later.

        Self { 
            ops,
            phase: phase
        }
    }

    /// Generate a PauliString from a string input
    /// May return an error if user inputs invalid chars
    /// Thus, for internal use `PauliString::new()` is preferred
    pub fn from_string(input: impl Into<String>, phase: u8) -> Result<Self, String> {
        let ops = input
        .into()
        .chars()
        .enumerate()
        .map(|(idx, char)| {
            Pauli::try_from(char)
            .map(|pauli| (idx, pauli))
        })
        .collect::<Result<Vec<_>, String>>()?;
        Ok(Self::new(ops, phase))
    }

    /// Explicit empty Pauli string.
    pub fn empty() -> Self {
        Self { ops: Vec::new(), phase: 0}
    }

    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

}

impl Mul<&PauliString> for PauliString {
    type Output = PauliString;

    fn mul(self, other: &PauliString) -> PauliString {
        // Build a map of qubit index -> pauli for other for O(1) lookup
        let other_map: std::collections::HashMap<usize, Pauli> = 
            other.ops.iter().map(|(idx, p)| (*idx, *p)).collect();
        
        let mut result_ops: Vec<(usize, Pauli)> = Vec::new();
        let mut phase = (self.phase + other.phase) % 4;
        
        for (qubit_idx, self_pauli) in &self.ops {
            if let Some(other_pauli) = other_map.get(qubit_idx) {
                let (p, result_pauli) = multiply_paulis(*self_pauli, *other_pauli);
                phase = (phase + p) % 4;
                
                // Only include in result if not identity
                if result_pauli != Pauli::I {
                    result_ops.push((*qubit_idx, result_pauli));
                }
            } else {
                // other doesn't act on this qubit, so result has self's pauli
                result_ops.push((*qubit_idx, *self_pauli));
            }
        }
        
        // Add any qubits that other acts on but self doesn't
        for (qubit_idx, other_pauli) in &other.ops {
            if !self.ops.iter().any(|(idx, _)| idx == qubit_idx) {
                result_ops.push((*qubit_idx, *other_pauli));
            }
        }
        
        PauliString::new(result_ops, phase)
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
            ],
            0
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
            ],
            0
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
        let ps = PauliString::new(vec![], 0);

        assert!(ps.ops.is_empty());

    }

    #[test]
    fn convert_string_to_paulistring() {
        PauliString::from_string("XZYIZZ", 0).unwrap();
    }

    #[test]
    fn invalid_pauli_chars_return_error() {
        assert!(PauliString::from_string("XZTAL", 0).is_err());
    }

    #[test]
    fn multiply_disjoint_pauli_strings_combines_ops() {
        let lhs = PauliString::new(vec![(0, Pauli::X)], 0);
        let rhs = PauliString::new(vec![(2, Pauli::Z)], 0);

        let product = lhs * &rhs;

        assert_eq!(product.phase, 0);
        assert_eq!(product.ops, vec![(0, Pauli::X), (2, Pauli::Z)]);
    }

    #[test]
    fn multiply_same_pauli_on_same_qubit_cancels_to_identity() {
        let lhs = PauliString::new(vec![(1, Pauli::Y)], 0);
        let rhs = PauliString::new(vec![(1, Pauli::Y)], 0);

        let product = lhs * &rhs;

        assert_eq!(product.phase, 0);
        assert!(product.ops.is_empty());
    }

    #[test]
    fn multiply_overlapping_terms_updates_phase_and_operator() {
        let lhs = PauliString::new(vec![(0, Pauli::X)], 0);
        let rhs = PauliString::new(vec![(0, Pauli::Y)], 0);

        let product = lhs * &rhs;

        // X * Y = iZ -> phase exponent 1 (mod 4)
        assert_eq!(product.phase, 1);
        assert_eq!(product.ops, vec![(0, Pauli::Z)]);
    }

    #[test]
    fn multiply_includes_input_phases() {
        let lhs = PauliString::new(vec![(0, Pauli::X)], 1); // iX
        let rhs = PauliString::new(vec![(0, Pauli::Y)], 3); // -iY

        let product = lhs * &rhs;

        // (iX)(-iY) = (+1)(XY) = iZ -> phase exponent 1
        assert_eq!(product.phase, 1);
        assert_eq!(product.ops, vec![(0, Pauli::Z)]);
    }

    #[test]
    fn multiply_result_is_canonical_sorted_by_index() {
        let lhs = PauliString::new(vec![(2, Pauli::Z)], 0);
        let rhs = PauliString::new(vec![(0, Pauli::X)], 0);

        let product = lhs * &rhs;

        assert_eq!(product.ops, vec![(0, Pauli::X), (2, Pauli::Z)]);
    }

    

}