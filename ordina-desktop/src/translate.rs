//! Deterministic draft → canonical Strategy translation — RFC-0004 §10/§12/
//! §14/§16.
//!
//! Translation is the single normative mapping from a submitted authoring
//! draft's confirmed content to `ordina`'s canonical typed construct set. Per
//! RFC-0004 §13's *one shared, normative translation implementation* rule, this
//! is the only place that mapping lives in the desktop authoring surface; no
//! view or widget reimplements it. Its observable input/output relationship is
//! deterministic (§12): the same confirmed draft, translated twice with an
//! identically-seeded minter, produces an identical Strategy — and, regardless
//! of minting, an identical revision-identity-bearing canonical form (§13).
//!
//! # The translation gate (§10) precedes Well-Formedness
//!
//! Before emitting anything, translation enforces §10's completeness gate and
//! §11's confirmations. A draft that has not addressed a minimum-contract
//! element (state 3), that carries unconfirmed content, or that has not been
//! submission-confirmed is **rejected here, before Well-Formedness is ever
//! invoked** ([`TranslationGateError`]). This is a distinct failure class from
//! a Well-Formedness defect (§10) — an omission is not an absence, and a
//! gate rejection is not a checker `Unusable` verdict.
//!
//! # Identity minting (§14)
//!
//! Translation is the authoritative assigner of both opaque identity domains
//! (§14.1 lineage, §14.2 occurrence). A hydrated construct the Trader did not
//! replace keeps its existing identities (§14.2.3/§17.4); a genuinely new one
//! mints fresh (§14.1/§14.2.4). Minting is a session-owned monotonic
//! [`IdentityMinter`] whose values are assigned in submission order, so the
//! order it induces is a deterministic function of the confirmed draft's own
//! structure (§14.3) rather than an incidental artifact. The concrete encoding
//! (a monotonic counter) is one §14.4 explicitly leaves to the implementation;
//! collision-freedom across the Trader's corpus (§14.3, PBD-0008 single-Trader
//! in-process) follows from seeding the counter above the corpus high-water.
//!
//! Per **ADR-0007 (ACCEPTED)**, `OccurrenceId` and its order never reach
//! revision identity, so two sessions confirming the same operative content may
//! mint different occurrence identities while remaining revision-equivalent
//! (§13) — exactly why [`IdentityMinter`] need not reproduce identical
//! occurrence handles across independent sessions to be conforming.

use ordina::model::condition::Condition;
use ordina::model::declared::Declared;
use ordina::model::opaque::{LineageId, OccurrenceId, OpaqueValue};
use ordina::model::strategy::Strategy;
use ordina::persistence::MintHighWater;

use crate::draft::{ConditionDraft, FieldState, StrategyDraft};

/// Why translation refused to emit a Strategy — RFC-0004 §10/§11.
///
/// A closed category, matched exhaustively. Every variant is a
/// **translation-layer** rejection that happens strictly before
/// Well-Formedness (§10) — never a checker defect and never a
/// Well-Formedness `Unusable` verdict.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslationGateError {
    /// §10: a minimum-contract element is still *not yet addressed* (state 3).
    /// Carries a stable identifier of which element, for Trader-facing copy.
    RequiredElementNotAddressed(RequiredElement),
    /// §11: a construct the Trader added or changed has not received its fresh
    /// per-rule confirmation. Carries the draft index of the offending
    /// condition.
    UnconfirmedContent { condition_index: usize },
    /// §11: the distinct final submission confirmation has not been given.
    SubmissionNotConfirmed,
}

/// The minimum-contract elements this bounded surface gates on (§10).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequiredElement {
    /// The envelope's semantic-model version — identity/metadata, part of the
    /// RFC-0001 §7 minimum contract.
    SemanticModelVersion,
}

/// The session-owned occurrence/lineage minter — RFC-0004 §14.2/§14.3/§14.4.
///
/// A monotonic counter shared across both opaque domains: every mint consumes
/// the next handle, so all minted identities are globally distinct and their
/// relative order follows the order translation emits them (submission order,
/// §14.3). Seed it **above** every handle already present in the Trader's
/// Strategy corpus so minted handles never collide with preserved ones
/// (§14.3 collision-freedom; PBD-0008 single-Trader in-process).
///
/// The concrete encoding (this counter) is implementation mechanism §14.4
/// leaves free; what is normative is the resulting order and collision-freedom,
/// both of which this satisfies.
#[derive(Debug, Clone)]
pub struct IdentityMinter {
    /// The next safe handle, or `Exhausted` — ADR-0009 §7. Backed by
    /// `ordina::persistence::MintHighWater` so exhaustion is an explicit state
    /// that survives a save/restart, never a wrapped or reused number.
    state: MintHighWater,
}

impl IdentityMinter {
    /// A minter seeded to hand out handles at and above `first_handle`. The
    /// caller supplies a value above its corpus high-water (§14.3).
    #[must_use]
    pub const fn seeded_at(first_handle: u64) -> Self {
        Self {
            state: MintHighWater::Next(first_handle),
        }
    }

