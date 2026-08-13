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

use crate::model::guard::Guard;
use crate::model::opaque::{LineageId, PrecedenceOrdinal};

/// A Direction outcome — `glossary.md`, Slice Contract §3.7.
///
/// A closed authoritative category (ADR-0005 §7.4) with exactly the two
/// admitted outcomes. Because a rule holds one of these, "zero outcomes",
/// "more than one outcome" and "an outcome outside the two named" are all
/// unrepresentable, and an outcome is always *declared* rather than derived.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Buy,
    Sell,
}

/// One declared Direction rule — Slice Contract §3.7.
#[derive(Debug, Clone)]
pub struct DirectionRule {
    pub lineage: LineageId,
    pub precedence: PrecedenceOrdinal,
    pub guard: Guard,
    pub outcome: Direction,
}

impl DirectionRule {
    #[must_use]
    pub const fn new(
        lineage: LineageId,
        precedence: PrecedenceOrdinal,
        guard: Guard,
        outcome: Direction,
    ) -> Self {
        Self {
            lineage,
            precedence,
            guard,
            outcome,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::OpaqueValue;

    fn rule(lineage: u64, precedence: u64, guard: Guard, outcome: Direction) -> DirectionRule {
        DirectionRule::new(
            LineageId::new(OpaqueValue::new(lineage)),
            PrecedenceOrdinal::new(OpaqueValue::new(precedence)),
            guard,
            outcome,
        )
    }

    #[test]
    fn a_rule_declares_one_outcome_and_one_precedence() {
        let declared = rule(1, 5, Guard::always_applicable(), Direction::Buy);

        assert_eq!(declared.outcome, Direction::Buy);
        assert!(declared.guard.is_syntactically_empty());
    }

    #[test]
    fn precedence_compares_by_the_supplied_order() {
        let lower = rule(1, 1, Guard::always_applicable(), Direction::Buy);
        let higher = rule(2, 9, Guard::always_applicable(), Direction::Sell);

        assert!(lower.precedence < higher.precedence);
    }

    #[test]
    fn duplicate_precedence_ordinals_remain_representable_for_check_9() {
        // A tie is only a defect when the guards overlap (§4.9). Deciding that
        // is check 9's job in Phase 5, so the shape must be constructible.
        let first = rule(1, 5, Guard::always_applicable(), Direction::Buy);
        let second = rule(2, 5, Guard::always_applicable(), Direction::Sell);

        assert_eq!(first.precedence, second.precedence);
    }
}
