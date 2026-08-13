//! Required-evidence declarations — Slice Contract §3.5.
//!
//! §3.5 splits the declaration's properties in two, and the split is
//! load-bearing because it decides which check owns a fault (M-1, closed by the
//! contract):
//!
//! * **Unconditional** — lineage identity and declared type. Their absence is
//!   check 1's business.
//! * **Type-dependent** — the dimension and the value set. Check 6 owns these
//!   *exclusively*: their presence, their absence, and their content.
//!
//! Modelling the type-dependent data **inside** the type makes the prohibited
//! shapes unrepresentable: a `BOOLEAN` declaration cannot carry a dimension and
//! a `NUMERIC` declaration cannot carry a value set, because there is no field
//! to put them in.

use crate::model::opaque::{DimensionSymbol, EnumValue, LineageId};

/// The declared type of an evidence item — Slice Contract §3.5.
///
/// A closed authoritative category (ADR-0005 §7.4): matching over it is
/// exhaustive and no catch-all `_` arm is permitted, so adding a fourth type
/// would break every match rather than pass silently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvidenceType {
    /// Carries its dimension, which §3.5 requires **iff** the type is numeric.
    Numeric { dimension: DimensionSymbol },
    /// Carries nothing further.
    Boolean,
    /// Carries its declared value set, which §3.5 requires **iff** the type is
    /// `ENUM`.
    ///
    /// The set is a plain sequence, and deliberately so: §5.1 check 6 must be
    /// able to detect an **empty** set and **duplicate** values. Enforcing
    /// non-emptiness or distinctness here would make those faults
    /// unrepresentable and leave check 6 with nothing to find — Phase 1
    /// represents, it does not validate.
    Enum { declared_values: Vec<EnumValue> },
}

/// One required-evidence declaration — Slice Contract §3.5.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceDeclaration {
    pub lineage: LineageId,
    pub declared_type: EvidenceType,
}

impl EvidenceDeclaration {
    #[must_use]
    pub const fn new(lineage: LineageId, declared_type: EvidenceType) -> Self {
        Self {
            lineage,
            declared_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::OpaqueValue;

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn dimension(handle: u64) -> DimensionSymbol {
        DimensionSymbol::new(OpaqueValue::new(handle))
    }

    fn enum_value(handle: u64) -> EnumValue {
        EnumValue::new(OpaqueValue::new(handle))
    }

    #[test]
    fn numeric_declaration_carries_a_dimension() {
        let declaration = EvidenceDeclaration::new(
            lineage(1),
            EvidenceType::Numeric {
                dimension: dimension(7),
            },
        );

        match declaration.declared_type {
            EvidenceType::Numeric { dimension: d } => assert_eq!(d, dimension(7)),
            EvidenceType::Boolean | EvidenceType::Enum { .. } => {
                panic!("constructed a numeric declaration")
            }
        }
    }

    #[test]
    fn boolean_declaration_carries_nothing_further() {
        let declaration = EvidenceDeclaration::new(lineage(2), EvidenceType::Boolean);

        assert_eq!(declaration.declared_type, EvidenceType::Boolean);
    }

    #[test]
    fn enum_declaration_carries_its_declared_values() {
        let declaration = EvidenceDeclaration::new(
            lineage(3),
            EvidenceType::Enum {
                declared_values: vec![enum_value(10), enum_value(11)],
            },
        );

        match declaration.declared_type {
            EvidenceType::Enum { declared_values } => assert_eq!(declared_values.len(), 2),
            EvidenceType::Numeric { .. } | EvidenceType::Boolean => {
                panic!("constructed an enum declaration")
            }
        }
    }

    #[test]
    fn check_6_faults_remain_representable() {
        // Phase 1 must not make these unrepresentable: §5.1 check 6 detects an
        // empty value set and duplicate values in Phase 5, and cannot detect
        // what cannot be built.
        let empty = EvidenceDeclaration::new(
            lineage(4),
            EvidenceType::Enum {
                declared_values: vec![],
            },
        );
        let duplicated = EvidenceDeclaration::new(
            lineage(5),
            EvidenceType::Enum {
                declared_values: vec![enum_value(10), enum_value(10)],
            },
        );

        // Matched one at a time, and exhaustively. Matching the pair as a tuple
        // would need a catch-all arm for the other eight combinations, and
        // ADR-0005 §7.4 prohibits a catch-all over a closed authoritative
        // category unconditionally — a fourth evidence type must break this
        // match rather than be absorbed silently.
        match empty.declared_type {
            EvidenceType::Enum { declared_values } => assert!(declared_values.is_empty()),
            EvidenceType::Numeric { .. } | EvidenceType::Boolean => {
                panic!("constructed an enum declaration")
            }
        }

        match duplicated.declared_type {
            EvidenceType::Enum { declared_values } => assert_eq!(declared_values.len(), 2),
            EvidenceType::Numeric { .. } | EvidenceType::Boolean => {
                panic!("constructed an enum declaration")
            }
        }
    }
}
