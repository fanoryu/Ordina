# RFC-0003: Well-Formedness-Failure UX

- **Status:** ACCEPTED
- **Author:** Principal Engineer
- **Created:** 2026-08-19
- **Decision date:** 2026-08-19
- **Accepted by:** Product Board, PBR-0018
- **Authorizing artifact:** PBA-0152 (2026-08-18; "N-22 RFC Drafting authorization"), under the **PBD-0007** post-Slice-1 roadmap-direction record ("Post-Slice-1 Roadmap Direction — N-22 Selected"); PBA-0153 (acceptance)
- **Review:** ARR-0053 (PASS — RFC-0003 INDEPENDENTLY VERIFIED; zero BLOCKER, zero MAJOR; one non-blocking MINOR (§9.7 stage-3 re-submission trigger/timing ambiguity); one non-blocking NOTE (transient re-evaluation UX state unspecified); remediation required: NO)
- **Originating source:** RFC-0001 §12 (Strategy Well-Formedness), §19 (N-22 opened), §21, §23
- **N-22 implementation authorization:** NOT AUTHORIZED by this acceptance. A separate implementation-authorization cycle is required.

> **Subordinate to** Product Constitution **v1.0 — ACTIVE — RATIFIED** and to the
> applicable Tier-2 Product Specifications: `ux-constitution.md`,
> `trading-decision-model.md`, `validation-model.md`, `user-zero.md`. This RFC
> may refine, but may not contradict, any of them, nor RFC-0001 (ACCEPTED,
> PBR-0003). It is a **product-and-UX-contract** RFC: it defines the
> platform-agnostic interaction contract for a Trader encountering an
> `UNUSABLE` Strategy — not a screen design, component library, framework, or
> persistence model.

---

## 1. Summary

RFC-0001 named **Strategy Well-Formedness** as a deterministic, rule-based
pre-use gate (§12) and opened **N-22** — "whether/how an 'unusable Strategy' is
surfaced to the trader" — as an unresolved UX question. Slice 1 has since
closed: the checker (`WellFormednessResult`, `DefectIdentity`, canonical
ordering, `NotEvaluatedRecord`) is implemented and frozen. This RFC defines the
platform-agnostic interaction contract for how a Trader is shown, and moves
from, an `UNUSABLE` Strategy verdict: how the verdict is distinguished from
trade-readiness statuses, how the complete defect set and its identity are
surfaced without exposing checker internals, how not-evaluated (suppressed)
checks are represented without implying an additional defect, how blocking
relationships are communicated, and how the Trader moves from diagnosis to a
correction that only the Trader ever makes. It selects no framework, screen
layout, component library, persistence model, API shape, or device-specific
behavior.

## 2. Motivation

Well-Formedness exists so that an unusable Strategy is caught before it can
participate in Decision Validation (RFC-0001 §12). But a gate that rejects
silently, or that is presented ambiguously, defeats its own purpose — the
Trader who owns Strategy authorship (Constitution C2; RFC-0001 §9) cannot
correct what they cannot see or understand. RFC-0001 deliberately left N-22
open because the checker's output contract did not yet exist to design
against (§19, §21). It now does: Slice 1 is closed, and `WellFormednessResult`
exposes a verdict, a complete ordered defect set, and not-evaluated metadata
with root-blocker identity (`src/result/mod.rs`, `src/result/identity.rs`,
`src/checks/prerequisite.rs`). Without a UX contract grounded in that real
data shape, an eventual implementation risks either inventing product
decisions ad hoc (violating "discussion → RFC → decision" ordering, CLAUDE.md
§5) or building a UI that silently repairs, conflates verdicts, or drops
defect/suppression information the checker took care to make precise. PBD-0007
selected N-22 as the next post-Slice-1 initiative for exactly this reason.

## 3. Constitution Check

**Boundary test (Constitution §2.1):**

- **Origination** — Unaffected. This RFC concerns only how an already-computed
  `UNUSABLE` verdict is presented and how the Trader is routed back to
  authoring. It does not touch Direction, Instrument, or Construction-value
  origination. The Trader remains the sole author of any correction (§9
  below).
