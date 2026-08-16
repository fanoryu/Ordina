//! Check 2 — stable identity / reference integrity — Slice Contract §5.1 check 2.
//!
//! Check 2 owns four categories (§5.1, §4.1, §4.7):
//!
//! - **`DUPLICATE_LINEAGE_IDENTITY`** — one defect **per duplicated lineage
//!   identity** (not per occurrence), over **every** lineage-bearing construct in
//!   the Strategy (§5.1: "every lineage identity in the structure is unique";
//!   §4.7). Anchored on the **envelope occurrence** with the **`LINEAGE`**
//!   discriminator (Revision 4 §C.5). A missing (Absent) lineage is not a value
//!   and never participates.
//! - **`UNRESOLVED_REFERENCE`** — one per reference whose target lineage does not
//!   resolve to **exactly one** declared item of the referenced kind. Zero targets
//!   is unresolved; **duplicate targets are ambiguous and therefore also
//!   unresolved** — §4.7: "the checker does not pick one."
//! - **`TYPE_MISMATCH`** — one per atom or derivation node that is ill-typed per
//!   §4.1 (type-tag, operator admissibility §3.6/§4.4, and NUMERIC dimension
//!   identity §4.6 for atoms; non-`NUMERIC` leaf for derivations).
//! - **`ENUM_VALUE_NOT_DECLARED`** — an `ENUM` atom whose literal value is not a
//!   **member** of the referent's declared value set (§4.1). Membership only.
//!
//! # Discriminators (§6.4.2)
//!
//! Guard-atom defects use **`ATOM`** (the atom triple); derivation-node defects
//! use **`NODE-PATH`** (the path from the derivation root); duplicate-lineage uses
//! **`LINEAGE`**. No new discriminator kind is introduced.
//!
//! # O-1 isolation
//!
//! `ENUM_VALUE_NOT_DECLARED` uses **membership** of an exact opaque `EnumValue` in
//! the declared collection (`Vec::contains`). It never depends on the collection's
//! order, equality, deduplication, or set/sequence semantics — so **O-1 stays
//! OPEN** and check 6 remains the exclusive owner of value-set validity.
//!
//! # Computability, not gating
//!
//! A defect is emitted only where its comparison is **defined**: typing runs only
//! against a determinate declared type (`One(tag)`), a NUMERIC dimension defect
//! only against a present declared dimension, and ENUM membership only against a
//! present value set. Where those are absent/multiple/out-of-domain the fault is
//! check 1's or check 6's, and check 2 emits nothing — this is the intrinsic
//! definition of each defect, **not** §5.2 prerequisite gating (no suppression,
//! not-evaluated record, or cross-check scheduling is computed here). Output is
//! defect production only — a `Vec<DefectCore>`.

use crate::model::declared::{Declared, DeclaredCategory};
use crate::model::derivation::{CandidateDerivation, DerivationExpression};
use crate::model::evidence::{EvidenceDeclaration, EvidenceTypeTag};
use crate::model::guard::{Atom, ComparisonOperator, Guard};
use crate::model::literal::Literal;
use crate::model::opaque::{LineageId, OccurrenceId};
use crate::model::strategy::Strategy;
use crate::result::category::DefectCategory;
use crate::result::discriminator::{AtomKey, ChildPosition, Discriminator, NodePath};
use crate::result::identity::DefectCore;

// ---------------------------------------------------------------------------
// DUPLICATE_LINEAGE_IDENTITY — one per duplicated identity, envelope-anchored.
// ---------------------------------------------------------------------------

/// Collect every **present** lineage identity in the Strategy, in a deterministic
/// traversal order over all lineage-bearing constructs (§5.1 "every lineage
/// identity in the structure").
fn present_lineages(strategy: &Strategy) -> Vec<LineageId> {
    let mut lineages = Vec::new();
    if let Declared::Present(lineage) = &strategy.lineage {
        lineages.push(*lineage);
    }
    let mut push_from = |declared: &Declared<LineageId>| {
        if let Declared::Present(lineage) = declared {
            lineages.push(*lineage);
        }
    };
    if let Declared::Present(items) = &strategy.evidence_declarations {
        for item in items {
            push_from(&item.lineage);
        }
    }
    if let Declared::Present(items) = &strategy.configuration_parameters {
        for item in items {
            push_from(&item.lineage);
        }
    }
    if let Declared::Present(items) = &strategy.conditions {
        for item in items {
            push_from(&item.lineage);
        }
    }
    if let Declared::Present(items) = &strategy.direction_rules {
        for item in items {
            push_from(&item.lineage);
        }
    }
    if let Declared::Present(items) = &strategy.candidate_derivations {
        for item in items {
            push_from(&item.lineage);
        }
    }
    lineages
}

