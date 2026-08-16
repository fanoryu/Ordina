//! Shared §4.10 guard analysis for Checks 9, 11, and 12.
//!
//! This is a narrowly-scoped Check-layer helper, shared directly by the three
//! Slice-G checks. It turns a [`Guard`] into per-reference constraint domains
//! using the existing `semantics::constraint` primitives and the accepted
//! §4.10 relations (contradiction / overlap / subsumption), and exposes exactly
//! the machinery those checks need.
//!
//! # Prerequisites, locally reconstructed
//!
//! §5.1 makes the Check 9/11/12 relations prerequisite on check 1, check 2, and
//! check 6 for the constructs they compare. This module reconstructs that
//! prerequisite **locally**, as pure guard analysis — no other check is called,
//! and no Phase-6 orchestration, not-evaluated record, verdict, or canonical
//! sorting is introduced:
//!
//! * every referenced [`LineageId`] resolves to **exactly one** present
//!   [`EvidenceDeclaration`] (else the analysis is non-computable → `None`),
//! * the referent's declared type is exactly one of `NUMERIC`/`BOOLEAN`/`ENUM`
//!   (an `Absent`/`Multiple`/`OutOfDomain` declared type is a check-6 defect →
//!   non-computable),
//! * each atom's literal type matches the referent's declared type and its
//!   operator is admissible for that type (a mismatch is a check-2
//!   `TYPE_MISMATCH` → non-computable),
//! * a `NUMERIC` referent's declared dimension matches the literal's dimension.
//!
//! A guard that fails any of these is returned as `None`; the calling check then
//! suppresses only the affected pair/condition. This is explicit prerequisite
//! gating (§5.2), not defect suppression.
//!
//! # Guard set semantics (O-4)
//!
//! The contract gives guards set semantics (§3.6/§4.5); `Guard.atoms` is a
//! `Vec` and O-4 remains OPEN. The domain builders are idempotent under
//! duplicate-identical atoms (an `x > 5` added twice is still `x > 5`), so the
//! per-reference domain is inherently set-semantic and no positional, traversal,
//! or generated-ordinal identity enters. O-4 is therefore preserved without
//! resolving it globally.

use std::collections::HashMap;

use crate::model::declared::{Declared, DeclaredCategory};
use crate::model::evidence::{EvidenceDeclaration, EvidenceTypeTag};
use crate::model::guard::{ComparisonOperator, Guard};
use crate::model::literal::Literal;
use crate::model::opaque::{EnumValue, LineageId};
use crate::model::strategy::Strategy;
use crate::numeric::ExactNumber;
use crate::semantics::constraint::EqualityOperator;
use crate::semantics::constraint::boolean::BooleanDomain;
use crate::semantics::constraint::enumerated::EnumDomain;
use crate::semantics::constraint::numeric::NumericDomain;

/// A per-reference constraint domain under §4.10, tagged by the referent's
/// declared evidence type. The three variants share one relation vocabulary
/// (`is_empty` / `overlaps` / `contains`); a cross-type comparison is
/// unreachable because one resolved lineage has exactly one declared type.
#[derive(Debug, Clone)]
pub enum RefDomain {
    /// A `NUMERIC` referent's interval-minus-excluded-points domain.
    Numeric(NumericDomain),
    /// A `BOOLEAN` referent's subset of `{true, false}`.
    Boolean(BooleanDomain),
    /// A `ENUM` referent's subset of the declared value set.
    Enum(EnumDomain),
}

impl RefDomain {
    /// Contradiction — the domain admits no value (§4.10).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Numeric(d) => d.is_empty(),
            Self::Boolean(d) => d.is_empty(),
            Self::Enum(d) => d.is_empty(),
        }
    }

    /// Overlap — the two domains share a value (§4.10).
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Numeric(a), Self::Numeric(b)) => a.overlaps(b),
            (Self::Boolean(a), Self::Boolean(b)) => a.overlaps(b),
            (Self::Enum(a), Self::Enum(b)) => a.overlaps(b),
            _ => false,
        }
    }

    /// Containment — `self` admits every value `other` admits (§4.10 subsumption).
    #[must_use]
    pub fn contains(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Numeric(a), Self::Numeric(b)) => a.contains(b),
            (Self::Boolean(a), Self::Boolean(b)) => a.contains(b),
            (Self::Enum(a), Self::Enum(b)) => a.contains(b),
            _ => false,
        }
    }

    /// The unconstrained domain for a referent: the full declared domain.
    /// Returns `None` if the declaration is not analyzable (no declared value
    /// set for an `ENUM` referent).
    #[must_use]
    pub fn unconstrained(decl: &EvidenceDeclaration) -> Option<Self> {
        match &decl.declared_type {
            DeclaredCategory::One(EvidenceTypeTag::Numeric) => {
                Some(Self::Numeric(NumericDomain::unconstrained()))
            }
            DeclaredCategory::One(EvidenceTypeTag::Boolean) => {
                Some(Self::Boolean(BooleanDomain::unconstrained()))
            }
            DeclaredCategory::One(EvidenceTypeTag::Enum) => match &decl.declared_values {
                Declared::Present(values) => Some(Self::Enum(EnumDomain::unconstrained(values))),
                Declared::Absent => None,
            },
            _ => None,
        }
    }
}

