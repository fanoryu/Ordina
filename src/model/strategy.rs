//! The Strategy envelope — Slice Contract §3.4.
//!
//! The envelope composes every other §3 construct. All seven properties are
//! required; §3.4 admits no property outside its table.
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
//!
//! # Why an absent collection is not an empty one
//!
//! §3.4 marks all seven properties required, and the check-13B prerequisite is
//! stated as "check 1 passes for the envelope's candidate-derivation
//! collection" — a check-1 gate that only means something if the collection
//! itself can be missing. The contract also distinguishes the two states
//! elsewhere: configuration parameters and conditions "may be empty", evidence
//! declarations "may be empty only if no reference exists", and "zero Direction
//! rules" is a specific check-8 defect condition rather than a check-1 one.
//!
//! Each collection is therefore a [`Declared`] sequence. Absent is check 1's
//! `MISSING_REQUIRED_PROPERTY`; present-and-empty is whatever the owning check
//! says it is.
//!
//! # What is *not* representable here, and why that is correct
//!
//! §3.4 sends "any property not in this table" to check 16's `UNKNOWN_CONSTRUCT`,
//! fail closed. Nothing below can hold one, and nothing below is intended to.
//! An undeclared property is a fault of the **external shape** presented to the
//! checker, detectable only before a typed `Strategy` exists — by the time this
//! struct has been built, the extra property has already been dropped or the
//! input has already been rejected. Adding an `Unknown` variant, an untyped
//! value tree, or a foreign-content field to make the category constructible
//! inside the typed model would move a boundary condition into the authoritative
//! model, and would amount to designing the boundary that owns it. That boundary
//! is not selected by Slice-1 Phase 1 and is not designed here.
//!
//! The `UNSUPPORTED_SEMANTIC_MODEL_VERSION` half of check 16 is different: it is
//! a typed property of the envelope, and both its faults — a token other than
//! `SM-1`, and an absent token — are representable below.

use crate::model::condition::Condition;
use crate::model::config::ConfigurationParameter;
use crate::model::declared::Declared;
use crate::model::derivation::CandidateDerivation;
use crate::model::direction::DirectionRule;
use crate::model::evidence::EvidenceDeclaration;
use crate::model::opaque::{LineageId, OccurrenceId, VersionToken};

/// A canonical Strategy as Slice 1 receives it — Slice Contract §3.4.
///
/// This is the input to the Well-Formedness checker. It arrives **already
/// canonical** and in memory (§2): nothing here parses, decodes or normalizes,
/// and the checker must not mutate it.
#[derive(Debug, Clone)]
pub struct Strategy {
    /// The envelope's canonical occurrence identity (Revision 4 §C.1). Mandatory
    /// and non-optional: it exists even when the envelope's lineage is absent, and
    /// it is the anchor for envelope- and Strategy/class-level defects (§C.4–§C.6).
    /// It is a distinct domain from `lineage` and never substitutes for it.
    pub occurrence: OccurrenceId,
    /// The envelope is an operative node (§5.1 check 15), so an absent lineage
    /// identity is `MISSING_OPERATIVE_PROVENANCE` under check 15 **exclusively**
    /// (Slice Contract Revision 3 §C.1); check 1 reports nothing for it.
    pub lineage: Declared<LineageId>,
    /// The declared semantic-model version. Check 16 compares it against the
    /// contract-declared constant `SM-1` in Phase 5. Any token is
    /// representable so an unsupported one stays detectable, and **absence** is
    /// representable because §5.1 check 16 names "absent or unsupported
    /// version" as one defect condition.
    pub semantic_model_version: Declared<VersionToken>,
    pub evidence_declarations: Declared<Vec<EvidenceDeclaration>>,
    pub configuration_parameters: Declared<Vec<ConfigurationParameter>>,
    pub conditions: Declared<Vec<Condition>>,
    pub direction_rules: Declared<Vec<DirectionRule>>,
    pub candidate_derivations: Declared<Vec<CandidateDerivation>>,
}

