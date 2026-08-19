//! The desktop-local correction loop — RFC-0004 §16/§17/§19, integrating the
//! N-22-B-1 revision-identity API (PBA-0168).
//!
//! This is the complete local correction workflow N-22-B's blocking question
//! was about: a Trader is shown a diagnosis, edits the offending content, the
//! edit is confirmed and translated, the **entire** Strategy is re-checked
//! fresh, and a Well-Formed result yields a revision identity. The pipeline is
//! exactly RFC-0004 §17's:
//!
//! > Canonical Strategy → Hydration → editable draft → Trader edit → explicit
//! > confirmation → deterministic translation → full fresh Well-Formedness
//! > check → (if Well-Formed) revision identity.
//!
//! # No separate correction pipeline (§16), full re-check only (§19)
//!
//! A correction-flow re-entry produces a new submission that re-enters the same
//! Draft → Translation → Well-Formed → Canonicalize → Identity pipeline as
//! first-time authoring — there is no "patch only the changed rule" path. Every
//! submission re-checks the **entire** Strategy and returns a **fresh**
//! [`WellFormednessResult`], never a delta (§19).
//!
//! # No speculative fixed state (§19)
//!
//! The committed Strategy, its diagnosis, and its revision identity change
//! **only** once a fresh Well-Formedness result exists. While a draft is being
//! edited, and on a translation-gate rejection, the previously committed
//! Strategy remains the operative one — an in-progress edit never silently
//! supersedes the revision a live decision may be bound to (§19, C5).
//!
//! # The checker context is caller-supplied canonical input
//!
//! `ordina`'s Well-Formedness pipeline requires two canonical contract inputs —
//! the dimensionless-symbol [`DimensionBinding`] (checks 14A/14B) and the
//! supported semantic-model version token (check 16). Per `opaque.rs`/§14.3
//! these *arrive as part of the canonical input*; the checker "never generates,
//! infers or chooses" them. The session therefore holds them as
//! [`CheckerContext`], exactly as any caller of `run_phase6` must — this is the
//! existing public checker contract, not a new architecture decision.

use ordina::checks::finalize::finalize_phase6;
use ordina::checks::orchestration::run_phase6;
use ordina::model::opaque::VersionToken;
use ordina::model::strategy::Strategy;
use ordina::result::compute_revision_identity;
use ordina::result::{RevisionIdentity, RevisionIdentityError, Verdict, WellFormednessResult};
use ordina::semantics::dimension::DimensionBinding;

use crate::draft::StrategyDraft;
use crate::hydrate::hydrate;
use crate::translate::{IdentityMinter, TranslationGateError, translate};

/// The canonical contract inputs the Well-Formedness pipeline requires —
/// supplied by the caller, never invented (`opaque.rs`, RFC-0004 §14.3).
#[derive(Debug, Clone)]
pub struct CheckerContext {
    /// The distinguished dimensionless-symbol binding (checks 14A/14B).
    pub binding: DimensionBinding,
    /// The contract-declared supported semantic-model version token (check 16).
    pub supported: VersionToken,
}

impl CheckerContext {
    /// Run the complete Well-Formedness pipeline over a whole Strategy — the
    /// public `run_phase6` → `finalize_phase6` composition. A pure function of
    /// the Strategy and this context.
    #[must_use]
    pub fn check(&self, strategy: &Strategy) -> WellFormednessResult {
        finalize_phase6(run_phase6(strategy, &self.binding, self.supported))
    }
}

/// Where the correction loop currently is (§17 pipeline).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    /// Showing the diagnosis for the committed Strategy; no active edit.
    Diagnosing,
    /// An editable draft is open (hydrated from the committed Strategy).
    Editing,
}

/// The complete local correction loop over one Trader's Strategy.
///
/// Holds the committed Strategy and its current diagnosis, the checker context,
/// the session-owned identity minter, and — while editing — the active draft.
pub struct CorrectionSession {
    context: CheckerContext,
    strategy: Strategy,
    result: WellFormednessResult,
    draft: Option<StrategyDraft>,
    minter: IdentityMinter,
}

impl CorrectionSession {
    /// Begin a session over an existing canonical `strategy`, running the
    /// initial full Well-Formedness check to establish its diagnosis.
    ///
    /// `mint_base` seeds the identity minter: it MUST be above every opaque
    /// handle already present in the Trader's Strategy corpus, so newly minted
    /// identities never collide with preserved ones (RFC-0004 §14.3; PBD-0008
    /// single-Trader in-process). Opaque handles are not externally readable,
    /// so the caller — which authored the corpus — supplies this high-water.
    #[must_use]
    pub fn begin(context: CheckerContext, strategy: Strategy, mint_base: u64) -> Self {
        let result = context.check(&strategy);
        Self {
            context,
            strategy,
            result,
            draft: None,
            minter: IdentityMinter::seeded_at(mint_base),
        }
    }

