//! The Strategy envelope — Slice Contract §3.4.
//!
//! The envelope composes every other §3 construct. All seven properties are
//! required; §3.4 admits no property outside its table, and an unknown property
//! or construct kind fails closed under check 16.
//!
//! # Why the collections are sequences rather than stronger types
//!
//! §3.4 constrains several collections — Direction rules must be non-empty, and
//! candidate derivations must hold each of the three roles exactly once — and
//! it would be tempting to encode those with a non-empty type or a fixed array.
//!
//! That would be a mistake. §5.1 assigns those faults to **check 8** (no
//! Direction rules) and **check 13B** (a missing or duplicated role), and a
//! check cannot detect what cannot be constructed. Phase 1 represents the
//! declared shape; Phase 5 judges it.

use crate::model::condition::Condition;
use crate::model::config::ConfigurationParameter;
use crate::model::derivation::CandidateDerivation;
use crate::model::direction::DirectionRule;
use crate::model::evidence::EvidenceDeclaration;
use crate::model::opaque::{LineageId, VersionToken};

/// A canonical Strategy as Slice 1 receives it — Slice Contract §3.4.
///
/// This is the input to the Well-Formedness checker. It arrives **already
/// canonical** and in memory (§2): nothing here parses, decodes or normalizes,
/// and the checker must not mutate it.
#[derive(Debug, Clone)]
pub struct Strategy {
    pub lineage: LineageId,
    /// The declared semantic-model version. Check 16 compares it against the
    /// contract-declared constant in Phase 5; any token is representable so the
    /// mismatch stays detectable.
    pub semantic_model_version: VersionToken,
    pub evidence_declarations: Vec<EvidenceDeclaration>,
    pub configuration_parameters: Vec<ConfigurationParameter>,
    pub conditions: Vec<Condition>,
    pub direction_rules: Vec<DirectionRule>,
    pub candidate_derivations: Vec<CandidateDerivation>,
}

impl Strategy {
    #[must_use]
    pub const fn new(
        lineage: LineageId,
        semantic_model_version: VersionToken,
        evidence_declarations: Vec<EvidenceDeclaration>,
        configuration_parameters: Vec<ConfigurationParameter>,
        conditions: Vec<Condition>,
        direction_rules: Vec<DirectionRule>,
        candidate_derivations: Vec<CandidateDerivation>,
    ) -> Self {
        Self {
            lineage,
            semantic_model_version,
            evidence_declarations,
            configuration_parameters,
            conditions,
            direction_rules,
            candidate_derivations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::condition::ConditionClassification;
    use crate::model::derivation::{CandidateRole, DerivationExpression};
    use crate::model::direction::Direction;
    use crate::model::evidence::EvidenceType;
    use crate::model::guard::{Atom, ComparisonOperator, Guard};
    use crate::model::literal::Literal;
    use crate::model::opaque::{DimensionSymbol, OpaqueValue, PrecedenceOrdinal};
    use crate::numeric::ExactNumber;

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

    /// Builds a Strategy exercising every §3 construct at once. This is the
    /// Phase-1 exit criterion made concrete: if any construct were not
    /// representable, this would not compile.
    fn complete_strategy() -> Strategy {
        let evidence = lineage(100);

        Strategy::new(
            lineage(1),
            VersionToken::new(OpaqueValue::new(1)),
            vec![EvidenceDeclaration::new(
                evidence,
                EvidenceType::Numeric { dimension: price() },
            )],
            vec![ConfigurationParameter::new(
                lineage(200),
                price(),
                price_literal(1),
                price_literal(10),
                true,
            )],
            vec![Condition::new(
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
                    lineage(400),
                    PrecedenceOrdinal::new(OpaqueValue::new(9)),
                    Guard::from_atoms(vec![Atom::new(
                        evidence,
                        ComparisonOperator::Greater,
                        price_literal(100),
                    )]),
                    Direction::Buy,
                ),
                // The mandatory lowest-precedence always-applicable rule (§3.7).
                DirectionRule::new(
                    lineage(401),
                    PrecedenceOrdinal::new(OpaqueValue::new(1)),
                    Guard::always_applicable(),
                    Direction::Sell,
                ),
            ],
            vec![
                CandidateDerivation::new(
                    lineage(500),
                    CandidateRole::Entry,
                    DerivationExpression::EvidenceReference(evidence),
                ),
                CandidateDerivation::new(
                    lineage(501),
                    CandidateRole::StopLoss,
                    DerivationExpression::Subtract {
                        left: Box::new(DerivationExpression::EvidenceReference(evidence)),
                        right: Box::new(DerivationExpression::ConfigurationReference(lineage(200))),
                    },
                ),
                CandidateDerivation::new(
                    lineage(502),
                    CandidateRole::TakeProfit,
                    DerivationExpression::Literal(price_literal(150)),
                ),
            ],
        )
    }

    #[test]
    fn every_section_3_construct_is_representable_together() {
        let strategy = complete_strategy();

        assert_eq!(strategy.evidence_declarations.len(), 1);
        assert_eq!(strategy.configuration_parameters.len(), 1);
        assert_eq!(strategy.conditions.len(), 1);
        assert_eq!(strategy.direction_rules.len(), 2);
        assert_eq!(strategy.candidate_derivations.len(), 3);
    }

    #[test]
    fn the_checker_input_is_owned_and_needs_no_mutation() {
        // §1.1 requires a pure function that does not mutate its input. Taking
        // the Strategy by shared reference is enough to inspect all of it, and
        // the same reference can be read again afterwards.
        fn count_rules(strategy: &Strategy) -> usize {
            strategy.direction_rules.len()
        }

        let strategy = complete_strategy();

        assert_eq!(count_rules(&strategy), 2);
        assert_eq!(count_rules(&strategy), 2);
    }

    #[test]
    fn check_8_and_13b_faults_remain_representable() {
        // No Direction rules at all, and a duplicated candidate role: check 8
        // and check 13B detect these in Phase 5, so they must be buildable.
        let degenerate = Strategy::new(
            lineage(2),
            VersionToken::new(OpaqueValue::new(1)),
            vec![],
            vec![],
            vec![],
            vec![],
            vec![
                CandidateDerivation::new(
                    lineage(600),
                    CandidateRole::Entry,
                    DerivationExpression::Literal(price_literal(1)),
                ),
                CandidateDerivation::new(
                    lineage(601),
                    CandidateRole::Entry,
                    DerivationExpression::Literal(price_literal(2)),
                ),
            ],
        );

        assert!(degenerate.direction_rules.is_empty());
        assert_eq!(degenerate.candidate_derivations.len(), 2);
    }
}