/// Check 2 duplicate-lineage: one `DUPLICATE_LINEAGE_IDENTITY` per lineage value
/// that appears more than once, anchored on the envelope occurrence with the
/// `LINEAGE` discriminator. Deterministic: a value is reported once, at its first
/// occurrence, independent of any hash iteration.
#[must_use]
pub fn check_duplicate_lineage(strategy: &Strategy) -> Vec<DefectCore> {
    let lineages = present_lineages(strategy);
    let mut defects = Vec::new();
    for (index, lineage) in lineages.iter().enumerate() {
        let is_first = !lineages[..index].contains(lineage);
        let is_duplicated = lineages[index + 1..].contains(lineage);
        if is_first && is_duplicated {
            defects.push(DefectCore::new(
                strategy.occurrence,
                DefectCategory::DuplicateLineageIdentity,
                Discriminator::Lineage(*lineage),
            ));
        }
    }
    defects
}

// ---------------------------------------------------------------------------
// Reference resolution — exact LineageId identity, exactly one target.
// ---------------------------------------------------------------------------

/// Resolve an evidence reference to the single declared evidence item with the
/// referenced lineage. `None` if **zero** items match (unresolved) or **more than
/// one** matches (ambiguous — §4.7, also unresolved: the checker never picks one).
fn resolve_evidence(strategy: &Strategy, referenced: LineageId) -> Option<&EvidenceDeclaration> {
    let Declared::Present(items) = &strategy.evidence_declarations else {
        return None;
    };
    let mut resolved = None;
    for item in items {
        if item.lineage == Declared::Present(referenced) {
            if resolved.is_some() {
                return None; // ambiguous ⇒ unresolved
            }
            resolved = Some(item);
        }
    }
    resolved
}

/// Whether a configuration reference resolves to **exactly one** declared
/// parameter. Zero or ambiguous (§4.7) ⇒ unresolved.
fn configuration_resolves(strategy: &Strategy, referenced: LineageId) -> bool {
    let Declared::Present(items) = &strategy.configuration_parameters else {
        return false;
    };
    items
        .iter()
        .filter(|item| item.lineage == Declared::Present(referenced))
        .count()
        == 1
}

// ---------------------------------------------------------------------------
// Guard atoms — UNRESOLVED_REFERENCE / TYPE_MISMATCH / ENUM_VALUE_NOT_DECLARED,
// ATOM discriminator, anchored on the enclosing construct occurrence.
// ---------------------------------------------------------------------------

/// The `ATOM` discriminator for one guard atom (§6.4.2 kind 4).
fn atom_discriminator(atom: &Atom) -> Discriminator {
    Discriminator::Atom(AtomKey::new(
        atom.evidence,
        atom.operator,
        atom.literal.clone(),
    ))
}

/// Whether the operator is admitted on a `BOOLEAN`/`ENUM` referent (§3.6, §4.4):
/// only equality and inequality; ordering is a type defect.
const fn operator_admitted_on_unordered(operator: ComparisonOperator) -> bool {
    matches!(
        operator,
        ComparisonOperator::Equal | ComparisonOperator::NotEqual
    )
}