/// Per-reference accumulated constraints for one lineage, gathered while a guard
/// is analyzed. The referent's type is fixed once its declaration resolves, so
/// every atom referencing that lineage shares one `tag`.
struct Pending<'a> {
    tag: EvidenceTypeTag,
    numeric: Vec<(ComparisonOperator, ExactNumber)>,
    boolean: Vec<(EqualityOperator, bool)>,
    enum_c: Vec<(EqualityOperator, EnumValue)>,
    enum_values: Option<&'a [EnumValue]>,
}

impl<'a> Pending<'a> {
    fn new(tag: EvidenceTypeTag, decl: &'a EvidenceDeclaration) -> Self {
        let enum_values = match &decl.declared_values {
            Declared::Present(values) => Some(values.as_slice()),
            Declared::Absent => None,
        };
        Self {
            tag,
            numeric: Vec::new(),
            boolean: Vec::new(),
            enum_c: Vec::new(),
            enum_values,
        }
    }
}

/// Resolves a referenced lineage to exactly one present evidence declaration.
///
/// Returns `None` when the lineage resolves to zero declarations (an absent
/// lineage is check 1's, never here) or more than one (a duplicate lineage is
/// check 2's `UNRESOLVED_REFERENCE`). This mirrors the exactly-one resolution
/// check 2 applies.
fn resolve_declaration(strategy: &Strategy, lineage: LineageId) -> Option<&EvidenceDeclaration> {
    let Declared::Present(decls) = &strategy.evidence_declarations else {
        return None;
    };
    let mut found: Option<&EvidenceDeclaration> = None;
    let mut count = 0usize;
    for decl in decls {
        let matches = matches!(&decl.lineage, Declared::Present(present_lineage) if *present_lineage == lineage);
        if matches {
            count += 1;
            if count > 1 {
                // Ambiguous — check 2's UNRESOLVED_REFERENCE.
                return None;
            }
            found = Some(decl);
        }
    }
    found
}

/// Maps a [`ComparisonOperator`] to its equality-only form, used for
/// `BOOLEAN`/`ENUM` referents (§3.6 admits only `=`/`!=` there). The caller has
/// already confirmed the operator is `Equal` or `NotEqual`, so the map is total.
fn to_equality(operator: ComparisonOperator) -> EqualityOperator {
    match operator {
        ComparisonOperator::Equal => EqualityOperator::Equal,
        ComparisonOperator::NotEqual => EqualityOperator::NotEqual,
        _ => unreachable!("equality-only operator expected for BOOLEAN/ENUM referent"),
    }
}

