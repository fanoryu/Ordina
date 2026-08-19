//! Desktop-local authoring draft — the editable, pre-canonical representation
//! RFC-0004 §7–§11/§14/§17 govern. **Not a Strategy** (§8): it has no revision
//! identity, no Well-Formedness verdict, and no guarantee of submission.
//!
//! # Bounded first Trader editing surface (PBA-0168)
//!
//! PBA-0168 authorizes *a bounded first Trader editing surface — not a complete
//! generic authoring product* and requires the exact supported construct/
//! property set to be documented. This draft **reconstitutes every** RFC-0001
//! §7 element the canonical Strategy declares (§17.1 — nothing is omitted or
//! truncated), but exposes **edit operations** for exactly:
//!
//! - the envelope's **semantic-model version** (`semantic_model_version`), as a
//!   full §9 three-state field (authored / declared-absent / not-yet-addressed);
//!   and
//! - the **conditions** collection — add a new condition, edit an existing
//!   condition's classification in place, and remove a condition.
//!
//! Every other element (envelope lineage, evidence declarations, configuration
//! parameters, direction rules, candidate derivations) is hydrated and carried
//! through translation **verbatim**, with its existing identities preserved
//! (§14/§17.4). Carrying an element unchanged is still full reconstitution:
//! the draft holds it, and re-translation reproduces it. This bound keeps the
//! surface small and reviewable without ever hydrating an incomplete draft.
//!
//! # The three field-presence states (§9)
//!
//! [`FieldState`] models §9's three translation-output states exactly:
//! *explicitly authored* (state 1), *explicitly declared absent* (state 2), and
//! *not yet addressed* (state 3). Translation (§10) rejects submission of a
//! required element still in state 3 — an **omission is not an absence**.
//!
//! # Confirmation (§11)
//!
//! Every editable construct carries its own `confirmed` flag (per-rule
//! confirmation), and the draft carries a single `submit_confirmed` flag (the
//! distinct final submission confirmation). Hydrated, unchanged content is
//! already-confirmed (§17.2 — no forced re-confirmation of untouched content);
//! any add or in-place change clears the relevant confirmation, requiring a
//! fresh one before the content may enter a submission.

use ordina::model::condition::ConditionClassification;
use ordina::model::config::ConfigurationParameter;
use ordina::model::declared::{Declared, DeclaredCategory};
use ordina::model::derivation::CandidateDerivation;
use ordina::model::direction::DirectionRule;
use ordina::model::evidence::EvidenceDeclaration;
use ordina::model::guard::Guard;
use ordina::model::opaque::{LineageId, OccurrenceId, VersionToken};

/// §9's three translation-output states for one contract element.
///
/// Deliberately distinct from `ordina`'s two-state [`Declared`] (present /
/// absent): a draft must additionally represent *not yet addressed* (state 3),
/// which is exactly the state §10's translation gate rejects and which
/// [`Declared`] — a property of an already-translated canonical model — cannot
/// hold. Hydration maps `Declared::Present`→[`FieldState::Authored`] and
/// `Declared::Absent`→[`FieldState::DeclaredAbsent`] (§17.1); state 3 arises
/// only in first-time authoring, never from hydrating an already-complete
/// Strategy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldState<T> {
    /// State 1 — the Trader confirmed a specific value.
    Authored(T),
    /// State 2 — the Trader affirmatively indicated the element does not apply.
    DeclaredAbsent,
    /// State 3 — the draft has not reached this element yet.
    NotYetAddressed,
}

/// The existing identities a hydrated construct carries forward (§14/§17.4).
///
/// Present on a construct reconstituted from an already-canonical Strategy;
/// absent (`None` at the [`ConditionDraft`] level) on a genuinely new construct
/// the Trader adds, which mints fresh identities at translation time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HydratedOrigin {
    pub(crate) occurrence: OccurrenceId,
    pub(crate) lineage: Declared<LineageId>,
}

/// One condition in the draft — RFC-0004 §7/§14/§17.
///
/// A hydrated condition (`origin: Some`) preserves its occurrence and lineage
/// identity on re-submission (§14.2.3, §17.4); a new condition (`origin: None`)
/// mints both fresh at translation (§14.1/§14.2.4). `classification` and
/// `guard` retain **all** of `ordina`'s declared states so an arbitrary
/// hydrated condition round-trips verbatim; the bounded edit operations only
/// ever set `classification` to a single [`ConditionClassification`].
#[derive(Debug, Clone)]
pub struct ConditionDraft {
    pub(crate) origin: Option<HydratedOrigin>,
    pub(crate) classification: DeclaredCategory<ConditionClassification>,
    pub(crate) guard: Declared<Guard>,
    pub(crate) confirmed: bool,
}

impl ConditionDraft {
    /// Whether this condition is a hydrated (identity-preserving) one.
    #[must_use]
    pub fn is_hydrated(&self) -> bool {
        self.origin.is_some()
    }

    /// Whether this condition has been confirmed for submission (§11).
    #[must_use]
    pub fn is_confirmed(&self) -> bool {
        self.confirmed
    }

    /// Give this condition its fresh per-rule confirmation (§11).
    pub fn confirm(&mut self) {
        self.confirmed = true;
    }

