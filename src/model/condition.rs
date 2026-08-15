//! Conditions — Slice Contract §3.10.
//!
//! A well-formed condition carries **exactly one** static classification for the
//! Strategy revision (RFC-0001 §10 / Q-11b; §4.8).
//!
//! # Why "exactly one" is not enforced by the type
//!
//! A two-variant enum field would make "zero classifications", "more than one"
//! and "a value outside the two named" unrepresentable — and §5.1 check 3 exists
//! precisely to report those three shapes as `INVALID_CONDITION_CLASSIFICATION`,
//! one defect per condition. A check cannot detect what cannot be built, so the
//! classification is a [`DeclaredCategory`], which keeps the well-formed shape
//! and all three faults apart. The *closed* two-value domain is still enforced:
//! `DeclaredCategory::One` admits nothing but the two named classifications.
//!
//! §4.8 is equally clear about what a classification does **not** mean here:
//! Phase 1 attaches no status meaning to either value. How a `BLOCKING` or
//! `TEMPORARY` condition resolves into a runtime status is owned entirely by
//! the Validation Model and is outside Slice 1.

use crate::model::declared::{Declared, DeclaredCategory};
use crate::model::guard::Guard;
use crate::model::opaque::{LineageId, OccurrenceId};

/// The static classification of a condition — Slice Contract §3.10, §4.8.
///
/// A closed authoritative category (ADR-0005 §7.4). It is a declared static
/// attribute: never derived from the guard, never influenced by evidence, and
/// never mutable at runtime — the type exposes no mutation path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionClassification {
    Blocking,
    Temporary,
}

/// One declared condition — Slice Contract §3.10.
///
/// All three §3.10 properties are unconditional, so each is a declaration state.
/// A condition is an operative node, so an absent lineage identity is check 15's
/// `MISSING_OPERATIVE_PROVENANCE` **exclusively** (Slice Contract Revision 3
/// §C.1) — check 1 reports nothing for it. An absent guard is check 1's
/// `MISSING_REQUIRED_PROPERTY`, and a zero / multiple / out-of-domain
/// classification is check 3's.
///
/// There is deliberately no field relating one condition to another: ordered
/// evidence relations are not admitted in Slice 1 (§3.1).
#[derive(Debug, Clone)]
pub struct Condition {
    /// Canonical occurrence identity (Revision 4 §C.1) — mandatory, present even
    /// when `lineage` is absent. A condition is an operative node, so lineage
    /// absence is check 15's; the occurrence identity anchors that defect.
    pub occurrence: OccurrenceId,
    pub lineage: Declared<LineageId>,
    pub classification: DeclaredCategory<ConditionClassification>,
    pub guard: Declared<Guard>,
}

impl Condition {
    /// A condition with every property declared exactly once — the well-formed
    /// shape. Malformed shapes are built from the fields directly.
    #[must_use]
    pub const fn new(
        occurrence: OccurrenceId,
        lineage: LineageId,
        classification: ConditionClassification,
        guard: Guard,
    ) -> Self {
        Self {
            occurrence,
            lineage: Declared::Present(lineage),
            classification: DeclaredCategory::One(classification),
            guard: Declared::Present(guard),
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

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    #[test]
    fn a_well_formed_condition_carries_exactly_one_classification() {
        let blocking = Condition::new(
            occ(1),
            lineage(1),
            ConditionClassification::Blocking,
            Guard::always_applicable(),
        );
        let temporary = Condition::new(
            occ(2),
            lineage(2),
            ConditionClassification::Temporary,
            Guard::always_applicable(),
        );

        assert_eq!(
            blocking.classification,
            DeclaredCategory::One(ConditionClassification::Blocking)
        );
        assert_eq!(
            temporary.classification,
            DeclaredCategory::One(ConditionClassification::Temporary)
        );
    }

    #[test]
    fn classification_is_exhaustively_matchable() {
        // No catch-all `_` arm (ADR-0005 §7.4): adding a third classification
        // must break this match rather than pass silently.
        let classification = ConditionClassification::Blocking;
        let named = match classification {
            ConditionClassification::Blocking => "blocking",
            ConditionClassification::Temporary => "temporary",
        };

        assert_eq!(named, "blocking");
    }

    #[test]
    fn check_3_faults_remain_representable() {
        // §5.1 check 3 reports a zero, multiple, or out-of-domain classification
        // as `INVALID_CONDITION_CLASSIFICATION` in Phase 5. All three must be
        // constructible now.
        let zero = Condition {
            occurrence: occ(3),
            lineage: Declared::Present(lineage(3)),
            classification: DeclaredCategory::Absent,
            guard: Declared::Present(Guard::always_applicable()),
        };
        let multiple = Condition {
            occurrence: occ(4),
            lineage: Declared::Present(lineage(4)),
            classification: DeclaredCategory::multiple(
                ConditionClassification::Blocking,
                ConditionClassification::Temporary,
                vec![],
            ),
            guard: Declared::Present(Guard::always_applicable()),
        };
        let out_of_domain = Condition {
            occurrence: occ(5),
            lineage: Declared::Present(lineage(5)),
            classification: DeclaredCategory::OutOfDomain,
            guard: Declared::Present(Guard::always_applicable()),
        };

        // Each fault is read back through an exhaustive match, so the three
        // remain distinct from one another and from the well-formed `One`
        // state. No catch-all `_` arm (ADR-0005 §7.4).
        match zero.classification {
            DeclaredCategory::Absent => {}
            DeclaredCategory::One(_)
            | DeclaredCategory::Multiple { .. }
            | DeclaredCategory::OutOfDomain => panic!("constructed an absent classification"),
        }

        match multiple.classification {
            DeclaredCategory::Multiple {
                first,
                second,
                rest,
            } => {
                assert_eq!(first, ConditionClassification::Blocking);
                assert_eq!(second, ConditionClassification::Temporary);
                assert!(rest.is_empty());
            }
            DeclaredCategory::Absent | DeclaredCategory::One(_) | DeclaredCategory::OutOfDomain => {
                panic!("constructed multiple classifications")
            }
        }

        match out_of_domain.classification {
            DeclaredCategory::OutOfDomain => {}
            DeclaredCategory::Absent
            | DeclaredCategory::One(_)
            | DeclaredCategory::Multiple { .. } => {
                panic!("constructed an out-of-domain classification")
            }
        }
    }

    #[test]
    fn check_1_and_check_15_absences_remain_representable() {
        // A condition is an operative node (§5.1 check 15), so an absent lineage
        // identity must be buildable; so must an absent guard (check 1).
        let anonymous = Condition {
            occurrence: occ(6),
            lineage: Declared::Absent,
            classification: DeclaredCategory::One(ConditionClassification::Blocking),
            guard: Declared::Absent,
        };

        assert!(anonymous.lineage.is_absent());
        assert!(anonymous.guard.is_absent());
    }
}
