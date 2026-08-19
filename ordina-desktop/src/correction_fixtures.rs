//! Developer-only canonical `Strategy` fixtures for the correction loop.
//!
//! **Not a Trader authoring path.** These construct canonical Strategies via
//! `ordina`'s public API so `main.rs` has a concrete correction session to
//! drive and so the correction-loop tests have realistic inputs. The
//! `clean_strategy` shape mirrors, field for field, the proven Well-Formed
//! fixture in `ordina`'s own conformance suite (`tests/conformance/main.rs`),
//! rebuilt here through the public API a downstream crate can reach.
//!
//! The distinguished contract inputs match that suite: the supported
//! semantic-model token is handle `1`, and the dimensionless symbol is handle
//! `0`, so a Strategy built here is checkable with [`checker_context`].

use ordina::model::condition::{Condition, ConditionClassification};
use ordina::model::config::ConfigurationParameter;
use ordina::model::declared::Declared;
use ordina::model::derivation::{CandidateDerivation, CandidateRole, DerivationExpression};
use ordina::model::direction::{Direction, DirectionRule};
use ordina::model::evidence::EvidenceDeclaration;
use ordina::model::guard::{Atom, ComparisonOperator, Guard};
use ordina::model::literal::Literal;
use ordina::model::opaque::{
    DimensionSymbol, LineageId, OccurrenceId, OpaqueValue, PrecedenceOrdinal, VersionToken,
};
use ordina::model::strategy::Strategy;
use ordina::numeric::ExactNumber;
use ordina::semantics::dimension::DimensionBinding;

use crate::correction::CheckerContext;

fn occ(handle: u64) -> OccurrenceId {
    OccurrenceId::new(OpaqueValue::new(handle))
}

fn lineage(handle: u64) -> LineageId {
    LineageId::new(OpaqueValue::new(handle))
}

fn price() -> DimensionSymbol {
    DimensionSymbol::new(OpaqueValue::new(1))
}

fn price_literal(value: i64) -> Literal {
    Literal::Numeric {
        value: ExactNumber::from_integer(value),
        dimension: price(),
    }
}

/// The contract-declared supported semantic-model version token (`SM-1`).
#[must_use]
pub fn supported_version() -> VersionToken {
    VersionToken::new(OpaqueValue::new(1))
}

/// The canonical checker context matching these fixtures — the dimensionless
/// symbol (handle `0`) and the supported token (handle `1`).
#[must_use]
pub fn checker_context() -> CheckerContext {
    CheckerContext {
        binding: DimensionBinding::new(DimensionSymbol::new(OpaqueValue::new(0))),
        supported: supported_version(),
    }
}

/// A high-water handle above every one these fixtures use (all `<= 502`), for
/// seeding a session's identity minter (RFC-0004 §14.3 collision-freedom).
pub const MINT_BASE: u64 = 10_000;

/// Every §3 construct present, well-formed, and mutually consistent — mirrors
/// `ordina`'s conformance `clean_strategy()`.
#[must_use]
pub fn clean_strategy() -> Strategy {
    let evidence = lineage(100);
    Strategy::new(
        occ(1),
        lineage(1),
        supported_version(),
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

/// A Strategy identical to [`clean_strategy`] except its semantic-model version
/// is **absent** — the single, correctable defect (check 16,
/// `UNSUPPORTED_SEMANTIC_MODEL_VERSION`). Setting the version to
/// [`supported_version`] and re-checking makes it Well-Formed: the fixture that
/// drives the end-to-end Unusable → edit → Well-Formed → revision-identity flow.
#[must_use]
pub fn unusable_missing_version() -> Strategy {
    let mut strategy = clean_strategy();
    strategy.semantic_model_version = Declared::Absent;
    strategy
}
