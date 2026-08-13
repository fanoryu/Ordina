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
//! # Why the type-dependent properties are siblings of the type, not payloads
//!
//! It is tempting to carry the dimension and the value set *inside* the declared
//! type, so that a `BOOLEAN` declaration has no field for a dimension and the
//! prohibited shape is unrepresentable. That reading is wrong, and §5.1.1 says so
//! plainly: "dimension present on a `BOOLEAN` or `ENUM` declaration" and "value
//! set present on a `NUMERIC` or `BOOLEAN` declaration" are **mandatory check-6
//! violations**, each mapped to a §3.5 property for defect ordering. So are the
//! converses — a `NUMERIC` declaration with no dimension, an `ENUM` declaration
//! with no value set.
//!
//! Making those shapes unrepresentable would not prevent a malformed Strategy; it
//! would delete four of check 6's seven violations, leaving the check unable to
//! report faults the contract requires it to report. The four properties are
//! therefore flat siblings, each independently present or absent, and the
//! *relationship* between the type tag and the other three is check 6's judgement
//! in Phase 5 rather than a construction rule.
//!
//! Structural prevention remains the right tool one level down: the type tag is a
//! closed three-variant enum, so a fourth evidence type is still unrepresentable.

use crate::model::declared::{Declared, DeclaredCategory};
use crate::model::opaque::{DimensionSymbol, EnumValue, LineageId};

/// The declared type of an evidence item — Slice Contract §3.5.
///
/// A closed authoritative category (ADR-0005 §7.4): matching over it is
/// exhaustive and no catch-all `_` arm is permitted, so adding a fourth type
/// would break every match rather than pass silently.
///
/// It is a **tag** and carries nothing: the dimension and the value set are
/// separate properties of the declaration (see the module note), and §5.1.1
/// discriminates check-6 defects by those properties' §3.5 positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceTypeTag {
    Numeric,
    Boolean,
    Enum,
}

/// One required-evidence declaration — Slice Contract §3.5.
///
/// All four properties are declaration states rather than bare values, and each
/// state is one the contract names:
///
/// | Field | Well-formed | Faults the contract requires to stay representable |
/// |---|---|---|
/// | `lineage` | present | absent → check 1 `MISSING_REQUIRED_PROPERTY` |
/// | `declared_type` | exactly one tag | absent → check 1; out-of-domain → check 6 (§5.1.1 row 1) |
/// | `declared_dimension` | present **iff** `Numeric` | present when forbidden, absent when required → check 6 (§5.1.1 rows 2–3) |
/// | `declared_values` | present **iff** `Enum` | present when forbidden, absent when required, empty, duplicate-bearing → check 6 (§5.1.1 rows 4–7) |
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceDeclaration {
    pub lineage: Declared<LineageId>,
    pub declared_type: DeclaredCategory<EvidenceTypeTag>,
    /// Required **iff** the declared type is `NUMERIC` (§3.5).
    pub declared_dimension: Declared<DimensionSymbol>,
    /// Required **iff** the declared type is `ENUM` (§3.5).
    ///
    /// The value set is a plain sequence, and deliberately so: §5.1 check 6 must
    /// be able to detect an **empty** set and **duplicate** values. Enforcing
    /// non-emptiness or distinctness here would make those faults
    /// unrepresentable and leave check 6 with nothing to find — Phase 1
    /// represents, it does not validate.
    pub declared_values: Declared<Vec<EnumValue>>,
}

