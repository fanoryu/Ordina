//! Conditions — Slice Contract §3.10.
//!
//! Every condition carries **exactly one** static classification for the
//! Strategy revision (RFC-0001 §10 / Q-11b; §4.8). Modelling the classification
//! as a closed two-variant enum makes "zero classifications", "more than one"
//! and "a value outside the two named" unrepresentable.
//!
//! §4.8 is equally clear about what a classification does **not** mean here:
//! Phase 1 attaches no status meaning to either value. How a `BLOCKING` or
//! `TEMPORARY` condition resolves into a runtime status is owned entirely by
//! the Validation Model and is outside Slice 1.

use crate::model::guard::Guard;
use crate::model::opaque::LineageId;

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
/// There is deliberately no field relating one condition to another: ordered
/// evidence relations are not admitted in Slice 1 (§3.1).
#[derive(Debug, Clone)]
pub struct Condition {
    pub lineage: LineageId,
    pub classification: ConditionClassification,
    pub guard: Guard,
}

impl Condition {
    #[must_use]
    pub const fn new(
        lineage: LineageId,
        classification: ConditionClassification,
        guard: Guard,
    ) -> Self {
        Self {
            lineage,
            classification,
            guard,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::OpaqueValue;

    #[test]
    fn a_condition_carries_exactly_one_classification() {
        let blocking = Condition::new(
            LineageId::new(OpaqueValue::new(1)),
            ConditionClassification::Blocking,
            Guard::always_applicable(),
        );
        let temporary = Condition::new(
            LineageId::new(OpaqueValue::new(2)),
            ConditionClassification::Temporary,
            Guard::always_applicable(),
        );

        assert_eq!(blocking.classification, ConditionClassification::Blocking);
        assert_eq!(temporary.classification, ConditionClassification::Temporary);
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
}
