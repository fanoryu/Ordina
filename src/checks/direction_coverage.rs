//! Check 8 — Direction coverage (Slice Contract §5.1 check 8, §3.7, §7.1).
//!
//! Slice 1 makes Direction coverage tractably decidable by the **default-rule**
//! form (§7.1): exactly one Direction rule must declare the empty
//! (always-applicable) guard, and it must hold the **lowest** declared
//! precedence of all rules. Check 8 verifies that structural property in time
//! linear in the rule count — it reads guards **only** to ask whether each is
//! the syntactic empty guard and performs **no** type, reference, overlap,
//! contradiction, or domain analysis. It therefore depends on **neither check 2
//! nor check 6**, and an unrelated type or reference defect in any rule must not
//! suppress it (§5.1 check 8 prerequisite).
//!
//! Three independent populations (§5.1 check 8, §6.4.1 ordinals 11–13):
//!
//! - `DIRECTION_COVERAGE_INCOMPLETE` — no always-applicable rule (which includes
//!   the zero-rules case). Its semantic subject is the Strategy as a whole, so it
//!   anchors on the **envelope** occurrence identity (Revision 4 §C.6) with the
//!   `NONE` discriminator; one per Strategy.
//! - `MULTIPLE_TOTAL_DIRECTION_RULES` — more than one always-applicable rule.
//!   Also Strategy-level: envelope occurrence, `NONE`, one per Strategy.
//! - `TOTAL_RULE_NOT_LOWEST_PRECEDENCE` — an always-applicable rule that is not
//!   at the lowest declared precedence. Tied to that offending **member**, so it
//!   anchors on **that rule's** occurrence identity (Revision 4 §C.6) with `NONE`;
//!   one per offending rule.
//!
//! The three populations are **independent §5.1 predicates** — the multiple-total
//! and not-lowest-precedence conditions may both hold on one Strategy and then
//! both are emitted, exactly as ER-0092 RULING A settled for the check-2 ENUM
//! overlap. No population is suppressed by another.
//!
//! **Presence vs. non-emptiness.** The *presence* of the `direction_rules`
//! collection is check 1's required-property business (§3.4; R3 §3.7 table); its
//! *non-emptiness* is check 8's. Check 8 therefore evaluates only over a
//! **present** collection: an absent collection yields a check-1
//! `MISSING_REQUIRED_PROPERTY`, not a check-8 coverage defect. This is intrinsic
//! predicate computability, not Phase-6 gating.
//!
//! Defect production only; deterministic; no prerequisite gating, verdict, or
//! canonical sorting (those are Phase 6).

use crate::model::declared::Declared;
use crate::model::direction::DirectionRule;
use crate::model::strategy::Strategy;
use crate::result::category::DefectCategory;
use crate::result::discriminator::Discriminator;
use crate::result::identity::DefectCore;

/// Whether a rule is **always-applicable**: its guard is present and is the
/// syntactic empty conjunction (§4.5). An **absent** guard is not the empty
/// guard — its absence is check 1's — so it does not count as always-applicable.
fn is_always_applicable(rule: &DirectionRule) -> bool {
    matches!(&rule.guard, Declared::Present(guard) if guard.is_syntactically_empty())
}

/// Whether an always-applicable rule is **not** at the lowest declared
/// precedence: its precedence is present and some declared rule carries a
/// **strictly lower** present precedence. This is exactly the "not the minimum"
/// predicate under the supplied §3.3.1 precedence order — never container,
/// traversal, source, or occurrence order. Rules whose precedence is absent are
/// outside the computable order (their absence is check 1's) and are ignored on
/// both sides of the comparison; the existence of one strictly-lower rule already
/// proves this rule is not lowest, so the predicate never manufactures a defect
/// on indeterminate data.
fn is_not_lowest_precedence(rule: &DirectionRule, all: &[DirectionRule]) -> bool {
    let Declared::Present(this) = &rule.precedence else {
        return false;
    };
    all.iter().any(|other| match &other.precedence {
        Declared::Present(that) => that < this,
        Declared::Absent => false,
    })
}

