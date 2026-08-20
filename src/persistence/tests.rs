//! ADR-0009 §16 conformance vectors for Dogfood-0 persistence (PBA-0181).
//!
//! These live inside the crate so they may assert opaque-handle values through
//! the `pub(crate)` accessors — the same transport surface persistence itself
//! uses — without any public raw-handle reader existing.

use super::*;
use crate::model::condition::{Condition, ConditionClassification};
use crate::model::config::ConfigurationParameter;
use crate::model::declared::{Declared, DeclaredCategory};
use crate::model::derivation::{CandidateDerivation, CandidateRole, DerivationExpression};
use crate::model::direction::{Direction, DirectionRule};
use crate::model::evidence::EvidenceDeclaration;
use crate::model::guard::{Atom, ComparisonOperator, Guard};
use crate::model::literal::Literal;
use crate::model::opaque::{
    DimensionSymbol, EnumValue, LineageId, OccurrenceId, OpaqueValue, PrecedenceOrdinal,
    VersionToken,
};
use crate::model::strategy::Strategy;
use crate::numeric::{Digit, ExactNumber, Sign};
use crate::result::compute_revision_identity;
use crate::semantics::dimension::DimensionBinding;

// -- constructors ----------------------------------------------------------

fn occ(h: u64) -> OccurrenceId {
    OccurrenceId::new(OpaqueValue::new(h))
}
fn lin(h: u64) -> LineageId {
    LineageId::new(OpaqueValue::new(h))
}
fn sym(h: u64) -> DimensionSymbol {
    DimensionSymbol::new(OpaqueValue::new(h))
}
fn env_num(h: u64) -> EnumValue {
    EnumValue::new(OpaqueValue::new(h))
}
fn ver(h: u64) -> VersionToken {
    VersionToken::new(OpaqueValue::new(h))
}
fn prec(h: u64) -> PrecedenceOrdinal {
    PrecedenceOrdinal::new(OpaqueValue::new(h))
}
fn binding(h: u64) -> DimensionBinding {
    DimensionBinding::new(sym(h))
}

fn envelope(strategy: Option<Strategy>, hw: MintHighWater) -> DogfoodEnvelope {
    DogfoodEnvelope {
        format_version: FORMAT_VERSION,
        binding: binding(7),
        supported: ver(3),
        strategy,
        mint_high_water: hw,
    }
}