/// Check 2 for one guard atom against its referent evidence declaration's
/// determinate declared type. At most one defect (§5.1 "one per atom").
fn check_typed_atom(
    anchor: OccurrenceId,
    atom: &Atom,
    tag: EvidenceTypeTag,
    referent: &EvidenceDeclaration,
) -> Vec<DefectCore> {
    let type_mismatch = || {
        vec![DefectCore::new(
            anchor,
            DefectCategory::TypeMismatch,
            atom_discriminator(atom),
        )]
    };
    match (&atom.literal, tag) {
        // Matching NUMERIC: every operator is admitted; the only remaining atom
        // fault is a dimension that differs from a *present* declared dimension
        // (§4.1, §4.6). An absent dimension is check 6's, not computable here.
        (Literal::Numeric { dimension, .. }, EvidenceTypeTag::Numeric) => {
            match &referent.declared_dimension {
                Declared::Present(declared) if declared != dimension => type_mismatch(),
                Declared::Present(_) | Declared::Absent => Vec::new(),
            }
        }
        // Matching BOOLEAN: only equality/inequality operators are admitted.
        (Literal::Boolean(_), EvidenceTypeTag::Boolean) => {
            if operator_admitted_on_unordered(atom.operator) {
                Vec::new()
            } else {
                type_mismatch()
            }
        }
        // Matching ENUM: ordering operators are a type defect; otherwise the value
        // must be a member of a *present* declared value set (membership only).
        (Literal::Enum(value), EvidenceTypeTag::Enum) => {
            if !operator_admitted_on_unordered(atom.operator) {
                type_mismatch()
            } else {
                match &referent.declared_values {
                    Declared::Present(set) if !set.contains(value) => {
                        vec![DefectCore::new(
                            anchor,
                            DefectCategory::EnumValueNotDeclared,
                            atom_discriminator(atom),
                        )]
                    }
                    Declared::Present(_) | Declared::Absent => Vec::new(),
                }
            }
        }
        // Type-tag mismatch — the literal's type differs from the referent's.
        (Literal::Numeric { .. }, EvidenceTypeTag::Boolean | EvidenceTypeTag::Enum)
        | (Literal::Boolean(_), EvidenceTypeTag::Numeric | EvidenceTypeTag::Enum)
        | (Literal::Enum(_), EvidenceTypeTag::Numeric | EvidenceTypeTag::Boolean) => {
            type_mismatch()
        }
    }
}

/// Check 2 for one guard, anchored on its enclosing construct occurrence.
fn check_guard(anchor: OccurrenceId, guard: &Guard, strategy: &Strategy) -> Vec<DefectCore> {
    let mut defects = Vec::new();
    for atom in &guard.atoms {
        match resolve_evidence(strategy, atom.evidence) {
            None => defects.push(DefectCore::new(
                anchor,
                DefectCategory::UnresolvedReference,
                atom_discriminator(atom),
            )),
            Some(referent) => match &referent.declared_type {
                // Type-check only against a determinate declared type; otherwise
                // the fault is check 1's / check 6's, not computable here.
                DeclaredCategory::One(tag) => {
                    defects.extend(check_typed_atom(anchor, atom, *tag, referent));
                }
                DeclaredCategory::Absent
                | DeclaredCategory::Multiple { .. }
                | DeclaredCategory::OutOfDomain => {}
            },
        }
    }
    defects
}

// ---------------------------------------------------------------------------
// Derivation leaves — UNRESOLVED_REFERENCE / TYPE_MISMATCH, NODE-PATH
// discriminator, anchored on the enclosing derivation occurrence.
// ---------------------------------------------------------------------------

fn node_path_defect(
    anchor: OccurrenceId,
    path: &[ChildPosition],
    category: DefectCategory,
) -> DefectCore {
    DefectCore::new(
        anchor,
        category,
        Discriminator::NodePath(NodePath::new(path.to_vec())),
    )
}

/// Check 2 well-typedness of one derivation leaf literal — every leaf must be
/// `NUMERIC` (§4.1). A non-`NUMERIC` literal leaf is a `TYPE_MISMATCH`.
fn check_leaf_literal(
    anchor: OccurrenceId,
    path: &[ChildPosition],
    literal: &Literal,
    defects: &mut Vec<DefectCore>,
) {
    match literal {
        Literal::Numeric { .. } => {}
        Literal::Boolean(_) | Literal::Enum(_) => {
            defects.push(node_path_defect(anchor, path, DefectCategory::TypeMismatch));
        }
    }
}