/// Check 8 over a whole Strategy. Returns a deterministic `Vec<DefectCore>`.
#[must_use]
pub fn check_8(strategy: &Strategy) -> Vec<DefectCore> {
    // Non-emptiness is check 8's; presence of the collection is check 1's.
    let Declared::Present(rules) = &strategy.direction_rules else {
        return Vec::new();
    };

    let mut defects = Vec::new();
    let total_count = rules.iter().filter(|r| is_always_applicable(r)).count();

    // DIRECTION_COVERAGE_INCOMPLETE — no always-applicable rule (includes the
    // empty-collection / zero-rules case). One per Strategy, envelope-anchored.
    if total_count == 0 {
        defects.push(DefectCore::new(
            strategy.occurrence,
            DefectCategory::DirectionCoverageIncomplete,
            Discriminator::None,
        ));
    }

    // MULTIPLE_TOTAL_DIRECTION_RULES — more than one always-applicable rule. One
    // per Strategy, envelope-anchored. Independent of the coverage predicate
    // (unreachable together) and of the not-lowest predicate below.
    if total_count > 1 {
        defects.push(DefectCore::new(
            strategy.occurrence,
            DefectCategory::MultipleTotalDirectionRules,
            Discriminator::None,
        ));
    }

    // TOTAL_RULE_NOT_LOWEST_PRECEDENCE — one per always-applicable rule that is
    // not at the lowest declared precedence, anchored on that rule's occurrence.
    // Evaluated independently of the multiple-total predicate.
    for rule in rules {
        if is_always_applicable(rule) && is_not_lowest_precedence(rule, rules) {
            defects.push(DefectCore::new(
                rule.occurrence,
                DefectCategory::TotalRuleNotLowestPrecedence,
                Discriminator::None,
            ));
        }
    }

    defects
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::declared::DeclaredCategory;
    use crate::model::direction::Direction;
    use crate::model::guard::{Atom, ComparisonOperator, Guard};
    use crate::model::literal::Literal;
    use crate::model::opaque::{LineageId, OccurrenceId, OpaqueValue, PrecedenceOrdinal};

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn precedence(handle: u64) -> PrecedenceOrdinal {
        PrecedenceOrdinal::new(OpaqueValue::new(handle))
    }

    /// A non-empty (non-total) guard — a single boolean atom. Its content is
    /// irrelevant to check 8; only its non-emptiness is.
    fn nonempty_guard() -> Guard {
        Guard::from_atoms(vec![Atom::new(
            lineage(900),
            ComparisonOperator::Equal,
            Literal::Boolean(true),
        )])
    }

    /// A well-formed rule with the given occurrence, precedence, guard, outcome.
    fn rule(occ_id: u64, prec: u64, guard: Guard, outcome: Direction) -> DirectionRule {
        DirectionRule::new(
            occ(occ_id),
            lineage(occ_id),
            precedence(prec),
            guard,
            outcome,
        )
    }

    /// An envelope (occurrence 1) carrying exactly the supplied rule collection;
    /// every other property is present-and-innocuous so only check 8 speaks.
    fn strategy_with(direction_rules: Declared<Vec<DirectionRule>>) -> Strategy {
        Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(lineage(1000)),
            semantic_model_version: Declared::Absent,
            evidence_declarations: Declared::Present(vec![]),
            configuration_parameters: Declared::Present(vec![]),
            conditions: Declared::Present(vec![]),
            direction_rules,
            candidate_derivations: Declared::Present(vec![]),
        }
    }

    fn categories(defects: &[DefectCore]) -> Vec<DefectCategory> {
        defects.iter().map(DefectCore::category).collect()
    }

    // -- DIRECTION_COVERAGE_INCOMPLETE -------------------------------------

    #[test]
    fn zero_direction_rules_is_coverage_incomplete_on_the_envelope() {
        let defects = check_8(&strategy_with(Declared::Present(vec![])));
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::DirectionCoverageIncomplete]
        );
        // Strategy-level: envelope occurrence, NONE discriminator.
        assert_eq!(defects[0].offending(), occ(1));
        assert!(matches!(defects[0].discriminator(), Discriminator::None));
    }

    #[test]
    fn rules_present_but_no_total_rule_is_coverage_incomplete() {
        // Two non-total rules ⇒ no always-applicable rule ⇒ coverage incomplete.
        let defects = check_8(&strategy_with(Declared::Present(vec![
            rule(10, 5, nonempty_guard(), Direction::Buy),
            rule(11, 6, nonempty_guard(), Direction::Sell),
        ])));
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::DirectionCoverageIncomplete]
        );
        assert_eq!(defects[0].offending(), occ(1));
    }

    #[test]
    fn an_absent_direction_rules_collection_is_check_1s_not_check_8s() {
        // Presence of the collection is check 1's; check 8 stays silent.
        assert!(check_8(&strategy_with(Declared::Absent)).is_empty());
    }

    // -- Well-formed coverage ----------------------------------------------

    #[test]
    fn exactly_one_lowest_precedence_total_rule_is_no_defect() {
        // One total rule at the lowest precedence, plus a higher non-total rule.
        let defects = check_8(&strategy_with(Declared::Present(vec![
            rule(10, 1, Guard::always_applicable(), Direction::Buy),
            rule(11, 9, nonempty_guard(), Direction::Sell),
        ])));
        assert!(defects.is_empty());
    }

    #[test]
    fn one_total_rule_never_triggers_the_multiple_total_defect() {
        let defects = check_8(&strategy_with(Declared::Present(vec![rule(
            10,
            1,
            Guard::always_applicable(),
            Direction::Buy,
        )])));
        assert!(defects.is_empty());
    }

    // -- MULTIPLE_TOTAL_DIRECTION_RULES ------------------------------------

    #[test]
    fn two_total_rules_at_the_lowest_precedence_is_multiple_total_only() {
        // Both total, both share the lowest precedence ⇒ neither is "not lowest",
        // so only the Strategy-level multiple-total defect (the equal-precedence
        // overlap is check 9's, not check 8's).
        let defects = check_8(&strategy_with(Declared::Present(vec![
            rule(10, 3, Guard::always_applicable(), Direction::Buy),
            rule(11, 3, Guard::always_applicable(), Direction::Sell),
        ])));
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::MultipleTotalDirectionRules]
        );
        assert_eq!(defects[0].offending(), occ(1));
        assert!(matches!(defects[0].discriminator(), Discriminator::None));
    }

    // -- TOTAL_RULE_NOT_LOWEST_PRECEDENCE ----------------------------------

    #[test]
    fn one_total_rule_not_at_lowest_precedence_is_not_lowest_only() {
        // Single total rule at precedence 9; a non-total rule sits below it at 1.
        let defects = check_8(&strategy_with(Declared::Present(vec![
            rule(10, 9, Guard::always_applicable(), Direction::Buy),
            rule(11, 1, nonempty_guard(), Direction::Sell),
        ])));
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::TotalRuleNotLowestPrecedence]
        );
        // Member-level: anchored on the offending total rule's occurrence.
        assert_eq!(defects[0].offending(), occ(10));
        assert!(matches!(defects[0].discriminator(), Discriminator::None));
    }

    #[test]
    fn precedence_uses_the_supplied_order_not_collection_position() {
        // The total rule is declared FIRST in the Vec but carries the LOWEST
        // precedence ordinal (1); the non-total rule declared second carries 9.
        // A position-based checker would wrongly flag the total rule; the
        // supplied precedence order must not.
        let defects = check_8(&strategy_with(Declared::Present(vec![
            rule(10, 1, Guard::always_applicable(), Direction::Buy),
            rule(11, 9, nonempty_guard(), Direction::Sell),
        ])));
        assert!(defects.is_empty());
    }

    #[test]
    fn multiple_total_and_not_lowest_co_occur_independently() {
        // ER-0092 RULING-A independence: two total rules (⇒ MULTIPLE_TOTAL) where
        // one (occ 11, prec 9) sits above a strictly-lower rule (prec 1) ⇒ that
        // rule also earns NOT_LOWEST. The lowest total rule (occ 10, prec 1) does
        // not. Both categories are emitted; neither suppresses the other.
        let defects = check_8(&strategy_with(Declared::Present(vec![
            rule(10, 1, Guard::always_applicable(), Direction::Buy),
            rule(11, 9, Guard::always_applicable(), Direction::Sell),
        ])));
        assert_eq!(
            categories(&defects),
            vec![
                DefectCategory::MultipleTotalDirectionRules,
                DefectCategory::TotalRuleNotLowestPrecedence,
            ]
        );
        // The Strategy-level defect anchors on the envelope; the member-level one
        // on the offending rule.
        assert_eq!(defects[0].offending(), occ(1));
        assert_eq!(defects[1].offending(), occ(11));
    }

    #[test]
    fn every_non_lowest_total_rule_earns_its_own_defect() {
        // Two total rules (prec 5, 9) both above a non-total rule at prec 1 ⇒ each
        // total rule is not lowest ⇒ two NOT_LOWEST defects plus one MULTIPLE.
        let defects = check_8(&strategy_with(Declared::Present(vec![
            rule(10, 5, Guard::always_applicable(), Direction::Buy),
            rule(11, 9, Guard::always_applicable(), Direction::Sell),
            rule(12, 1, nonempty_guard(), Direction::Buy),
        ])));
        assert_eq!(
            categories(&defects),
            vec![
                DefectCategory::MultipleTotalDirectionRules,
                DefectCategory::TotalRuleNotLowestPrecedence,
                DefectCategory::TotalRuleNotLowestPrecedence,
            ]
        );
        assert_eq!(defects[1].offending(), occ(10));
        assert_eq!(defects[2].offending(), occ(11));
    }

    // -- Computability boundary (no check-1/2/6 dependency) ----------------

    #[test]
    fn an_absent_guard_rule_is_not_treated_as_always_applicable() {
        // A rule whose guard is absent (check 1's fault) is not the empty guard,
        // so it does not count as a total rule; with no total rule present, the
        // Strategy is coverage-incomplete. Check 8 never manufactures totality.
        let malformed = DirectionRule {
            occurrence: occ(10),
            lineage: Declared::Present(lineage(10)),
            precedence: Declared::Present(precedence(1)),
            guard: Declared::Absent,
            outcome: DeclaredCategory::One(Direction::Buy),
        };
        let defects = check_8(&strategy_with(Declared::Present(vec![malformed])));
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::DirectionCoverageIncomplete]
        );
    }

    #[test]
    fn an_absent_precedence_total_rule_is_not_flagged_not_lowest() {
        // The sole total rule has an absent precedence (check 1's fault). "Not
        // lowest" is not computable for it, so no NOT_LOWEST defect is invented;
        // it is still the only total rule, so no coverage/multiple defect either.
        let total_absent_prec = DirectionRule {
            occurrence: occ(10),
            lineage: Declared::Present(lineage(10)),
            precedence: Declared::Absent,
            guard: Declared::Present(Guard::always_applicable()),
            outcome: DeclaredCategory::One(Direction::Buy),
        };
        let below = rule(11, 1, nonempty_guard(), Direction::Sell);
        let defects = check_8(&strategy_with(Declared::Present(vec![
            total_absent_prec,
            below,
        ])));
        assert!(defects.is_empty());
    }

    #[test]
    fn a_type_defect_in_a_rule_guard_does_not_suppress_check_8() {
        // The total rule's precedence/guard are well-formed, so check 8 passes,
        // regardless of any (hypothetical) type/reference fault a guard atom in
        // the OTHER rule would raise under check 2. Check 8 reads no atom content.
        let defects = check_8(&strategy_with(Declared::Present(vec![
            rule(10, 1, Guard::always_applicable(), Direction::Buy),
            // Non-total rule with an arbitrary atom — check 8 ignores its content.
            rule(11, 9, nonempty_guard(), Direction::Sell),
        ])));
        assert!(defects.is_empty());
    }
}