    /// Edit the classification **in place** (§14.1/§14.2.3 — identity
    /// preserved). Requires a fresh per-rule confirmation afterward (§11): the
    /// change clears the confirmed flag.
    pub fn set_classification(&mut self, classification: ConditionClassification) {
        self.classification = DeclaredCategory::One(classification);
        self.confirmed = false;
    }
}

/// The editable authoring draft (§7–§11/§14/§17).
///
/// Holds the envelope's preserved occurrence identity, the bounded editable
/// fields (`semantic_model_version`, `conditions`), and every other element
/// carried through verbatim. `submit_confirmed` is §11's distinct final
/// submission confirmation, separate from each construct's per-rule flag.
#[derive(Debug, Clone)]
pub struct StrategyDraft {
    pub(crate) envelope_occurrence: OccurrenceId,
    pub(crate) lineage: Declared<LineageId>,
    pub(crate) semantic_model_version: FieldState<VersionToken>,
    pub(crate) conditions: Vec<ConditionDraft>,
    /// Whether the conditions collection is *present* (§9 state 1/2) rather
    /// than declared-absent. Preserves the absent-vs-present-empty distinction
    /// through a hydrate→translate round-trip: an absent collection stays
    /// absent, a present-but-empty one stays present. Adding a condition makes
    /// the collection present; removing the last one leaves it present-empty.
    pub(crate) conditions_present: bool,
    pub(crate) evidence_declarations: Declared<Vec<EvidenceDeclaration>>,
    pub(crate) configuration_parameters: Declared<Vec<ConfigurationParameter>>,
    pub(crate) direction_rules: Declared<Vec<DirectionRule>>,
    pub(crate) candidate_derivations: Declared<Vec<CandidateDerivation>>,
    pub(crate) submit_confirmed: bool,
}

impl StrategyDraft {
    /// The current semantic-model-version field state (§9).
    #[must_use]
    pub fn semantic_model_version(&self) -> &FieldState<VersionToken> {
        &self.semantic_model_version
    }

    /// Set the semantic-model version to an explicit value (§9 state 1).
    ///
    /// Editing the envelope is a change to submittable content, so it clears
    /// the final submission confirmation (§11): the Trader must re-submit.
    pub fn set_semantic_model_version(&mut self, token: VersionToken) {
        self.semantic_model_version = FieldState::Authored(token);
        self.submit_confirmed = false;
    }

    /// Mark the semantic-model version explicitly declared-absent (§9 state 2).
    pub fn declare_semantic_model_version_absent(&mut self) {
        self.semantic_model_version = FieldState::DeclaredAbsent;
        self.submit_confirmed = false;
    }

    /// Return the semantic-model version to *not yet addressed* (§9 state 3).
    /// Submission is then gated by §10 until it reaches state 1 or 2.
    pub fn clear_semantic_model_version(&mut self) {
        self.semantic_model_version = FieldState::NotYetAddressed;
        self.submit_confirmed = false;
    }

    /// The conditions currently in the draft (read-only).
    #[must_use]
    pub fn conditions(&self) -> &[ConditionDraft] {
        &self.conditions
    }

    /// Whether the conditions collection is declared-absent (§9 state 2) rather
    /// than present. A present-but-empty collection returns `false`.
    #[must_use]
    pub fn conditions_declared_absent(&self) -> bool {
        !self.conditions_present
    }

    /// Mutable access to one condition, for a bounded in-place edit (§14.2.3).
    /// Returns `None` if `index` is out of range.
    pub fn condition_mut(&mut self, index: usize) -> Option<&mut ConditionDraft> {
        let condition = self.conditions.get_mut(index)?;
        // Any in-place edit invalidates the standing submission confirmation.
        self.submit_confirmed = false;
        Some(condition)
    }

    /// Add a genuinely new condition (§14.1/§14.2.4 — mints fresh identities at
    /// translation). The condition starts **unconfirmed** (§11) and is returned
    /// by index so the caller can confirm it after review. Adding content
    /// clears the final submission confirmation.
    pub fn add_condition(
        &mut self,
        classification: ConditionClassification,
        guard: Guard,
    ) -> usize {
        self.conditions.push(ConditionDraft {
            origin: None,
            classification: DeclaredCategory::One(classification),
            guard: Declared::Present(guard),
            confirmed: false,
        });
        // Adding content to a declared-absent collection makes it present.
        self.conditions_present = true;
        self.submit_confirmed = false;
        self.conditions.len() - 1
    }

    /// Remove a condition the Trader explicitly deletes (§14.2.5/§17.6). Its
    /// occurrence identity is simply absent from the next submission and is
    /// never reused. Returns `true` if a condition was removed.
    pub fn remove_condition(&mut self, index: usize) -> bool {
        if index >= self.conditions.len() {
            return false;
        }
        self.conditions.remove(index);
        self.submit_confirmed = false;
        true
    }

    /// Whether the final submission confirmation has been given (§11).
    #[must_use]
    pub fn is_submission_confirmed(&self) -> bool {
        self.submit_confirmed
    }

    /// Give §11's distinct final submission confirmation. This is the "submit"
    /// action, separate from every per-rule confirmation; it does not itself
    /// confirm any unconfirmed construct (translation still gates on those).
    pub fn confirm_submission(&mut self) {
        self.submit_confirmed = true;
    }
}
