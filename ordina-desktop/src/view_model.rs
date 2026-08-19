//! Pure, egui-free translation from `ordina`'s frozen Slice-1 result contract
//! (`WellFormednessResult`) to a Trader-facing, presentation-ready shape.
//!
//! RFC-0003 (ACCEPTED, PBR-0018) §9.4 requires that a defect be presented
//! through a stable, Trader-facing description while its underlying
//! `DefectIdentity` remains the system-of-record identity, never discarded.
//! §9.3 requires presentation order be a pure function of defect identity —
//! never incidental (hash/iteration/insertion order) — while remaining free
//! to differ from the checker's own canonical order.
//!
//! No Strategy-content model exists yet (N-25, OPEN): `DefectCore::offending`
//! is an opaque `OccurrenceId`, not Strategy-authored text, and the
//! discriminator payload types (`PropertyPosition`, `NodePath`, `AtomKey`)
//! expose no public accessors beyond construction and ordering — by design,
//! per §6.4's occurrence-anchor/comparison-only contract. The "where"
//! description is therefore necessarily generic (an opaque construct
//! reference) until N-25 resolves how Strategy content maps onto these
//! identities — a known, unavoidable limit of N-22-A's scope, not an
//! oversight — see PBA-0156.
//!
//! PBA-0157 (bounded remediation) additionally requires that same-category
//! siblings on the same construct remain *visibly distinguishable* even
//! without a payload accessor. [`annotate_sibling_variants`] supplies that:
//! a stable, `Ord`-derived rank suffix distinguishes siblings without ever
//! reading a field no public accessor exposes.

use ordina::model::derivation::CandidateRole;
use ordina::result::{
    DefectCategory, DefectIdentity, Discriminator, NotEvaluatedRecord, WellFormednessResult,
};

/// A single defect, translated for the Trader — RFC-0003 §9.4.
///
/// `identity` is preserved, never discarded: it remains available for a
/// future "Why?" detail view and for correlating a not-evaluated record's
/// `blocking` link back to the defect it names (§9.6).
#[derive(Debug, Clone)]
pub struct TraderFacingDefect {
    pub identity: DefectIdentity,
    /// "What" is wrong — a category-derived, plain-language description.
    pub headline: String,
    /// Enough of the discriminator's *kind* to distinguish this defect from
    /// a same-category sibling on the same construct (§9.4, third bullet).
    pub detail: String,
}

/// A single not-evaluated (suppressed) check, translated for the Trader —
/// RFC-0003 §9.5-9.6. Never a defect; always structurally distinct.
#[derive(Debug, Clone)]
pub struct TraderFacingNotEvaluated {
    /// States inability to evaluate, never failure (§9.5).
    pub message: String,
    /// The defect that caused the suppression — always present (§9.6).
    /// Kept as the full `DefectIdentity`, not summarized away.
    pub blocking: DefectIdentity,
}

/// The complete diagnosis view — everything RFC-0003 §9.1-9.6, §9.11 requires
/// be reachable, with no drip-feed and no defect/not-evaluated conflation.
#[derive(Debug, Clone)]
pub struct DiagnosisViewModel {
    pub verdict: ordina::result::Verdict,
    /// Presentation order (§9.3) — identity-derived and stable, not
    /// necessarily canonical. Complete: every defect the checker found.
    pub defects: Vec<TraderFacingDefect>,
    /// Complete: every suppressed check, structurally separate from defects.
    pub not_evaluated: Vec<TraderFacingNotEvaluated>,
}

