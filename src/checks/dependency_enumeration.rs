//! Phase-6 Slice 6A — structural dependency enumeration (PBA-0135 / ER-0110).
//!
//! Slice 6A owns exactly one thing: **pure structural enumeration** of the
//! declaration lineages a [`Guard`] or a [`DerivationExpression`] structurally
//! depends on. This is the raw material later Phase-6 slices (6B) need to
//! correlate one unit of work against another check's already-produced
//! defects, per the Slice Contract §5.2 prerequisite graph — for example,
//! "check 6 passes for every evidence declaration this guard references"
//! (checks 9, 11, 12) or "check 2 passes for that derivation's leaves" (check
//! 14A).
//!
//! # What Slice 6A does **not** do
//!
//! This module answers only "which lineages does this structure mention,"
//! never "is that mention valid." It performs:
//!
//! - **no** reference resolution (Check 2's exact-one-resolution rule);
//! - **no** type or dimension validation (Checks 2, 6, 14A, 14B);
//! - **no** `additive_dimension` / `scaling_dimension` computation;
//! - **no** guard-analysis (overlap, subsumption, contradiction — Checks 9,
//!   11, 12's `guard_analysis` helper);
//! - **no** derivation-form or role-completeness validation (Checks 13A,
//!   13B);
//! - **no** prerequisite-satisfaction decision, `NotEvaluatedRecord`
//!   construction, blocker selection, defect aggregation, canonical sorting,
//!   or verdict assembly — all Slice 6B/6C/6D and later.
//!
//! It calls no prior check, needs no [`crate::semantics::dimension::DimensionBinding`],
//! and produces no [`crate::result::identity::DefectCore`] of any kind.
//!
//! # Dependency membership is a set, not a multiset
//!
//! §5.2's prerequisite edges are phrased as membership questions — "check 6
//! passes for every evidence declaration THAT guard's references" — never as
//! multiplicity questions. A guard or derivation that mentions the same
//! lineage twice has exactly one dependency on it, not two. Both helpers
//! therefore return a **deduplicated, canonically-ordered** `Vec<LineageId>`:
//! the declared §3.3.1 total order sorts it, `dedup` (post-sort) collapses
//! repeated mentions, and the result is independent of `Guard.atoms` /
//! `DerivationExpression` traversal or storage order. This mirrors the
//! existing sort-then-dedup pattern in
//! [`crate::checks::guard_analysis::every_reference`] — the same contract-valid
//! mechanism, reused for the same reason (ADR-0005 §7.6: no `HashMap`/`HashSet`
//! iteration order may influence an authoritative result), applied here to a
//! narrower, dimension-agnostic enumeration.
//!
//! # `Guard.atoms` (O-4)
//!
//! [`guard_dependencies`] reads `Guard.atoms` **structurally only**: it visits
//! each atom once to read its `evidence` lineage, never mutates the guard,
//! never assigns semantic meaning (empty-conjunction, overlap, contradiction)
//! to the atom set, and never depends on `atoms`' storage order for its
//! result (permutation changes traversal order only; the sorted, deduplicated
//! output is identical — proven by test). This is not a new interpretation of
//! guard semantics and does not touch O-4, which **remains OPEN/prospective**.
//!
//! # Lineage-to-occurrence resolution (bounded remediation, PBA-0136 / ER-0111)
//!
//! [`guard_dependencies`]/[`derivation_dependencies`] answer "which lineages
//! does this structure mention" — but a later slice correlating against
//! Check-6's or Check-1's already-produced defects needs the **occurrence**
//! identity those checks anchor on, not the lineage alone: Check 6
//! (`INVALID_EVIDENCE_DECLARATION`) and a malformed evidence/configuration
//! declaration's Check-1 defect are both anchored on **the offending
//! declaration's own occurrence** (§6.3), never on its lineage. [`evidence_occurrence`]
//! and [`configuration_occurrence`] close that gap with a **semantics-free
//! structural lookup**: given a target lineage and the declaration collection
//! it names, they return the **unique** matching declaration's occurrence, or
//! `None` if zero or more than one declaration shares that lineage.
//!
//! This is **not** Check 2's reference-resolution rule restated: it makes no
//! judgement about whether a *reference* is well-formed, emits no defect, and
//! answers a narrower question a later correlation step needs regardless of
//! whether the reference itself is valid — "if exactly one declaration has
//! this lineage, where is it." Zero or ambiguous matches both yield `None`,
//! matching the exact-one shape §4.7 already establishes, but this module
//! draws no conclusion from that shape — it neither reports a defect nor
//! decides that a check should be suppressed. Evidence and configuration
//! resolution are kept as two **separate** functions, each searching only its
//! own declaration collection — `LineageId` values from the two domains are
//! never compared across collections, and nothing here assumes a lineage
//! naming an evidence item cannot coincidentally equal one naming a
//! configuration parameter.

