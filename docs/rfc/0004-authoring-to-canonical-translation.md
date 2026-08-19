# RFC-0004: Authoring-to-Canonical Translation Contract

- **Status:** PROPOSED
- **Author:** Principal Engineer
- **Created:** 2026-08-19
- **Decision date:** —
- **Authorizing artifact:** PBA-0158 ("N-25 RFC Drafting Authorization"), under
  the **PBD-0007** post-Slice-1 roadmap-direction record, following N-22-A
  CLOSED/COMPLETE (ARR-0055) and N-22-B's block on this RFC
- **Remediation:** bounded remediation of four MAJOR findings from an
  independent read-only review (0 BLOCKER, 4 MAJOR, 2 MINOR, 2 NOTE) —
  OccurrenceId minting (§14.2), opaque-handle order validity (§14.3),
  canonical→draft hydration (§17), and multi-surface convergence
  architecture (§13) — authorized by **PBA-0159**. Awaiting independent
  re-review; RFC remains PROPOSED.
- **Originating source:** ADR-0002 §15 ("Later RFC — authoring-to-canonical
  translation contract (and N-25)"); ADR-0003 §16 ("N-25 (authoring-to-canonical
  translation)"); `open-questions.md` N-25; RFC-0001 §9 (hybrid authoring model,
  Q-16); RFC-0003 §9.7 stage 2 ("this reuses whatever authoring surface RFC-0001
  §9's hybrid authoring model establishes; this RFC does not define that
  surface")

> **Subordinate to** Product Constitution **v1.0 — ACTIVE — RATIFIED** and to
> the applicable Tier-2/Tier-3 authority: `trading-decision-model.md`,
> `validation-model.md`, `ux-constitution.md`, `user-zero.md`,
> `system-context.md`, RFC-0001 (ACCEPTED, PBR-0003), RFC-0003 (ACCEPTED,
> PBR-0018), ADR-0002 (ACCEPTED, PBR-0006), ADR-0003 (ACCEPTED, PBR-0007),
> ADR-0004 (ACCEPTED). This RFC may refine, but may not contradict, any of
> them. It is a **product-and-contract** RFC: it defines the contract by which
> any authoring surface (wizard, template, visual builder, natural-language
> assistance) produces the one authoritative canonical `Strategy`. It selects
> no UI, no framework, no serialization, no persistence, no evaluator, and no
> concrete identity-generation technology. **Authoring interaction design ≠
> the translation contract.**

---

## 1. Summary

RFC-0001 §9 established a **hybrid authoring model**: any assistive mechanism
may help a Trader *express* a Strategy, subject to eight constitutional
guards (trader ownership, explicitness, inspectability, determinism,
per-rule confirmation, NL non-authority, no AI decision authority, no
Ordina-originated content). It deliberately deferred *how* that expression
becomes the one canonical `Strategy` ADR-0002 defines and ADR-0003 identifies
— that gap is **N-25**. RFC-0003 (Well-Formedness-failure UX) went further
and named the same gap on the correction side: its diagnosis-to-correction
flow (§9.7 stage 2) explicitly "reuses whatever authoring surface RFC-0001
§9's hybrid authoring model establishes" without defining it, and N-22-A
(the diagnosis-only desktop surface, now CLOSED) was built against
**developer-only sample fixtures**, not a Trader authoring path, precisely
because this contract did not yet exist. N-22-B (the correction workflow) is
formally blocked on this RFC.

This RFC defines the **authoring-to-canonical translation contract**: the
rules by which any authoring surface's output — regardless of its
interaction shape — is deterministically mapped onto the ADR-0002 Hybrid
Typed Semantic Model, ordered against ADR-0003's Well-Formed → canonicalize
→ identity pipeline, subject to RFC-0003's diagnosis-and-correction UX. It
does not design a wizard, a template library, a visual builder, or an NL
assistant. It does not reopen ADR-0002's semantic model, ADR-0003's
canonicalization/identity mechanics, or ADR-0004's Well-Formedness checking
mechanism.

## 2. Motivation

Three concrete gaps currently block downstream work:

1. **N-22-B cannot start.** RFC-0003's correction workflow requires "the
   point in Strategy authoring where the offending content lives" (§9.7
   stage 2) — but no accepted authority defines what an authoring surface
   *is*, what it may produce, or how its output relates to a canonical
   `Strategy`. Building N-22-B against an undefined target risks the same
   drift RFC-0001 §9 was written to prevent (Ordina silently originating
   content).
2. **Multiple authoring surfaces have no convergence rule.** RFC-0001 §9
   permits wizard, template, visual, and NL-assisted authoring
   simultaneously. Without a translation contract, nothing guarantees two
   different surfaces expressing the *same* Trader intent produce the *same*
   canonical `Strategy` (ADR-0003 §7 I2's equivalence guarantee is about
   canonical forms, not about authoring-surface convergence onto one).
3. **Identity minting has no authoring-side owner.** ADR-0003 §9 defines a
   dual-identity architecture (lineage identity + revision identity) and
   states plainly "no identifier-generation technology... is selected." The
   `ordina` model's identity types (`LineageId`, `OccurrenceId`, and siblings
   in `src/model/opaque.rs`) are constructed from a caller-supplied `u64`
   handle — nothing in accepted authority says how an authoring surface
   picks that handle, when, or how it avoids collision across multiple
   authoring sessions, templates, or edits.

This RFC closes the product/contract half of these gaps so a later
implementation cycle (N-22-B or a dedicated authoring surface) has an
authoritative target.

## 3. Goals

1. Define authoring-input boundaries — what an authoring surface may and may
   not produce, and where its output stops being "authoring" and starts
   being "the canonical Strategy."
2. Define the draft/incomplete-state representation an authoring surface
   uses before a Strategy is submitted for Well-Formedness.
3. Define field presence/absence mapping and default-handling at the
   authoring boundary, consistent with ADR-0002 §10b (no implicit defaults)
   and ADR-0003 §6 (explicit/default/absent canonicalization).
4. Define omission semantics — what it means for an authoring surface to
   leave a contract element (RFC-0001 §7) unaddressed.
5. Define the explicit Trader confirmation model for translation, extending
   RFC-0001 §9's per-rule confirmation guard to the translation step itself.
6. Define determinism requirements for the translation function.
7. Define the convergence rule: multiple authoring forms → one canonical
   Strategy.
8. Define identity assignment/minting for new constructs, compatible with
   ADR-0003 §9's dual-identity architecture, without selecting a generation
   technology.
9. Define duplicate-handling — both duplicate authoring submissions and
   duplicate constructs within one submission.
10. Define canonicalization timing/handoff — where authoring's
    responsibility ends and ADR-0003's canonicalize → identity pipeline
    begins.
11. Define the AI/suggestion-assistance boundary for translation
    specifically (RFC-0001 §9 already bounds authoring generally; this RFC
    applies it to the mechanical act of translation).
12. Define the edit → full re-check handoff, extending RFC-0003 §9.7 stage
    3's "fresh, not partial" re-check rule to authoring edits generally
    (not only correction-flow edits).
13. Flag, rather than silently resolve, whether a new intermediate
    draft/view-model type is architecture-grade, whether new public
    `ordina` API is required, whether ADR-0003 already covers identity
    minting, and whether a downstream ADR is needed.

## 4. Non-Goals