impl DiagnosisViewModel {
    /// Builds the view model from an already-computed `WellFormednessResult`.
    /// A pure function: no I/O, no mutation of the checker's own result.
    #[must_use]
    pub fn from_result(result: &WellFormednessResult) -> Self {
        let mut defects: Vec<TraderFacingDefect> = result
            .defects()
            .iter()
            .map(|identity| TraderFacingDefect {
                identity: identity.clone(),
                headline: category_headline(identity.core().category()),
                detail: discriminator_detail(identity.core().discriminator()),
            })
            .collect();

        // PBA-0157 MAJOR remediation: distinguish same-category siblings on
        // the same construct (RFC-0003 §9.4, third bullet) by their relative
        // rank under `DefectCore`'s own `Ord` — never by reading a payload
        // field no public accessor exposes (`PropertyPosition`/`NodePath`
        // remain opaque). Two defects that share both category and
        // occurrence necessarily differ in `Discriminator` (else they would
        // be the identical defect), and `Discriminator` is itself publicly
        // `Ord`-comparable, so a stable relative rank is always available.
        annotate_sibling_variants(&mut defects);

        // §9.3: presentation order is a pure, identity-derived function of
        // the defect set. Primary key: category ordinal (deliberately
        // differs from raw canonical order, whose leading key is the
        // internal check-number ordinal). PBA-0157 MINOR remediation:
        // same-category ties are now broken by `DefectCore`'s own total
        // `Ord` (check, then occurrence, then category, then discriminator)
        // rather than by preserving whatever order the caller happened to
        // supply — deterministic under any input permutation, never
        // incidental, and never hash-based.
        defects.sort_by(|a, b| {
            a.identity
                .core()
                .category()
                .ordinal()
                .cmp(&b.identity.core().category().ordinal())
                .then_with(|| a.identity.core().cmp(b.identity.core()))
        });

        let not_evaluated = result
            .not_evaluated()
            .iter()
            .map(not_evaluated_view)
            .collect();

        Self {
            verdict: result.verdict(),
            defects,
            not_evaluated,
        }
    }

    /// §9.11: for a given defect, the not-evaluated records it is blocking —
    /// discoverable from the defect side too, not only the reverse link.
    #[must_use]
    pub fn blocked_by(&self, defect: &DefectIdentity) -> Vec<&TraderFacingNotEvaluated> {
        self.not_evaluated
            .iter()
            .filter(|ne| ne.blocking.try_eq(defect) == Some(true))
            .collect()
    }
}

/// PBA-0157 MAJOR remediation. For every group of defects sharing both
/// category and offending occurrence (i.e. §9.4's "same-category sibling on
/// the same construct"), appends a stable, identity-derived rank suffix —
/// "(N of M such issues on this construct)" — so siblings are visibly
/// distinguishable even when their discriminator's internal payload
/// (`PropertyPosition`, `NodePath`, ...) has no public accessor to read.
///
/// Deliberately avoids any hash-based grouping (consistent with the
/// determinism discipline `ordina` itself applies to conformance-visible
/// ordering, ADR-0005 §7.6): grouping and ranking both use only `Ord`
/// comparisons over public data (`DefectCore`, `DefectCategory`,
/// `OccurrenceId`), which are already used elsewhere in this module.
fn annotate_sibling_variants(defects: &mut [TraderFacingDefect]) {
    let mut order: Vec<usize> = (0..defects.len()).collect();
    order.sort_by(|&i, &j| {
        let a = defects[i].identity.core();
        let b = defects[j].identity.core();
        a.category()
            .ordinal()
            .cmp(&b.category().ordinal())
            .then_with(|| a.offending().cmp(&b.offending()))
            .then_with(|| a.cmp(b))
    });

    let mut start = 0;
    while start < order.len() {
        let mut end = start + 1;
        while end < order.len() {
            let a = defects[order[start]].identity.core();
            let b = defects[order[end]].identity.core();
            if a.category().ordinal() == b.category().ordinal() && a.offending() == b.offending() {
                end += 1;
            } else {
                break;
            }
        }

        let group_size = end - start;
        if group_size > 1 {
            for (rank, &position) in order[start..end].iter().enumerate() {
                defects[position].detail.push_str(&format!(
                    " ({} of {group_size} such issues on this construct)",
                    rank + 1
                ));
            }
        }

        start = end;
    }
}

fn not_evaluated_view(record: &NotEvaluatedRecord) -> TraderFacingNotEvaluated {
    TraderFacingNotEvaluated {
        // §9.5: inability to evaluate, never phrased as failure.
        message: "not yet checked (blocked by another issue)".to_owned(),
        blocking: record.blocking().clone(),
    }
}