use crate::model::declared::Declared;
use crate::model::derivation::DerivationExpression;
use crate::model::guard::Guard;
use crate::model::opaque::{LineageId, OccurrenceId};
use crate::model::strategy::Strategy;

/// The evidence-declaration lineages `guard` structurally references.
///
/// One entry per **distinct** lineage named by an atom's `evidence` field,
/// canonically ordered (§3.3.1) and deduplicated — independent of
/// `guard.atoms`' storage order. An empty guard (the always-applicable
/// conjunction, §4.5) has no dependencies.
#[must_use]
pub fn guard_dependencies(guard: &Guard) -> Vec<LineageId> {
    let mut lineages: Vec<LineageId> = guard.atoms.iter().map(|atom| atom.evidence).collect();
    lineages.sort();
    lineages.dedup();
    lineages
}

/// The evidence/configuration-declaration lineages `expression` structurally
/// references.
///
/// One entry per **distinct** lineage named by an [`DerivationExpression::EvidenceReference`]
/// or [`DerivationExpression::ConfigurationReference`] leaf anywhere in the
/// tree, canonically ordered (§3.3.1) and deduplicated. A `Literal` leaf
/// contributes no dependency — it names no declaration. `Add`/`Subtract`/
/// `Multiply` recurse into every child; `Multiply`'s `scalar` is a bare
/// [`crate::model::literal::Literal`], never a further expression, so only its
/// `operand` subtree is walked. Exhaustive match over the closed six-variant
/// enum, no catch-all arm (ADR-0005 §7.4) — a future grammar variant forces
/// this function to be reconsidered at compile time.
#[must_use]
pub fn derivation_dependencies(expression: &DerivationExpression) -> Vec<LineageId> {
    let mut lineages = Vec::new();
    collect_derivation_dependencies(expression, &mut lineages);
    lineages.sort();
    lineages.dedup();
    lineages
}

/// Recursive collection step for [`derivation_dependencies`]; appends every
/// referenced lineage found, in traversal order, with duplicates — the
/// caller sorts and dedups once at the end.
fn collect_derivation_dependencies(expression: &DerivationExpression, into: &mut Vec<LineageId>) {
    match expression {
        // No catch-all `_` arm (ADR-0005 §7.4).
        DerivationExpression::Literal(_) => {}
        DerivationExpression::EvidenceReference(lineage)
        | DerivationExpression::ConfigurationReference(lineage) => into.push(*lineage),
        DerivationExpression::Add { left, right }
        | DerivationExpression::Subtract { left, right } => {
            collect_derivation_dependencies(left, into);
            collect_derivation_dependencies(right, into);
        }
        DerivationExpression::Multiply { operand, .. } => {
            collect_derivation_dependencies(operand, into);
        }
    }
}

/// The occurrence identity of the **unique** evidence declaration carrying
/// `lineage`, or `None` if zero or more than one declaration in
/// `strategy.evidence_declarations` shares it (including an absent
/// collection).
///
/// A structural identity lookup only — it draws no conclusion from the
/// zero/ambiguous case beyond `None`; it emits no defect and makes no
/// prerequisite-satisfaction judgement. Searches only the evidence
/// collection; a configuration parameter sharing the same lineage value is
/// never considered.
#[must_use]
pub fn evidence_occurrence(strategy: &Strategy, lineage: LineageId) -> Option<OccurrenceId> {
    let Declared::Present(items) = &strategy.evidence_declarations else {
        return None;
    };
    let mut resolved = None;
    for item in items {
        if item.lineage == Declared::Present(lineage) {
            if resolved.is_some() {
                return None;
            }
            resolved = Some(item.occurrence);
        }
    }
    resolved
}