/// A Strategy exercising every construct, every declaration state, both
/// categorical multiplicities, nested derivation expressions, guards, and an
/// exact decimal `ExactNumber`. Max mint-domain handle present is 50.
fn rich_strategy() -> Strategy {
    // 12.5 = 25/2, a terminating decimal.
    let twelve_five =
        ExactNumber::from_decimal(Sign::Positive, &[Digit::One, Digit::Two], &[Digit::Five]);
    let neg_int = ExactNumber::from_integer(-42);

    let numeric_evidence = EvidenceDeclaration::numeric(occ(10), lin(11), sym(12));
    let boolean_evidence = EvidenceDeclaration::boolean(occ(13), lin(14));
    let enum_evidence = EvidenceDeclaration {
        occurrence: occ(15),
        lineage: Declared::Present(lin(16)),
        declared_type: DeclaredCategory::One(crate::model::evidence::EvidenceTypeTag::Enum),
        declared_dimension: Declared::Absent,
        declared_values: Declared::Present(vec![env_num(17), env_num(18)]),
    };
    // An evidence with an OutOfDomain type and absent lineage (state coverage).
    let odd_evidence = EvidenceDeclaration {
        occurrence: occ(19),
        lineage: Declared::Absent,
        declared_type: DeclaredCategory::OutOfDomain,
        declared_dimension: Declared::Present(sym(20)),
        declared_values: Declared::Present(vec![]), // present-empty
    };

    let config = ConfigurationParameter {
        occurrence: occ(21),
        lineage: Declared::Present(lin(22)),
        dimension: Declared::Present(sym(23)),
        lower_bound: Declared::Present(Literal::Numeric {
            value: neg_int,
            dimension: sym(23),
        }),
        upper_bound: Declared::Present(Literal::Numeric {
            value: twelve_five,
            dimension: sym(23),
        }),
        adjustable: Declared::Present(true),
    };

    let guard = Guard::from_atoms(vec![
        Atom::new(
            lin(24),
            ComparisonOperator::GreaterOrEqual,
            Literal::Boolean(false),
        ),
        Atom::new(
            lin(25),
            ComparisonOperator::NotEqual,
            Literal::Enum(env_num(26)),
        ),
    ]);
    let condition = Condition {
        occurrence: occ(27),
        lineage: Declared::Present(lin(28)),
        classification: DeclaredCategory::One(ConditionClassification::Blocking),
        guard: Declared::Present(guard),
    };
    let condition_multi = Condition {
        occurrence: occ(29),
        lineage: Declared::Absent,
        classification: DeclaredCategory::Multiple {
            first: ConditionClassification::Blocking,
            second: ConditionClassification::Temporary,
            rest: vec![],
        },
        guard: Declared::Absent,
    };

    let direction = DirectionRule {
        occurrence: occ(30),
        lineage: Declared::Present(lin(31)),
        precedence: Declared::Present(prec(32)),
        guard: Declared::Present(Guard::always_applicable()),
        outcome: DeclaredCategory::Multiple {
            first: Direction::Buy,
            second: Direction::Sell,
            rest: vec![],
        },
    };

    // Nested derivation: (evidence(40) + config-ref(41)) * 2
    let expr = DerivationExpression::Multiply {
        operand: Box::new(DerivationExpression::Add {
            left: Box::new(DerivationExpression::EvidenceReference(lin(40))),
            right: Box::new(DerivationExpression::ConfigurationReference(lin(41))),
        }),
        scalar: Literal::Numeric {
            value: ExactNumber::from_integer(2),
            dimension: sym(23),
        },
    };
    let candidate = CandidateDerivation {
        occurrence: occ(45),
        lineage: Declared::Present(lin(46)),
        role: Declared::Present(CandidateRole::Entry),
        expression: Declared::Present(expr),
    };

    Strategy {
        occurrence: occ(1),
        lineage: Declared::Present(lin(50)),
        semantic_model_version: Declared::Present(ver(3)),
        evidence_declarations: Declared::Present(vec![
            numeric_evidence,
            boolean_evidence,
            enum_evidence,
            odd_evidence,
        ]),
        configuration_parameters: Declared::Present(vec![config]),
        conditions: Declared::Present(vec![condition, condition_multi]),
        direction_rules: Declared::Present(vec![direction]),
        candidate_derivations: Declared::Present(vec![candidate]),
    }
}

fn revision(strategy: &Strategy) -> Vec<u8> {
    // A stable fingerprint for equality assertions without needing PartialEq.
    match compute_revision_identity(strategy) {
        Ok(id) => format!("{id:?}").into_bytes(),
        Err(_) => b"<not-well-formed>".to_vec(),
    }
}

// -- 1-5, 23: full round-trip, opaque, context, states, tags, identity -----

#[test]
fn complete_strategy_round_trips_losslessly() {
    let env = envelope(Some(rich_strategy()), MintHighWater::Next(51));
    let bytes1 = serialize_envelope(&env);
    let env2 = deserialize_envelope(&bytes1).expect("valid envelope");
    let bytes2 = serialize_envelope(&env2);
    // Lossless: re-serialization is byte-identical.
    assert_eq!(bytes1, bytes2, "envelope did not round-trip losslessly");
    // Semantic identity is preserved (recomputed, not read from storage).
    assert_eq!(
        revision(env.strategy.as_ref().unwrap()),
        revision(env2.strategy.as_ref().unwrap()),
        "recomputed RevisionIdentity changed across round-trip"
    );
    // Context and high-water round-trip exactly.
    assert_eq!(
        env2.binding.dimensionless().identity_handle(),
        binding(7).dimensionless().identity_handle()
    );
    assert_eq!(env2.supported.identity_handle(), ver(3).identity_handle());
    assert_eq!(env2.mint_high_water, MintHighWater::Next(51));
}

