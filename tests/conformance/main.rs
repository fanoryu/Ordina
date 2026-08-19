//! Phase-7 conformance test suite — First Runnable Slice Contract §8,
//! obligation 8.
//!
//! The acceptance test for the slice is that two independently written
//! conforming implementations produce the same verdict, defect set, defect
//! identities, defect ordering and not-evaluated metadata for the same
//! input. This target exercises the crate's real public API end-to-end
//! (`Strategy` fixture → `run_phase6` → `finalize_phase6` →
//! `WellFormednessResult`) and asserts on the *complete* observable result,
//! not merely "contains at least one expected item".
//!
//! # Dimension → test map (ER-0121 boundary determination)
//!
//! | # | Dimension                                          | Test(s)                                                              |
//! |---|-----------------------------------------------------|------------------------------------------------------------------------|
//! | 1 | WELL-FORMED outcome                                  | `clean_strategy_is_well_formed_with_no_defects_and_no_not_evaluated`   |
//! | 2 | UNUSABLE outcome                                     | `multiple_defects_yield_unusable_in_exact_canonical_order`             |
//! | 3 | complete defect-set preservation                     | `multiple_defects_yield_unusable_in_exact_canonical_order`             |
//! | 4 | defect identity (category/occurrence/discriminator)  | `multiple_defects_yield_unusable_in_exact_canonical_order`, `defect_identity_includes_the_correct_discriminator` |
//! | 5 | canonical defect ordering                            | `multiple_defects_yield_unusable_in_exact_canonical_order`             |
//! | 6 | permutation-order independence                       | `permuted_input_order_yields_identical_result`                         |
//! | 7 | prerequisite gating                                  | `check2_blocks_14a_blocks_14b_chain`                                   |
//! | 8 | not-evaluated metadata                               | `check2_blocks_14a_blocks_14b_chain`                                   |
//! | 9 | single-unit NotEvaluatedRecord identity              | `check2_blocks_14a_blocks_14b_chain`                                   |
//! |10 | pairwise NotEvaluatedRecord identity/orientation     | `pairwise_not_evaluated_canonicalizes_orientation`                     |
//! |11 | root-blocker propagation (incl. transitive)          | `check2_blocks_14a_blocks_14b_chain`                                   |
//! |12 | unrelated-unit non-suppression                       | `unrelated_defect_does_not_suppress_other_checks`                      |
//! |13 | multiple simultaneous defects                        | `multiple_defects_yield_unusable_in_exact_canonical_order`             |
//! |14 | multiple simultaneous suppressed evaluations         | `check2_blocks_14a_blocks_14b_chain`                                   |
//! |15 | repeated-invocation determinism                      | `repeated_invocation_is_deterministic`                                 |

use ordina::checks::finalize::finalize_phase6;
use ordina::checks::orchestration::run_phase6;
use ordina::model::condition::{Condition, ConditionClassification};
use ordina::model::config::ConfigurationParameter;
use ordina::model::declared::{Declared, DeclaredCategory};
use ordina::model::derivation::{CandidateDerivation, CandidateRole, DerivationExpression};
use ordina::model::direction::{Direction, DirectionRule};
use ordina::model::evidence::{EvidenceDeclaration, EvidenceTypeTag};
use ordina::model::guard::{Atom, ComparisonOperator, Guard};
use ordina::model::literal::Literal;
use ordina::model::opaque::{
    DimensionSymbol, EnumValue, LineageId, OccurrenceId, OpaqueValue, PrecedenceOrdinal,
    VersionToken,
};
use ordina::model::strategy::Strategy;
use ordina::numeric::ExactNumber;
use ordina::result::category::{CheckNumber, DefectCategory};
use ordina::result::discriminator::{Discriminator, PropertyPosition};
use ordina::result::verdict::Verdict;
use ordina::semantics::dimension::DimensionBinding;

// ---------------------------------------------------------------------------
// Fixture helpers — mirror the construction pattern in
// `src/checks/orchestration.rs`'s own `#[cfg(test)]` fixtures, rebuilt here
// using only the crate's public API (external test binaries cannot reach
// crate-internal `#[cfg(test)]` helpers).
// ---------------------------------------------------------------------------

fn occ(handle: u64) -> OccurrenceId {
    OccurrenceId::new(OpaqueValue::new(handle))
}

fn lineage(handle: u64) -> LineageId {
    LineageId::new(OpaqueValue::new(handle))
}