/// §9.4 "what" — a plain-language, category-derived headline.
///
/// Exact Trader-facing copy remains a downstream product-content decision
/// (RFC-0003 §13); this mapping supplies the minimum text N-22-A's diagnosis
/// view needs to exist at all, not a final copy determination.
fn category_headline(category: DefectCategory) -> String {
    match category {
        DefectCategory::MissingRequiredProperty => "A required property is missing".to_owned(),
        DefectCategory::DuplicateLineageIdentity => {
            "A construct is declared more than once".to_owned()
        }
        DefectCategory::UnresolvedReference => "A reference could not be resolved".to_owned(),
        DefectCategory::TypeMismatch => "A value has the wrong type".to_owned(),
        DefectCategory::EnumValueNotDeclared => "An enumerated value isn't recognized".to_owned(),
        DefectCategory::InvalidConditionClassification => {
            "A condition isn't classified correctly".to_owned()
        }
        DefectCategory::InvertedConfigurationBounds => {
            "A configuration's bounds are inverted".to_owned()
        }
        DefectCategory::ConfigurationBoundDimensionMismatch => {
            "A configuration bound has a mismatched dimension".to_owned()
        }
        DefectCategory::ConfigurationInStructuralPosition => {
            "A configuration value is used where structure is required".to_owned()
        }
        DefectCategory::InvalidEvidenceDeclaration => {
            "An evidence declaration is invalid".to_owned()
        }
        DefectCategory::DirectionCoverageIncomplete => {
            "Direction logic doesn't cover every case".to_owned()
        }
        DefectCategory::MultipleTotalDirectionRules => {
            "More than one Direction rule claims to be total".to_owned()
        }
        DefectCategory::TotalRuleNotLowestPrecedence => {
            "The total Direction rule isn't last in precedence".to_owned()
        }
        DefectCategory::DirectionPrecedenceTie => {
            "Two Direction rules tie in precedence".to_owned()
        }
        DefectCategory::InvalidDirectionOutcome => {
            "A Direction rule's outcome is invalid".to_owned()
        }
        DefectCategory::UnreachableContradictoryGuard => {
            "A rule's condition can never be true".to_owned()
        }
        DefectCategory::UnreachableSubsumedRule => "A rule can never be reached".to_owned(),
        DefectCategory::ContradictoryConditionGuard => "A condition contradicts itself".to_owned(),
        DefectCategory::InvalidDerivationForm => {
            "A candidate-value derivation uses a prohibited form".to_owned()
        }
        DefectCategory::MissingCandidateDerivation => {
            "A required candidate-value derivation is missing".to_owned()
        }
        DefectCategory::DuplicateCandidateDerivation => {
            "A candidate-value derivation is declared more than once".to_owned()
        }
        DefectCategory::DimensionMismatch => {
            "A derivation mixes incompatible dimensions".to_owned()
        }
        DefectCategory::CandidateDimensionDivergence => {
            "Candidate derivations disagree on dimension".to_owned()
        }
        DefectCategory::MissingOperativeProvenance => {
            "An operative rule is missing required provenance".to_owned()
        }
        DefectCategory::UnsupportedSemanticModelVersion => {
            "This Strategy uses an unsupported semantic model version".to_owned()
        }
        DefectCategory::UnknownConstruct => "An unrecognized construct was found".to_owned(),
    }
}

/// §9.4 "enough of the discriminator's meaning" — plain-language, limited to
/// the discriminator's *kind* since no public accessor exposes the payload
/// of `Property`/`NodePath`/`Atom` beyond construction and comparison (see
/// module doc). Still sufficient to distinguish a same-category sibling
/// defect by *what kind* of thing differs, if not yet *which specific* one.
fn discriminator_detail(discriminator: &Discriminator) -> String {
    match discriminator {
        Discriminator::None => String::new(),
        Discriminator::Property(_) => "affects a specific required property".to_owned(),
        Discriminator::Role(role) => format!("affects the {} role", role_label(*role)),
        Discriminator::Atom(_) => "affects a specific guard condition".to_owned(),
        Discriminator::NodePath(_) => "affects a specific part of the derivation".to_owned(),
        Discriminator::Counterpart(_) => "affects a related, paired construct too".to_owned(),
        Discriminator::Lineage(_) => "affects a specific duplicated declaration".to_owned(),
    }
}