impl Strategy {
    /// A Strategy with every envelope property declared — the well-formed
    /// envelope shape. It says nothing about the contents: the collections are
    /// taken as given, and every check from 1 to 16 still has work to do.
    /// Malformed envelopes are built from the fields directly.
    ///
    /// The envelope has seven declared properties plus its canonical occurrence
    /// identity (Revision 4 §C.1), so the constructor takes eight arguments by
    /// necessity — one per §3.4 property and the occurrence anchor.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        occurrence: OccurrenceId,
        lineage: LineageId,
        semantic_model_version: VersionToken,
        evidence_declarations: Vec<EvidenceDeclaration>,
        configuration_parameters: Vec<ConfigurationParameter>,
        conditions: Vec<Condition>,
        direction_rules: Vec<DirectionRule>,
        candidate_derivations: Vec<CandidateDerivation>,
    ) -> Self {
        Self {
            occurrence,
            lineage: Declared::Present(lineage),
            semantic_model_version: Declared::Present(semantic_model_version),
            evidence_declarations: Declared::Present(evidence_declarations),
            configuration_parameters: Declared::Present(configuration_parameters),
            conditions: Declared::Present(conditions),
            direction_rules: Declared::Present(direction_rules),
            candidate_derivations: Declared::Present(candidate_derivations),
        }
    }

    /// An envelope with **no** property declared at all.
    ///
    /// Every check-1 envelope absence at once, and check 15's and check 16's
    /// absences with them. Phase 1 asserts only that it can be built.
    #[must_use]
    pub const fn wholly_undeclared(occurrence: OccurrenceId) -> Self {
        Self {
            occurrence,
            lineage: Declared::Absent,
            semantic_model_version: Declared::Absent,
            evidence_declarations: Declared::Absent,
            configuration_parameters: Declared::Absent,
            conditions: Declared::Absent,
            direction_rules: Declared::Absent,
            candidate_derivations: Declared::Absent,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::condition::ConditionClassification;
    use crate::model::declared::DeclaredCategory;
    use crate::model::derivation::{CandidateRole, DerivationExpression};
    use crate::model::direction::Direction;
    use crate::model::guard::{Atom, ComparisonOperator, Guard};
    use crate::model::literal::Literal;
    use crate::model::opaque::{DimensionSymbol, OpaqueValue, PrecedenceOrdinal};
    use crate::numeric::ExactNumber;

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
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
            occ(1),
            lineage(1),
            VersionToken::new(OpaqueValue::new(1)),
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
                // The mandatory lowest-precedence always-applicable rule (§3.7).
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

    /// The contents of a `Declared` collection, for tests that only care about
    /// what is inside a present one.
    fn present<T>(declared: &Declared<Vec<T>>) -> &[T] {
        match declared {
            // No catch-all `_` arm (ADR-0005 §7.4).
            Declared::Present(values) => values,
            Declared::Absent => panic!("expected a present collection"),
        }
    }

    #[test]
    fn every_section_3_construct_is_representable_together() {
        let strategy = complete_strategy();

        assert_eq!(present(&strategy.evidence_declarations).len(), 1);
        assert_eq!(present(&strategy.configuration_parameters).len(), 1);
        assert_eq!(present(&strategy.conditions).len(), 1);
        assert_eq!(present(&strategy.direction_rules).len(), 2);
        assert_eq!(present(&strategy.candidate_derivations).len(), 3);
    }

    #[test]
    fn the_checker_input_is_owned_and_needs_no_mutation() {
        // §1.1 requires a pure function that does not mutate its input. Taking
        // the Strategy by shared reference is enough to inspect all of it, and
        // the same reference can be read again afterwards.
        fn count_rules(strategy: &Strategy) -> usize {
            present(&strategy.direction_rules).len()
        }

        let strategy = complete_strategy();

        assert_eq!(count_rules(&strategy), 2);
        assert_eq!(count_rules(&strategy), 2);
    }

    #[test]
    fn check_8_and_13b_faults_remain_representable() {
        // No Direction rules at all, and a duplicated candidate role: check 8
        // and check 13B detect these in Phase 5, so they must be buildable.
        // Note these are *present but empty* / *present but duplicated*
        // collections — a different state from the absent collections below.
        let degenerate = Strategy::new(
            occ(2),
            lineage(2),
            VersionToken::new(OpaqueValue::new(1)),
            vec![],
            vec![],
            vec![],
            vec![],
            vec![
                CandidateDerivation::new(
                    occ(600),
                    lineage(600),
                    CandidateRole::Entry,
                    DerivationExpression::Literal(price_literal(1)),
                ),
                CandidateDerivation::new(
                    occ(601),
                    lineage(601),
                    CandidateRole::Entry,
                    DerivationExpression::Literal(price_literal(2)),
                ),
            ],
        );

        assert!(present(&degenerate.direction_rules).is_empty());
        assert_eq!(present(&degenerate.candidate_derivations).len(), 2);
    }

    #[test]
    fn check_1_and_check_15_envelope_absences_remain_representable() {
        // §3.4 marks all seven properties required. Their absences are owned by
        // three checks (Revision 3 §C.3): the lineage identity by check 15
        // exclusively, the semantic-model version by check 16 exclusively, and
        // the five collections (positions 3-7) by check 1. All seven absences
        // must be representable regardless of which check owns each.
        let bare = Strategy::wholly_undeclared(occ(2));

        assert!(bare.lineage.is_absent());
        assert!(bare.semantic_model_version.is_absent());
        assert!(bare.evidence_declarations.is_absent());
        assert!(bare.configuration_parameters.is_absent());
        assert!(bare.conditions.is_absent());
        assert!(bare.direction_rules.is_absent());
        assert!(bare.candidate_derivations.is_absent());
    }

    #[test]
    fn an_absent_collection_is_not_an_empty_collection() {
        // The distinction check 1 and check 8 depend on: "no candidate-derivation
        // collection" is a different state from "an empty one".
        let empty = complete_strategy();
        let absent = Strategy::wholly_undeclared(occ(2));

        assert!(empty.candidate_derivations.is_present());
        assert!(absent.candidate_derivations.is_absent());
    }

    #[test]
    fn both_check_16_version_faults_remain_representable() {
        // §5.1 check 16: "absent or unsupported version". The unsupported case
        // is any token that is not the contract's `SM-1`; Phase 1 neither knows
        // nor decides which token that is — it only keeps both shapes buildable.
        let unsupported = Strategy::new(
            occ(3),
            lineage(3),
            VersionToken::new(OpaqueValue::new(999)),
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        );
        let absent = Strategy::wholly_undeclared(occ(4));

        assert!(unsupported.semantic_model_version.is_present());
        assert!(absent.semantic_model_version.is_absent());
    }

    #[test]
    fn a_strategy_of_wholly_malformed_constructs_is_representable() {
        // The remediation's point, end to end: an envelope that is itself
        // well-formed may still contain constructs in every malformed state the
        // contract names. None of it is judged here.
        let malformed = Strategy::new(
            occ(4),
            lineage(4),
            VersionToken::new(OpaqueValue::new(1)),
            vec![EvidenceDeclaration {
                occurrence: occ(1000),
                lineage: Declared::Absent,
                declared_type: DeclaredCategory::OutOfDomain,
                declared_dimension: Declared::Present(price()),
                declared_values: Declared::Present(vec![]),
            }],
            vec![ConfigurationParameter {
                occurrence: occ(1001),
                lineage: Declared::Absent,
                dimension: Declared::Absent,
                lower_bound: Declared::Absent,
                upper_bound: Declared::Absent,
                adjustable: Declared::Absent,
            }],
            vec![Condition {
                occurrence: occ(1002),
                lineage: Declared::Absent,
                classification: DeclaredCategory::Absent,
                guard: Declared::Absent,
            }],
            vec![DirectionRule {
                occurrence: occ(1003),
                lineage: Declared::Absent,
                precedence: Declared::Absent,
                guard: Declared::Absent,
                outcome: DeclaredCategory::multiple(Direction::Buy, Direction::Sell, vec![]),
            }],
            vec![CandidateDerivation {
                occurrence: occ(1004),
                lineage: Declared::Absent,
                role: Declared::Absent,
                expression: Declared::Absent,
            }],
        );

        assert_eq!(present(&malformed.evidence_declarations).len(), 1);
        assert_eq!(present(&malformed.configuration_parameters).len(), 1);
        assert_eq!(present(&malformed.conditions).len(), 1);
        assert_eq!(present(&malformed.direction_rules).len(), 1);
        assert_eq!(present(&malformed.candidate_derivations).len(), 1);
    }

    // -- Revision 4 §C.1 canonical occurrence identity ----------------------
    //
    // These assert only the Phase-1 consequence of accepted Revision 4: every
    // construct occurrence carries a mandatory canonical occurrence identity,
    // available even when lineage is absent or duplicated, distinct from lineage,
    // and ordered by the caller-supplied occurrence order. No defect is produced
    // and no check is run here — that is Phase 4 / Phase 5.

    #[test]
    fn every_construct_occurrence_including_the_envelope_exposes_an_occurrence_id() {
        // Universal coverage (§C.1): the envelope and all five child kinds expose
        // a canonical occurrence identity. If any kind lacked one this would not
        // compile, because the field is mandatory and non-optional.
        let strategy = complete_strategy();

        assert_eq!(strategy.occurrence, occ(1));
        assert_eq!(
            present(&strategy.evidence_declarations)[0].occurrence,
            occ(100)
        );
        assert_eq!(
            present(&strategy.configuration_parameters)[0].occurrence,
            occ(200)
        );
        assert_eq!(present(&strategy.conditions)[0].occurrence, occ(300));
        assert_eq!(present(&strategy.direction_rules)[0].occurrence, occ(400));
        assert_eq!(
            present(&strategy.candidate_derivations)[0].occurrence,
            occ(500)
        );
    }

    #[test]
    fn absent_lineage_does_not_imply_absent_occurrence_identity() {
        // §C.1 / §22: a construct whose lineage is absent still has an occurrence
        // identity, so its check-1 / check-15 defect stays anchorable in Phase 4.
        let envelope = Strategy::wholly_undeclared(occ(7));
        assert!(envelope.lineage.is_absent());
        assert_eq!(envelope.occurrence, occ(7));

        let declaration = EvidenceDeclaration {
            occurrence: occ(8),
            lineage: Declared::Absent,
            declared_type: DeclaredCategory::Absent,
            declared_dimension: Declared::Absent,
            declared_values: Declared::Absent,
        };
        assert!(declaration.lineage.is_absent());
        assert_eq!(declaration.occurrence, occ(8));
    }

    #[test]
    fn two_occurrences_may_share_one_lineage_yet_stay_distinct() {
        // §C.1 / §23: duplicate lineage does not collapse occurrences. Both
        // declarations carry lineage 900; their distinct occurrence identities
        // keep them individually addressable (so their per-occurrence defects will
        // not collide under Revision 4 §6.3).
        let a = EvidenceDeclaration::numeric(occ(10), lineage(900), price());
        let b = EvidenceDeclaration::numeric(occ(11), lineage(900), price());

        assert_eq!(a.lineage, b.lineage);
        assert_ne!(a.occurrence, b.occurrence);
    }

    #[test]
    fn identical_content_siblings_stay_distinct_by_occurrence() {
        // §C.1: two byte-identical declarations (same lineage, same fields) remain
        // distinct occurrences. Only the occurrence identity separates them.
        let a = EvidenceDeclaration::boolean(occ(20), lineage(901));
        let b = EvidenceDeclaration::boolean(occ(21), lineage(901));

        assert_eq!(a.lineage, b.lineage);
        assert_eq!(a.declared_type, b.declared_type);
        assert_ne!(a.occurrence, b.occurrence);
    }

    #[test]
    fn occurrence_order_is_the_supplied_order_not_traversal_or_container_order() {
        // §C.1 / §C.3: the occurrence order is the caller-supplied total order,
        // read from the handle — never derived from Vec position or traversal.
        // Here two declarations are placed in a collection in one order while
        // their supplied occurrence order is the *opposite*, so sorting by the
        // supplied order must disagree with the container order.
        let first_in_container = EvidenceDeclaration::boolean(occ(31), lineage(1));
        let second_in_container = EvidenceDeclaration::boolean(occ(30), lineage(2));
        let collection = [first_in_container, second_in_container];

        let container_order: Vec<_> = collection.iter().map(|d| d.occurrence).collect();
        assert_eq!(container_order, vec![occ(31), occ(30)]);

        let mut by_supplied_order = container_order.clone();
        by_supplied_order.sort();
        // The supplied order is occ(30) < occ(31) — not the container order.
        assert_eq!(by_supplied_order, vec![occ(30), occ(31)]);
        assert_ne!(by_supplied_order, container_order);
    }
}