This RFC does **not** define: a wizard flow, template catalog, visual-builder
component set, or NL-assistant design; screen layouts or interaction
copy; a DSL, schema, or serialization format; a concrete identity-generation
algorithm (UUID, counter, hash-derived, or otherwise); persistence,
storage, or database mechanism; the Well-Formedness checking mechanism
(ADR-0004, unchanged); canonicalization or revision-identity mechanics
(ADR-0003, unchanged); the semantic model itself (ADR-0002, unchanged);
evaluator semantics or N-24; jurisdiction, retention, or erasure mechanics
(Q-08, F-8); platform/framework selection beyond what PBD-0008 (desktop,
in-process) already fixes; or N-22-B's concrete implementation.

## 5. Governing Product Constraints

- **C2 (four pillars)** — trader pre-commitment, deterministic derivation,
  traceable provenance, trader agency; the translation function is a
  **mechanical mapping**, never a decision-making one.
- **C3 / E1** — rules validate, AI explains; translation is deterministic;
  no AI/score/probability may resolve an authoring ambiguity on the
  Trader's behalf.
- **C4** — every canonical construct produced by translation carries the
  provenance ADR-0002/ADR-0003 already require; translation invents no new
  provenance category.
- **C5** — translation output feeds directly into ADR-0003's revision
  identity; translation must never produce non-reproducible or
  non-deterministic canonical content.
- **C9** — Modify Inputs / re-validation discipline extends to Strategy
  authoring edits: a change re-enters translation and a full Well-Formedness
  re-check, never a partial patch.
- **D11 / RFC-0001 §9** — the hybrid authoring model's eight guards apply
  in full to translation: trader ownership, explicitness, inspectability,
  determinism, per-rule confirmation, NL non-authority, no AI decision
  authority, no Ordina-originated content.
- **RFC-0003 §9.7–§9.9** — diagnosis-to-correction and no-silent-repair
  guards apply identically when the "authoring surface" being handed off to
  is a correction re-entry into translation, not only first-time authoring.

## 6. Terminology

Terms follow `glossary.md` and extend RFC-0001 §6 / ADR-0003 §4. This RFC
introduces, for Product Board adoption:

- **Authoring surface** — any mechanism (wizard, template, visual builder,
  NL-assisted expression, or a combination) through which a Trader
  expresses Strategy content. RFC-0001 §9's Layer B.
- **Authoring draft** — the in-progress, possibly incomplete representation
  of Trader intent an authoring surface holds *before* translation is
  invoked. Never itself the canonical Strategy; never Well-Formedness-
  checked; never assigned a revision identity.
- **Translation** — the deterministic function mapping a **submitted**
  authoring draft onto the ADR-0002 canonical semantic model, prior to
  Well-Formedness (ADR-0004) and canonicalization/identity (ADR-0003).
  RFC-0001 §9's Layer C, made concrete at the contract level (not the
  representation level — §9 there remains deferred).
- **Submission** — the explicit Trader action that ends the authoring-draft
  state and invokes translation for a specific, bounded set of authored
  content. Distinct from Trader per-rule confirmation (§9 below), which
  happens *before* submission, per operative rule.
- **Minted identity** — an umbrella term covering two distinct domains
  translation assigns, kept separate everywhere except this shared label:
  - **Minted lineage identity** — a lineage identity (ADR-0003 §4 layer 3)
    newly assigned by translation to a construct that did not previously
    exist in any prior Strategy revision (§14.1).
  - **Minted occurrence identity** — an `OccurrenceId` (Slice Contract
    Revision 4 §C.1) newly assigned by translation to each construct
    occurrence it emits (§14.2). Unlike lineage identity, occurrence
    identity carries no cross-revision continuity and — per **ADR-0007,
    ACCEPTED** — is categorically outside ADR-0003 §6's canonicalization
    taxonomy and never participates in revision identity (§13, §14.3).
  A bare "minted identity" reference elsewhere in this RFC means either or
  both, as context requires; §14 and §17 always specify which domain applies.

## 7. Authoring-Input Boundaries

**What an authoring surface may hold, pre-submission:** anything —
free text, partially specified rules, placeholder values, incomplete
ordered-evidence sequences, unconfirmed template selections, in-progress
NL expression. Nothing in an authoring draft is normative, checked, or
identity-bearing. This is a deliberate, wide boundary: constraining draft
*shape* would re-litigate RFC-0001 §9's Layer C deferral, which this RFC
does not do.

**What crosses into translation input:** only Trader-**confirmed** content
(RFC-0001 §9 guard 5). An authoring surface MUST NOT submit unconfirmed
content to translation as though it were confirmed — this is the
translation-specific instance of RFC-0003 §9.9's no-silent-repair
prohibition, applied to origination rather than correction: an unconfirmed
suggestion becoming authoritative by inclusion in a submission is the same
defect as an unconfirmed correction becoming authoritative by not being
rejected in time.

**What translation may never do:** invent, infer, or default an operative
rule the Trader did not confirm (C2, RFC-0001 §9 guard 8); this holds even
when a template or NL assistant proposed the content — proposal confers no
authority (RFC-0001 §9 guard 7; RFC-0003 §9.9).

## 8. Draft / Incomplete State

An authoring draft is **explicitly not** a Strategy. It has:

- no revision identity (ADR-0003 §4 layer 2) — identity is assigned only to
  translation *output*, never to a draft;
- no Well-Formedness verdict — Well-Formedness (ADR-0004) evaluates
  canonical output, never a draft directly;