- **Determinism** — Unaffected. Well-Formedness is already a deterministic,
  rule-based gate (RFC-0001 §12); this RFC does not add AI, scoring, or
  probability to the gate itself or to how its result is characterized. AI, if
  used at all in a future "Why?"-style explanation of a defect, is confined to
  explanation with no decision authority (ux-constitution.md §2), exactly as
  elsewhere in the product.
- **Provenance** — Strengthened. §6 and §7 below require that every defect
  and every not-evaluated record remain traceable to its checker-produced
  identity and its blocking defect, without requiring the Trader to
  understand the checker's internal identity model (occurrence anchors,
  discriminator kinds, canonical order).
- **Gating** — Unaffected and reinforced. §5 and §10 make explicit that
  `UNUSABLE` is not, and must never be presented as, READY, WAIT, or
  DO_NOT_TRADE, and that a Strategy failing Well-Formedness cannot enter the
  Decision Validation lifecycle Well-Formedness gates (RFC-0001 §12).
- **Restraint** — Unaffected. Nothing in this contract implies or guarantees
  that a corrected Strategy will produce a favorable outcome; it addresses
  only how a defect is diagnosed and how correction ownership is preserved.

**Further:**

- **Principles served:** 4 (every decision must be explainable — extended
  here to "every Well-Formedness failure must be explainable"), 5 (simplicity
  over information), 8 (protect the Trader from impulsive decisions — an
  `UNUSABLE` Strategy must not be nudged past), 10 (rules validate; AI
  explains), 12 (the simplest interface that preserves good decisions wins).
- **Constraints touched:** C2 (trader agency — correction is trader-owned),
  C3 (rules validate — the gate result is not overridden by presentation),
  C4 (explainability/provenance — extended to defect and suppression
  identity), C9 (modification cannot bypass validation — analogously, defect
  correction cannot bypass Well-Formedness re-evaluation).
- **Validation dimensions touched:** None directly. Well-Formedness is
  RFC-0001 §12's own pre-use gate, prior to and distinct from the three
  Decision Validation dimensions (Strategy, Trader Readiness, Risk) owned by
  `validation-model.md`. This RFC does not touch Strategy Validation, Trader
  Readiness, or Risk Validation.
- **Status semantics / resolution model / lifecycle:** **No.** This RFC does
  not change `validation-model.md` §4's resolution model or any
  `trading-decision-model.md` lifecycle stage. `UNUSABLE` remains, exactly as
  RFC-0001 §12 fixed it, a pre-use gate outcome distinct from READY/WAIT/DO
  NOT TRADE — not a fourth status folded into that vocabulary. This RFC
  defines presentation of an existing outcome, not a new outcome.
- **`ux-constitution.md` compliance, especially §1, §5, §5A–§5F:** Yes — see
  §11 below, which extracts and applies the already-accepted constraints
  directly; no new accessibility or interaction requirement is invented here.

No boundary-test answer is "no" or "unclear." No conflict identified.

## 4. Goals

1. Define how `UNUSABLE` is presented so it is never conflated with
   READY/WAIT/DO_NOT_TRADE.
2. Define how the complete defect set (not just the first defect) is
   surfaced to the Trader.
3. Decide, explicitly, whether presentation order must track canonical
   defect order.
4. Define how defect identity is preserved in the UX without requiring the
   Trader to understand the checker's internal identity model.
5. Define how not-evaluated (suppressed) metadata is represented without
   implying a defect occurred.
6. Define how prerequisite/blocking relationships between a suppressed check
   and its root blocker are communicated.
7. Define the diagnosis-to-correction flow shape (not screen mockups).
8. Preserve Trader ownership of every operative correction.
9. Prohibit silent repair of operative Strategy rules.
10. Prohibit presenting `UNUSABLE` as READY/WAIT/DO_NOT_TRADE.
11. Define behavior when multiple defects and multiple suppressions coexist
    in one result.
12. State accessibility/platform-neutral interaction requirements already
    established by `ux-constitution.md`.
13. State exclusions and open questions explicitly.

## 5. Non-Goals