impl EvidenceDeclaration {
    /// A well-formed `NUMERIC` declaration: dimension present, no value set.
    #[must_use]
    pub const fn numeric(lineage: LineageId, dimension: DimensionSymbol) -> Self {
        Self {
            lineage: Declared::Present(lineage),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Numeric),
            declared_dimension: Declared::Present(dimension),
            declared_values: Declared::Absent,
        }
    }

    /// A well-formed `BOOLEAN` declaration: neither dimension nor value set.
    #[must_use]
    pub const fn boolean(lineage: LineageId) -> Self {
        Self {
            lineage: Declared::Present(lineage),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Boolean),
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        }
    }

    /// A well-formed `ENUM` declaration: value set present, no dimension.
    ///
    /// The values are taken as given. Whether the set is non-empty and
    /// duplicate-free is check 6's judgement in Phase 5, not this constructor's.
    #[must_use]
    pub const fn enumerated(lineage: LineageId, declared_values: Vec<EnumValue>) -> Self {
        Self {
            lineage: Declared::Present(lineage),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Enum),
            declared_dimension: Declared::Absent,
            declared_values: Declared::Present(declared_values),
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
    fn a_well_formed_numeric_declaration_carries_a_dimension_and_no_value_set() {
        let declaration = EvidenceDeclaration::numeric(lineage(1), dimension(7));

        assert_eq!(
            declaration.declared_type,
            DeclaredCategory::One(EvidenceTypeTag::Numeric)
        );
        assert_eq!(
            declaration.declared_dimension,
            Declared::Present(dimension(7))
        );
        assert!(declaration.declared_values.is_absent());
    }

    #[test]
    fn a_well_formed_boolean_declaration_carries_neither() {
        let declaration = EvidenceDeclaration::boolean(lineage(2));

        assert!(declaration.declared_dimension.is_absent());
        assert!(declaration.declared_values.is_absent());
    }

    #[test]
    fn a_well_formed_enum_declaration_carries_its_values_and_no_dimension() {
        let declaration =
            EvidenceDeclaration::enumerated(lineage(3), vec![enum_value(10), enum_value(11)]);

        assert!(declaration.declared_dimension.is_absent());
        assert_eq!(
            declaration.declared_values,
            Declared::Present(vec![enum_value(10), enum_value(11)])
        );
    }

    #[test]
    fn the_evidence_type_tag_is_exhaustively_matchable() {
        // No catch-all `_` arm (ADR-0005 §7.4): a fourth evidence type must
        // break this match rather than pass silently.
        for tag in [
            EvidenceTypeTag::Numeric,
            EvidenceTypeTag::Boolean,
            EvidenceTypeTag::Enum,
        ] {
            let named = match tag {
                EvidenceTypeTag::Numeric => "numeric",
                EvidenceTypeTag::Boolean => "boolean",
                EvidenceTypeTag::Enum => "enum",
            };
            assert!(!named.is_empty());
        }
    }

    // ---------------------------------------------------------------------
    // Representability of the faults §5.1.1 assigns to check 6, and of the
    // check-1 absences §3.5 marks unconditional. Phase 1 asserts only that each
    // shape can be *built* — no check judges any of them yet.
    // ---------------------------------------------------------------------

    #[test]
    fn check_1_absences_remain_representable() {
        // §3.5 marks lineage identity and declared type Unconditional, so their
        // absence is check 1's `MISSING_REQUIRED_PROPERTY` in Phase 5.
        let declaration = EvidenceDeclaration {
            lineage: Declared::Absent,
            declared_type: DeclaredCategory::Absent,
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        };

        assert!(declaration.lineage.is_absent());

        // An **absent** declared type is check 1's; an out-of-domain one is
        // check 6's (§5.1.1 row 1). The match keeps the two apart rather than
        // letting a summary conflate them. No catch-all `_` arm (ADR-0005
        // §7.4).
        match declaration.declared_type {
            DeclaredCategory::Absent => {}
            DeclaredCategory::One(_)
            | DeclaredCategory::Multiple { .. }
            | DeclaredCategory::OutOfDomain => panic!("constructed an absent declared type"),
        }
    }

    #[test]
    fn an_out_of_domain_declared_type_remains_representable() {
        // §5.1.1 row 1: "declared type is not one of NUMERIC / BOOLEAN / ENUM".
        let declaration = EvidenceDeclaration {
            lineage: Declared::Present(lineage(4)),
            declared_type: DeclaredCategory::OutOfDomain,
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        };

        assert_eq!(
            declaration.declared_type,
            DeclaredCategory::<EvidenceTypeTag>::OutOfDomain
        );
    }

    #[test]
    fn a_dimension_absent_where_required_remains_representable() {
        // §5.1.1 row 2: a NUMERIC declaration with no dimension.
        let declaration = EvidenceDeclaration {
            lineage: Declared::Present(lineage(5)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Numeric),
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        };

        assert!(declaration.declared_dimension.is_absent());
    }

    #[test]
    fn a_dimension_present_where_forbidden_remains_representable() {
        // §5.1.1 row 3: a dimension on a BOOLEAN or ENUM declaration.
        let on_boolean = EvidenceDeclaration {
            lineage: Declared::Present(lineage(6)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Boolean),
            declared_dimension: Declared::Present(dimension(7)),
            declared_values: Declared::Absent,
        };
        let on_enum = EvidenceDeclaration {
            lineage: Declared::Present(lineage(7)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Enum),
            declared_dimension: Declared::Present(dimension(7)),
            declared_values: Declared::Present(vec![enum_value(10)]),
        };

        assert!(on_boolean.declared_dimension.is_present());
        assert!(on_enum.declared_dimension.is_present());
    }

    #[test]
    fn a_value_set_absent_where_required_remains_representable() {
        // §5.1.1 row 4: an ENUM declaration with no value set.
        let declaration = EvidenceDeclaration {
            lineage: Declared::Present(lineage(8)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Enum),
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        };

        assert!(declaration.declared_values.is_absent());
    }

    #[test]
    fn a_value_set_present_where_forbidden_remains_representable() {
        // §5.1.1 row 7: a value set on a NUMERIC or BOOLEAN declaration.
        let on_numeric = EvidenceDeclaration {
            lineage: Declared::Present(lineage(9)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Numeric),
            declared_dimension: Declared::Present(dimension(7)),
            declared_values: Declared::Present(vec![enum_value(10)]),
        };
        let on_boolean = EvidenceDeclaration {
            lineage: Declared::Present(lineage(10)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Boolean),
            declared_dimension: Declared::Absent,
            declared_values: Declared::Present(vec![enum_value(10)]),
        };

        assert!(on_numeric.declared_values.is_present());
        assert!(on_boolean.declared_values.is_present());
    }

    #[test]
    fn empty_and_duplicate_enum_value_sets_remain_representable() {
        // §5.1.1 rows 5 and 6.
        let empty = EvidenceDeclaration::enumerated(lineage(11), vec![]);
        let duplicated =
            EvidenceDeclaration::enumerated(lineage(12), vec![enum_value(10), enum_value(10)]);

        match empty.declared_values {
            // No catch-all `_` arm (ADR-0005 §7.4).
            Declared::Present(values) => assert!(values.is_empty()),
            Declared::Absent => panic!("constructed a present value set"),
        }

        match duplicated.declared_values {
            Declared::Present(values) => assert_eq!(values, vec![enum_value(10), enum_value(10)]),
            Declared::Absent => panic!("constructed a present value set"),
        }
    }
}