#[test]
fn all_opaque_identities_preserved_exactly() {
    let env = envelope(Some(rich_strategy()), MintHighWater::Next(51));
    let bytes = serialize_envelope(&env);
    let env2 = deserialize_envelope(&bytes).unwrap();
    let s = env2.strategy.unwrap();
    // Spot-check exact handles across domains, read via pub(crate) accessors.
    assert_eq!(s.occurrence.identity_handle(), 1);
    match &s.lineage {
        Declared::Present(l) => assert_eq!(l.identity_handle(), 50),
        Declared::Absent => panic!("lineage lost"),
    }
    // Enum values in an evidence declaration.
    let ev = match &s.evidence_declarations {
        Declared::Present(v) => &v[2],
        Declared::Absent => panic!(),
    };
    match &ev.declared_values {
        Declared::Present(vs) => {
            assert_eq!(vs[0].identity_handle(), 17);
            assert_eq!(vs[1].identity_handle(), 18);
        }
        Declared::Absent => panic!(),
    }
}

#[test]
fn declaration_state_distinctions_are_preserved() {
    // Absent vs present-empty vs out-of-domain must produce distinct bytes.
    let mut absent = rich_strategy();
    absent.conditions = Declared::Absent;
    let mut present_empty = rich_strategy();
    present_empty.conditions = Declared::Present(vec![]);

    let b_absent = serialize_envelope(&envelope(Some(absent), MintHighWater::Next(51)));
    let b_empty = serialize_envelope(&envelope(Some(present_empty), MintHighWater::Next(51)));
    assert_ne!(b_absent, b_empty, "absent vs present-empty collapsed");
}

#[test]
fn enum_tag_distinctions_are_preserved() {
    let mut buy = rich_strategy();
    let mut sell = rich_strategy();
    if let Declared::Present(rules) = &mut buy.direction_rules {
        rules[0].outcome = DeclaredCategory::One(Direction::Buy);
    }
    if let Declared::Present(rules) = &mut sell.direction_rules {
        rules[0].outcome = DeclaredCategory::One(Direction::Sell);
    }
    let b_buy = serialize_envelope(&envelope(Some(buy), MintHighWater::Next(51)));
    let b_sell = serialize_envelope(&envelope(Some(sell), MintHighWater::Next(51)));
    assert_ne!(b_buy, b_sell, "Buy and Sell serialized identically");
}

#[test]
fn none_strategy_round_trips() {
    let env = envelope(None, MintHighWater::Next(0));
    let b = serialize_envelope(&env);
    let env2 = deserialize_envelope(&b).unwrap();
    assert!(env2.strategy.is_none());
    assert_eq!(env2.mint_high_water, MintHighWater::Next(0));
}

// -- 6-10: ExactNumber domain (MINOR-1) ------------------------------------

#[test]
fn exactnumber_huge_valid_round_trips() {
    use num_bigint::BigInt;
    // 10^40 / 2^3 = a huge exact terminating decimal.
    let num: BigInt = "1".parse::<BigInt>().unwrap() * BigInt::from(10).pow(40);
    let den = BigInt::from(8);
    let n = ExactNumber::from_ratio_exact(num.clone(), den).expect("valid terminating decimal");
    // Reconstruct from its own reduced ratio and confirm exact equality.
    let r = n.as_ratio();
    let n2 = ExactNumber::from_ratio_exact(r.numer().clone(), r.denom().clone()).unwrap();
    assert_eq!(n, n2);
}

#[test]
fn exactnumber_rejects_out_of_domain_and_malformed() {
    use num_bigint::BigInt;
    // 1/3 — non-terminating — rejected.
    assert!(ExactNumber::from_ratio_exact(BigInt::from(1), BigInt::from(3)).is_err());
    // denominator zero — rejected.
    assert!(ExactNumber::from_ratio_exact(BigInt::from(1), BigInt::from(0)).is_err());
    // negative denominator — rejected.
    assert!(ExactNumber::from_ratio_exact(BigInt::from(1), BigInt::from(-2)).is_err());
    // valid: zero, one, one-half, negative half.
    assert!(ExactNumber::from_ratio_exact(BigInt::from(0), BigInt::from(1)).is_ok());
    assert!(ExactNumber::from_ratio_exact(BigInt::from(1), BigInt::from(1)).is_ok());
    assert!(ExactNumber::from_ratio_exact(BigInt::from(1), BigInt::from(2)).is_ok());
    assert!(ExactNumber::from_ratio_exact(BigInt::from(-1), BigInt::from(2)).is_ok());
    // 1/20 = 0.05, terminating (den 2^2*5) — accepted.
    assert!(ExactNumber::from_ratio_exact(BigInt::from(1), BigInt::from(20)).is_ok());
}

