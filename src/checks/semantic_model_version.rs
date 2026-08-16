//! Check 16 — semantic-model version / unknown construct — Slice Contract §5.1
//! check 16.
//!
//! Check 16 owns two categories. This slice implements the reachable one — the
//! semantic-model version — and records why the other is structurally
//! unreachable in the Slice-1 typed model.
//!
//! # `UNSUPPORTED_SEMANTIC_MODEL_VERSION` (reachable)
//!
//! The Strategy envelope's `semantic_model_version` must be **present and equal to
//! the contract-declared supported token `SM-1`** (§3.4.1). An **absent** token or
//! **any other** token is one `UNSUPPORTED_SEMANTIC_MODEL_VERSION` defect, anchored
//! on the envelope occurrence. This absence is **check 16's exclusively** — check 1
//! never reports a missing version (Revision 3 §C.2).
//!
//! ## Why the supported token is a parameter
//!
//! `SM-1` is an **opaque identifier compared by exact equality** that the contract
//! declares and **an implementation must not select** (§3.4.1); version tokens are
//! caller-supplied opaque values (§3.3.1), and the Phase-1 model deliberately does
//! **not** encode which token is `SM-1`. The contract-declared supported token is
//! therefore supplied to [`check_16`] as canonical contract input and compared by
//! equality only. The checker invents nothing, and two conforming checkers given
//! the same supported token agree. This does not widen or reinterpret `SM-1`, and
//! it does not touch the runtime evaluator-semantics version (**N-24 stays OPEN**).
//!
//! # `UNKNOWN_CONSTRUCT` (structurally unreachable here)
//!
//! §5.1 check 16 also names `UNKNOWN_CONSTRUCT` for "any unknown construct kind or
//! undeclared property", fail closed. In the Slice-1 typed model that state is
//! **unrepresentable by construction**: the `Strategy` (§3.4) admits no property
//! outside its table, and every construct is a closed typed shape — an undeclared
//! property is dropped or rejected at the external-shape boundary **before** a
//! typed `Strategy` exists (see `model::strategy`). There is therefore no input
//! this function could receive that carries an unknown construct, so it emits no
//! `UNKNOWN_CONSTRUCT` and **none is invented**: manufacturing an untyped
//! value tree to make the category reachable would expand Phase 1, which this
//! slice must not do. The category remains defined in the result model for a
//! future slice/boundary that owns external-shape admission.
//!
//! This is defect production only — a `Vec<DefectCore>`, no gating, verdict, or
//! canonical sort.

use crate::model::declared::Declared;
use crate::model::opaque::VersionToken;
use crate::model::strategy::Strategy;
use crate::result::category::DefectCategory;
use crate::result::discriminator::Discriminator;
use crate::result::identity::DefectCore;

/// Check 16 — the semantic-model version half — over a Strategy.
///
/// `supported` is the contract-declared `SM-1` token, supplied as canonical
/// contract input (§3.4.1). The envelope's version is compared to it by exact
/// equality (§3.3.1): absent, or present-and-unequal, is one
/// `UNSUPPORTED_SEMANTIC_MODEL_VERSION` anchored on the envelope occurrence, with
/// the NONE discriminator (there is at most one per Strategy, so key 2 already
/// distinguishes it).
///
/// `UNKNOWN_CONSTRUCT` is unreachable in the typed model (see the module docs), so
/// it is never emitted here.
#[must_use]
pub fn check_16(strategy: &Strategy, supported: VersionToken) -> Vec<DefectCore> {
    let unsupported = match &strategy.semantic_model_version {
        // Absent version → check 16 (not check 1).
        Declared::Absent => true,
        // Present but not the contract-declared supported token → unsupported.
        Declared::Present(version) => *version != supported,
    };

    if unsupported {
        vec![DefectCore::new(
            strategy.occurrence,
            DefectCategory::UnsupportedSemanticModelVersion,
            Discriminator::None,
        )]
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::opaque::{LineageId, OccurrenceId, OpaqueValue};
    use crate::result::category::CheckNumber;

    fn occ(handle: u64) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(handle))
    }

    fn version(handle: u64) -> VersionToken {
        VersionToken::new(OpaqueValue::new(handle))
    }

    /// The contract-declared supported token for these tests. Its concrete handle
    /// is irrelevant — check 16 compares by equality against whatever token the
    /// caller supplies as supported.
    fn supported() -> VersionToken {
        version(1)
    }

    fn envelope(semantic_model_version: Declared<VersionToken>) -> Strategy {
        Strategy {
            occurrence: occ(1),
            lineage: Declared::Present(LineageId::new(OpaqueValue::new(1))),
            semantic_model_version,
            evidence_declarations: Declared::Present(vec![]),
            configuration_parameters: Declared::Present(vec![]),
            conditions: Declared::Present(vec![]),
            direction_rules: Declared::Present(vec![]),
            candidate_derivations: Declared::Present(vec![]),
        }
    }

    fn single_version_defect(defects: &[DefectCore]) -> &DefectCore {
        assert_eq!(defects.len(), 1);
        let defect = &defects[0];
        assert_eq!(
            defect.category(),
            DefectCategory::UnsupportedSemanticModelVersion
        );
        assert_eq!(defect.category().check(), CheckNumber::C16);
        match defect.discriminator() {
            Discriminator::None => {}
            Discriminator::Property(_)
            | Discriminator::Role(_)
            | Discriminator::Atom(_)
            | Discriminator::NodePath(_)
            | Discriminator::Counterpart(_)
            | Discriminator::Lineage(_) => panic!("check 16 must use the NONE discriminator"),
        }
        defect
    }

    #[test]
    fn absent_version_yields_one_unsupported_defect() {
        let strategy = envelope(Declared::Absent);
        let defects = check_16(&strategy, supported());
        let defect = single_version_defect(&defects);
        assert_eq!(defect.offending(), occ(1)); // envelope anchor
    }

    #[test]
    fn supported_version_yields_no_defect() {
        let strategy = envelope(Declared::Present(supported()));
        assert!(check_16(&strategy, supported()).is_empty());
    }

    #[test]
    fn unsupported_token_yields_one_defect_anchored_on_the_envelope() {
        let strategy = envelope(Declared::Present(version(999)));
        let defects = check_16(&strategy, supported());
        let defect = single_version_defect(&defects);
        assert_eq!(defect.offending(), occ(1));
    }

    #[test]
    fn envelope_lineage_absence_does_not_prevent_check_sixteen_anchoring() {
        // The version defect anchors on the envelope occurrence regardless of the
        // envelope's lineage state.
        let mut strategy = envelope(Declared::Absent);
        strategy.lineage = Declared::Absent;
        let defects = check_16(&strategy, supported());
        let defect = single_version_defect(&defects);
        assert_eq!(defect.offending(), occ(1));
    }

    #[test]
    fn check_sixteen_emits_only_its_own_category() {
        // Never a MISSING_REQUIRED_PROPERTY (check 1) or MISSING_OPERATIVE_PROVENANCE
        // (check 15) — and never UNKNOWN_CONSTRUCT, which is unrepresentable in the
        // typed model (see module docs).
        for strategy in [
            envelope(Declared::Absent),
            envelope(Declared::Present(version(999))),
            envelope(Declared::Present(supported())),
        ] {
            for defect in check_16(&strategy, supported()) {
                assert_eq!(
                    defect.category(),
                    DefectCategory::UnsupportedSemanticModelVersion
                );
            }
        }
    }
}