/// Walk one derivation expression, emitting check-2 leaf defects with the
/// NODE-PATH from the derivation root (`left operand = 1`, `right operand = 2`).
fn check_derivation_expression(
    anchor: OccurrenceId,
    expression: &DerivationExpression,
    path: &[ChildPosition],
    strategy: &Strategy,
    defects: &mut Vec<DefectCore>,
) {
    match expression {
        DerivationExpression::Literal(literal) => {
            check_leaf_literal(anchor, path, literal, defects);
        }
        DerivationExpression::EvidenceReference(referenced) => {
            match resolve_evidence(strategy, *referenced) {
                None => defects.push(node_path_defect(
                    anchor,
                    path,
                    DefectCategory::UnresolvedReference,
                )),
                // A resolved evidence leaf must be NUMERIC (§4.1); a determinate
                // non-NUMERIC type is a mismatch, a non-determinate one is not
                // computable here.
                Some(referent) => match &referent.declared_type {
                    DeclaredCategory::One(EvidenceTypeTag::Numeric) => {}
                    DeclaredCategory::One(EvidenceTypeTag::Boolean | EvidenceTypeTag::Enum) => {
                        defects.push(node_path_defect(anchor, path, DefectCategory::TypeMismatch));
                    }
                    DeclaredCategory::Absent
                    | DeclaredCategory::Multiple { .. }
                    | DeclaredCategory::OutOfDomain => {}
                },
            }
        }
        DerivationExpression::ConfigurationReference(referenced) => {
            // Configuration parameters are NUMERIC-only (§3.9), so a resolved
            // config leaf is always well-typed; only resolution can fail.
            if !configuration_resolves(strategy, *referenced) {
                defects.push(node_path_defect(
                    anchor,
                    path,
                    DefectCategory::UnresolvedReference,
                ));
            }
        }
        DerivationExpression::Add { left, right }
        | DerivationExpression::Subtract { left, right } => {
            let mut left_path = path.to_vec();
            left_path.push(ChildPosition::LeftOperand);
            check_derivation_expression(anchor, left, &left_path, strategy, defects);
            let mut right_path = path.to_vec();
            right_path.push(ChildPosition::RightOperand);
            check_derivation_expression(anchor, right, &right_path, strategy, defects);
        }
        DerivationExpression::Multiply { operand, scalar } => {
            let mut operand_path = path.to_vec();
            operand_path.push(ChildPosition::LeftOperand);
            check_derivation_expression(anchor, operand, &operand_path, strategy, defects);
            // The scalar is a literal leaf and must itself be NUMERIC (§4.1);
            // its dimensionlessness is check 14's, not check 2's.
            let mut scalar_path = path.to_vec();
            scalar_path.push(ChildPosition::RightOperand);
            check_leaf_literal(anchor, &scalar_path, scalar, defects);
        }
    }
}

/// Check 2 for one candidate derivation, anchored on its occurrence.
fn check_derivation(derivation: &CandidateDerivation, strategy: &Strategy) -> Vec<DefectCore> {
    let mut defects = Vec::new();
    if let Declared::Present(expression) = &derivation.expression {
        check_derivation_expression(
            derivation.occurrence,
            expression,
            &[],
            strategy,
            &mut defects,
        );
    }
    defects
}

// ---------------------------------------------------------------------------
// Aggregate.
// ---------------------------------------------------------------------------