- no guarantee of eventual submission — a Trader may abandon a draft at any
  point without consequence (mirrors RFC-0003 §9.7's "a Trader may leave
  with the Strategy still `Unusable`" neutrality).

**Boundary determination.** Whether an incomplete/abandoned draft requires
any persisted representation at all is an authoring-surface design and
persistence-architecture question — **explicitly out of scope here** (§13,
open question). This RFC only requires that *if* a draft is persisted, it
remains distinguishable from a submitted, translated, canonical Strategy at
every layer (never assigned a revision identity, never Well-Formedness-
evaluated, never presented via RFC-0003's diagnosis surface as though it
were a checked `Unusable` result).

## 9. Field Presence / Absence Mapping and Defaults

Translation must resolve every RFC-0001 §7 contract element to one of
exactly three translation-output states, mirroring ADR-0002 §10b's
no-implicit-defaults invariant and ADR-0003 §6's explicit/default/absent
row:

1. **Explicitly authored** — the Trader confirmed a specific value/rule.
   Translation carries it through unchanged into the canonical model.
2. **Explicitly declared absent** — the Trader affirmatively indicated a
   contract element does not apply (e.g., "no applicability filter beyond
   the selected instrument"). Translation MUST represent this as an
   explicit absence marker, never as a silently-omitted field — consistent
   with ADR-0002 §10b(2)'s fail-closed posture for unknown constructs,
   applied here to unknown *intent*: an authoring surface that cannot tell
   "not yet addressed" from "deliberately absent" MUST treat the field as
   **not yet addressed** (state 3), never guess "absent."
3. **Not yet addressed** — the draft has not reached this contract element.
   Translation MUST NOT submit a draft with contract-required elements in
   this state (§10); a surface that allows submission before every
   RFC-0001 §7 required element reaches state 1 or 2 is non-conforming.

**Defaults.** Translation introduces **no implicit default value** for any
identity-bearing (ID) canonical property (ADR-0003 §6). Where an authoring
surface offers a Trader-selectable *default option* (e.g., a template's
suggested threshold), selecting it is an ordinary explicit authoring act
(state 1) — the value is confirmed, not defaulted. The distinction is
whether the Trader took an affirmative confirming action, not whether the
value happened to match a suggestion.

## 10. Omission Semantics

An authoring surface MAY allow a Trader to submit a draft omitting a
contract element **only** where RFC-0001 §7 itself treats that element as
optional (e.g., no configuration parameters at all is a valid, if minimal,
Strategy). For every element RFC-0001 §7 lists as part of the **minimum**
contract (identity/metadata, applicability, evidence requirements, ordered
rules, Direction, conditions, candidate values, integrity), translation
MUST reject submission — before invoking Well-Formedness — of a draft still
in the "not yet addressed" state (§9) for that element.

This is a **translation-layer gate**, distinct from and prior to
Well-Formedness (ADR-0004): Well-Formedness evaluates whether *submitted,
translated* canonical content is usable; this gate ensures translation
never even attempts to submit a structurally incomplete draft as though it
were a complete authoring act. A Well-Formedness defect (e.g.,
`STRATEGY_INCOMPLETE_DIRECTION_LOGIC`) and a translation-gate rejection
("you have not yet addressed Direction logic") are different failure
classes and MUST be presented differently to the Trader — a translation-
gate rejection is not a checker defect (§9.5's RFC-0003 not-evaluated
concept is unrelated and MUST NOT be reused here).

## 11. Explicit Trader Confirmation

RFC-0001 §9 guard 5 requires per-rule confirmation before an operative rule
becomes authoritative. This RFC makes explicit that **translation invocation
itself requires a distinct, final Trader confirmation** — a "submit" action
separate from any per-rule confirmations that preceded it. Rationale:
per-rule confirmation establishes that *each individual rule* is
Trader-owned; submission confirmation establishes that the Trader intends
*this specific set of confirmed rules, taken together*, to become a
translation input now. Neither confirmation substitutes for the other.

**Re-confirmation on re-entry.** Where a draft is resumed after being
abandoned, or where a correction workflow (RFC-0003 §9.7 stage 2) re-enters
authoring for an existing canonical Strategy, every previously-confirmed
rule that the Trader did not change remains confirmed (no forced
re-confirmation of untouched content — this would violate RFC-0001 §9's
"never a burden beyond what a beginner can sustain" spirit without serving
any guard); every rule the Trader adds or changes requires its own fresh
per-rule confirmation before it may enter the next submission.

## 12. Deterministic Translation

Translation MUST be a **pure, deterministic function** of {the submitted
authoring draft's confirmed content} → {canonical semantic-model
constructs}. Concretely:

- the **same** confirmed draft content, translated twice, MUST produce
  **identical** canonical output (excluding minted identities where the
  minting policy is itself non-reused across submissions — §14);
- translation MUST NOT consult ambient state (wall-clock time, session
  state, prior unrelated Strategies) to decide *what* canonical content to
  produce — this mirrors ADR-0002 §10's determinism prohibitions, applied
  to the authoring boundary rather than the evaluated Strategy itself;
- translation MUST NOT involve AI/model judgment in deciding canonical
  **structure** — an AI assistant may help a Trader *arrive at* confirmed
  content (RFC-0001 §9 guard 7), but the mapping from confirmed content to
  canonical constructs is mechanical, table-driven, or otherwise
  rule-based, never model-inferred (C3/E1).

**Non-goal clarification.** This does not require translation to be
*implemented* as a pure function in any particular language sense — only
that its observable input/output relationship is deterministic. No
implementation technology is selected.

## 13. Multiple Authoring Forms → One Canonical Strategy

RFC-0001 §9 permits wizard, template, visual, and NL-assisted authoring to
coexist. This RFC establishes the **convergence rule**:

> Two authoring sessions — regardless of which authoring surface(s) they
> used — that result in the Trader confirming the **same** set of operative
> rule content MUST translate to canonical constructs with the **same**
> **revision-identity-bearing canonical semantic form** (and therefore, per
> ADR-0003 §7 I2, the same revision identity).

**Convergence is defined over revision-identity-bearing canonical content
only, not over checker/conformance-addressing artifacts.** Per **ADR-0007
(ACCEPTED)**, `OccurrenceId` — the addressing identity §14.2 mints for every
construct occurrence — is categorically outside ADR-0003 §6's taxonomy and
never participates in ADR-0003 §7 revision-identity derivation, including its
supplied opaque/numeric order (ADR-0007 rulings 2–4). The convergence rule
above therefore does **not** require two sessions to produce identical
`OccurrenceId` assignments or identical occurrence-handle order — only
identical revision-identity-bearing content. §14.3's requirement that minted
occurrence-handle order be a deterministic function of each session's own
confirmed-submission structure is fully compatible with convergence for
exactly this reason: two sessions confirming the same operative content in a
different interaction order may legitimately mint different `OccurrenceId`
assignments/order without that difference ever reaching, or being required to
reach, the revision-identity computation this rule protects.

**This is a conformance requirement translation must be architected to
guarantee — it does not follow automatically from determinism (§12) plus
ADR-0003 §7 I2 alone.** Determinism (§12) only guarantees that **one**
translator, run twice, agrees with itself; ADR-0003 §7 I2 only guarantees
that **equal canonical forms** collide to one revision identity. Neither
statement, individually or combined, guarantees that **two independently
implemented** translators — a wizard's own code and a visual builder's own
code — compute the *same* mapping from confirmed content to canonical form
in the first place. Two independently-written, individually-deterministic
functions are not thereby the same function. The convergence rule above is
therefore a requirement this RFC imposes on the **architecture** of
translation, not a fact derivable from already-accepted authority alone.

**Required architecture: one shared, normative translation
implementation.** To make the convergence rule genuinely satisfiable and
testable, every authoring surface (wizard, template, visual builder,
NL-assisted, or any future surface) **MUST invoke one shared, single
normative translation implementation** for the actual confirmed-content →
canonical-construct mapping (§7–§12, §14, §16) — no authoring surface may
independently reimplement translation semantics, even if it produces
"equivalent" output by its own judgment. This is the minimum architecture
that makes convergence checkable at all: with one shared implementation,
convergence follows directly from §12's determinism (the same function,
given the same input, produces the same output — this part genuinely *is*
a direct consequence of determinism, unlike the previous framing's claim
about *independent* implementations); without a shared implementation,
convergence would require an entirely separate, currently-undefined
cross-implementation equivalence proof obligation this RFC does not
attempt to specify.

**What this requires, and does not require, architecturally.** "One shared
implementation" is a **conformance requirement on how many independent
translation implementations may exist**, not a claim about where that
implementation lives, what language it is written in, or how it is
packaged — those remain open per §16/§25 (a caller-composed function built
from public `ordina` constructors, a dedicated `ordina`-owned entry point,
or an authoring-layer library shared by every surface are all compatible
with this requirement; which one is chosen is downstream, §24). This RFC
does **not** require that authoring surfaces share any other code, UI, or
representation — only the confirmed-content → canonical-construct mapping
itself.

**Testability.** Under this architecture, convergence is testable by a
single conformance suite exercising the one shared translation
implementation directly with varied confirmed-content orderings/surface
provenances (mirroring the permutation-independence testing pattern already
established for canonical ordering in the Slice-1 conformance suite,
`tests/conformance/main.rs`) — it does not require standing up multiple
independent authoring surfaces to test cross-implementation agreement,
because there is, by this requirement, only one implementation to test.

**What this does not solve.** Two authoring sessions expressing what a
Trader subjectively considers "the same strategy" but confirming
*materially different* rule content (e.g., a slightly different threshold)
are **not** required to converge — they are different Strategies, correctly
so (ADR-0003 §7 I3 sensitivity). Convergence is about translation fidelity,
not about strategy-authoring UX guiding Traders toward consistent choices —
the latter is authoring-surface design, out of scope here.

## 14. Identity Assignment / Minting for New Constructs

Two distinct opaque domains require assignment during translation, and this
RFC governs both. Both are **existing, accepted domains** — `src/model/opaque.rs`
(governed by the Slice Contract, PBD-0006's normative-engineering-contract
convention) and ADR-0003 §4/§9 — this section introduces **no new identity
domain**; it applies existing authority to the authoring/translation moment,
exactly as ADR-0003 §9's table already does for editing events generally.

### 14.1 Lineage Identity Minting

ADR-0003 §9 defines the dual-identity architecture and explicitly defers
identifier-generation technology. This RFC defines the **minting contract**
translation must satisfy for lineage identity (ADR-0003 §4 layer 3), without
selecting that technology:

1. **When to mint.** Translation mints a **new** lineage identity for every
   construct that has no prior lineage identity to inherit — i.e., every
   genuinely new rule, condition, evidence declaration, or other operative
   construct a Trader authors for the first time. A construct the Trader is
   *editing* (correction flow, §17's hydration, or an ordinary revision per
   RFC-0001 §14) reuses its **existing** lineage identity per ADR-0003 §9's
   table — translation MUST NOT mint a new lineage identity for an
   edited-in-place construct, and MUST mint one for a construct the Trader
   replaces, splits, merges, or copies, exactly per that table.
2. **No semantic leakage.** A minted lineage identity carries no semantic
   content of its own (ADR-0003 §9: lineage identity is about authorship
   continuity, not meaning) — translation MUST NOT derive it from operative
   content in a way that would make two semantically-identical-but-
   independently-authored constructs accidentally share a lineage identity
   (ADR-0003 §9's Copy/Clone row: "must not share the source's lineage
   identity merely because initial semantics are identical").

### 14.2 Occurrence Identity Minting

`src/model/opaque.rs` (governed by the Slice Contract, Revision 4 §C.1)
defines `OccurrenceId` as a **separate, mandatory, non-optional** opaque
domain from lineage identity — "it identifies one concrete construct
occurrence in the canonical input and carries no product semantics... it is
distinct from lineage identity and never substitutes for it," and "every
occurrence has exactly one, even a malformed one." Every construct
translation emits therefore requires a valid `OccurrenceId`, and this RFC
extends its minting contract accordingly, symmetric with §14.1:

1. **Who assigns it, and when.** Translation is the authoritative assigner
   of occurrence identity for every construct occurrence it emits, during
   translation, strictly before the canonical `Strategy` is handed to
   Well-Formedness (§16) — never left implicit, defaulted, or inferred by
   a downstream checker or canonicalization step. This is a direct
   application of `opaque.rs`'s own framing: the checker "never generates,
   infers or chooses" an occurrence order or identity — it must already
   arrive as part of the canonical input translation produces.
2. **Uniqueness.** Every `OccurrenceId` translation mints MUST be unique
   within the Strategy input it belongs to, per `opaque.rs`'s "unique within
   one Strategy input" requirement — including across constructs that are
   otherwise content-identical (§15 already establishes that content-
   identical constructs in a non-deduplicating collection remain distinct;
   `OccurrenceId` is the mechanism by which they stay distinguishable to the
   checker).
3. **Preservation across hydration/editing (§17).** Where an existing
   construct is hydrated into a draft (§17) and the Trader edits it
   in place without replacing it, translation MUST preserve its existing
   `OccurrenceId` on re-submission — an edited-in-place construct is the
   same occurrence with changed content, not a new occurrence. Where the
   Trader's edit constitutes a genuine replacement (the prior construct is
   removed and a materially different one takes its place, mirroring
   ADR-0003 §9's Replacement row for lineage identity), translation MUST
   mint a **new** `OccurrenceId` for the replacement; the old one is not
   reused.
4. **New constructs.** Every genuinely new construct (first-time authoring,
   or a construct added during an edit) receives a newly minted
   `OccurrenceId`, minted fresh in the same submission that first emits it.
5. **Removed constructs.** A construct the Trader explicitly removes during
   editing (§17) simply does not appear in the next submission's emitted
   Strategy — its `OccurrenceId` is not reused for anything else, and
   translation does not emit a tombstone occurrence record (occurrence
   identity, unlike lineage identity, carries no cross-revision historical-
   continuity requirement under `opaque.rs`/the Slice Contract; ADR-0003 §9's
   lineage-tombstone requirement is unaffected and remains lineage-identity-
   only).

### 14.3 Opaque-Handle Order Validity

`src/model/opaque.rs` states a requirement this RFC's original text did not
engage with and must not contradict: for every opaque domain built on
`OpaqueValue` (including `LineageId` and `OccurrenceId`), "**both** the
values of an opaque domain **and** a deterministic total order over them
[must] arrive as part of the already-canonical input... The checker never
generates, infers or chooses an order," and "two conforming checkers agree
because they receive the same order, never because they agree on a
representation." Because translation's output **is** that already-canonical
input (§16), translation — not any later stage — is where this order-supply
obligation must be satisfied. This RFC accordingly clarifies, without
redesigning canonicalization or the ordering `opaque.rs`/ADR-0003 §6 already
fix:

- **What is already fixed by existing authority (not decided here).** For
  `OccurrenceId`, `opaque.rs` and the Slice Contract already require the
  order to be "ordering only, carries no semantics" (Revision 4 §C.1) —
  i.e. occurrence order is a **structural addressing order**, not itself
  operative content, but it must still be a genuine deterministic total
  order, consistently assigned, not an incidental artifact of an unrelated
  encoding (ADR-0005 §7.5's prohibition on deriving authoritative order
  from incidental representation, cited by `opaque.rs` itself, binds
  translation exactly as it binds any other producer of canonical input).
  For `LineageId`, ADR-0003 §6 separately classifies rule/node identifiers
  as **CX (cosmetic)** unless reference-bearing — a cosmetic rename does not
  fork revision identity — which this RFC does not reopen; `opaque.rs`'s
  `Ord` implementation for `LineageId` exists for the Slice Contract's own
  structural purposes (e.g. §6.4 sorting) and does not, by itself, make
  lineage order operative/identity-bearing at the ADR-0003 revision-identity
  layer.
- **What this RFC requires of translation (new, minimum normative
  requirement).** The total order translation assigns to the `OccurrenceId`
  handles it mints for one submission MUST be a **deterministic function of
  the confirmed draft's own declared/authored structure** — concretely, the
  order in which the confirmed content occurs in the Trader-confirmed
  submission (§11) — and MUST NOT be assigned from an unrelated source (a
  random value, a wall-clock timestamp, a hash of unrelated content, or
  any other source whose relative order carries no relationship to the
  authored structure). This closes the gap the independent review
  identified: a **content-addressed or randomly-generated handle whose
  numeric value does not encode any authored-order relationship — including
  a naively-generated UUID — is NOT, by itself, a conforming
  `OccurrenceId`-minting strategy**, because its order would not be "the
  contract-supplied order" `opaque.rs` requires; it would be an arbitrary
  encoding accident. A generation technology remains free to use any
  encoding (UUID, counter, or otherwise) **provided** the resulting
  handles, when compared via each domain's `Ord`, reproduce the required
  authored-order relationship — that remaining freedom (concrete encoding,
  as opposed to the order it must reproduce) is genuinely implementation
  mechanism and is left unselected here, consistent with ADR-0003's own
  posture on identity-generation technology.
- **Collision-freedom.** Both minted domains MUST be collision-free across
  the full history the Strategy revision model can reference — at minimum,
  across every construct ever minted within one Trader's Strategy corpus.
  Translation is not required to guarantee collision-freedom across
  independent installations (single-Trader, in-process, per PBD-0008)
  unless a later persistence/sync architecture requires it — that
  requirement, if any, is downstream (§24).
- **Revision-identity consequence (settled by ADR-0007, ACCEPTED).**
  `OccurrenceId`'s order-supply obligation above governs only the checker's
  addressing/conformance layer — it has **no bearing on Strategy revision
  identity**. ADR-0007 rules `OccurrenceId` categorically outside ADR-0003
  §6's taxonomy and §7's revision-identity computation, including its
  supplied order (ADR-0007 rulings 2–4). Consequently, this section's
  submission-order-derivation requirement is a pure translation-fidelity/
  conformance rule, not an identity rule: two Trader-confirmed sessions with
  identical revision-identity-bearing content may legitimately mint
  different `OccurrenceId` order under this section while remaining
  revision-equivalent under §13. `LineageId`'s CX/cosmetic classification
  (ADR-0003 §6, unchanged, unreopened by ADR-0007) already established the
  analogous point for lineage order; ADR-0007 closes the same question for
  occurrence order with citable, accepted architecture rather than analogy.

### 14.4 Trader-Invisible by Default

Minted identities (both domains) are an internal provenance/addressing
mechanism (C4); nothing in this RFC requires exposing raw identity values to
the Trader — RFC-0003 §9.4 already establishes that Trader-facing defect
presentation maps *from* `DefectIdentity` without exposing checker
internals; the same principle applies to authoring: the Trader interacts
with named, described constructs, and minted identity is plumbing
underneath.

**Explicitly not decided here (§24):** the concrete generation encoding
(UUID, monotonic counter, content-derived handle, or other) for either
domain, provided §14.3's order requirement is satisfied; whether minting
happens client-side in the authoring surface or is mediated by a public
`ordina` API call; whether ADR-0003 §9's table and `opaque.rs`'s existing
text are fully sufficient guidance or need an implementation-level
elaboration.

## 15. Duplicate Handling

Two distinct duplicate scenarios:

**Duplicate submission.** A Trader submits the same draft twice (e.g., a
double-click, a retried network-equivalent operation — though PBD-0008's
in-process desktop surface makes network retries unlikely, a UI
double-submit remains possible). Translation MUST be **idempotent under
determinism** for genuinely identical confirmed content presented as a
*first-time authoring* submission: per §12/§13, translating the same
confirmed content twice produces the same canonical form. Whether the
authoring surface additionally prevents a literal double-submit at the UI
level (so the Trader is not asked to confirm twice) is an authoring-surface
design concern, not a translation-contract requirement.

**Duplicate constructs within one submission.** Where a Trader's confirmed
draft contains two constructs that would translate to the same canonical
semantic form (e.g., two identically-configured conditions), translation
MUST NOT silently deduplicate them into one construct **unless** the
canonical model's own unordered-collection semantics (ADR-0003 §6:
"members that share a canonical form are treated as the same member")
already governs that collection — in which case deduplication is not a
translation decision at all, it is canonicalization's existing, accepted
behavior, and translation defers to it rather than reimplementing it.
Where the collection is **not** unordered/deduplicating by ADR-0003 §6
(e.g., a semantically ordered collection where two entries are
independently meaningful even if superficially identical), translation
MUST preserve both as distinct constructs with distinct minted identities
(§14.1 lineage, §14.2 occurrence) — silent collapse there would be an
uninvited Ordina decision (C2).

## 16. Canonicalization Timing / Handoff

Reaffirming, not amending, ADR-0003 §5's ordering:

> **Draft (authoring) → Translation → Well-Formed (ADR-0004) → Canonicalize
> (ADR-0003 §6) → Revision Identity (ADR-0003 §7).**

Translation's output is the **typed model input** to Well-Formedness — it
is not itself canonical, and it does not compute or approximate a revision
identity. Concretely:

- translation MUST produce a fully-typed ADR-0002 construct set (no partial
  or placeholder typed values) for every submission that passed §10's
  completeness gate;
- translation MUST NOT invoke, duplicate, or approximate ADR-0003 §6
  canonicalization rules (ordering normalization, numeric normalization,
  etc.) — that responsibility remains exclusively ADR-0003's, exercised
  after translation, not folded into it;
- translation MUST NOT assign or predict a revision identity — only
  ADR-0003's content-addressed identity computation, run over the
  canonicalized form, does that;
- a translation-gate rejection (§10) happens **before** Well-Formedness is
  ever invoked — an incomplete draft never reaches the checker at all, and
  is therefore never a Well-Formedness `Unusable` verdict (§10 already
  states this; restated here for the pipeline-ordering record).

This handoff boundary is a **necessary but not sufficient** part of the
answer to N-22-B's blocking question: it establishes that a correction-flow
re-entry into authoring produces a new submission, which re-enters
translation, which re-enters the same Draft → Translation → Well-Formed →
Canonicalize → Identity pipeline as first-time authoring — there is no
separate "correction pipeline." It does **not**, by itself, define how a
correction-flow re-entry obtains the draft content to re-submit in the first
place — that reverse direction is §17's subject.

## 17. Canonical → Draft Hydration

§16 defines the forward pipeline (draft → canonical). RFC-0003 §9.7 stage 2
requires the reverse: a Trader navigating from a diagnosed defect must be
"taken to the point in Strategy authoring where the offending content
lives." This is impossible without first reconstituting the **existing**
canonical Strategy's confirmed content into an editable authoring draft —
this section defines that reverse contract, without which N-22-B's
correction workflow (and RFC-0001 §14's ordinary post-submission revision
path) has no defined starting point. Hydration is the mirror operation of
translation (§16), sharing its guards, not a separate authority.

**Pipeline.** Canonical Strategy → **Hydration** → editable authoring draft
→ Trader edit → explicit confirmation (§11) → deterministic translation
(§16) → full fresh Well-Formedness check (§19). Hydration is a **new
step inserted before** the existing Draft → Translation pipeline; it does
not alter, reorder, or duplicate any step §16 already defines.

1. **Scope of reconstitution.** Hydration reconstitutes **every** RFC-0001
   §7 contract element the canonical Strategy declares — explicitly authored
   and explicitly declared-absent content alike (§9's states 1 and 2) — into
   the draft's corresponding representation. Hydration MUST NOT omit,
   truncate, or selectively reconstitute content (e.g., hydrating only the
   defect-adjacent construct while silently dropping the rest of the
   Strategy) unless the authoring surface's own navigation model
   legitimately scopes what is *displayed* at once (RFC-0003 §9.7 stage 2's
   "per-defect" navigation is a display/navigation concern, not license to
   hydrate an incomplete draft — the full Strategy's content remains
   reconstituted underneath, consistent with §19's "entire Strategy, not a
   delta" re-check requirement).
2. **Confirmation status of hydrated content.** Content hydration produces
   from an already-canonical, already-Well-Formed Strategy is represented in
   the draft as **already Trader-confirmed** (§11) — hydration is not itself
   a fresh confirmation event, and it is not an opportunity for the Trader
   to be asked to reconfirm content they are not changing. This directly
   implements §11's existing "no forced re-confirmation of untouched
   content" rule, which presupposed but did not, until now, define hydration
   as that rule's content source.
3. **No silent modification.** Hydration MUST NOT change, normalize,
   canonicalize, repair, reorder in a Trader-visible way, or otherwise alter
   the operative meaning of the content it reconstitutes. Hydration is a
   read/reconstitution operation only — it carries none of translation's
   authority to interpret or resolve ambiguity, because there is no
   ambiguity to resolve: the source is already canonical, already
   Well-Formed content. Any apparent transformation during hydration is
   presentation-only (e.g., mapping a canonical construct to a
   human-readable authoring-surface widget) and MUST NOT change what
   re-translating the untouched hydrated content would produce.
4. **Identity preservation.** For every hydrated construct the Trader does
   not replace, hydration MUST preserve both its lineage identity (§14.1)
   and its occurrence identity (§14.2) through to the next submission —
   this is what makes §14.1's "edited-in-place construct reuses its existing
   lineage identity" and §14.2's "translation MUST preserve its existing
   `OccurrenceId` on re-submission" rules implementable: hydration is the
   step that carries those existing identities from the canonical source
   into the editable draft so translation has them to preserve. Hydration
   MUST NOT mint new identities for unchanged, hydrated content — minting
   remains exclusively translation's role (§14), applied only to genuinely
   new or genuinely replaced constructs the Trader introduces after
   hydration.
5. **New constructs after hydration.** A construct the Trader adds after
   hydration is ordinary new authoring content (§7) and follows §14's
   ordinary minting rules — nothing about having entered through a
   correction-flow re-entry changes this.
6. **Removed constructs.** A construct present in the hydrated draft that
   the Trader explicitly, affirmatively removes (an explicit Trader action,
   not an inferred one — mirroring §9's explicit/absent distinction, applied
   to removal) simply does not appear in the next submission; per §14.2.5,
   translation does not re-mint or reuse its occurrence identity for
   anything else, and per ADR-0003 §9's Deletion row, its lineage identity
   is retained as a tombstone for historical provenance (C4) — hydration
   does not decide this, it is unchanged, already-accepted ADR-0003
   behavior that a subsequent translation invocation exercises normally.

**What hydration is not.** Hydration is not a second translation function,
not a new checker, and not a new persistent representation — it operates
entirely on data already produced by a prior, accepted translation +
Well-Formedness + canonicalization cycle (§16), reading it back into the
same conceptual draft state §7–§11 already govern. No new public `ordina`
API is required by this section beyond what §16 already leaves open (§24);
hydration may be implemented entirely as authoring-surface-local logic that
reads the existing public accessors on the canonical `Strategy` and its
constructs (the same frozen, public accessor surface N-22-A's diagnosis view
already exercises) and populates draft state from them.

## 18. AI / Suggestion Assistance Boundary

RFC-0001 §9 guard 7 already establishes that AI may "clarify or translate
expression" but gains no decision authority, and RFC-0003 §9.9 already
prohibits any suggestion from becoming authoritative without per-rule
Trader confirmation. This RFC adds no new AI capability and narrows
nothing already granted; it makes one addition specific to the mechanical
translation step defined here:

**An AI/suggestion mechanism MAY propose the *mapping* from NL or
under-specified input to a candidate canonical construct** (e.g., "this
sounds like a Sequence rule with these two conditions") **but that proposal
is itself just another form of authoring-draft content** (§7) — it is not
translation output, has no identity, and does not bypass §11's submission
confirmation or §9's per-rule confirmation. The Trader confirming an
AI-proposed mapping is functionally identical, under this contract, to the
Trader confirming a hand-authored rule: both become confirmed draft content
subject to the same translation function. AI proposing structure faster is
a UX efficiency, not a new authority (C3).

## 19. Edit → Full Re-Check Handoff

Extending RFC-0003 §9.7 stage 3 beyond the correction-specific case: **any**
edit to a submitted, translated Strategy — whether reached via RFC-0003's
correction flow (entering through §17's hydration), an ordinary RFC-0001
§14 revision, or a Trader independently reopening authoring — MUST re-enter
the full pipeline (§16) from Translation forward. Specifically:

- there is no "patch only the changed rule" path that skips
  re-translation, re-Well-Formedness, or re-canonicalization;
- a Trader who edits one condition and resubmits receives a **fresh**,
  complete Well-Formedness result over the **entire** Strategy, not a
  delta — mirroring RFC-0003 §9.7 stage 3's explicit prohibition on
  "re-check only the fixed rule," here generalized from the
  correction-of-a-defect case to the edit-of-anything case, since nothing
  in RFC-0001 §14 or ADR-0003 distinguishes them at the translation layer;
- until that fresh result returns, the prior Strategy revision (if any)
  remains the operative one for any Decision Validation already bound to
  it (C5 — no retroactive rewrite; ADR-0003 §7 I3/§8) — an in-progress
  edit never silently supersedes a revision a live decision is bound to.

## 20. Boundary / Constitution Check

| Prohibited | Guard |
|---|---|
| translation infers or defaults an operative rule the Trader did not confirm | §7, §9 — explicit/absent/not-yet-addressed states only; no implicit defaults |
| a proposal (template, NL, AI) becomes authoritative without confirmation | §7, §18 — proposal is draft content only, never translation output |
| translation is non-deterministic or consults ambient state | §12 |
| different authoring surfaces silently diverge on identical confirmed intent | §13 — one shared, normative translation implementation, not independently reimplemented divergent translators |
| minted lineage identity encodes semantic content or is derived from operative content | §14.1.2 |
| minted occurrence-identity order is assigned from an unrelated/arbitrary source | §14.3 |
| translation silently deduplicates non-unordered constructs | §15 |
| translation approximates or predicts canonicalization/revision identity | §16 |
| hydration silently changes, normalizes, or repairs operative content | §17.3 |
| hydration mints new identity for unchanged content, or forces unnecessary re-confirmation | §17.2, §17.4 |
| an edit skips full re-translation/re-check | §19 |
| a translation-gate rejection is presented as a Well-Formedness defect | §10 |
| a draft is presented as though it were a checked, canonical Strategy | §8 |

**§2.1 five-part boundary test:** *Origination* — the Trader authors and
confirms every operative rule; translation is a mechanical mapping with no
discretion over content, only over representation (§7, §12); hydration
reconstitutes without altering (§17). *Determinism* — translation is a pure
deterministic function, invoked as one shared implementation so
cross-surface agreement is a tested consequence, not an assumption (§12,
§13); AI is confined to expression assistance, never structural decision
(§12, §18). *Provenance* — minted lineage and occurrence identities, and
translated provenance, carry through unchanged into ADR-0003's
identity-bearing operative provenance (§14, §6); hydration preserves rather
than re-derives them (§17.4). *Gating* — the translation-gate (§10) and
Well-Formedness (unchanged, ADR-0004) remain distinct, sequential gates;
neither is bypassed or merged (§16). *Restraint* — nothing here implies or
guarantees a Strategy will be Well-Formed or profitable; translation only
maps confirmed intent faithfully. **No boundary failure identified**,
provided §7–§19's guards are normative in the accepted RFC.

## 21. Alternatives Considered

- **No translation contract — leave each authoring surface to define its
  own mapping ad hoc.** Rejected: this is the status quo that produced the
  N-22-B block and the fixture-only limitation N-22-A had to disclose; it
  also risks exactly the convergence failure §13 closes (two surfaces
  silently producing different canonical Strategies for the same Trader
  intent), undermining ADR-0003's equivalence guarantee at the one layer
  it does not itself police (translation is upstream of canonicalization).
- **Define one single mandatory authoring surface (e.g., wizard-only) and
  skip the convergence question.** Rejected: contradicts RFC-0001 §9's
  already-accepted hybrid model and Q-16's resolution; would require
  reopening an accepted RFC to narrow authoring to one surface, which this
  RFC has no authority or reason to do.
- **Let translation output be intermediate/lossy and rely on
  Well-Formedness to catch translation errors.** Rejected: conflates two
  different failure classes (an incomplete draft vs. a structurally
  ill-formed but complete Strategy — §10) and would let translation defects
  masquerade as Trader-authoring defects, misattributing responsibility
  away from the surface that actually failed.
- **Have translation itself assign revision identity (skip the
  ADR-0003 handoff).** Rejected: would duplicate or bypass ADR-0003's
  content-addressed identity computation, violating ADR-0003 §5's ordering
  and risking two independent identity-computation paths drifting apart —
  exactly the drift ADR-0003 §7 I8 (reproducibility) exists to prevent.
- **Mint identities centrally through a required server/database call.**
  Rejected as a requirement: PBD-0008 already fixes a local, in-process,
  single-Trader desktop surface with no server-side persistence; requiring
  central minting would contradict that ruling. (A future architecture MAY
  still choose a mediating local API call — §14.4 leaves that open — but
  this RFC does not require one.)

## 22. What This Makes Harder

- **Ad hoc authoring-surface prototyping** becomes harder: any new
  authoring surface must invoke the one shared, normative translation
  implementation (§12–§17) rather than build its own, and any surface
  reaching correction/edit must implement hydration (§17) correctly. This
  is intentional — it is the guard against silent divergence (§13).
- **"Quick and dirty" AI-generated strategies** become structurally
  impossible to ship without a confirmation step (§18) — by design, this
  closes a C2-adjacent risk RFC-0001 §9 already named, but it does mean any
  future AI-heavy authoring surface carries more UX friction than a
  fully-automated one would.
- **A single shared translation implementation is now a hard architectural
  requirement** (§13), not merely a convention: a future implementation
  cannot let each authoring surface reimplement translation independently,
  even if doing so seems locally convenient. This trades some
  implementation flexibility for convergence guarantees the review found
  otherwise unenforceable.

## 23. Risks

New entries for `docs/product/risk-register.md` (register alignment
deferred to a separately authorized cycle, per precedent — RFC-0001 §18,
RFC-0003's equivalent):

- **R-F — Translation drift between authoring surfaces.** A future
  implementation fails to route every authoring surface through the one
  shared translation implementation §13 now requires, letting a surface
  reimplement translation semantics independently and drift from it.
  *Mitigation:* §13's shared-implementation requirement, stated as an
  architecture constraint rather than an assumed consequence; a future
  implementation-level conformance suite (downstream) can directly verify
  only one implementation exists.
- **R-G — Minted-identity collision or order-invalidity.** A minting
  implementation fails §14.3's collision-freedom or order-validity
  requirement for either lineage or occurrence identity — e.g. by
  generating handles from a source (random UUID, unrelated hash) whose
  order does not encode the required authored-order relationship,
  producing canonical input the checker cannot correctly interpret without
  detecting the defect. *Mitigation:* §14.2–§14.3 requirements stated
  explicitly now, before any minting implementation exists, so a future
  ADR/implementation is reviewed against them.
- **R-J — Hydration silently alters operative content.** A hydration
  implementation, in the course of reconstituting canonical content into an
  editable draft representation, introduces a lossy or normalizing
  transformation that changes what re-translating the untouched hydrated
  content would produce, without the Trader ever confirming that change.
  *Mitigation:* §17.3's explicit no-silent-modification prohibition; a
  future conformance suite should assert hydrate-then-immediately-
  retranslate-without-editing is a no-op on the canonical form.
- **R-H — Translation-gate / Well-Formedness conflation.** An
  implementation or a UX surface presents a translation-gate rejection
  (§10) as though it were a checker-produced defect, confusing the
  Trader about which layer failed and misapplying RFC-0003's diagnosis
  contract to content Well-Formedness never actually evaluated.
  *Mitigation:* §10, §16 explicit separation; a future UX design must
  distinguish the two failure classes visibly.
- **R-I — AI-assisted mapping proposals normalized into de facto
  authority.** Over time, a UX that always shows an AI mapping proposal
  pre-filled and "just needing" a single bulk confirmation click drifts
  toward the RFC-0003 §9.9 "un-rejected suggestion becomes authoritative"
  failure mode in practice even while nominally requiring confirmation.
  *Mitigation:* §18's per-rule confirmation requirement is unchanged from
  RFC-0001 §9 / RFC-0003 §9.9 — carried forward, not weakened; a future UX
  review should specifically test for this drift.

## 24. Open Questions / Deferred Decisions

Per the authorizing PBA-0158, the following are **flagged, not resolved,**
by this RFC:

- **Whether a new intermediate draft/view-model type is architecture-grade.**
  §8 defines the *conceptual* draft/canonical distinction at the product
  level. Whether that distinction requires a new public `ordina` type (as
  opposed to being an authoring-surface-local, non-`ordina` concern) is an
  implementation/architecture question this RFC does not decide. **Leaning
  consideration for the next cycle:** because `ordina`'s existing model
  types (`Strategy`, `Declared<T>`, the identity types) are already typed
  and public, a case can be made that a "draft" needs no new `ordina`-owned
  type at all — it could remain entirely authoring-surface-local until
  §11's submission moment, at which point it must already be expressed in
  terms of existing `ordina` constructors. This RFC does not require that
  outcome; it only observes it as a plausible resolution for a future
  architecture determination.
- **Whether new public `ordina` API is required.** §14's minting contract
  (both lineage and occurrence identity), §16's handoff boundary, and §17's
  hydration contract do not obviously require new public surface beyond
  what N-22-A already found sufficient (existing `pub const fn new`
  constructors on every identity type, and the existing public accessors on
  `Strategy` and its constructs, per the frozen public API N-22-A's
  implementation reports confirmed complete for diagnosis). Whether
  *translation itself* (as opposed to diagnosis) needs a new `ordina`-owned
  entry point (e.g., a single `translate_and_validate` convenience
  function, or an occurrence-identity-order-validation helper per §14.3)
  or can be composed entirely by a caller from existing public constructors
  plus the existing `run_phase6`/`finalize_phase6` orchestration is left to
  a future implementation-scoping determination.
- **Whether ADR-0003 §9 already fully covers identity minting — RESOLVED for
  the revision-identity boundary by ADR-0007 (ACCEPTED).** §14.1 (lineage
  identity) remains an application of ADR-0003 §9's existing table to the
  authoring moment, not new identity architecture. §14.2–§14.3 (occurrence
  identity and opaque-handle order validity) apply `src/model/opaque.rs`/the
  Slice Contract to the same moment; the question of whether that
  classification is durable against ADR-0003's own taxonomy — flagged open in
  earlier drafts of this RFC — is now settled: **ADR-0007** explicitly rules
  `OccurrenceId` categorically outside ADR-0003 §6/§7, closing the gap via a
  proper narrow supersession rather than this RFC's own product-level
  assertion. What remains genuinely open (unaffected by ADR-0007, which
  decided only the classification question): whether a new intermediate
  draft/view-model type or new public `ordina` API is required — see the
  two bullets above, unchanged.
- **Whether a downstream ADR becomes necessary.** This RFC's own
  determination (§25) is that no new persistent data shape, field, ordering
  rule, or `src/**` change is proposed — but see §25 for the explicit
  architectural-commitment analysis and its caveats.
- **N-25 register closure.** This RFC, on acceptance, would resolve N-25 to
  the extent stated here; the canonical open-questions register update is
  a separate, later governance-synchronization cycle (matching RFC-0001/
  RFC-0003 precedent — register alignment is never folded into RFC drafting
  or acceptance itself).
- **Q-08, N-24, F-8** remain untouched, unresolved, and unnarrowed by this
  RFC.
- **Draft persistence.** §8 explicitly leaves open whether/how an abandoned
  or in-progress draft is persisted between sessions — an authoring-surface
  and, if persisted, persistence-architecture question (gated by Q-06's
  already-resolved desktop/in-process boundary, but not otherwise decided
  here).
- **Cross-surface conformance testing mechanism.** §22 names this as an
  implied future obligation; no test architecture is designed here.

## 25. Architectural-Commitment Determination

**ADR requirement: NO**, with the following explicit analysis (an RFC that
silently decided this without showing its work would itself violate the
"do not silently decide" instruction under which this RFC was authorized):

- This RFC proposes no new persistent data shape or field — §14.1's
  lineage-minting contract is expressed entirely in terms of ADR-0003 §9's
  already-accepted architecture; §14.2–§14.3's occurrence-identity and
  order-validity contract is expressed entirely in terms of
  `src/model/opaque.rs`/the Slice Contract's already-accepted, already-
  existing `OccurrenceId` domain and order-supply requirement, with its
  revision-identity boundary now formally confirmed by **ADR-0007
  (ACCEPTED)** rather than resting on this RFC's own product-level reading.
  Neither adds a new identity *layer* or *domain* — both apply existing,
  accepted authority (from three distinct governing documents — ADR-0003 §9,
  the Slice Contract, and now ADR-0007) to the authoring/translation moment
  specifically.
- §17's hydration contract likewise proposes no new persistent shape: it
  operates entirely on data an already-accepted translation + Well-
  Formedness + canonicalization cycle already produced, read back through
  existing public accessors (§17, "What hydration is not").
- §13's shared-translation-implementation requirement is an **architecture
  constraint on implementation structure** (how many independent
  implementations may exist), not a new persistent data shape, canonical
  construct, or ordering rule.
- This RFC proposes no new canonicalization or ordering rule — §16
  explicitly forbids translation from approximating or reimplementing
  ADR-0003 §6; it only fixes the *handoff point*, which is already implied
  by ADR-0003 §5's stated ordering (Well-Formed → canonicalize → identity),
  now extended one step earlier (draft → translate → that same pipeline).
  §14.3's order-validity requirement is likewise not a new canonicalization
  rule — it requires translation to satisfy an order-supply obligation
  `opaque.rs` already imposes on any producer of canonical input; it does
  not change what that obligation is.
- This RFC proposes no `src/**` change — it is a product/contract document;
  §24 explicitly leaves open whether any future public API addition is
  warranted, and defers that decision rather than making it here.
- **Caveat.** If a future implementation-scoping cycle concludes that
  §14.1's lineage-minting contract, §14.2's occurrence-minting contract,
  §14.3's order-validity requirement, or §17's hydration contract, in
  practice, require a *new* public `ordina` entry point with its own
  invariants beyond what ADR-0003 §9, `src/model/opaque.rs`/the Slice
  Contract, and ADR-0007 already state, that addition may itself cross the
  ADR-grade
  threshold (new production-dependency-free but new normative architecture
  surface) — mirroring how ADR-0006 was required for the desktop toolkit
  despite ADR-0005 §6.2's existing dependency set. This RFC does not
  pre-judge that outcome; it flags it as the concrete trigger condition a
  future determination should check. §13's shared-implementation
  requirement is a further, independent candidate trigger: if a future
  cycle concludes the shared implementation must be a new `ordina`-owned
  public entry point (rather than caller-composed or authoring-layer-local)
  with its own invariants, that too may cross the ADR-grade threshold —
  this RFC does not decide where the shared implementation lives (§13).

## 26. Acceptance Criteria

For a future implementation to be judged faithful to this RFC:

1. Exactly **one** shared, normative translation implementation exists;
   every authoring surface invokes it rather than independently
   reimplementing translation semantics (§13); that implementation is
   deterministic per §12, making §13's convergence rule a direct,
   testable consequence rather than an assumed one; convergence is
   verified over revision-identity-bearing canonical content — two
   sessions confirming equivalent operative content in different
   interaction orders may mint different `OccurrenceId` assignments/order
   without failing convergence, per **ADR-0007 (ACCEPTED)**.
2. No authoring draft is ever presented, stored as, or treated as a
   checked, canonical, identity-bearing Strategy (§8).
3. Every RFC-0001 §7 minimum-contract element reaches an explicit
   authored/absent state before submission is possible; incomplete drafts
   are rejected by the translation gate, never passed to Well-Formedness
   (§9, §10).
4. Translation invocation requires its own explicit Trader confirmation,
   distinct from and in addition to per-rule confirmation (§11).
5. Minted lineage identities are collision-free, semantically inert, and
   follow ADR-0003 §9's existing event table without introducing a new
   identity layer (§14.1); minted occurrence identities are unique within
   their Strategy input and assigned exclusively by translation before
   Well-Formedness (§14.2); the total order encoded by every minted handle
   is a deterministic function of the confirmed draft's authored/declared
   structure, never an unrelated source (§14.3).
6. Duplicate constructs are handled per §15's distinction between
   unordered-collection deduplication (ADR-0003 §6, unchanged) and
   preserved-distinct constructs.
7. Translation never computes, approximates, or predicts a revision
   identity; that remains exclusively ADR-0003's (§16).
8. Hydration reconstitutes the full existing Strategy's confirmed content
   into an editable draft, marks it already-confirmed (not newly accepted
   by hydration), preserves existing lineage/occurrence identities for
   unchanged content, and never silently changes operative meaning (§17).
9. AI/suggestion mechanisms may propose mappings but never bypass
   confirmation; a proposal is draft content, never translation output
   (§18).
10. Every edit — correction-flow (via §17 hydration) or ordinary revision —
    re-enters the full pipeline from translation forward; no partial
    re-check exists (§19).
11. A translation-gate rejection is visually and structurally distinguished
    from a Well-Formedness defect in any Trader-facing surface (§10, §16).

## 27. Recommendation

Accept this RFC as the product-and-contract basis for authoring-to-canonical
translation. On acceptance: (a) N-25 is resolved to the extent stated here,
with canonical open-questions register alignment deferred to a separate
authorized synchronization cycle (matching RFC-0001/RFC-0003 precedent);
(b) N-22-B remains blocked until (i) this RFC is accepted and (ii) a
separate N-22-B implementation-scoping/authorization cycle is run against
it, exactly as N-22-A required its own scoping/authorization after RFC-0003
and ADR-0006 were accepted; (c) risks R-F…R-I are recorded for a later
register-alignment cycle; (d) §24's flagged questions — new intermediate
type, new public API, ADR-0003 §9 sufficiency, downstream ADR need — are
explicitly preserved as open, to be picked up by a future read-only
determination in the established PBA-0147/PBA-0150-style pattern before any
implementation authorization, rather than assumed answered by this
acceptance.