This RFC does **not** define: a screen layout, wireframe, or mockup; a UI
framework, component library, or design system; a persistence model or API
shape for defects, not-evaluated records, or corrections; device-specific
input handling (touch, keyboard, voice); a platform choice (Q-06, still
open); jurisdiction-specific behavior (Q-08, still open); Well-Formedness
evaluator/runtime semantics beyond what Slice 1 already fixes (N-24, still
open); the authoring-to-canonical translation mechanism (N-25, still open);
data privacy or erasure behavior for defect/correction records (F-8, still
open); any change to `WellFormednessResult`, `DefectIdentity`,
`NotEvaluatedRecord`, or canonical ordering semantics — those are Slice 1,
accepted and closed, and this RFC treats them as a **frozen, consumed**
contract; and the name or boundary "Slice 2" — no accepted authority
establishes that name or a scope for it (PBA-0152 explicitly withholds this
authorization; PBD-0007 explicitly states the initiative is not named
"Slice 2").

## 6. Governing Product Constraints

- **C2 / Trader agency** — every correction to Strategy content remains a
  Trader act; Ordina may report, never repair.
- **C3 / Rules validate; AI explains** — the Well-Formedness verdict is
  rule-produced; nothing in this UX may let an explanation, summary, or
  assistive text override or soften the verdict.
- **C4 / Explainability without decision authority** — every defect and every
  not-evaluated record must support inspection grounded in the data the
  checker actually produced (verdict, defect identity, blocking identity);
  explanation carries no authority to change the verdict.
- **ux-constitution.md §1** — the primary decision view exposes only
  READY / WAIT / DO_NOT_TRADE. `UNUSABLE` is not a fourth member of that set
  (§10 below).
- **ux-constitution.md §2** — the "Why?" path stays grounded in evidence
  available at validation time and may not originate new content; applied
  here to defect explanation.
- **ux-constitution.md §3** — progressive disclosure; the primary surface is
  minimal, detail on request.
- **ux-constitution.md §5B** — no direct editing of calculated outputs as a
  status-preserving action; applied here as no "fix" affordance that edits
  operative Strategy content without the Trader authoring the change and
  re-triggering Well-Formedness.

## 7. Terminology

Terms follow `glossary.md` and RFC-0001 §6. This RFC does not introduce new
Tier-2 vocabulary. Where this RFC uses "defect," "not-evaluated," "blocking,"
or "canonical order," it uses them exactly as Slice 1's result contract
(`src/result/mod.rs`, `src/result/identity.rs`, `src/result/discriminator.rs`,
`src/checks/prerequisite.rs`) defines them — see §8.

## 8. The Data Contract This RFC Consumes

Slice 1 (closed, frozen) produces a `WellFormednessResult` with:

- a `Verdict` — `WellFormed` or `Unusable`, and the defect set is **empty iff
  `WellFormed`** (structurally enforced — an `Unusable` result cannot be
  constructed without at least one defect);
- a **complete, ordered** `defects: &[DefectIdentity]` — every defect the
  checker found, not a "first defect" only;
- a **complete** `not_evaluated: &[NotEvaluatedRecord]` — every check whose
  evaluation was suppressed because a prerequisite it depends on was itself
  defective;
- a bound `CheckerSemanticsVersion` (`WF-1`).

Each `DefectIdentity` carries a `DefectCore` — the offending construct's
occurrence identity, a `DefectCategory`, and a `Discriminator` — under a
**total canonical order** (§6.4: check number, then occurrence anchor, then
category ordinal, then discriminator). This order exists so two conforming
checkers produce byte-identical results; it is an **identity/reproducibility
guarantee**, not a UX mandate (§ Detailed Design, point 3).

Each `NotEvaluatedRecord` carries: which check was suppressed, the
occurrence identity of the construct (or, for a pairwise-gated check, the
canonically-oriented pair) whose evaluation did not run, and the
`blocking: DefectIdentity` of the defect that caused the suppression. A
not-evaluated record is never itself a defect — the checker keeps the two
sets structurally distinct (`defects` vs. `not_evaluated`), and this RFC
requires the UX preserve that distinction (§ Detailed Design, point 5).