/// Check 2 over a whole Strategy — duplicate lineage, then every present guard
/// (conditions and Direction rules) and candidate derivation. Defect production
/// only; deterministic; no prerequisite gating.
#[must_use]
pub fn check_2(strategy: &Strategy) -> Vec<DefectCore> {
    let mut defects = check_duplicate_lineage(strategy);

    if let Declared::Present(items) = &strategy.conditions {
        for condition in items {
            if let Declared::Present(guard) = &condition.guard {
                defects.extend(check_guard(condition.occurrence, guard, strategy));
            }
        }
    }
    if let Declared::Present(items) = &strategy.direction_rules {
        for rule in items {
            if let Declared::Present(guard) = &rule.guard {
                defects.extend(check_guard(rule.occurrence, guard, strategy));
            }
        }
    }
    if let Declared::Present(items) = &strategy.candidate_derivations {
        for derivation in items {
            defects.extend(check_derivation(derivation, strategy));
        }
    }

    defects
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::condition::Condition;
    use crate::model::opaque::{DimensionSymbol, EnumValue, OpaqueValue};
    use crate::numeric::ExactNumber;

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn lineage(handle: u64) -> LineageId {
        LineageId::new(OpaqueValue::new(handle))
    }

    fn dim(handle: u64) -> DimensionSymbol {
        DimensionSymbol::new(OpaqueValue::new(handle))
    }

    fn enum_val(handle: u64) -> EnumValue {
        EnumValue::new(OpaqueValue::new(handle))
    }

    fn numeric_literal(value: i64, dimension: u64) -> Literal {
        Literal::Numeric {
            value: ExactNumber::from_integer(value),
            dimension: dim(dimension),
        }
    }

    /// A minimal envelope with all collections present-and-empty and a present
    /// lineage/version, to be populated per test.
    fn base() -> Strategy {
        Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(lineage(1000)),
            semantic_model_version: Declared::Absent,
            evidence_declarations: Declared::Present(vec![]),
            configuration_parameters: Declared::Present(vec![]),
            conditions: Declared::Present(vec![]),
            direction_rules: Declared::Present(vec![]),
            candidate_derivations: Declared::Present(vec![]),
        }
    }

    fn numeric_evidence(occurrence: u64, lin: u64, dimension: u64) -> EvidenceDeclaration {
        EvidenceDeclaration {
            occurrence: occ(occurrence),
            lineage: Declared::Present(lineage(lin)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Numeric),
            declared_dimension: Declared::Present(dim(dimension)),
            declared_values: Declared::Absent,
        }
    }

    fn enum_evidence(occurrence: u64, lin: u64, values: Vec<EnumValue>) -> EvidenceDeclaration {
        EvidenceDeclaration {
            occurrence: occ(occurrence),
            lineage: Declared::Present(lineage(lin)),
            declared_type: DeclaredCategory::One(EvidenceTypeTag::Enum),
            declared_dimension: Declared::Absent,
            declared_values: Declared::Present(values),
        }
    }

    fn categories(defects: &[DefectCore]) -> Vec<DefectCategory> {
        defects.iter().map(DefectCore::category).collect()
    }

    // -- DUPLICATE_LINEAGE_IDENTITY ----------------------------------------

    #[test]
    fn one_duplicated_lineage_class_yields_one_envelope_anchored_lineage_defect() {
        let mut strategy = base();
        strategy.evidence_declarations = Declared::Present(vec![
            numeric_evidence(10, 4, 1),
            numeric_evidence(11, 4, 1), // lineage 4 duplicated
        ]);
        let defects = check_duplicate_lineage(&strategy);
        assert_eq!(defects.len(), 1);
        assert_eq!(
            defects[0].category(),
            DefectCategory::DuplicateLineageIdentity
        );
        assert_eq!(defects[0].offending(), occ(1)); // envelope anchor
        match defects[0].discriminator() {
            Discriminator::Lineage(l) => assert_eq!(*l, lineage(4)),
            _ => panic!("duplicate lineage must use the LINEAGE discriminator"),
        }
    }

    #[test]
    fn two_duplicated_classes_yield_two_distinct_defects_not_per_occurrence() {
        let mut strategy = base();
        // L4 twice, L7 twice ⇒ exactly two class defects (not four per-occurrence).
        strategy.evidence_declarations = Declared::Present(vec![
            numeric_evidence(10, 4, 1),
            numeric_evidence(11, 4, 1),
            numeric_evidence(12, 7, 1),
            numeric_evidence(13, 7, 1),
        ]);
        let defects = check_duplicate_lineage(&strategy);
        assert_eq!(defects.len(), 2);
        let lineages: Vec<_> = defects
            .iter()
            .map(|d| match d.discriminator() {
                Discriminator::Lineage(l) => *l,
                _ => panic!("expected LINEAGE"),
            })
            .collect();
        assert!(lineages.contains(&lineage(4)) && lineages.contains(&lineage(7)));
        assert_ne!(defects[0].cmp(&defects[1]), core::cmp::Ordering::Equal);
    }

    #[test]
    fn absent_lineage_does_not_count_as_a_duplicate() {
        let mut strategy = base();
        let mut a = numeric_evidence(10, 4, 1);
        a.lineage = Declared::Absent;
        let mut b = numeric_evidence(11, 4, 1);
        b.lineage = Declared::Absent;
        strategy.evidence_declarations = Declared::Present(vec![a, b]);
        assert!(check_duplicate_lineage(&strategy).is_empty());
    }

    #[test]
    fn duplicate_lineage_spans_all_construct_kinds() {
        // Envelope lineage 1000 shared with an evidence declaration ⇒ one class.
        let mut strategy = base();
        strategy.evidence_declarations = Declared::Present(vec![numeric_evidence(10, 1000, 1)]);
        let defects = check_duplicate_lineage(&strategy);
        assert_eq!(defects.len(), 1);
        match defects[0].discriminator() {
            Discriminator::Lineage(l) => assert_eq!(*l, lineage(1000)),
            _ => panic!("expected LINEAGE"),
        }
    }

    // -- UNRESOLVED_REFERENCE ----------------------------------------------

    fn guarded_condition(occurrence: u64, atom: Atom) -> Condition {
        use crate::model::condition::ConditionClassification;
        Condition {
            occurrence: occ(occurrence),
            lineage: Declared::Present(lineage(500)),
            classification: DeclaredCategory::One(ConditionClassification::Blocking),
            guard: Declared::Present(Guard::from_atoms(vec![atom])),
        }
    }

    #[test]
    fn a_resolvable_evidence_reference_is_not_unresolved() {
        let mut strategy = base();
        strategy.evidence_declarations = Declared::Present(vec![numeric_evidence(10, 4, 1)]);
        strategy.conditions = Declared::Present(vec![guarded_condition(
            20,
            Atom::new(
                lineage(4),
                ComparisonOperator::Greater,
                numeric_literal(5, 1),
            ),
        )]);
        assert!(check_2(&strategy).is_empty());
    }

    #[test]
    fn an_absent_target_is_an_unresolved_reference_on_the_enclosing_construct() {
        let mut strategy = base();
        strategy.conditions = Declared::Present(vec![guarded_condition(
            20,
            Atom::new(
                lineage(999),
                ComparisonOperator::Greater,
                numeric_literal(5, 1),
            ),
        )]);
        let defects = check_2(&strategy);
        assert_eq!(defects.len(), 1);
        assert_eq!(defects[0].category(), DefectCategory::UnresolvedReference);
        assert_eq!(defects[0].offending(), occ(20)); // the condition
        assert!(matches!(defects[0].discriminator(), Discriminator::Atom(_)));
    }

    #[test]
    fn a_duplicate_target_lineage_makes_the_reference_unresolved_not_arbitrarily_selected() {
        let mut strategy = base();
        // Two evidence declarations with lineage 4 ⇒ the reference is ambiguous.
        strategy.evidence_declarations =
            Declared::Present(vec![numeric_evidence(10, 4, 1), numeric_evidence(11, 4, 1)]);
        strategy.conditions = Declared::Present(vec![guarded_condition(
            20,
            Atom::new(
                lineage(4),
                ComparisonOperator::Greater,
                numeric_literal(5, 1),
            ),
        )]);
        let cats = categories(&check_2(&strategy));
        // One duplicate-lineage class defect AND one unresolved-reference defect.
        assert!(cats.contains(&DefectCategory::DuplicateLineageIdentity));
        assert!(cats.contains(&DefectCategory::UnresolvedReference));
    }

    // -- TYPE_MISMATCH (atoms) ---------------------------------------------

    #[test]
    fn a_type_matched_atom_is_not_a_mismatch() {
        let mut strategy = base();
        strategy.evidence_declarations = Declared::Present(vec![numeric_evidence(10, 4, 1)]);
        strategy.conditions = Declared::Present(vec![guarded_condition(
            20,
            Atom::new(
                lineage(4),
                ComparisonOperator::Greater,
                numeric_literal(5, 1),
            ),
        )]);
        assert!(check_2(&strategy).is_empty());
    }

    #[test]
    fn a_boolean_literal_on_a_numeric_referent_is_a_type_mismatch() {
        let mut strategy = base();
        strategy.evidence_declarations = Declared::Present(vec![numeric_evidence(10, 4, 1)]);
        strategy.conditions = Declared::Present(vec![guarded_condition(
            20,
            Atom::new(
                lineage(4),
                ComparisonOperator::Equal,
                Literal::Boolean(true),
            ),
        )]);
        let defects = check_2(&strategy);
        assert_eq!(categories(&defects), vec![DefectCategory::TypeMismatch]);
        assert_eq!(defects[0].offending(), occ(20));
    }

    #[test]
    fn an_ordering_operator_on_a_boolean_referent_is_a_type_mismatch() {
        let mut strategy = base();
        let mut boolean = numeric_evidence(10, 4, 1);
        boolean.declared_type = DeclaredCategory::One(EvidenceTypeTag::Boolean);
        boolean.declared_dimension = Declared::Absent;
        strategy.evidence_declarations = Declared::Present(vec![boolean]);
        strategy.conditions = Declared::Present(vec![guarded_condition(
            20,
            Atom::new(lineage(4), ComparisonOperator::Less, Literal::Boolean(true)),
        )]);
        assert_eq!(
            categories(&check_2(&strategy)),
            vec![DefectCategory::TypeMismatch]
        );
    }

    #[test]
    fn a_numeric_dimension_mismatch_is_a_type_mismatch() {
        let mut strategy = base();
        strategy.evidence_declarations = Declared::Present(vec![numeric_evidence(10, 4, 1)]);
        strategy.conditions = Declared::Present(vec![guarded_condition(
            20,
            Atom::new(
                lineage(4),
                ComparisonOperator::Greater,
                numeric_literal(5, 2),
            ), // dim 2 != 1
        )]);
        assert_eq!(
            categories(&check_2(&strategy)),
            vec![DefectCategory::TypeMismatch]
        );
    }

    #[test]
    fn typing_is_not_computed_against_an_absent_declared_type() {
        let mut strategy = base();
        let mut untyped = numeric_evidence(10, 4, 1);
        untyped.declared_type = DeclaredCategory::Absent; // check 1's fault, not check 2's
        strategy.evidence_declarations = Declared::Present(vec![untyped]);
        strategy.conditions = Declared::Present(vec![guarded_condition(
            20,
            Atom::new(
                lineage(4),
                ComparisonOperator::Equal,
                Literal::Boolean(true),
            ),
        )]);
        // Reference resolves (one declaration with lineage 4), but no determinate
        // type ⇒ no TYPE_MISMATCH from check 2.
        assert!(check_2(&strategy).is_empty());
    }

    // -- ENUM_VALUE_NOT_DECLARED / O-1 -------------------------------------

    #[test]
    fn a_declared_enum_value_is_not_a_defect() {
        let mut strategy = base();
        strategy.evidence_declarations =
            Declared::Present(vec![enum_evidence(10, 4, vec![enum_val(1), enum_val(2)])]);
        strategy.conditions = Declared::Present(vec![guarded_condition(
            20,
            Atom::new(
                lineage(4),
                ComparisonOperator::Equal,
                Literal::Enum(enum_val(2)),
            ),
        )]);
        assert!(check_2(&strategy).is_empty());
    }

    #[test]
    fn an_undeclared_enum_value_is_enum_value_not_declared() {
        let mut strategy = base();
        strategy.evidence_declarations =
            Declared::Present(vec![enum_evidence(10, 4, vec![enum_val(1), enum_val(2)])]);
        strategy.conditions = Declared::Present(vec![guarded_condition(
            20,
            Atom::new(
                lineage(4),
                ComparisonOperator::Equal,
                Literal::Enum(enum_val(9)),
            ),
        )]);
        let defects = check_2(&strategy);
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::EnumValueNotDeclared]
        );
        assert_eq!(defects[0].offending(), occ(20));
    }

    #[test]
    fn enum_membership_ignores_order_and_duplication() {
        // O-1 isolation: membership depends only on presence of the exact value,
        // never on declared_values order or duplication.
        let value = enum_val(2);
        let ordered = enum_evidence(10, 4, vec![enum_val(1), enum_val(2), enum_val(3)]);
        let reordered = enum_evidence(10, 4, vec![enum_val(3), enum_val(2), enum_val(1)]);
        let duplicated = enum_evidence(10, 4, vec![enum_val(2), enum_val(2)]);
        for evidence in [ordered, reordered, duplicated] {
            let mut strategy = base();
            strategy.evidence_declarations = Declared::Present(vec![evidence]);
            strategy.conditions = Declared::Present(vec![guarded_condition(
                20,
                Atom::new(lineage(4), ComparisonOperator::Equal, Literal::Enum(value)),
            )]);
            // The value IS a member in every arrangement ⇒ no defect.
            assert!(check_2(&strategy).is_empty());
        }
    }

    // -- Derivation nodes (NODE-PATH) --------------------------------------

    fn derivation(occurrence: u64, expression: DerivationExpression) -> CandidateDerivation {
        use crate::model::derivation::CandidateRole;
        CandidateDerivation {
            occurrence: occ(occurrence),
            lineage: Declared::Present(lineage(600)),
            role: Declared::Present(CandidateRole::Entry),
            expression: Declared::Present(expression),
        }
    }

    #[test]
    fn an_unresolved_derivation_leaf_is_unresolved_with_a_node_path() {
        let mut strategy = base();
        strategy.candidate_derivations = Declared::Present(vec![derivation(
            30,
            DerivationExpression::EvidenceReference(lineage(999)),
        )]);
        let defects = check_2(&strategy);
        assert_eq!(
            categories(&defects),
            vec![DefectCategory::UnresolvedReference]
        );
        assert_eq!(defects[0].offending(), occ(30));
        assert!(matches!(
            defects[0].discriminator(),
            Discriminator::NodePath(_)
        ));
    }

    #[test]
    fn a_non_numeric_literal_leaf_is_a_type_mismatch() {
        let mut strategy = base();
        strategy.candidate_derivations = Declared::Present(vec![derivation(
            30,
            DerivationExpression::Literal(Literal::Boolean(true)),
        )]);
        assert_eq!(
            categories(&check_2(&strategy)),
            vec![DefectCategory::TypeMismatch]
        );
    }

    #[test]
    fn a_resolved_non_numeric_evidence_leaf_is_a_type_mismatch() {
        let mut strategy = base();
        let mut boolean = numeric_evidence(10, 4, 1);
        boolean.declared_type = DeclaredCategory::One(EvidenceTypeTag::Boolean);
        boolean.declared_dimension = Declared::Absent;
        strategy.evidence_declarations = Declared::Present(vec![boolean]);
        strategy.candidate_derivations = Declared::Present(vec![derivation(
            30,
            DerivationExpression::EvidenceReference(lineage(4)),
        )]);
        assert_eq!(
            categories(&check_2(&strategy)),
            vec![DefectCategory::TypeMismatch]
        );
    }

    #[test]
    fn a_well_typed_numeric_derivation_has_no_check_two_defect() {
        let mut strategy = base();
        strategy.evidence_declarations = Declared::Present(vec![numeric_evidence(10, 4, 1)]);
        strategy.candidate_derivations = Declared::Present(vec![derivation(
            30,
            DerivationExpression::Subtract {
                left: Box::new(DerivationExpression::EvidenceReference(lineage(4))),
                right: Box::new(DerivationExpression::Literal(numeric_literal(1, 1))),
            },
        )]);
        assert!(check_2(&strategy).is_empty());
    }

    #[test]
    fn nested_leaf_defects_carry_distinct_node_paths() {
        // Two bad leaves under a Subtract ⇒ two NODE-PATH defects that differ.
        let mut strategy = base();
        strategy.candidate_derivations = Declared::Present(vec![derivation(
            30,
            DerivationExpression::Subtract {
                left: Box::new(DerivationExpression::Literal(Literal::Boolean(true))),
                right: Box::new(DerivationExpression::EvidenceReference(lineage(999))),
            },
        )]);
        let defects = check_2(&strategy);
        assert_eq!(defects.len(), 2);
        assert_ne!(defects[0].cmp(&defects[1]), core::cmp::Ordering::Equal);
    }

    // -- occurrence anchoring under malformed lineage ----------------------

    #[test]
    fn sibling_atoms_referencing_one_absent_target_stay_distinct_by_anchor() {
        use crate::model::condition::ConditionClassification;
        // Two conditions with DISTINCT lineages (so no duplicate-lineage defect),
        // each referencing the same absent target: distinct anchors keep the two
        // UNRESOLVED_REFERENCE defects distinct.
        let condition = |occurrence: u64, lin: u64| Condition {
            occurrence: occ(occurrence),
            lineage: Declared::Present(lineage(lin)),
            classification: DeclaredCategory::One(ConditionClassification::Blocking),
            guard: Declared::Present(Guard::from_atoms(vec![Atom::new(
                lineage(999),
                ComparisonOperator::Equal,
                Literal::Boolean(true),
            )])),
        };
        let mut strategy = base();
        strategy.conditions = Declared::Present(vec![condition(20, 500), condition(21, 501)]);
        let defects = check_2(&strategy);
        assert_eq!(
            categories(&defects),
            vec![
                DefectCategory::UnresolvedReference,
                DefectCategory::UnresolvedReference,
            ]
        );
        assert_eq!(defects[0].offending(), occ(20));
        assert_eq!(defects[1].offending(), occ(21));
        assert_ne!(defects[0].cmp(&defects[1]), core::cmp::Ordering::Equal);
    }
}