fn price() -> DimensionSymbol {
    DimensionSymbol::new(OpaqueValue::new(1))
}

fn dimensionless() -> DimensionSymbol {
    DimensionSymbol::new(OpaqueValue::new(0))
}

fn price_literal(value: i64) -> Literal {
    Literal::Numeric {
        value: ExactNumber::from_integer(value),
        dimension: price(),
    }
}

fn supported() -> VersionToken {
    VersionToken::new(OpaqueValue::new(1))
}

fn binding() -> DimensionBinding {
    DimensionBinding::new(dimensionless())
}

/// Every §3 construct present, well-formed, and mutually consistent.
fn clean_strategy() -> Strategy {
    let evidence = lineage(100);
    Strategy::new(
        occ(1),
        lineage(1),
        supported(),
        vec![EvidenceDeclaration::numeric(occ(100), evidence, price())],
        vec![ConfigurationParameter::new(
            occ(200),
            lineage(200),
            price(),
            price_literal(1),
            price_literal(10),
            true,
        )],
        vec![Condition::new(
            occ(300),
            lineage(300),
            ConditionClassification::Blocking,
            Guard::from_atoms(vec![Atom::new(
                evidence,
                ComparisonOperator::Greater,
                price_literal(50),
            )]),
        )],
        vec![
            DirectionRule::new(
                occ(400),
                lineage(400),
                PrecedenceOrdinal::new(OpaqueValue::new(9)),
                Guard::from_atoms(vec![Atom::new(
                    evidence,
                    ComparisonOperator::Greater,
                    price_literal(100),
                )]),
                Direction::Buy,
            ),
            DirectionRule::new(
                occ(401),
                lineage(401),
                PrecedenceOrdinal::new(OpaqueValue::new(1)),
                Guard::always_applicable(),
                Direction::Sell,
            ),
        ],
        vec![
            CandidateDerivation::new(
                occ(500),
                lineage(500),
                CandidateRole::Entry,
                DerivationExpression::EvidenceReference(evidence),
            ),
            CandidateDerivation::new(
                occ(501),
                lineage(501),
                CandidateRole::StopLoss,
                DerivationExpression::Subtract {
                    left: Box::new(DerivationExpression::EvidenceReference(evidence)),
                    right: Box::new(DerivationExpression::ConfigurationReference(lineage(200))),
                },
            ),
            CandidateDerivation::new(
                occ(502),
                lineage(502),
                CandidateRole::TakeProfit,
                DerivationExpression::Literal(price_literal(150)),
            ),
        ],
    )
}

fn bad_enum_evidence(occurrence: OccurrenceId, bad_lineage: LineageId) -> EvidenceDeclaration {
    EvidenceDeclaration {
        occurrence,
        lineage: Declared::Present(bad_lineage),
        declared_type: DeclaredCategory::One(EvidenceTypeTag::Enum),
        declared_dimension: Declared::Absent,
        declared_values: Declared::Present(vec![]),
    }
}

fn numeric_evidence(occurrence: OccurrenceId, evidence_lineage: LineageId) -> EvidenceDeclaration {
    EvidenceDeclaration {
        occurrence,
        lineage: Declared::Present(evidence_lineage),
        declared_type: DeclaredCategory::One(EvidenceTypeTag::Numeric),
        declared_dimension: Declared::Present(price()),
        declared_values: Declared::Absent,
    }
}

// ---------------------------------------------------------------------------
// 1: WELL-FORMED outcome.
// ---------------------------------------------------------------------------

/// Proves dimension 1 (WELL-FORMED outcome): a fully clean strategy produces
/// the exact `WellFormed` verdict with empty defect and not-evaluated sets —
/// asserted on the real `WellFormednessResult`, not the raw execution state.
#[test]
fn clean_strategy_is_well_formed_with_no_defects_and_no_not_evaluated() {
    let strategy = clean_strategy();
    let state = run_phase6(&strategy, &binding(), supported());
    let result = finalize_phase6(state);

    assert_eq!(result.verdict(), Verdict::WellFormed);
    assert!(result.defects().is_empty(), "{:?}", result.defects());
    assert!(
        result.not_evaluated().is_empty(),
        "{:?}",
        result.not_evaluated()
    );
}

// ---------------------------------------------------------------------------
// 2, 3, 4, 5, 13: UNUSABLE outcome, complete defect-set preservation, defect
// identity, canonical ordering, multiple simultaneous defects.
// ---------------------------------------------------------------------------

