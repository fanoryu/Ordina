//! Canonical Strategy → editable draft hydration — RFC-0004 §17.
//!
//! Hydration is the reverse of translation (§16): it reconstitutes an existing,
//! already-canonical, already-Well-Formed [`Strategy`] into an editable
//! [`StrategyDraft`] so a correction workflow (RFC-0003 §9.7 stage 2) or an
//! ordinary RFC-0001 §14 revision has a defined starting point.
//!
//! It obeys §17's guards exactly:
//!
//! - **Full reconstitution (§17.1).** Every element the Strategy declares —
//!   authored (state 1) and declared-absent (state 2) alike — is reconstituted;
//!   nothing is omitted or truncated. The bounded editable fields become their
//!   [`FieldState`]/[`ConditionDraft`] representations; every other element is
//!   carried through verbatim (still reconstituted — the draft holds it).
//! - **Already-confirmed (§17.2).** Hydrated content is marked
//!   already-Trader-confirmed; hydration is not a fresh confirmation event and
//!   never asks the Trader to reconfirm content they have not changed.
//! - **No silent modification (§17.3).** Hydration copies content unchanged; it
//!   does not normalize, canonicalize, repair, or reorder. It reads only the
//!   frozen public accessors on `Strategy` — the same surface N-22-A's
//!   diagnosis view already uses — and never mints or alters an identity
//!   (§17.4): minting is exclusively translation's role.
//!
//! Hydration is not a second translation function, a new checker, or a new
//! persistent representation (§17 "What hydration is not") — it is pure,
//! egui-free reconstitution.

use ordina::model::declared::Declared;
use ordina::model::strategy::Strategy;

use crate::draft::{ConditionDraft, FieldState, HydratedOrigin, StrategyDraft};

/// Reconstitute an already-canonical, already-Well-Formed [`Strategy`] into an
/// editable [`StrategyDraft`] — RFC-0004 §17.
///
/// A pure function: no I/O, no mutation of the source, no identity minting, no
/// canonicalization. `strategy` is taken by reference and cloned into the
/// draft's carried-through fields; the source is left untouched.
#[must_use]
pub fn hydrate(strategy: &Strategy) -> StrategyDraft {
    StrategyDraft {
        envelope_occurrence: strategy.occurrence,
        // Envelope lineage is carried verbatim (not editable in the bounded
        // surface) — reconstituted, unchanged (§17.1/§17.3).
        lineage: strategy.lineage,
        // §17.1: a declared-absent version reconstitutes as state 2, an
        // authored one as state 1 — never as state 3, which no already-complete
        // Strategy can be in.
        semantic_model_version: hydrate_semantic_model_version(strategy),
        conditions: hydrate_conditions(strategy),
        // §17.1: preserve the collection's absent-vs-present distinction.
        conditions_present: matches!(strategy.conditions, Declared::Present(_)),
        evidence_declarations: strategy.evidence_declarations.clone(),
        configuration_parameters: strategy.configuration_parameters.clone(),
        direction_rules: strategy.direction_rules.clone(),
        candidate_derivations: strategy.candidate_derivations.clone(),
        // §17.2: hydrated content is already Trader-confirmed, at both the
        // per-construct level (below) and the submission level here.
        submit_confirmed: true,
    }
}

/// §17.1 mapping of the envelope's `Declared<VersionToken>` to a §9 state.
fn hydrate_semantic_model_version(
    strategy: &Strategy,
) -> FieldState<ordina::model::opaque::VersionToken> {
    match &strategy.semantic_model_version {
        // No catch-all arm — `Declared` is a closed two-state category.
        Declared::Present(token) => FieldState::Authored(*token),
        Declared::Absent => FieldState::DeclaredAbsent,
    }
}

/// §17.1/§17.4 reconstitution of the conditions collection.
///
/// Each hydrated condition keeps its occurrence and lineage identity (via
/// [`HydratedOrigin`]) so translation can preserve them (§14.2.3); its
/// classification and guard are copied verbatim, in every declared state, so an
/// arbitrary condition round-trips unchanged. Every hydrated condition is
/// already-confirmed (§17.2).
fn hydrate_conditions(strategy: &Strategy) -> Vec<ConditionDraft> {
    match &strategy.conditions {
        Declared::Absent => Vec::new(),
        Declared::Present(conditions) => conditions
            .iter()
            .map(|condition| ConditionDraft {
                origin: Some(HydratedOrigin {
                    occurrence: condition.occurrence,
                    lineage: condition.lineage,
                }),
                classification: condition.classification.clone(),
                guard: condition.guard.clone(),
                confirmed: true,
            })
            .collect(),
    }
}