    /// A minter restored from a persisted [`MintHighWater`] (ADR-0009 §7) — the
    /// reload path. A restored `Exhausted` state refuses further minting.
    #[must_use]
    pub const fn from_high_water(state: MintHighWater) -> Self {
        Self { state }
    }

    /// The current high-water, for persisting the envelope (ADR-0009 §7).
    #[must_use]
    pub const fn high_water(&self) -> MintHighWater {
        self.state
    }

    fn next(&mut self) -> u64 {
        // Checked advancement (ADR-0009 §7): never wraps, never reuses. The
        // correction loop must refuse to mint on an exhausted corpus before
        // reaching this point (the desktop checks `high_water().seed()` /
        // exhaustion when opening an editing session), so this fail-closed
        // panic is an unreachable last-resort guard, never a normal path.
        match self.state.issue() {
            Some((handle, next)) => {
                self.state = next;
                handle
            }
            None => panic!(
                "identity mint domain exhausted (u64::MAX reached) — ADR-0009 §7: \
                 the correction loop must refuse to mint on an exhausted corpus"
            ),
        }
    }

    fn mint_occurrence(&mut self) -> OccurrenceId {
        OccurrenceId::new(OpaqueValue::new(self.next()))
    }

    fn mint_lineage(&mut self) -> LineageId {
        LineageId::new(OpaqueValue::new(self.next()))
    }
}

/// Translate a submitted draft into a canonical [`Strategy`] — RFC-0004
/// §10/§12/§14/§16.
///
/// Enforces §10's gate and §11's confirmations first (returning a
/// [`TranslationGateError`] without emitting anything if they are unmet), then
/// emits a fully-typed construct set, preserving hydrated identities and
/// minting fresh ones for new constructs via `minter`. The output is the typed
/// model input to Well-Formedness (§16) — it is **not** canonicalized and
/// carries **no** revision identity; those remain ADR-0003's, run after this.
///
/// # Errors
///
/// Returns [`TranslationGateError`] if a minimum-contract element is not yet
/// addressed (§10), if any construct is unconfirmed (§11), or if the final
/// submission confirmation is absent (§11).
pub fn translate(
    draft: &StrategyDraft,
    minter: &mut IdentityMinter,
) -> Result<Strategy, TranslationGateError> {
    // §10 completeness gate — required elements must be in state 1 or 2.
    let semantic_model_version = match &draft.semantic_model_version {
        FieldState::Authored(token) => Declared::Present(*token),
        FieldState::DeclaredAbsent => Declared::Absent,
        FieldState::NotYetAddressed => {
            return Err(TranslationGateError::RequiredElementNotAddressed(
                RequiredElement::SemanticModelVersion,
            ));
        }
    };

    // §11 per-rule confirmation — every editable construct must be confirmed.
    if let Some(index) = draft.conditions.iter().position(|c| !c.confirmed) {
        return Err(TranslationGateError::UnconfirmedContent {
            condition_index: index,
        });
    }

    // §11 final submission confirmation — the distinct "submit" act.
    if !draft.submit_confirmed {
        return Err(TranslationGateError::SubmissionNotConfirmed);
    }

    // Gate passed: emit a fully-typed construct set (§16). Conditions are
    // emitted in draft order, so any minted occurrence handles follow the
    // confirmed submission's structure (§14.3).
    let conditions = if draft.conditions_present {
        Declared::Present(
            draft
                .conditions
                .iter()
                .map(|condition| translate_condition(condition, minter))
                .collect(),
        )
    } else {
        Declared::Absent
    };

    Ok(Strategy {
        // The envelope is a preserved occurrence (§14.2.3) — never re-minted.
        occurrence: draft.envelope_occurrence,
        lineage: draft.lineage,
        semantic_model_version,
        evidence_declarations: draft.evidence_declarations.clone(),
        configuration_parameters: draft.configuration_parameters.clone(),
        conditions,
        direction_rules: draft.direction_rules.clone(),
        candidate_derivations: draft.candidate_derivations.clone(),
    })
}

/// Translate one condition draft to a canonical [`Condition`] — §14.
///
/// A hydrated condition preserves its occurrence and lineage (§14.2.3/§17.4); a
/// new one mints both fresh (§14.1/§14.2.4). Classification and guard are
/// carried through verbatim.
fn translate_condition(draft: &ConditionDraft, minter: &mut IdentityMinter) -> Condition {
    let (occurrence, lineage) = match &draft.origin {
        // Hydrated, edited-in-place: same occurrence, same lineage.
        Some(origin) => (origin.occurrence, origin.lineage),
        // New construct: mint both fresh, in submission order.
        None => (
            minter.mint_occurrence(),
            Declared::Present(minter.mint_lineage()),
        ),
    };
    Condition {
        occurrence,
        lineage,
        classification: draft.classification.clone(),
        guard: draft.guard.clone(),
    }
}