/// The occurrence identity of the **unique** configuration parameter carrying
/// `lineage`, or `None` if zero or more than one parameter in
/// `strategy.configuration_parameters` shares it (including an absent
/// collection).
///
/// A structural identity lookup only, mirroring [`evidence_occurrence`] over
/// the separate configuration-parameter collection — never the evidence
/// collection.
#[must_use]
pub fn configuration_occurrence(strategy: &Strategy, lineage: LineageId) -> Option<OccurrenceId> {
    let Declared::Present(items) = &strategy.configuration_parameters else {
        return None;
    };
    let mut resolved = None;
    for item in items {
        if item.lineage == Declared::Present(lineage) {
            if resolved.is_some() {
                return None;
            }
            resolved = Some(item.occurrence);
        }
    }
    resolved
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::config::ConfigurationParameter;
    use crate::model::derivation::DerivationExpression;
    use crate::model::evidence::EvidenceDeclaration;
    use crate::model::guard::{Atom, ComparisonOperator, Guard};
    use crate::model::literal::Literal;
    use crate::model::opaque::{DimensionSymbol, LineageId, OccurrenceId, OpaqueValue};
    use crate::numeric::ExactNumber;

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn dim(handle: u64) -> DimensionSymbol {
        DimensionSymbol::new(OpaqueValue::new(handle))
    }

    fn strategy_with(
        evidence: Vec<EvidenceDeclaration>,
        configuration: Vec<ConfigurationParameter>,
    ) -> Strategy {
        Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(lineage(1000)),
            semantic_model_version: Declared::Absent,
            evidence_declarations: Declared::Present(evidence),
            configuration_parameters: Declared::Present(configuration),
            conditions: Declared::Present(vec![]),
            direction_rules: Declared::Present(vec![]),
            candidate_derivations: Declared::Present(vec![]),
        }
    }

    fn strategy_with_absent_collections() -> Strategy {
        Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(lineage(1000)),
            semantic_model_version: Declared::Absent,
            evidence_declarations: Declared::Absent,
            configuration_parameters: Declared::Absent,
            conditions: Declared::Present(vec![]),
            direction_rules: Declared::Present(vec![]),
            candidate_derivations: Declared::Present(vec![]),
        }
    }

    fn numeric_literal(value: i64) -> Literal {
        Literal::Numeric {
            value: ExactNumber::from_integer(value),
            dimension: DimensionSymbol::new(OpaqueValue::new(1)),
        }
    }

    fn atom(evidence_handle: u64) -> Atom {
        Atom::new(
            lineage(evidence_handle),
            ComparisonOperator::Greater,
            numeric_literal(1),
        )
    }

    // == guard enumeration ===================================================

    #[test]
    fn empty_guard_has_no_dependencies() {
        assert!(guard_dependencies(&Guard::always_applicable()).is_empty());
    }

    #[test]
    fn one_referenced_declaration() {
        let guard = Guard::from_atoms(vec![atom(1)]);
        assert_eq!(guard_dependencies(&guard), vec![lineage(1)]);
    }

    #[test]
    fn multiple_different_referenced_declarations() {
        let guard = Guard::from_atoms(vec![atom(3), atom(1), atom(2)]);
        // Canonically ordered, not traversal-ordered.
        assert_eq!(
            guard_dependencies(&guard),
            vec![lineage(1), lineage(2), lineage(3)]
        );
    }

    #[test]
    fn repeated_structural_reference_to_the_same_declaration_is_one_dependency() {
        let guard = Guard::from_atoms(vec![
            Atom::new(lineage(1), ComparisonOperator::Greater, numeric_literal(1)),
            Atom::new(lineage(1), ComparisonOperator::Less, numeric_literal(9)),
        ]);
        assert_eq!(guard_dependencies(&guard), vec![lineage(1)]);
    }

    #[test]
    fn permuted_guard_atoms_yield_an_identical_result() {
        let forward = Guard::from_atoms(vec![atom(1), atom(2), atom(3)]);
        let reversed = Guard::from_atoms(vec![atom(3), atom(2), atom(1)]);
        assert_eq!(guard_dependencies(&forward), guard_dependencies(&reversed));
    }

    #[test]
    fn multiple_atom_forms_all_contribute_their_evidence_lineage() {
        // Every ComparisonOperator variant and every Literal kind is a valid
        // atom form; each contributes only its `evidence` lineage regardless
        // of operator or literal kind.
        let guard = Guard::from_atoms(vec![
            Atom::new(lineage(1), ComparisonOperator::Less, numeric_literal(1)),
            Atom::new(
                lineage(2),
                ComparisonOperator::LessOrEqual,
                Literal::Boolean(true),
            ),
            Atom::new(
                lineage(3),
                ComparisonOperator::NotEqual,
                Literal::Enum(crate::model::opaque::EnumValue::new(OpaqueValue::new(9))),
            ),
        ]);
        assert_eq!(
            guard_dependencies(&guard),
            vec![lineage(1), lineage(2), lineage(3)]
        );
    }

    // == derivation enumeration ==============================================

    #[test]
    fn numeric_literal_only_has_no_dependencies() {
        let expr = DerivationExpression::Literal(numeric_literal(5));
        assert!(derivation_dependencies(&expr).is_empty());
    }

    #[test]
    fn evidence_reference_is_its_own_dependency() {
        let expr = DerivationExpression::EvidenceReference(lineage(1));
        assert_eq!(derivation_dependencies(&expr), vec![lineage(1)]);
    }

    #[test]
    fn configuration_reference_is_its_own_dependency() {
        let expr = DerivationExpression::ConfigurationReference(lineage(1));
        assert_eq!(derivation_dependencies(&expr), vec![lineage(1)]);
    }

    #[test]
    fn add_recurses_into_both_operands() {
        let expr = DerivationExpression::Add {
            left: Box::new(DerivationExpression::EvidenceReference(lineage(1))),
            right: Box::new(DerivationExpression::ConfigurationReference(lineage(2))),
        };
        assert_eq!(derivation_dependencies(&expr), vec![lineage(1), lineage(2)]);
    }

    #[test]
    fn subtract_recurses_into_both_operands() {
        let expr = DerivationExpression::Subtract {
            left: Box::new(DerivationExpression::EvidenceReference(lineage(1))),
            right: Box::new(DerivationExpression::ConfigurationReference(lineage(2))),
        };
        assert_eq!(derivation_dependencies(&expr), vec![lineage(1), lineage(2)]);
    }

    #[test]
    fn multiply_recurses_only_into_the_operand_never_the_literal_scalar() {
        let expr = DerivationExpression::Multiply {
            operand: Box::new(DerivationExpression::EvidenceReference(lineage(1))),
            scalar: numeric_literal(3),
        };
        assert_eq!(derivation_dependencies(&expr), vec![lineage(1)]);
    }

    #[test]
    fn deeply_nested_expression_collects_every_leaf_reference() {
        let expr = DerivationExpression::Add {
            left: Box::new(DerivationExpression::Multiply {
                operand: Box::new(DerivationExpression::EvidenceReference(lineage(1))),
                scalar: numeric_literal(2),
            }),
            right: Box::new(DerivationExpression::Subtract {
                left: Box::new(DerivationExpression::ConfigurationReference(lineage(2))),
                right: Box::new(DerivationExpression::Literal(numeric_literal(9))),
            }),
        };
        assert_eq!(derivation_dependencies(&expr), vec![lineage(1), lineage(2)]);
    }

    #[test]
    fn repeated_reference_is_one_dependency() {
        let expr = DerivationExpression::Add {
            left: Box::new(DerivationExpression::EvidenceReference(lineage(1))),
            right: Box::new(DerivationExpression::EvidenceReference(lineage(1))),
        };
        assert_eq!(derivation_dependencies(&expr), vec![lineage(1)]);
    }

    #[test]
    fn multiple_distinct_references_are_canonically_ordered() {
        let expr = DerivationExpression::Add {
            left: Box::new(DerivationExpression::EvidenceReference(lineage(3))),
            right: Box::new(DerivationExpression::ConfigurationReference(lineage(1))),
        };
        assert_eq!(derivation_dependencies(&expr), vec![lineage(1), lineage(3)]);
    }

    #[test]
    fn structurally_equivalent_trees_with_swapped_operand_order_yield_an_identical_result() {
        let a = DerivationExpression::Add {
            left: Box::new(DerivationExpression::EvidenceReference(lineage(1))),
            right: Box::new(DerivationExpression::ConfigurationReference(lineage(2))),
        };
        let b = DerivationExpression::Add {
            left: Box::new(DerivationExpression::ConfigurationReference(lineage(2))),
            right: Box::new(DerivationExpression::EvidenceReference(lineage(1))),
        };
        assert_eq!(derivation_dependencies(&a), derivation_dependencies(&b));
    }

    #[test]
    fn repeated_invocation_gives_identical_output() {
        let expr = DerivationExpression::Add {
            left: Box::new(DerivationExpression::EvidenceReference(lineage(1))),
            right: Box::new(DerivationExpression::ConfigurationReference(lineage(2))),
        };
        assert_eq!(
            derivation_dependencies(&expr),
            derivation_dependencies(&expr)
        );
    }

    // == boundary: no defects, no resolution, no dimension, no gating =======

    #[test]
    fn helper_return_types_carry_no_defect_or_result_machinery() {
        // Both helpers return Vec<LineageId> only. This test exists to make the
        // boundary explicit and self-documenting: if either signature ever
        // changes to return a DefectCore, NotEvaluatedRecord, or
        // WellFormednessResult, this test's own type ascriptions stop
        // compiling, forcing the change to be noticed.
        let guard_result: Vec<LineageId> = guard_dependencies(&Guard::always_applicable());
        let derivation_result: Vec<LineageId> =
            derivation_dependencies(&DerivationExpression::Literal(numeric_literal(1)));
        assert!(guard_result.is_empty());
        assert!(derivation_result.is_empty());
    }

    // == evidence_occurrence =================================================

    #[test]
    fn evidence_absent_collection_resolves_to_none() {
        let strategy = strategy_with_absent_collections();
        assert_eq!(evidence_occurrence(&strategy, lineage(1)), None);
    }

    #[test]
    fn evidence_zero_matching_lineage_resolves_to_none() {
        let strategy = strategy_with(
            vec![EvidenceDeclaration::numeric(occ(10), lineage(1), dim(1))],
            vec![],
        );
        assert_eq!(evidence_occurrence(&strategy, lineage(999)), None);
    }

    #[test]
    fn evidence_exactly_one_match_resolves_to_its_occurrence() {
        let strategy = strategy_with(
            vec![EvidenceDeclaration::numeric(occ(10), lineage(1), dim(1))],
            vec![],
        );
        assert_eq!(evidence_occurrence(&strategy, lineage(1)), Some(occ(10)));
    }

    #[test]
    fn evidence_duplicate_matching_lineage_resolves_to_none() {
        let strategy = strategy_with(
            vec![
                EvidenceDeclaration::numeric(occ(10), lineage(1), dim(1)),
                EvidenceDeclaration::numeric(occ(11), lineage(1), dim(2)),
            ],
            vec![],
        );
        assert_eq!(evidence_occurrence(&strategy, lineage(1)), None);
    }

    #[test]
    fn evidence_unrelated_declarations_do_not_affect_the_result() {
        let strategy = strategy_with(
            vec![
                EvidenceDeclaration::numeric(occ(9), lineage(0), dim(1)),
                EvidenceDeclaration::numeric(occ(10), lineage(1), dim(1)),
                EvidenceDeclaration::numeric(occ(12), lineage(2), dim(1)),
            ],
            vec![],
        );
        assert_eq!(evidence_occurrence(&strategy, lineage(1)), Some(occ(10)));
    }

    #[test]
    fn evidence_collection_permutation_produces_an_identical_result() {
        let forward = strategy_with(
            vec![
                EvidenceDeclaration::numeric(occ(9), lineage(0), dim(1)),
                EvidenceDeclaration::numeric(occ(10), lineage(1), dim(1)),
            ],
            vec![],
        );
        let reversed = strategy_with(
            vec![
                EvidenceDeclaration::numeric(occ(10), lineage(1), dim(1)),
                EvidenceDeclaration::numeric(occ(9), lineage(0), dim(1)),
            ],
            vec![],
        );
        assert_eq!(
            evidence_occurrence(&forward, lineage(1)),
            evidence_occurrence(&reversed, lineage(1))
        );
    }

    #[test]
    fn evidence_repeated_invocation_is_stable() {
        let strategy = strategy_with(
            vec![EvidenceDeclaration::numeric(occ(10), lineage(1), dim(1))],
            vec![],
        );
        assert_eq!(
            evidence_occurrence(&strategy, lineage(1)),
            evidence_occurrence(&strategy, lineage(1))
        );
    }

    #[test]
    fn evidence_lineage_never_resolves_against_the_configuration_collection() {
        // A configuration parameter shares the same underlying opaque handle as
        // the target lineage; evidence_occurrence must not cross domains.
        let param = ConfigurationParameter::new(
            occ(20),
            lineage(1),
            dim(1),
            Literal::Numeric {
                value: ExactNumber::from_integer(0),
                dimension: dim(1),
            },
            Literal::Numeric {
                value: ExactNumber::from_integer(9),
                dimension: dim(1),
            },
            true,
        );
        let strategy = strategy_with(vec![], vec![param]);
        assert_eq!(evidence_occurrence(&strategy, lineage(1)), None);
    }

    // == configuration_occurrence =============================================

    fn config_param(occurrence: u64, lin: u64) -> ConfigurationParameter {
        ConfigurationParameter::new(
            occ(occurrence),
            lineage(lin),
            dim(1),
            Literal::Numeric {
                value: ExactNumber::from_integer(0),
                dimension: dim(1),
            },
            Literal::Numeric {
                value: ExactNumber::from_integer(9),
                dimension: dim(1),
            },
            true,
        )
    }

    #[test]
    fn configuration_absent_collection_resolves_to_none() {
        let strategy = strategy_with_absent_collections();
        assert_eq!(configuration_occurrence(&strategy, lineage(1)), None);
    }

    #[test]
    fn configuration_zero_matching_lineage_resolves_to_none() {
        let strategy = strategy_with(vec![], vec![config_param(20, 1)]);
        assert_eq!(configuration_occurrence(&strategy, lineage(999)), None);
    }

    #[test]
    fn configuration_exactly_one_match_resolves_to_its_occurrence() {
        let strategy = strategy_with(vec![], vec![config_param(20, 1)]);
        assert_eq!(
            configuration_occurrence(&strategy, lineage(1)),
            Some(occ(20))
        );
    }

    #[test]
    fn configuration_duplicate_matching_lineage_resolves_to_none() {
        let strategy = strategy_with(vec![], vec![config_param(20, 1), config_param(21, 1)]);
        assert_eq!(configuration_occurrence(&strategy, lineage(1)), None);
    }

    #[test]
    fn configuration_unrelated_entries_do_not_affect_the_result() {
        let strategy = strategy_with(
            vec![],
            vec![
                config_param(19, 0),
                config_param(20, 1),
                config_param(22, 2),
            ],
        );
        assert_eq!(
            configuration_occurrence(&strategy, lineage(1)),
            Some(occ(20))
        );
    }

    #[test]
    fn configuration_collection_permutation_produces_an_identical_result() {
        let forward = strategy_with(vec![], vec![config_param(19, 0), config_param(20, 1)]);
        let reversed = strategy_with(vec![], vec![config_param(20, 1), config_param(19, 0)]);
        assert_eq!(
            configuration_occurrence(&forward, lineage(1)),
            configuration_occurrence(&reversed, lineage(1))
        );
    }

    #[test]
    fn configuration_repeated_invocation_is_stable() {
        let strategy = strategy_with(vec![], vec![config_param(20, 1)]);
        assert_eq!(
            configuration_occurrence(&strategy, lineage(1)),
            configuration_occurrence(&strategy, lineage(1))
        );
    }

    #[test]
    fn configuration_lineage_never_resolves_against_the_evidence_collection() {
        let evidence = EvidenceDeclaration::numeric(occ(10), lineage(1), dim(1));
        let strategy = strategy_with(vec![evidence], vec![]);
        assert_eq!(configuration_occurrence(&strategy, lineage(1)), None);
    }

    // == integration with guard/derivation dependency enumeration ============

    #[test]
    fn guard_dependency_resolves_to_the_expected_evidence_occurrence() {
        let strategy = strategy_with(
            vec![EvidenceDeclaration::numeric(occ(10), lineage(1), dim(1))],
            vec![],
        );
        let guard = Guard::from_atoms(vec![atom(1)]);
        let dependencies = guard_dependencies(&guard);
        assert_eq!(dependencies, vec![lineage(1)]);
        assert_eq!(
            evidence_occurrence(&strategy, dependencies[0]),
            Some(occ(10))
        );
    }

    #[test]
    fn derivation_dependency_resolves_to_the_expected_configuration_occurrence() {
        let strategy = strategy_with(vec![], vec![config_param(20, 1)]);
        let expr = DerivationExpression::ConfigurationReference(lineage(1));
        let dependencies = derivation_dependencies(&expr);
        assert_eq!(dependencies, vec![lineage(1)]);
        assert_eq!(
            configuration_occurrence(&strategy, dependencies[0]),
            Some(occ(20))
        );
    }
}