#[test]
fn out_of_domain_rational_in_envelope_fails_closed() {
    // Serialize a numeric-bearing strategy, then corrupt a denominator to 3.
    let env = envelope(Some(rich_strategy()), MintHighWater::Next(51));
    let json = String::from_utf8(serialize_envelope(&env)).unwrap();
    // The config upper bound is 12.5 = 25/2; make it 25/3, a non-terminating
    // rational outside ExactNumber's decimal domain.
    let corrupted = json.replacen(
        "\"num\":\"25\",\"den\":\"2\"",
        "\"num\":\"25\",\"den\":\"3\"",
        1,
    );
    assert_ne!(
        json, corrupted,
        "test setup: expected the 25/2 literal to corrupt"
    );
    let err = deserialize_envelope(corrupted.as_bytes()).unwrap_err();
    assert_eq!(err, EnvelopeLoadError::RationalOutOfDomain);
}

// -- 11-14: format/version/malformed/high-water ----------------------------

#[test]
fn unknown_format_version_is_rejected() {
    let env = envelope(Some(rich_strategy()), MintHighWater::Next(51));
    let json = String::from_utf8(serialize_envelope(&env)).unwrap();
    let bumped = json.replace("\"format_version\":1", "\"format_version\":2");
    match deserialize_envelope(bumped.as_bytes()) {
        Err(EnvelopeLoadError::UnsupportedVersion(2)) => {}
        other => panic!("expected UnsupportedVersion(2), got {other:?}"),
    }
}

#[test]
fn malformed_and_truncated_json_fail_closed() {
    assert_eq!(
        deserialize_envelope(b"").unwrap_err(),
        EnvelopeLoadError::Malformed
    );
    assert_eq!(
        deserialize_envelope(b"{not json").unwrap_err(),
        EnvelopeLoadError::Malformed
    );
    let env = envelope(Some(rich_strategy()), MintHighWater::Next(51));
    let bytes = serialize_envelope(&env);
    let truncated = &bytes[..bytes.len() / 2];
    assert_eq!(
        deserialize_envelope(truncated).unwrap_err(),
        EnvelopeLoadError::Malformed
    );
}

#[test]
fn invalid_enum_tag_fails_closed() {
    let env = envelope(Some(rich_strategy()), MintHighWater::Next(51));
    let json = String::from_utf8(serialize_envelope(&env)).unwrap();
    // Corrupt a stable operator tag to a non-existent one.
    let corrupted = json.replacen("\"greater_or_equal\"", "\"nonsense_operator\"", 1);
    assert_ne!(json, corrupted, "test setup: expected an operator tag");
    assert_eq!(
        deserialize_envelope(corrupted.as_bytes()).unwrap_err(),
        EnvelopeLoadError::Malformed
    );
}

#[test]
fn high_water_below_corpus_minimum_is_rejected() {
    // rich_strategy has max relevant handle 50 → minimum safe = 51.
    let env = envelope(Some(rich_strategy()), MintHighWater::Next(40));
    let bytes = serialize_envelope(&env);
    assert_eq!(
        deserialize_envelope(&bytes).unwrap_err(),
        EnvelopeLoadError::HighWaterInconsistent
    );
}

// -- 15-20: high-water derivation / exhaustion (MAJOR-1) -------------------

#[test]
fn empty_corpus_has_zero_initial_high_water() {
    let s = Strategy::wholly_undeclared(occ(0));
    // Only occurrence 0 present → max 0 → min safe = 1. Use a fresh occ(0).
    assert_eq!(minimum_safe_high_water(&s), Ok(1));
}

#[test]
fn max_below_u64max_yields_next_plus_one() {
    assert_eq!(minimum_safe_high_water(&rich_strategy()), Ok(51));
    let s = Strategy::wholly_undeclared(occ(u64::MAX - 1));
    assert_eq!(minimum_safe_high_water(&s), Ok(u64::MAX));
}