/// Analyzes a guard into per-reference constraint domains (§4.10).
///
/// Returns `None` when any referenced declaration is unresolved, malformed under
/// check 6, or type-incompatible with one of its atoms — i.e. when the guard is
/// not analyzable under the explicit prerequisites. The returned map is keyed by
/// the referenced [`LineageId`]; a lineage the guard does not reference is
/// represented by [`RefDomain::unconstrained`] on demand by the relation
/// helpers, never stored here.
#[must_use]
pub fn analyze_guard(strategy: &Strategy, guard: &Guard) -> Option<HashMap<LineageId, RefDomain>> {
    let mut buckets: HashMap<LineageId, Pending> = HashMap::new();
    for atom in &guard.atoms {
        let decl = resolve_declaration(strategy, atom.evidence)?;
        let tag = match &decl.declared_type {
            DeclaredCategory::One(tag) => *tag,
            // Absent / Multiple / OutOfDomain declared type → check 6 owns it.
            _ => return None,
        };
        let pending = buckets
            .entry(atom.evidence)
            .or_insert_with(|| Pending::new(tag, decl));
        match (tag, &atom.literal, atom.operator) {
            (EvidenceTypeTag::Numeric, Literal::Numeric { value, dimension }, operator) => {
                // The literal's declared dimension must match the referent's
                // declared dimension (§4.6); otherwise it is a check-2
                // TYPE_MISMATCH.
                match &decl.declared_dimension {
                    Declared::Present(declared_dim) if *declared_dim == *dimension => {
                        pending.numeric.push((operator, value.clone()));
                    }
                    _ => return None,
                }
            }
            (
                EvidenceTypeTag::Boolean,
                Literal::Boolean(value),
                ComparisonOperator::Equal | ComparisonOperator::NotEqual,
            ) => {
                pending.boolean.push((to_equality(atom.operator), *value));
            }
            (
                EvidenceTypeTag::Enum,
                Literal::Enum(value),
                ComparisonOperator::Equal | ComparisonOperator::NotEqual,
            ) => {
                pending.enum_c.push((to_equality(atom.operator), *value));
            }
            // Type mismatch (wrong literal shape) or inadmissible operator for
            // the referent's type → check-2 TYPE_MISMATCH → non-computable.
            _ => return None,
        }
    }

    let mut domains = HashMap::with_capacity(buckets.len());
    for (lineage, pending) in buckets {
        let domain = match pending.tag {
            EvidenceTypeTag::Numeric => {
                RefDomain::Numeric(NumericDomain::from_constraints(&pending.numeric))
            }
            EvidenceTypeTag::Boolean => {
                RefDomain::Boolean(BooleanDomain::from_constraints(&pending.boolean))
            }
            EvidenceTypeTag::Enum => {
                let values = pending.enum_values?;
                RefDomain::Enum(EnumDomain::from_constraints(values, &pending.enum_c))
            }
        };
        domains.insert(lineage, domain);
    }
    Some(domains)
}

/// Whether a guard is contradictory: some referenced lineage's domain is empty
/// (§4.10). Unreferenced lineages are unconstrained and never empty under the
/// analyzable precondition.
#[must_use]
pub fn is_contradictory(domains: &HashMap<LineageId, RefDomain>) -> bool {
    domains.values().any(RefDomain::is_empty)
}

/// Reduces a binary per-reference relation over the **union** of both guards'
/// referenced lineages. For a lineage referenced by only one guard, the other's
/// domain is its unconstrained (full declared) domain.
///
/// `combine` is `overlaps` for overlap and `contains` (self ⊇ other) for
/// subsumption. The relation holds only if it holds for **every** referenced
/// lineage. Returns `false` if any lineage is non-computable for either guard.
fn every_reference<F>(
    strategy: &Strategy,
    a: &HashMap<LineageId, RefDomain>,
    b: &HashMap<LineageId, RefDomain>,
    combine: F,
) -> bool
where
    F: Fn(&RefDomain, &RefDomain) -> bool,
{
    let mut lineages: Vec<LineageId> = a.keys().chain(b.keys()).copied().collect();
    lineages.sort();
    lineages.dedup();
    for lineage in lineages {
        let Some(decl) = resolve_declaration(strategy, lineage) else {
            return false;
        };
        let da = a
            .get(&lineage)
            .cloned()
            .or_else(|| RefDomain::unconstrained(decl));
        let db = b
            .get(&lineage)
            .cloned()
            .or_else(|| RefDomain::unconstrained(decl));
        match (da, db) {
            (Some(x), Some(y)) => {
                if !combine(&x, &y) {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}

/// Overlap (§4.10): `D(G1, e) ∩ D(G2, e)` non-empty for every referenced `e`.
#[must_use]
pub fn guards_overlap(
    strategy: &Strategy,
    a: &HashMap<LineageId, RefDomain>,
    b: &HashMap<LineageId, RefDomain>,
) -> bool {
    every_reference(strategy, a, b, RefDomain::overlaps)
}

/// Subsumption (§4.10): `D(G2, e) ⊆ D(G1, e)` for every referenced `e`.
#[must_use]
pub fn guard_subsumes(
    strategy: &Strategy,
    outer: &HashMap<LineageId, RefDomain>,
    inner: &HashMap<LineageId, RefDomain>,
) -> bool {
    // `outer.contains(inner)` ⟺ `D(outer, e) ⊇ D(inner, e)` ⟺ `D(inner, e) ⊆ D(outer, e)`.
    every_reference(strategy, outer, inner, RefDomain::contains)
}