/// Proves dimensions 2 (UNUSABLE outcome), 3 (complete defect-set
/// preservation), 4 (defect identity), 5 (canonical ordering) and 13
/// (multiple simultaneous defects): a strategy with two independent,
/// unrelated defects on two different checks/occurrences produces `Unusable`
/// with EXACTLY those two defects, in exact canonical order (check number,
/// then occurrence, then category, then discriminator), each with the exact
/// expected category and offending occurrence.
#[test]
fn multiple_defects_yield_unusable_in_exact_canonical_order() {
    let mut strategy = clean_strategy();

    // Check 4 defect: inverted configuration bounds on occ(200).
    strategy.configuration_parameters = Declared::Present(vec![ConfigurationParameter::new(
        occ(200),
        lineage(200),
        price(),
        price_literal(10),
        price_literal(1),
        true,
    )]);

    // Check 2 defect: unresolved reference on the ENTRY derivation, occ(500).
    if let Declared::Present(derivations) = &mut strategy.candidate_derivations {
        for derivation in derivations.iter_mut() {
            if derivation.role == Declared::Present(CandidateRole::Entry) {
                derivation.expression =
                    Declared::Present(DerivationExpression::EvidenceReference(lineage(999)));
            }
        }
    }

    let state = run_phase6(&strategy, &binding(), supported());
    let result = finalize_phase6(state);

    assert_eq!(result.verdict(), Verdict::Unusable);
    assert_eq!(result.defects().len(), 2, "{:?}", result.defects());

    // Canonical order: check 2 (UNRESOLVED_REFERENCE on occ(500)) sorts
    // before check 4 (INVERTED_CONFIGURATION_BOUNDS on occ(200)) because
    // check-number is the dominant key.
    let first = result.defects()[0].core();
    assert_eq!(first.offending(), occ(500));
    assert_eq!(first.category(), DefectCategory::UnresolvedReference);

    let second = result.defects()[1].core();
    assert_eq!(second.offending(), occ(200));
    assert_eq!(
        second.category(),
        DefectCategory::InvertedConfigurationBounds
    );

    // Every produced defect is present — no defect is silently dropped.
    assert!(
        result
            .defects()
            .iter()
            .any(|d| d.core().offending() == occ(500)
                && d.core().category() == DefectCategory::UnresolvedReference)
    );
    assert!(
        result
            .defects()
            .iter()
            .any(|d| d.core().offending() == occ(200)
                && d.core().category() == DefectCategory::InvertedConfigurationBounds)
    );
}

// ---------------------------------------------------------------------------
// 4 (discriminator component): defect identity, non-trivial discriminator.
// ---------------------------------------------------------------------------