#[test]
fn u64_max_handle_exhausts_fail_closed() {
    let s = Strategy::wholly_undeclared(occ(u64::MAX));
    assert_eq!(minimum_safe_high_water(&s), Err(HighWaterExhausted));
}

#[test]
fn persisted_exhausted_round_trips_and_survives() {
    // A corpus containing u64::MAX must persist with Exhausted, not Next.
    let s = Strategy::wholly_undeclared(occ(u64::MAX));
    let env = envelope(Some(s), MintHighWater::Exhausted);
    let bytes = serialize_envelope(&env);
    let env2 = deserialize_envelope(&bytes).expect("Exhausted corpus loads");
    assert_eq!(env2.mint_high_water, MintHighWater::Exhausted);
    assert_eq!(env2.mint_high_water.seed(), None); // refuses to seed a minter
}

#[test]
fn exhausted_corpus_with_next_high_water_is_rejected() {
    // Corpus contains u64::MAX but stored Next(_) contradicts exhaustion.
    let s = Strategy::wholly_undeclared(occ(u64::MAX));
    let env = envelope(Some(s), MintHighWater::Next(5));
    let bytes = serialize_envelope(&env);
    assert_eq!(
        deserialize_envelope(&bytes).unwrap_err(),
        EnvelopeLoadError::HighWaterExhausted
    );
}

#[test]
fn issue_never_wraps_to_zero_at_boundary() {
    // Next(u64::MAX) issues u64::MAX exactly once, then Exhausted — never 0.
    let (h, next) = MintHighWater::Next(u64::MAX).issue().unwrap();
    assert_eq!(h, u64::MAX);
    assert_eq!(next, MintHighWater::Exhausted);
    assert!(MintHighWater::Exhausted.issue().is_none()); // no reuse after exhaustion
    // A normal step advances by exactly one, no wrap.
    let (h0, n0) = MintHighWater::Next(5).issue().unwrap();
    assert_eq!(h0, 5);
    assert_eq!(n0, MintHighWater::Next(6));
}

// -- 21-22: mint continuity across restart ---------------------------------

#[test]
fn remove_highest_then_restart_then_mint_never_reuses() {
    // Session 1: corpus with max handle 50; mint a new handle 51 and 52.
    let seed = minimum_safe_high_water(&rich_strategy()).unwrap(); // 51
    let hw = MintHighWater::Next(seed);
    let (h51, hw) = hw.issue().unwrap();
    let (h52, hw) = hw.issue().unwrap();
    assert_eq!((h51, h52), (51, 52));
    // Persist high-water (52 was highest issued → next is 53), then "remove" the
    // highest construct so the present max drops back to 50. Save Next(53).
    let saved = hw; // MintHighWater::Next(53)
    assert_eq!(saved, MintHighWater::Next(53));

    // Session 2 (restart): reload the explicit high-water — it is authoritative
    // and exceeds removed handles (RFC-0004 §14.2.5). The corpus scan alone would
    // wrongly suggest 51; the persisted value prevents reuse of 51/52.
    let env = envelope(Some(rich_strategy()), saved);
    let bytes = serialize_envelope(&env);
    let env2 = deserialize_envelope(&bytes).unwrap();
    let restart_seed = env2.mint_high_water.seed().unwrap();
    assert_eq!(
        restart_seed, 53,
        "removed handles 51/52 must not be reusable"
    );
    let (h_new, _) = env2.mint_high_water.issue().unwrap();
    assert!(
        h_new > 52,
        "next mint must exceed every prior-session handle"
    );
}

#[test]
fn restart_mint_continuity_no_collision() {
    let env = envelope(Some(rich_strategy()), MintHighWater::Next(51));
    let bytes = serialize_envelope(&env);
    let env2 = deserialize_envelope(&bytes).unwrap();
    let seed = env2.mint_high_water.seed().unwrap();
    // Every present handle is < seed, so the first mint cannot collide.
    assert_eq!(seed, 51);
    assert_eq!(
        minimum_safe_high_water(env2.strategy.as_ref().unwrap()),
        Ok(51)
    );
}