fn role_label(role: CandidateRole) -> &'static str {
    match role {
        CandidateRole::Entry => "entry",
        CandidateRole::StopLoss => "stop-loss",
        CandidateRole::TakeProfit => "take-profit",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ordina::model::opaque::{OccurrenceId, OpaqueValue};
    use ordina::result::{
        CheckNumber, ChildPosition, DefectCore, NodePath, PropertyPosition, Verdict,
    };

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn defect(handle: u64, category: DefectCategory) -> DefectIdentity {
        DefectIdentity::unqualified(DefectCore::new(occ(handle), category, Discriminator::None))
    }

    fn defect_with_property(
        handle: u64,
        category: DefectCategory,
        position: u32,
    ) -> DefectIdentity {
        DefectIdentity::unqualified(DefectCore::new(
            occ(handle),
            category,
            Discriminator::Property(PropertyPosition::new(position)),
        ))
    }

    fn defect_with_node_path(
        handle: u64,
        category: DefectCategory,
        steps: Vec<ChildPosition>,
    ) -> DefectIdentity {
        DefectIdentity::unqualified(DefectCore::new(
            occ(handle),
            category,
            Discriminator::NodePath(NodePath::new(steps)),
        ))
    }

    #[test]
    fn well_formed_yields_empty_diagnosis() {
        let result = WellFormednessResult::well_formed(Vec::new());
        let vm = DiagnosisViewModel::from_result(&result);
        assert_eq!(vm.verdict, Verdict::WellFormed);
        assert!(vm.defects.is_empty());
        assert!(vm.not_evaluated.is_empty());
    }

    #[test]
    fn every_defect_is_reachable_and_identity_is_preserved() {
        let d1 = defect(10, DefectCategory::MissingRequiredProperty);
        let d2 = defect(11, DefectCategory::UnresolvedReference);
        let result = WellFormednessResult::unusable(d1.clone(), vec![d2.clone()], Vec::new());

        let vm = DiagnosisViewModel::from_result(&result);

        assert_eq!(vm.verdict, Verdict::Unusable);
        assert_eq!(
            vm.defects.len(),
            2,
            "every defect must be reachable, none dropped"
        );
        let identities: Vec<_> = vm.defects.iter().map(|d| d.identity.clone()).collect();
        assert!(identities.iter().any(|i| i.try_eq(&d1) == Some(true)));
        assert!(identities.iter().any(|i| i.try_eq(&d2) == Some(true)));
    }

    #[test]
    fn presentation_order_is_a_pure_function_of_category_never_incidental() {
        // Two defects whose canonical (check-number-first) order and
        // category-ordinal order disagree — category-based presentation
        // order must reorder them, deterministically, regardless of how
        // they were supplied.
        let low_check_high_category = defect(1, DefectCategory::UnknownConstruct);
        let high_check_low_category = defect(2, DefectCategory::MissingRequiredProperty);

        let result = WellFormednessResult::unusable(
            low_check_high_category.clone(),
            vec![high_check_low_category.clone()],
            Vec::new(),
        );
        let vm = DiagnosisViewModel::from_result(&result);

        assert!(
            vm.defects[0].identity.try_eq(&high_check_low_category) == Some(true),
            "presentation order must sort by category ordinal, not canonical check-number order"
        );
        assert!(vm.defects[1].identity.try_eq(&low_check_high_category) == Some(true));
    }

    #[test]
    fn presentation_order_is_deterministic_under_permutation() {
        let a = defect(1, DefectCategory::MissingRequiredProperty);
        let b = defect(2, DefectCategory::TypeMismatch);
        let c = defect(3, DefectCategory::DimensionMismatch);

        let forward =
            WellFormednessResult::unusable(a.clone(), vec![b.clone(), c.clone()], Vec::new());
        let reversed =
            WellFormednessResult::unusable(c.clone(), vec![b.clone(), a.clone()], Vec::new());

        let vm_forward = DiagnosisViewModel::from_result(&forward);
        let vm_reversed = DiagnosisViewModel::from_result(&reversed);

        let order_forward: Vec<_> = vm_forward
            .defects
            .iter()
            .map(|d| d.headline.clone())
            .collect();
        let order_reversed: Vec<_> = vm_reversed
            .defects
            .iter()
            .map(|d| d.headline.clone())
            .collect();
        assert_eq!(
            order_forward, order_reversed,
            "presentation order must depend only on defect identity, never input insertion order"
        );
    }

    #[test]
    fn property_siblings_on_the_same_construct_are_visibly_distinguishable() {
        // PBA-0157 MAJOR remediation: same category, same occurrence,
        // differing only in the (unreadable) Property position.
        let sibling_one = defect_with_property(1, DefectCategory::MissingRequiredProperty, 1);
        let sibling_two = defect_with_property(1, DefectCategory::MissingRequiredProperty, 2);
        let result = WellFormednessResult::unusable(sibling_one, vec![sibling_two], Vec::new());
        let vm = DiagnosisViewModel::from_result(&result);

        assert_eq!(vm.defects.len(), 2);
        assert_ne!(
            vm.defects[0].detail, vm.defects[1].detail,
            "same-category siblings on the same construct must render distinguishable text"
        );
    }

    #[test]
    fn node_path_siblings_on_the_same_construct_are_visibly_distinguishable() {
        // Property/NodePath cases are both covered, per PBA-0157's scope.
        let sibling_one = defect_with_node_path(
            1,
            DefectCategory::DimensionMismatch,
            vec![ChildPosition::LeftOperand],
        );
        let sibling_two = defect_with_node_path(
            1,
            DefectCategory::DimensionMismatch,
            vec![ChildPosition::RightOperand],
        );
        let result = WellFormednessResult::unusable(sibling_one, vec![sibling_two], Vec::new());
        let vm = DiagnosisViewModel::from_result(&result);

        assert_eq!(vm.defects.len(), 2);
        assert_ne!(
            vm.defects[0].detail, vm.defects[1].detail,
            "NodePath-discriminated same-category siblings on the same construct must be distinguishable too"
        );
    }

    #[test]
    fn distinct_constructs_sharing_a_category_are_not_annotated_as_siblings() {
        // §9.4's requirement is scoped to siblings "on the same construct" —
        // defects on different occurrences must not be spuriously annotated.
        let on_construct_a = defect(1, DefectCategory::MissingRequiredProperty);
        let on_construct_b = defect(2, DefectCategory::MissingRequiredProperty);
        let result =
            WellFormednessResult::unusable(on_construct_a, vec![on_construct_b], Vec::new());
        let vm = DiagnosisViewModel::from_result(&result);

        assert!(
            vm.defects.iter().all(|d| d.detail.is_empty()),
            "defects on distinct constructs must not receive a sibling-rank suffix: {:?}",
            vm.defects
        );
    }

    #[test]
    fn sibling_rank_is_identity_derived_and_stable_under_input_reversal() {
        // PBA-0157 MINOR remediation, exercised together with the MAJOR fix:
        // reversing input order for same-category siblings on the same
        // construct must yield the same rank assignment, not a positional
        // one.
        let sibling_one = defect_with_property(1, DefectCategory::MissingRequiredProperty, 1);
        let sibling_two = defect_with_property(1, DefectCategory::MissingRequiredProperty, 2);

        let forward = WellFormednessResult::unusable(
            sibling_one.clone(),
            vec![sibling_two.clone()],
            Vec::new(),
        );
        let reversed = WellFormednessResult::unusable(
            sibling_two.clone(),
            vec![sibling_one.clone()],
            Vec::new(),
        );

        let vm_forward = DiagnosisViewModel::from_result(&forward);
        let vm_reversed = DiagnosisViewModel::from_result(&reversed);

        let detail_for = |vm: &DiagnosisViewModel, id: &DefectIdentity| {
            vm.defects
                .iter()
                .find(|d| d.identity.try_eq(id) == Some(true))
                .expect("defect present")
                .detail
                .clone()
        };

        assert_eq!(
            detail_for(&vm_forward, &sibling_one),
            detail_for(&vm_reversed, &sibling_one),
            "a given defect's sibling rank must not depend on input order"
        );
        assert_eq!(
            detail_for(&vm_forward, &sibling_two),
            detail_for(&vm_reversed, &sibling_two)
        );
    }

    #[test]
    fn presentation_order_same_category_tie_break_is_identity_derived_not_positional() {
        // PBA-0157 MINOR remediation, direct test: two same-category
        // defects supplied in non-canonical (occurrence-descending) order
        // must still resolve to the same relative presentation order
        // regardless of which one was passed as the "first" defect.
        let lower_occurrence = defect(1, DefectCategory::MissingRequiredProperty);
        let higher_occurrence = defect(2, DefectCategory::MissingRequiredProperty);

        let supplied_high_first = WellFormednessResult::unusable(
            higher_occurrence.clone(),
            vec![lower_occurrence.clone()],
            Vec::new(),
        );
        let supplied_low_first = WellFormednessResult::unusable(
            lower_occurrence.clone(),
            vec![higher_occurrence.clone()],
            Vec::new(),
        );

        let vm_high_first = DiagnosisViewModel::from_result(&supplied_high_first);
        let vm_low_first = DiagnosisViewModel::from_result(&supplied_low_first);

        assert!(
            vm_high_first.defects[0].identity.try_eq(&lower_occurrence) == Some(true),
            "same-category tie-break must order by occurrence identity, not input position"
        );
        assert!(
            vm_low_first.defects[0].identity.try_eq(&lower_occurrence) == Some(true),
            "the identity-derived tie-break must be identical regardless of caller input order"
        );
    }

    #[test]
    fn not_evaluated_is_structurally_separate_and_never_phrased_as_failure() {
        let blocker = defect(1, DefectCategory::UnresolvedReference);
        let not_evaluated = NotEvaluatedRecord::new(CheckNumber::C13A, occ(2), blocker.clone());
        let result = WellFormednessResult::unusable(blocker, Vec::new(), vec![not_evaluated]);

        let vm = DiagnosisViewModel::from_result(&result);

        assert_eq!(
            vm.defects.len(),
            1,
            "not-evaluated must never be counted as a defect"
        );
        assert_eq!(vm.not_evaluated.len(), 1);
        assert!(
            !vm.not_evaluated[0].message.to_lowercase().contains("fail"),
            "not-evaluated language must state inability to evaluate, never failure"
        );
    }

    #[test]
    fn blocker_link_is_navigable_from_the_defect_side() {
        let blocker = defect(1, DefectCategory::UnresolvedReference);
        let suppressed_a = NotEvaluatedRecord::new(CheckNumber::C13A, occ(2), blocker.clone());
        let suppressed_b = NotEvaluatedRecord::new(CheckNumber::C14A, occ(3), blocker.clone());
        let result = WellFormednessResult::unusable(
            blocker.clone(),
            Vec::new(),
            vec![suppressed_a, suppressed_b],
        );

        let vm = DiagnosisViewModel::from_result(&result);
        let fanned_out = vm.blocked_by(&blocker);

        assert_eq!(
            fanned_out.len(),
            2,
            "a defect blocking multiple suppressions must expose the full fan-out from the defect side (§9.11)"
        );
    }

    #[test]
    fn multiple_defects_and_suppressions_all_remain_enumerable() {
        let d1 = defect(1, DefectCategory::MissingRequiredProperty);
        let d2 = defect(2, DefectCategory::UnresolvedReference);
        let d3 = defect(3, DefectCategory::TypeMismatch);
        let ne1 = NotEvaluatedRecord::new(CheckNumber::C13A, occ(4), d1.clone());
        let ne2 = NotEvaluatedRecord::new(CheckNumber::C14A, occ(5), d2.clone());

        let result = WellFormednessResult::unusable(d1, vec![d2, d3], vec![ne1, ne2]);
        let vm = DiagnosisViewModel::from_result(&result);

        assert_eq!(
            vm.defects.len(),
            3,
            "N >= 1 defects must all be enumerable, no collapsing"
        );
        assert_eq!(
            vm.not_evaluated.len(),
            2,
            "M >= 0 not-evaluated records must all be enumerable"
        );
    }
}