/// Proves the discriminator component of dimension 4 (defect identity): a
/// malformed enum evidence declaration (empty declared value set) produces a
/// real check-6 `INVALID_EVIDENCE_DECLARATION` defect via the actual pipeline
/// — not a manufactured `DefectIdentity` — whose identity is checked/category/
/// occurrence AND whose `Discriminator` is asserted to be the exact
/// `Property(PropertyPosition::new(4))` (the §3.5 declared-value-set
/// property position). An implementation whose category/occurrence matched
/// but whose discriminator computation was wrong (e.g. the wrong property
/// position, or a different discriminator kind entirely) would fail this
/// test even though `multiple_defects_yield_unusable_in_exact_canonical_order`
/// (which never inspects `.discriminator()`) would not catch it.
#[test]
fn defect_identity_includes_the_correct_discriminator() {
    let mut strategy = clean_strategy();
    strategy.evidence_declarations = Declared::Present(vec![
        numeric_evidence(occ(100), lineage(100)),
        bad_enum_evidence(occ(103), lineage(903)),
    ]);

    let state = run_phase6(&strategy, &binding(), supported());

    assert_eq!(state.defects.len(), 1, "{:?}", state.defects);
    let defect = &state.defects[0];
    assert_eq!(defect.offending(), occ(103));
    assert_eq!(
        defect.category(),
        DefectCategory::InvalidEvidenceDeclaration
    );
    match defect.discriminator() {
        Discriminator::Property(position) => {
            assert_eq!(*position, PropertyPosition::new(4));
        }
        other => panic!("expected Discriminator::Property(4), got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// 6: permutation-order independence.
// ---------------------------------------------------------------------------

/// Proves dimension 6 (permutation-order independence): TWO semantically
/// distinct check-6 defects (malformed evidence declarations at occ(101) and
/// occ(102), otherwise identical in category/discriminator so occurrence is
/// the only ordering key) are fed in raw insertion order **opposite** their
/// canonical order in `forward`, and in canonical-matching insertion order in
/// `backward`. Both must still produce the identical, exact canonical
/// sequence (occ(101) before occ(102)) — a plausible implementation that
/// merely preserves raw insertion/discovery order instead of calling
/// `canonical_sort` would produce occ(102)-before-occ(101) for `forward` and
/// fail the explicit order assertion below, distinguishing it from a
/// genuinely canonical-sorting implementation.
#[test]
fn permuted_input_order_yields_identical_result() {
    let mut forward = clean_strategy();
    // Raw insertion order is occ(102), then occ(101) — the reverse of the
    // canonical (occurrence-ascending) order, so a non-canonicalizing
    // implementation would emit them in the wrong sequence here.
    forward.evidence_declarations = Declared::Present(vec![
        numeric_evidence(occ(100), lineage(100)),
        bad_enum_evidence(occ(102), lineage(902)),
        bad_enum_evidence(occ(101), lineage(901)),
    ]);

    // Raw insertion order now matches canonical order.
    let mut backward = forward.clone();
    if let Declared::Present(items) = &mut backward.evidence_declarations {
        items.reverse();
    }

    let result_a = finalize_phase6(run_phase6(&forward, &binding(), supported()));
    let result_b = finalize_phase6(run_phase6(&backward, &binding(), supported()));

    for (label, result) in [("forward", &result_a), ("backward", &result_b)] {
        assert_eq!(result.verdict(), Verdict::Unusable, "{label}");
        assert_eq!(result.defects().len(), 2, "{label}: {:?}", result.defects());
        let first = result.defects()[0].core();
        assert_eq!(first.offending(), occ(101), "{label}");
        assert_eq!(
            first.category(),
            DefectCategory::InvalidEvidenceDeclaration,
            "{label}"
        );
        let second = result.defects()[1].core();
        assert_eq!(second.offending(), occ(102), "{label}");
        assert_eq!(
            second.category(),
            DefectCategory::InvalidEvidenceDeclaration,
            "{label}"
        );
    }

    assert_eq!(result_a.verdict(), result_b.verdict());
    assert_eq!(result_a.defects().len(), result_b.defects().len());
    assert_eq!(
        result_a.not_evaluated().len(),
        result_b.not_evaluated().len()
    );
    for (a, b) in result_a.defects().iter().zip(result_b.defects().iter()) {
        assert_eq!(a.try_eq(b), Some(true), "{:?} vs {:?}", a, b);
    }
    for (a, b) in result_a
        .not_evaluated()
        .iter()
        .zip(result_b.not_evaluated().iter())
    {
        assert_eq!(a.check(), b.check());
        assert_eq!(a.construct(), b.construct());
        assert_eq!(a.counterpart(), b.counterpart());
        assert_eq!(a.blocking().try_eq(b.blocking()), Some(true));
    }
}

// ---------------------------------------------------------------------------
// 7, 8, 9, 11, 14: prerequisite gating, not-evaluated metadata, single-unit
// identity, root-blocker propagation (transitive), multiple simultaneous
// suppressed evaluations.
// ---------------------------------------------------------------------------

/// Proves dimensions 7 (prerequisite gating), 8 (not-evaluated metadata), 9
/// (single-unit `NotEvaluatedRecord` identity: `construct` set, `counterpart
/// == None`), 11 (root-blocker propagation across a transitive chain: check 2
/// blocks 13A and 14A for the ENTRY derivation, which then transitively
/// blocks 14B — the SAME root-blocker identity is inherited, never
/// re-derived) and 14 (multiple simultaneous suppressed evaluations: 13A,
/// 14A and 14B are all suppressed by one root cause in one run).
#[test]
fn check2_blocks_14a_blocks_14b_chain() {
    let mut strategy = clean_strategy();
    // ENTRY references a nonexistent evidence lineage: UNRESOLVED_REFERENCE
    // anchored on the ENTRY derivation's own occurrence, occ(500).
    if let Declared::Present(derivations) = &mut strategy.candidate_derivations {
        for derivation in derivations.iter_mut() {
            if derivation.role == Declared::Present(CandidateRole::Entry) {
                derivation.expression =
                    Declared::Present(DerivationExpression::EvidenceReference(lineage(999)));
            }
        }
    }

    let state = run_phase6(&strategy, &binding(), supported());

    // The real upstream defect: check 2's UNRESOLVED_REFERENCE on occ(500).
    let root_defect = state
        .defects
        .iter()
        .find(|d| d.offending() == occ(500) && d.category() == DefectCategory::UnresolvedReference)
        .expect("check 2 must produce the root UNRESOLVED_REFERENCE defect");

    // Exactly three suppressed units in this run: 13A/ENTRY, 14A/ENTRY,
    // 14B/envelope — dimension 14 (multiple simultaneous suppressions).
    assert_eq!(state.not_evaluated.len(), 3, "{:?}", state.not_evaluated);

    // 13A/ENTRY — single-unit identity (dimension 9).
    let record_13a = state
        .not_evaluated
        .iter()
        .find(|r| matches!(r.check(), CheckNumber::C13A))
        .expect("13A must be not-evaluated for ENTRY");
    assert_eq!(record_13a.construct(), occ(500));
    assert_eq!(record_13a.counterpart(), None);
    assert_eq!(
        record_13a.blocking().core().offending(),
        root_defect.offending()
    );
    assert_eq!(
        record_13a.blocking().core().category(),
        root_defect.category()
    );

    // 14A/ENTRY — single-unit identity, same root blocker (dimension 11).
    let record_14a = state
        .not_evaluated
        .iter()
        .find(|r| matches!(r.check(), CheckNumber::C14A))
        .expect("14A must be not-evaluated for ENTRY");
    assert_eq!(record_14a.construct(), occ(500));
    assert_eq!(record_14a.counterpart(), None);
    assert_eq!(
        record_14a.blocking().core().offending(),
        root_defect.offending()
    );
    assert_eq!(
        record_14a.blocking().core().category(),
        root_defect.category()
    );

    // 14B/envelope — transitively inherits the SAME root-blocker identity
    // (dimension 11: root-blocker propagation, including transitive chains).
    let record_14b = state
        .not_evaluated
        .iter()
        .find(|r| matches!(r.check(), CheckNumber::C14B))
        .expect("14B must be not-evaluated, transitively blocked via 14A");
    assert_eq!(record_14b.construct(), occ(1));
    assert_eq!(record_14b.counterpart(), None);
    assert_eq!(
        record_14b.blocking().try_eq(record_14a.blocking()),
        Some(true),
        "14B's root blocker must be the same identity inherited from 14A, never re-derived"
    );
    assert_eq!(
        record_14a.blocking().try_eq(record_13a.blocking()),
        Some(true),
        "14A's root blocker must be the same identity as 13A's, never re-derived"
    );

    // The whole chain survives finalize_phase6 unchanged (Unusable, with the
    // not-evaluated set carried through verbatim).
    let result = finalize_phase6(state);
    assert_eq!(result.verdict(), Verdict::Unusable);
    assert_eq!(result.not_evaluated().len(), 3);
}

// ---------------------------------------------------------------------------
// 10: pairwise NotEvaluatedRecord identity / canonical orientation.
// ---------------------------------------------------------------------------

/// Proves dimension 10 (pairwise `NotEvaluatedRecord` identity and canonical
/// orientation): two Direction rules whose guards both reference the same
/// malformed evidence declaration produce a pairwise-gated check-9
/// not-evaluated unit whose `construct`/`counterpart` are canonicalized to
/// the lower-sorting occurrence regardless of declaration order — proven by
/// building the same fixture with the rule occurrences in the opposite
/// insertion order and observing the identical canonical orientation.
#[test]
fn pairwise_not_evaluated_canonicalizes_orientation() {
    fn strategy_with_rule_order(first: OccurrenceId, second: OccurrenceId) -> Strategy {
        let mut strategy = clean_strategy();
        let bad_evidence = lineage(901);
        strategy.evidence_declarations = Declared::Present(vec![
            numeric_evidence(occ(100), lineage(100)),
            bad_enum_evidence(occ(102), bad_evidence),
        ]);
        strategy.direction_rules = Declared::Present(vec![
            DirectionRule::new(
                first,
                lineage(400),
                PrecedenceOrdinal::new(OpaqueValue::new(9)),
                Guard::from_atoms(vec![Atom::new(
                    bad_evidence,
                    ComparisonOperator::Equal,
                    Literal::Enum(EnumValue::new(OpaqueValue::new(1))),
                )]),
                Direction::Buy,
            ),
            DirectionRule::new(
                second,
                lineage(401),
                PrecedenceOrdinal::new(OpaqueValue::new(1)),
                Guard::always_applicable(),
                Direction::Sell,
            ),
        ]);
        strategy
    }

    let low_first = strategy_with_rule_order(occ(400), occ(401));
    let high_first = strategy_with_rule_order(occ(401), occ(400));

    let state_low_first = run_phase6(&low_first, &binding(), supported());
    let state_high_first = run_phase6(&high_first, &binding(), supported());

    for state in [&state_low_first, &state_high_first] {
        let record = state
            .not_evaluated
            .iter()
            .find(|r| matches!(r.check(), CheckNumber::C9))
            .expect("expected a pairwise check-9 not-evaluated record");
        assert_eq!(record.construct(), occ(400));
        assert_eq!(record.counterpart(), Some(occ(401)));
    }
}

// ---------------------------------------------------------------------------
// 12: unrelated-unit non-suppression.
// ---------------------------------------------------------------------------

/// Proves dimension 12 (unrelated-unit non-suppression): a defect on check 4
/// (inverted configuration bounds), which has no §5.2 prerequisite edge, does
/// not suppress any other check — the not-evaluated set stays completely
/// empty even though the defect set is non-empty.
#[test]
fn unrelated_defect_does_not_suppress_other_checks() {
    let mut strategy = clean_strategy();
    strategy.configuration_parameters = Declared::Present(vec![ConfigurationParameter::new(
        occ(200),
        lineage(200),
        price(),
        price_literal(10),
        price_literal(1),
        true,
    )]);

    let state = run_phase6(&strategy, &binding(), supported());

    assert_eq!(state.defects.len(), 1);
    assert_eq!(
        state.defects[0].category(),
        DefectCategory::InvertedConfigurationBounds
    );
    assert!(state.not_evaluated.is_empty(), "{:?}", state.not_evaluated);

    let result = finalize_phase6(state);
    assert_eq!(result.verdict(), Verdict::Unusable);
    assert!(result.not_evaluated().is_empty());
}

// ---------------------------------------------------------------------------
// 15: repeated-invocation determinism.
// ---------------------------------------------------------------------------

/// Proves dimension 15 (repeated-invocation determinism): invoking the full
/// pipeline twice on the identical input produces an identical complete
/// result — verdict, full defect sequence (by identity), and full
/// not-evaluated sequence (by identity) — for both a well-formed input and an
/// unusable one with prerequisite gating active.
#[test]
fn repeated_invocation_is_deterministic() {
    // Well-formed case.
    let clean = clean_strategy();
    let r1 = finalize_phase6(run_phase6(&clean, &binding(), supported()));
    let r2 = finalize_phase6(run_phase6(&clean, &binding(), supported()));
    assert_eq!(r1.verdict(), r2.verdict());
    assert_eq!(r1.defects().len(), r2.defects().len());
    assert_eq!(r1.not_evaluated().len(), r2.not_evaluated().len());

    // Unusable + gating case: reuse the check2-blocks-14a-blocks-14b fixture.
    let mut strategy = clean_strategy();
    if let Declared::Present(derivations) = &mut strategy.candidate_derivations {
        for derivation in derivations.iter_mut() {
            if derivation.role == Declared::Present(CandidateRole::Entry) {
                derivation.expression =
                    Declared::Present(DerivationExpression::EvidenceReference(lineage(999)));
            }
        }
    }

    let a = finalize_phase6(run_phase6(&strategy, &binding(), supported()));
    let b = finalize_phase6(run_phase6(&strategy, &binding(), supported()));

    assert_eq!(a.verdict(), b.verdict());
    assert_eq!(a.defects().len(), b.defects().len());
    for (x, y) in a.defects().iter().zip(b.defects().iter()) {
        assert_eq!(x.try_eq(y), Some(true));
    }
    assert_eq!(a.not_evaluated().len(), b.not_evaluated().len());
    for (x, y) in a.not_evaluated().iter().zip(b.not_evaluated().iter()) {
        assert_eq!(x.check(), y.check());
        assert_eq!(x.construct(), y.construct());
        assert_eq!(x.counterpart(), y.counterpart());
        assert_eq!(x.blocking().try_eq(y.blocking()), Some(true));
    }
}
