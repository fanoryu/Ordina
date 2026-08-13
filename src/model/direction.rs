//! Direction rules — Slice Contract §3.7.
//!
//! # Precedence is declared, never positional
//!
//! §3.7 requires the precedence ordinal to be **explicitly declared** rather
//! than taken from position in a collection. The reason is worth keeping in
//! view: if precedence came from position, two rules could never tie, and
//! check 9 — which exists precisely to reject a tie between co-applicable rules
//! — would be vacuous. A declared ordinal makes ties **authorable**, and
//! therefore detectable.
//!
//! Duplicate ordinals are consequently left representable here. Whether a
//! duplicate is a defect depends on whether the two guards overlap (§4.9), and
//! that is check 9's judgement in Phase 5, not a construction rule.
//!
//! # The outcome follows the same reasoning
//!
//! §3.7 prohibits "a rule declaring zero or more than one outcome" and "an
//! outcome outside `{BUY, SELL}`", and §5.1 check 10 reports all three as
//! `INVALID_DIRECTION_OUTCOME`, one defect per rule. A bare two-variant field
//! would make check 10 vacuous, so the outcome is a [`DeclaredCategory`]. The
//! closed `{BUY, SELL}` domain is still enforced: `DeclaredCategory::One` admits
//! nothing else.

use crate::model::declared::{Declared, DeclaredCategory};
use crate::model::guard::Guard;
use crate::model::opaque::{LineageId, PrecedenceOrdinal};

/// A Direction outcome — `glossary.md`, Slice Contract §3.7.
///
/// A closed authoritative category (ADR-0005 §7.4) with exactly the two
/// admitted outcomes, and always *declared* rather than derived.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Buy,
    Sell,
}

/// One declared Direction rule — Slice Contract §3.7.
///
/// All four §3.7 properties are unconditional. Check 8 states its own
/// prerequisite in exactly these terms — "for every Direction rule, its
/// **precedence ordinal** and its **guard** are present (check 1 for those two
/// properties on that rule)" — which is only meaningful if their absence is
/// representable, so both are declaration states.
#[derive(Debug, Clone)]
pub struct DirectionRule {
    pub lineage: Declared<LineageId>,
    pub precedence: Declared<PrecedenceOrdinal>,
    pub guard: Declared<Guard>,
    pub outcome: DeclaredCategory<Direction>,
}

impl DirectionRule {
    /// A rule with every property declared exactly once — the well-formed
    /// shape. Malformed shapes are built from the fields directly.
    #[must_use]
    pub const fn new(
        lineage: LineageId,
        precedence: PrecedenceOrdinal,
        guard: Guard,
        outcome: Direction,
    ) -> Self {
        Self {
            lineage: Declared::Present(lineage),
            precedence: Declared::Present(precedence),
            guard: Declared::Present(guard),
            outcome: DeclaredCategory::One(outcome),
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

    fn precedence(handle: u64) -> PrecedenceOrdinal {
        PrecedenceOrdinal::new(OpaqueValue::new(handle))
    }

    fn rule(id: u64, ordinal: u64, guard: Guard, outcome: Direction) -> DirectionRule {
        DirectionRule::new(lineage(id), precedence(ordinal), guard, outcome)
    }

    #[test]
    fn a_well_formed_rule_declares_one_outcome_and_one_precedence() {
        let declared = rule(1, 5, Guard::always_applicable(), Direction::Buy);

        assert_eq!(declared.outcome, DeclaredCategory::One(Direction::Buy));
        assert_eq!(declared.precedence, Declared::Present(precedence(5)));
        match declared.guard {
            // No catch-all `_` arm (ADR-0005 §7.4).
            Declared::Present(guard) => assert!(guard.is_syntactically_empty()),
            Declared::Absent => panic!("constructed a present guard"),
        }
    }

    #[test]
    fn precedence_compares_by_the_supplied_order() {
        assert!(precedence(1) < precedence(9));
    }

    #[test]
    fn duplicate_precedence_ordinals_remain_representable_for_check_9() {
        // A tie is only a defect when the guards overlap (§4.9). Deciding that
        // is check 9's job in Phase 5, so the shape must be constructible.
        let first = rule(1, 5, Guard::always_applicable(), Direction::Buy);
        let second = rule(2, 5, Guard::always_applicable(), Direction::Sell);

        assert_eq!(first.precedence, second.precedence);
    }

    #[test]
    fn check_10_faults_remain_representable() {
        // §5.1 check 10 reports a zero, multiple, or out-of-domain outcome as
        // `INVALID_DIRECTION_OUTCOME` in Phase 5.
        let zero = DirectionRule {
            lineage: Declared::Present(lineage(3)),
            precedence: Declared::Present(precedence(1)),
            guard: Declared::Present(Guard::always_applicable()),
            outcome: DeclaredCategory::Absent,
        };
        let multiple = DirectionRule {
            lineage: Declared::Present(lineage(4)),
            precedence: Declared::Present(precedence(2)),
            guard: Declared::Present(Guard::always_applicable()),
            outcome: DeclaredCategory::multiple(Direction::Buy, Direction::Sell, vec![]),
        };
        let out_of_domain = DirectionRule {
            lineage: Declared::Present(lineage(5)),
            precedence: Declared::Present(precedence(3)),
            guard: Declared::Present(Guard::always_applicable()),
            outcome: DeclaredCategory::OutOfDomain,
        };

        // Each fault is read back through an exhaustive match, so the three
        // remain distinct from one another and from the well-formed `One`
        // state. No catch-all `_` arm (ADR-0005 §7.4).
        match zero.outcome {
            DeclaredCategory::Absent => {}
            DeclaredCategory::One(_)
            | DeclaredCategory::Multiple { .. }
            | DeclaredCategory::OutOfDomain => panic!("constructed an absent outcome"),
        }

        match multiple.outcome {
            DeclaredCategory::Multiple {
                first,
                second,
                rest,
            } => {
                assert_eq!(first, Direction::Buy);
                assert_eq!(second, Direction::Sell);
                assert!(rest.is_empty());
            }
            DeclaredCategory::Absent | DeclaredCategory::One(_) | DeclaredCategory::OutOfDomain => {
                panic!("constructed multiple outcomes")
            }
        }

        match out_of_domain.outcome {
            DeclaredCategory::OutOfDomain => {}
            DeclaredCategory::Absent
            | DeclaredCategory::One(_)
            | DeclaredCategory::Multiple { .. } => panic!("constructed an out-of-domain outcome"),
        }
    }

    #[test]
    fn check_1_check_8_and_check_15_absences_remain_representable() {
        // Check 8's stated prerequisite is that a rule's precedence ordinal and
        // guard are present; check 15 anchors on the lineage identity of an
        // operative node. All three absences must therefore be buildable.
        let bare = DirectionRule {
            lineage: Declared::Absent,
            precedence: Declared::Absent,
            guard: Declared::Absent,
            outcome: DeclaredCategory::One(Direction::Buy),
        };

        assert!(bare.lineage.is_absent());
        assert!(bare.precedence.is_absent());
        assert!(bare.guard.is_absent());
    }
}