    /// The canonical checker context (its supported token, for offering the
    /// bounded semantic-version edit).
    #[must_use]
    pub fn context(&self) -> &CheckerContext {
        &self.context
    }

    /// The current pipeline stage (§17).
    #[must_use]
    pub fn stage(&self) -> Stage {
        if self.draft.is_some() {
            Stage::Editing
        } else {
            Stage::Diagnosing
        }
    }

    /// The committed Strategy's current diagnosis (§19 — always a fresh,
    /// whole-Strategy result, never a delta).
    #[must_use]
    pub fn result(&self) -> &WellFormednessResult {
        &self.result
    }

    /// The committed Strategy itself (read-only).
    #[must_use]
    pub fn strategy(&self) -> &Strategy {
        &self.strategy
    }

    /// The committed Strategy's revision identity (ADR-0008, via N-22-B-1) —
    /// **only** when it is Well-Formed.
    ///
    /// ADR-0003 §5 fixes the ordering `Well-Formed → canonicalize → identity`,
    /// and [`compute_revision_identity`] takes Well-Formedness as a caller
    /// precondition (N-22-B-1). The session establishes Well-Formedness via the
    /// checker before computing identity, so an `Unusable` Strategy correctly
    /// has **no** revision identity here. Returns `None` while `Unusable`, and
    /// the computed identity while `WellFormed`.
    ///
    /// # Errors
    ///
    /// Propagates [`RevisionIdentityError`] from `compute_revision_identity`;
    /// under the established Well-Formedness precondition this does not arise.
    pub fn revision_identity(&self) -> Option<Result<RevisionIdentity, RevisionIdentityError>> {
        match self.result.verdict() {
            Verdict::WellFormed => Some(compute_revision_identity(&self.strategy)),
            Verdict::Unusable => None,
        }
    }

    /// Open an editable draft by hydrating the committed Strategy (§17). Does
    /// **not** alter the committed Strategy, result, or identity — the prior
    /// revision remains operative until a submission returns a fresh result
    /// (§19). Idempotent: re-opening keeps any in-progress draft.
    pub fn begin_edit(&mut self) {
        if self.draft.is_none() {
            self.draft = Some(hydrate(&self.strategy));
        }
    }

    /// Abandon the active draft without consequence (§8) — the committed
    /// Strategy and its diagnosis are untouched.
    pub fn abandon_edit(&mut self) {
        self.draft = None;
    }

    /// The active draft, for the Trader's bounded edits. `None` when not
    /// editing.
    pub fn draft_mut(&mut self) -> Option<&mut StrategyDraft> {
        self.draft.as_mut()
    }

    /// Submit the active draft: translate it (§10/§14), run a **full fresh**
    /// Well-Formedness check over the whole resulting Strategy (§19), and — only
    /// then — commit the new Strategy and its diagnosis, closing the draft.
    ///
    /// On a translation-gate rejection nothing is committed and the draft stays
    /// open (§19 — no speculative fixed state); the committed Strategy and its
    /// revision identity are unchanged.
    ///
    /// # Errors
    ///
    /// Returns [`SubmitError::NoDraft`] if no draft is open, or
    /// [`SubmitError::Gate`] wrapping the [`TranslationGateError`] if the draft
    /// fails §10/§11.
    pub fn submit(&mut self) -> Result<Verdict, SubmitError> {
        let draft = self.draft.as_ref().ok_or(SubmitError::NoDraft)?;
        // Translate first. A gate rejection leaves everything committed intact.
        let new_strategy = translate(draft, &mut self.minter).map_err(SubmitError::Gate)?;
        // §19: full fresh re-check over the ENTIRE Strategy, not a delta.
        let new_result = self.context.check(&new_strategy);
        let verdict = new_result.verdict();
        // Commit only now that a fresh result exists (§19 — no speculative
        // fixed state before this point).
        self.strategy = new_strategy;
        self.result = new_result;
        self.draft = None;
        Ok(verdict)
    }
}

/// Why a [`CorrectionSession::submit`] did not commit a new Strategy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmitError {
    /// No draft was open to submit.
    NoDraft,
    /// The draft failed the §10/§11 translation gate; nothing was committed.
    Gate(TranslationGateError),
}