This RFC does not read implementation internals beyond this result surface;
it treats the checker as a black box that produces exactly these four
things.

## 9. Detailed Design

### 9.1 How `UNUSABLE` is presented (point 1)

`UNUSABLE` is a **distinct state**, never a value inside, a variant of, or a
visual near-neighbor of the READY / WAIT / DO_NOT_TRADE set
(ux-constitution.md §1). Concretely: the interaction contract requires that
any surface capable of showing READY/WAIT/DO_NOT_TRADE for a Strategy must,
when that Strategy's Well-Formedness verdict is `Unusable`, show a
**categorically different presentation** — different label, different
affordances, and no shared color/iconography role with any of the three
trade-readiness statuses — such that a Trader cannot mistake "this Strategy
cannot yet be evaluated" for a trade-readiness signal about a decision. The
Strategy is simply **not offered** for Decision Validation while `Unusable`;
there is no "explain around it" path (Constitution §2.1 Gating; RFC-0001
§12's Boundary).

### 9.2 Surfacing the complete defect set (point 2)

Every element of `defects()` is surfaced — the interaction contract
prohibits any presentation that shows only the first defect, a summary count
without access to each item, or a "fix one, see the next" drip-feed that
hides the existence of the remaining defects. Progressive disclosure
(ux-constitution.md §3) governs **how much detail** each defect shows by
default, not **whether** every defect is reachable: a minimal default view
may show a count and list of defect headlines, but every defect in the set
must be individually reachable from that view before any correction is
required. Consistent with §9.1's non-negotiable Gating: the Trader is never
required to "solve" one defect to discover a second exists.

### 9.3 Canonical order vs. presentation order (point 3) — the explicit call

**Decision: canonical defect order is an identity/reproducibility guarantee
internal to the checker contract (§6.4). Presentation order is free to
differ, and by default SHOULD differ where a more Trader-useful order
exists — subject to two hard constraints.**

Reasoning: §6.4's canonical order exists so that two conforming
implementations, or two runs over the same input, produce the *same
sequence* of the *same identities* — a determinism/testability property
(the Slice Contract's own conformance dimension 6, "insertion/permutation-
order independence"). It says nothing about what a Trader finds easiest to
act on. A Trader is generally better served by, for example, grouping
defects by the Strategy section they concern (Direction logic, condition
classification, evidence declarations, …) or by severity/blocking-ness
(defects that themselves block other checks first) than by an ordering
whose leading key is an internal check-number ordinal.

The two hard constraints on any presentation reordering:

1. **Reordering must be a pure function of the defect set's own identities**
   (category, occurrence anchor, discriminator, blocking relationships) —
   never of incidental factors such as hash-map iteration, wall-clock
   discovery order, or randomization. This keeps presentation
   deterministic even though it need not be canonical (a corollary of
   Constitution C3/C4 — every user-visible ordering must itself be
   explicable, not incidental).
2. **Every defect's identity must remain independently addressable**
   regardless of presentation order — the Trader can always ask "why is
   this specific defect here" and get an answer grounded in that defect's
   own `DefectIdentity`, not in its position (§9.4). Presentation order is
   cosmetic; it must never become part of how a defect is identified or
   referenced (e.g., "defect #3" must not be used as the defect's name if
   reordering could change what "#3" means between renders).

This RFC does not select a specific presentation grouping (that is
implementation-detail territory, N-25/downstream-architecture-adjacent) —
it fixes only that canonical order is not a presentation mandate, and that
any presentation order chosen must be identity-derived and stable, not
incidental.

### 9.4 Defect identity without exposing the checker's internal model (point 4)

The Trader must never need to understand "occurrence identity,"
"discriminator kind," "canonical order," or `DefectCategory` ordinals as
such. The UX contract instead requires that each defect be presented through
a **stable, Trader-facing description** derived from — but not identical in
vocabulary to — its `DefectIdentity`:

- **what** is wrong (a category-derived, plain-language defect description);
- **where** it is wrong (the offending construct, described in terms of the
  Strategy content the Trader authored — e.g., "the Direction rule for
  condition X" — not in terms of an internal occurrence handle);
- enough of the discriminator's meaning, in plain language, to distinguish
  this defect from a same-category sibling on the same construct (e.g.,
  "the second required property is missing" rather than exposing
  `PropertyPosition(2)` as such).

The underlying `DefectIdentity` remains the system-of-record identity for
provenance, deduplication across a re-check, and any "Why?"-style detail
view — it is translated for display, never discarded. This satisfies C4
(provenance) while respecting ux-constitution.md §3 (progressive
disclosure): internal identity is available on request/underneath, not
imposed on the primary view.

### 9.5 Representing not-evaluated metadata without implying a defect (point 5)

A `NotEvaluatedRecord` is never rendered inside, adjacent-in-styling to, or
counted together with, the defect set. The interaction contract requires:

- a **separate section or category**, visually and structurally distinct
  from "defects," for suppressed/not-evaluated checks;
- language that states *inability to evaluate*, never *failure* — e.g.
  "not yet checked (blocked by another issue)" rather than any phrasing that
  reads as an error, warning-of-the-same-severity, or additional defect;
  no error iconography shared with the defect set;
- a not-evaluated count that is **never added to** the defect count in any
  headline/summary presentation (e.g. "3 issues" must mean three defects,
  not three defects plus suppressions folded together).

This directly enforces §6.2's structural distinction (`defects` vs.
`not_evaluated` are different fields, never merged) at the presentation
layer, and closes the specific risk the drafting brief calls out: a
suppressed check is not itself wrong, and must never look like one.

### 9.6 Communicating blocking relationships (point 6)

Each `NotEvaluatedRecord` names the `blocking: DefectIdentity` that caused
its suppression. The UX contract requires this be surfaced as an explicit,
navigable link: "this could not be checked because of [that other defect]"
— using the same Trader-facing defect description (§9.4) for the blocker,
and, where the interaction surface supports it, a direct way to jump from
the not-evaluated item to the blocking defect's own entry in the defect set.
This is presentation of an already-computed fact (the checker resolves the
canonical-minimum blocker per Ruling A precedent recorded in the governance
history, PBA-0143) — this RFC does not add, remove, or reinterpret blocker
computation; it only requires the existing `blocking` field be shown, not
hidden or summarized away as "N checks skipped."

Where a not-evaluated record is pairwise (two constructs jointly gated), the
UX must name both members it concerns, not only the canonical `construct`
member — the Trader authored both halves of that pair and needs to know
both are affected.

### 9.7 Diagnosis-to-correction flow shape (point 7)

The flow has three stages, described structurally (no screens):

1. **Diagnosis** — the Trader sees the `Unusable` state (§9.1), the complete
   defect set (§9.2) and the not-evaluated set (§9.5–9.6), each individually
   inspectable (§9.4).
2. **Navigation to authorship** — from a specific defect, the Trader is
   taken to the point in Strategy authoring where the offending content
   lives (the construct §9.4 identifies) — this reuses whatever authoring
   surface RFC-0001 §9's hybrid authoring model establishes; this RFC does
   not define that surface, only that diagnosis must be able to hand off to
   it per-defect, not only "re-open the whole Strategy."
3. **Re-submission and re-check** — after the Trader edits authored content
   and confirms it (RFC-0001 §9's per-rule confirmation), the *entire*
   Strategy is re-submitted to Well-Formedness as a fresh check — never a
   partial "re-check only the fixed rule." A defect the Trader believes
   fixed is only actually resolved when the full result comes back
   `WellFormed` or with a strictly smaller defect set; the UX must not
   assume or preemptively mark a defect "fixed" before that fresh result
   returns (mirrors C9 — modification invalidates the prior evaluation and
   requires complete re-validation, applied here to Well-Formedness rather
   than Decision Validation).

Stages 1–3 may repeat; nothing here requires all defects be fixed in one
pass. A Trader may leave with the Strategy still `Unusable` — this is not a
failure state for the product (Constitution §5's neutrality principle,
analogically: a Strategy that stays `Unusable` while the Trader takes time
is not itself a bad outcome).

### 9.8 Trader ownership of every operative correction (point 8)

Every change to operative Strategy content that resolves a defect is an
explicit Trader action — RFC-0001 §9's authoring-model guards (explicit,
inspectable, deterministic, trader-confirmed operative rules) apply
identically to a *correction* as they do to original authoring. No
assistive mechanism (guided suggestion, template, natural-language
paraphrase) may become authoritative without that same confirmation step.
This RFC requires nothing new here beyond applying RFC-0001 §9 to the
correction path — it does not relax or duplicate that guard.

### 9.9 Prohibition: no silent repair (point 9)

**Explicit, absolute prohibition.** No UX affordance may apply a change to
operative Strategy content — a rule, a condition classification, a
Direction rule, a candidate-value derivation, an applicability filter, a
configuration bound — without an explicit, individually confirmed Trader
action for *that specific change*. This includes, and is not limited to:

- an "auto-fix" or "fix all" button that applies a change without per-change
  confirmation;
- a suggested correction that becomes authoritative merely by being
  displayed, selected implicitly, or not rejected within some time window;
- any assistive/AI-generated repair proposal treated as already-applied
  rather than as a proposal awaiting explicit trader confirmation (per rule,
  not in bulk) before it is authoritative.

A tool **may** propose a correction as inspectable, explicit, trader-facing
suggested text or structure — that is within RFC-0001 §9's hybrid authoring
model — but the proposal has no authority until the Trader confirms it
per-rule, exactly as original authoring requires. This closes the specific
adversarial risk named in the drafting brief.

### 9.10 Prohibition: no state-label ambiguity (point 10)

**Explicit, absolute prohibition.** No surface may present `UNUSABLE` using
the vocabulary, iconography, color role, or layout position reserved for
READY, WAIT, or DO_NOT_TRADE (ux-constitution.md §1, §5F). In particular:
`UNUSABLE` must never be phrased as a form of DO_NOT_TRADE ("this Strategy
says don't trade" is wrong — DO_NOT_TRADE is a Decision Validation status
about a specific decision attempt; `UNUSABLE` is a pre-use gate outcome about
the Strategy itself, and the Strategy in an `UNUSABLE` state cannot reach
Decision Validation at all, so it cannot legitimately produce any of the
three statuses). Any Strategy-satisfaction display alongside a real
DO_NOT_TRADE (ux-constitution.md §5F, PBD-0003) is a separate, already-
governed concept and out of this RFC's scope; this RFC's prohibition is
that `UNUSABLE` never borrows that display's vocabulary either.

### 9.11 Multiple defects and multiple suppressions together (point 11)

The general case — not the single-defect happy path — is: N ≥ 1 defects and
M ≥ 0 not-evaluated records in one result, where defects may themselves be
the `blocking` identity for zero, one, or several not-evaluated records
(fan-out), and where a not-evaluated record's blocker is always itself a
member of `defects()` (never another not-evaluated record — Slice 1's model
resolves suppression to its root blocking *defect*, not to a chain of
suppressions, per the governance record's Ruling A canonical-minimum/
transitive-propagation resolution). The UX contract requires:

- both sets remain fully enumerable regardless of size (§9.2, §9.5) — no
  collapsing into "many issues found" without drill-down;
- the defect-to-suppression fan-out is discoverable **from the defect
  side** too: viewing a specific defect should let the Trader see which (if
  any) not-evaluated checks it is blocking, not only the reverse link from
  §9.6;
- fixing the root-blocking defect is the Trader's natural next action for
  resolving a whole cluster of suppressions — the UX may *suggest* this
  ordering as a hint (e.g., "resolving this may allow N other checks to
  run"), but per §9.7 stage 3, whether it actually does is only known once
  the Strategy is re-submitted and re-checked in full; the UX must not
  claim in advance that fixing one defect *will* resolve specific
  suppressions.

### 9.12 Accessibility / platform-neutral requirements (point 12)

This RFC invents no new accessibility requirement. It cites and applies
what `ux-constitution.md` already establishes:

- **§1 (single primary status surface)** — applied in §9.1: `UNUSABLE`
  gets its own distinct, minimal primary presentation, not layered onto the
  READY/WAIT/DO_NOT_TRADE surface.
- **§2 (Why? path grounded in evidence, no new inputs, no origination)** —
  applied in §9.4/§9.6: defect and blocker explanation stays grounded in
  the checker's own result; it may not introduce new decision inputs or
  originate suggestions beyond what §9.9 permits as inspectable, trader-
  confirmed proposals.
- **§3 (progressive disclosure)** — applied throughout: minimal default,
  full detail on request, per §9.2/§9.4.
- **§4 (reduce screen time)** — a Trader who leaves with the Strategy still
  `Unusable` and returns later is not a failure mode this RFC's flow (§9.7)
  penalizes or discourages continuing engagement to "fix."
- **§5 (forbidden surfaces — no confidence/probability/score, no
  persuasive/urgency language, no outcome guarantee)** — applied to defect
  and correction language: no defect may be phrased as more urgent, more
  likely to matter, or scored, and no correction flow may use persuasive
  urgency language (§5D's notification-neutrality principle applied by
  analogy to defect-resolution prompts).
- **§5B (no direct editing of calculated outputs as a status-preserving
  action)** — applied in §9.9: correction is never presented as a way to
  keep some prior state; it is authored content that triggers full
  re-evaluation.

Q-06 (platform/form factor) remains unresolved by this document, exactly as
`ux-constitution.md` §7 already states; every requirement above is written
to hold regardless of the eventual platform.

### 9.13 Exclusions and unresolved questions

See §5 (Non-Goals) and §12/§13 below.

## 10. Alternatives Considered

1. **Present `UNUSABLE` as a special case of DO_NOT_TRADE.** Rejected: this
   is exactly the state-label conflation ux-constitution.md §1 and RFC-0001
   §12 already forbid — `UNUSABLE` is a pre-use gate outcome about the
   Strategy, not a Decision Validation status about a trade attempt, and
   folding them would corrupt the status vocabulary R-10 (RFC-0001 §17)
   exists to protect.
2. **Show only the first/most-severe defect, revealing others as each is
   fixed ("wizard" style).** Rejected as the *default*: it violates "the
   complete defect set is surfaced" (point 2) and risks the Trader
   under-estimating how much correction remains, in tension with Principle
   8 (protect from impulsive decisions) — a Trader who thinks one fix
   suffices may act prematurely elsewhere. A step-by-step *view* of an
   already-fully-enumerated set remains permitted (progressive disclosure);
   a step-by-step *reveal* of a partially-hidden set is not.
3. **Auto-apply mechanically unambiguous fixes (e.g., a clearly missing
   required property with only one legal value).** Rejected: no fix is ever
   "unambiguous enough" to bypass Trader confirmation without contradicting
   C2/C9 and RFC-0001 §9's per-rule confirmation guard — mechanical
   obviousness is a UX-convenience judgment, not a constitutional exemption,
   and creates exactly the erosion path R-B (RFC-0001 §17) warns against.
4. **Require presentation order to exactly track canonical order, to avoid
   ambiguity.** Considered and rejected in §9.3: canonical order is a
   reproducibility guarantee, not a UX requirement, and forcing it into
   presentation would needlessly subordinate Trader usability to an
   internal identity concern the Trader never needs to know exists.

## 11. What This Makes Harder

- A future implementation cannot ship a fast "fix it for me" affordance
  without an explicit, per-rule Trader confirmation step — this is a
  deliberate cost, not an oversight.
- A future implementation cannot collapse defects and suppressions into one
  undifferentiated "problems" list for simplicity — the two must stay
  visually and structurally distinct.
- A future implementation cannot use defect-count or suppression-count as a
  single scary headline number combining both — they must be counted and
  shown separately.
- Any presentation-order choice must be justified as identity-derived and
  stable; arbitrary or incidental ordering (e.g. raw insertion order,
  hash-based grouping) is not permitted even though canonical order is not
  mandated.

## 12. Risks

Carried for later register alignment, mirroring RFC-0001 §17's model:

- **R-F** — A future implementation reads "presentation order need not be
  canonical" (§9.3) as license for incidental/nondeterministic ordering.
  *Mitigation:* §9.3's two hard constraints are explicit and must be
  quoted in any downstream implementation plan.
- **R-G** — A "suggested correction" affordance (§9.9) drifts toward
  auto-apply through UX pressure (defaults, pre-selection, single-click
  "accept all"). *Mitigation:* per-rule confirmation is named as absolute,
  not a default; extends R-A/R-B (RFC-0001 §17).
- **R-H** — Not-evaluated styling drifts toward defect-equivalent severity
  over time as the UI evolves. *Mitigation:* §9.5's structural-separation
  requirement is stated as a hard contract point, not a stylistic
  preference.
- **R-I** — Partial re-check (checking only the "fixed" rule instead of the
  whole Strategy) is implemented for performance reasons, silently
  reintroducing stale defect state. *Mitigation:* §9.7 stage 3 requires
  full re-submission explicitly.

## 13. Open Questions

- **Presentation grouping** — this RFC fixes that presentation order need
  not be canonical (§9.3) but does not select a specific grouping strategy
  (by Strategy section, by severity, by blocking fan-out, …); left to
  downstream design within the §9.3 constraints.
- **How much of the checker's plain-language defect description is
  authored by product copy vs. derived programmatically from
  `DefectCategory`/`Discriminator`** — a downstream architecture question,
  not resolved here.
- **Whether/how a "Why?"-style deep explanation of a defect may use AI
  assistance** — permitted in principle under ux-constitution.md §2 (no
  decision authority), but the specific mechanism is not designed here.

**Explicitly out of scope, per PBA-0152 and this RFC's own boundary — none
of the following are resolved, narrowed, or silently touched:**

- **Q-06** — platform / form factor. Remains open; every requirement in
  this RFC is written to hold regardless of platform.
- **Q-08** — jurisdiction. Not addressed.
- **N-24** — Well-Formedness evaluator/runtime semantics beyond what Slice
  1 already fixes. Not addressed; this RFC treats the Slice 1 result
  contract as given and frozen.
- **N-25** — the authoring-to-canonical translation mechanism. Not
  addressed; §9.7 stage 2 references "wherever the authoring surface
  lives" without defining it.
- **F-8** — privacy/erasure of defect or correction records. Not addressed.

This initiative is explicitly **not** "Slice 2" — no accepted product or
domain authority establishes that name or an implementation boundary for
it (PBD-0007, PBA-0152).

## 14. Architectural-Commitment Determination

**Determination: NO — this RFC does not require a new ADR.**

Justification: every data shape this RFC's interaction contract references
(`WellFormednessResult`, `DefectIdentity`/`DefectCore`, `Discriminator`,
`NotEvaluatedRecord`, canonical ordering) is Slice 1's existing, accepted,
closed, frozen output contract, consumed here exactly as-is (§8). This RFC
proposes no new persistent data shape, no new field, no new ordering rule,
and no change to `src/**`. Its only "new" surface is a *presentation*
mapping from that existing data to Trader-facing language and structure
(§9.4), which is a UX/product-contract concern (same genre as RFC-0001's own
Strategy contract) — architecture decisions (ADRs) record technology/
implementation choices for an accepted contract, and no such choice is made
or required here; PBA-0152 itself does not authorize an ADR, and nothing in
this draft found a gap that would require one.

## 15. Recommendation

This RFC is submitted as **PROPOSED**, not for acceptance this cycle
(consistent with PBA-0152's scope, which authorizes drafting only). It
recommends: (a) Product Board review of the §9.3 canonical-order-vs-
presentation-order decision in particular, as the one point requiring
judgment beyond restating existing authority; (b) on any future acceptance,
recording risks R-F…R-I in `docs/product/risk-register.md`; (c) treating the
concrete authoring-surface handoff (§9.7 stage 2, N-25-adjacent) and any
AI-assisted defect explanation mechanism as follow-on RFC/design work, not
resolved here.

## 16. Open Items Requiring Separate Authorization

None beyond what §13 already lists as open questions. No `src/**`/`tests/**`
change, no Cargo/toolchain change, and no accepted contract/ADR/RFC change
is proposed or required by this document.
