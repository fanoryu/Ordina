# ADR-0007: OccurrenceId Classification and Revision-Identity Scope

- **Status:** PROPOSED
- **Date:** 2026-08-19
- **Deciders:** Principal Engineer; to be accepted by Product Board
- **Related:** ADR-0003 (ACCEPTED, PBR-0007 — **explicitly superseded, narrowly, by
  this ADR**); ADR-0002 (ACCEPTED, PBR-0006 — unchanged, not reopened); ADR-0004
  (ACCEPTED — cited, not reopened, specifically its W-1 resolution); Slice Contract
  Revision 4 (`first-runnable-slice-contract-r04.md` §C.1–§C.2, ACCEPTED PBR-0016 —
  cited, not amended); `src/model/opaque.rs` (current source, cited as evidence);
  RFC-0004 (PROPOSED — downstream consumer of this ruling, not edited here);
  authorized by **PBA-0162**, following the read-only determination under
  **PBA-0161** (outcome C — current authority insufficient to decide without this
  amendment)

> This ADR decides **one narrow question only**: whether `OccurrenceId` (Slice
> Contract Revision 4 §C.1's construct-occurrence addressing identity) is inside
> or outside ADR-0003 §6's canonicalization taxonomy and §7's revision-identity
> computation. It **explicitly, narrowly supersedes ADR-0003** for this one
> question and for no other. It does **not** reopen canonicalization ordering
> generally, `LineageId` semantics, revision-token format, duplicate handling,
> provenance classification, evaluator semantics, serialization, N-24, Q-08, F-8,
> or any N-25 product semantics beyond this classification's direct consequence.
> ADR-0002 and ADR-0004 are **not** reopened; they are read as fixed inputs.

---

## Context

ADR-0003 (accepted 2026-08-12) defines Strategy revision identity as a
content-addressed function of the **canonical semantic form** — "the ADR-0002
typed model after Well-Formedness and canonicalization" (§2, §5) — and provides
an exhaustive ID/CX/PX/INV classification table (§6) that every canonical
Strategy property must carry exactly one entry in before it may participate in
revision-identity derivation (§6.1's closure rule). The closure rule explicitly
forbids ad hoc classification and requires "an explicit semantic-model/version
decision" for any property appearing after ADR-0003's acceptance.

Four days later, the First Runnable Slice Contract's Revision 4 (accepted
2026-08-16, PBR-0016, under an entirely separate authorization track — PBA-0088
through PBA-0095 — that never cross-referenced or amended ADR-0003) introduced a
**sixth opaque domain**, `OccurrenceId`: a mandatory, caller-supplied,
totally-ordered identity assigned to every construct occurrence in the
Well-Formedness checker's input, explicitly described as "an addressing and
deterministic-conformance mechanism only" that "carries no product semantics"
(§C.1–§C.2) and is never used to replace lineage identity in any semantic role.

`src/model/strategy.rs`'s own module documentation confirms the Slice-1
`Strategy` struct — the one that carries this field — "is the input to the
Well-Formedness checker," and the Slice Contract's own A2 table (row 16,
`first-runnable-slice-contract.md` line 734) states plainly: "Slice 1 computes
**no** canonical form and **no** revision identity; it establishes only the
*preconditions* ADR-0003 I9 requires." ADR-0004's own accepted resolution of
open question W-1 goes further, formally distinguishing the Well-Formedness
checker's own input representation from ADR-0003's canonical form: "A WF
analysis may use a deterministic local analysis normalization that yields no
revision identity, no canonical serialization, and **does not supersede ADR-0003
canonicalization**" (`open-questions.md` W-1, citing `adr/0004-...` §7).

The gap this exposes: ADR-0003's §6 table was authored before `OccurrenceId`
existed, has no entry for it, and no accepted document has ever formally decided
whether it is inside or outside the taxonomy §6.1 requires every canonical
Strategy property to be classified under. RFC-0004's identity-minting and
multi-surface-convergence sections (§13, §14.2–§14.3) depend on this answer, and
a genuine, read-only Product Board determination (PBA-0161) found current
authority — taken piece by piece — insufficient to certify an answer without
performing the explicit classification act ADR-0003 §6.1 itself names as the
required remedy. This ADR performs that act.

## Decision

**We will classify `OccurrenceId` (and any future construct-occurrence
addressing identity of the same kind) as categorically outside ADR-0003 §6's
canonicalization taxonomy and §7's revision-identity computation — it is not an
ID, CX, PX, or INV property of the canonical semantic form; it is a property of
a distinct, already-architecturally-recognized data domain (the Well-Formedness
checker's own local input representation, per ADR-0004's accepted W-1
resolution) that ADR-0003 does not govern at all.**

Concretely:

1. **Taxonomy membership (question 1).** `OccurrenceId` is **not** a "canonical
   Strategy property" within the meaning of ADR-0003 §6/§6.1. ADR-0003 §6
   classifies properties of **the canonical semantic form** — the ADR-0002 typed
   model after Well-Formedness and canonicalization (ADR-0003 §2, §5).
   `OccurrenceId` is instead a property of the **Well-Formedness checker's own
   input representation** — the Slice-1 `Strategy` struct, which ADR-0004's W-1
   resolution already formally distinguishes from ADR-0003's canonical form
   ("does not supersede ADR-0003 canonicalization") and which the Slice Contract
   itself states computes "no canonical form and no revision identity" (A2 table
   row 16). It therefore never enters ADR-0003's taxonomy at all — not because it
   was overlooked, but because it belongs, by existing accepted architecture, to
   a different pipeline stage than the one §6 classifies.
2. **Revision-identity participation (question 2).** `OccurrenceId` **does not
   participate** in Strategy revision-identity computation (ADR-0003 §7). It is
   never carried into, derived from, or required by the canonical semantic form;
   the content-addressed identity ADR-0003 §7 computes is a function of ID-class
   canonical properties only, none of which `OccurrenceId` is or resembles.
3. **Opaque/numeric ordering participation (question 3).** For the same reason,
   the **total order** a translator or other producer assigns to `OccurrenceId`
   handles (Slice Contract §3.3.1, Revision 4 §C.1) **does not participate** in
   revision identity either. It is an addressing property of the checker-input
   layer, not a property — ordered or otherwise — of the canonical semantic form
   ADR-0003 §6 governs (cf. §6's own "Field/property ordering — CX: ... ordering
   carries no meaning" row, which this ruling is consistent with in spirit, while
   remaining a distinct ruling: `OccurrenceId` order is not merely CX-classified,
   it is outside the taxonomy entirely, because `OccurrenceId` itself is).
4. **Equivalence under differing occurrence assignment (question 4).** **Yes** —
   two Strategies whose canonical semantic content (per ADR-0003 §6's existing ID
   rows) is identical, but whose `OccurrenceId` assignments or supplied order
   differ, remain **revision-equivalent**: they receive the same revision
   identity, because revision identity is computed over the canonical semantic
   form, from which `OccurrenceId` is categorically absent under this ruling.
5. **Reconciliation with Revision 4 §C.1–§C.2 (question 5).** This ruling is not
   an independent judgment call layered onto Revision 4's "carries no product
   semantics" / "addressing and deterministic-conformance mechanism only"
   framing — it is the direct architectural consequence of taking that framing,
   plus ADR-0004's already-accepted W-1 distinction between the checker's local
   input and ADR-0003's canonical form, at face value. Revision 4 never claimed
   `OccurrenceId` was part of the ADR-0002/ADR-0003 canonical model; this ADR
   confirms, formally and for the first time, that it correctly is not.
6. **§6.1 closure-rule applicability after this amendment (question 6).**
   ADR-0003 §6.1's fail-closed rule ("an unclassified property fails closed to
   INV") **never applied to `OccurrenceId` in the first place**, under this
   ruling — it applies only to properties *of the canonical semantic form*, and
   `OccurrenceId` is not one. This ADR's classification act is the "explicit
   semantic-model/version decision" §6.1 requires before that fact may be relied
   upon — it discharges the requirement by stating explicitly that the property
   is out of scope, rather than leaving that conclusion as an unstated inference.
   No existing ADR-0003 ID/CX/PX/INV row changes meaning; none is added; the
   table is untouched.
7. **Supersession scope (question 7).** This ADR supersedes ADR-0003 **only** to
   the extent of adding this one boundary clarification: that ADR-0003 §6's
   taxonomy is scoped to the canonical semantic form and does not extend to, and
   has never needed to extend to, construct-occurrence/addressing identities of
   the Well-Formedness-checker-input layer. ADR-0003's own decision text (§1–§17),
   its five identity layers (§4), its canonicalization table (§6, unamended, zero
   rows changed), its invariants (§7, I1–I10, all unchanged), its lineage
   architecture (§9), and every other section remain **fully in force, verbatim,
   and unreopened**. Where this ADR is silent, ADR-0003 governs exactly as
   before.
8. **Consequence for RFC-0004 (question 8).** See "Effect on RFC-0004" below.

## Decision Drivers

- **ADR-0003 §6.1's own named remedy.** The closure rule anticipates exactly this
  situation — a property appearing after acceptance — and requires "an explicit
  semantic-model/version decision," never a silent or ad hoc one. This ADR is
  that decision, and nothing more.
- **ADR-0004's already-accepted W-1 resolution.** The formal distinction between
  the Well-Formedness checker's own "local analysis normalization" input and
  ADR-0003's canonical form is not new architecture invented here — it is cited,
  existing, accepted text this ADR relies on rather than reopens.
- **Internal consistency (the closure-rule tension).** A literal, unamended
  reading of §6.1 would fail-close every Strategy to INV the moment any construct
  carries a mandatory but unclassified `OccurrenceId` — which is every Strategy,
  always, per Revision 4. That reading is incompatible with Slice-1 having ever
  closed and with Strategies being expected to progress toward Well-Formedness
  verdicts and eventual revision identities. Resolving this tension without
  inventing a new canonicalization rule is the smallest possible fix; excluding
  `OccurrenceId` from the taxonomy entirely (rather than inventing a fifth
  ID/CX/PX/INV-adjacent category, or forcing it into one of the four existing
  ones where it fits none) is that smallest fix.
- **Narrowness (PBA-0162).** The authorization requires the smallest durable
  decision capable of discharging §6.1 — not a general identity-architecture
  redesign.

## Consequences

### Positive

- Closes the RFC-0004 §13/§14.3 interaction gap the PBA-0160 second remediation
  cycle could not close on its own authority.
- Discharges ADR-0003 §6.1's own named obligation explicitly, rather than leaving
  it as an informally-shared assumption across governance cycles.
- Adds zero new architecture surface: no new identity layer, no new
  canonicalization rule, no new ID/CX/PX/INV row, no `src/**` change.
- Confirms, formally, that the Well-Formedness-checker-input/canonical-form
  boundary ADR-0004's W-1 resolution already drew is load-bearing beyond
  Well-Formedness itself — it is also the boundary that resolves this
  classification question.

### Negative

- A future construct genuinely needing occurrence-level addressing to survive
  *into* the canonical semantic form (for a reason not yet identified) would
  require its own new ADR-0003-superseding decision — this ADR does not
  anticipate or design for that case, only rules on `OccurrenceId` as it exists
  today.
- Establishes a precedent that Slice-Contract-introduced constructs are not
  automatically ADR-0003-taxonomy members — future Slice Contract revisions that
  add new opaque domains will need this same question asked explicitly, rather
  than assumed either way.

### Neutral / accepted trade-offs

- This ADR is deliberately minimal — it settles one boundary question and
  changes nothing else. A broader identity-architecture review remains possible
  later but is not required by this decision.

## Reversibility

**Cheap.** This ADR adds a scope clarification; it does not alter any existing
revision identity computation, canonicalization rule, or accepted invariant. A
future ADR could, if a genuine need arose, bring `OccurrenceId` (or a successor
addressing domain) into the taxonomy without invalidating any already-computed
revision identity, since none currently depends on it.

## Alternatives Rejected

- **Classify `OccurrenceId` as PX (preserved, excluded from identity) within
  ADR-0003 §6's existing table.** Rejected: PX is defined for properties that
  *are* canonical Strategy properties but are non-operative metadata (timestamps,
  authoring provenance, names/descriptions) — properties that exist *within* the
  canonical semantic form and are merely excluded from the identity computation.
  `OccurrenceId` is not a property of the canonical semantic form at all; forcing
  it into PX would misrepresent it as belonging to a domain it never entered,
  and would require the canonical semantic form itself to carry an occurrence
  field ADR-0002's model never defined.
- **Classify it as CX (canonicalized away).** Rejected for the same reason — CX
  presupposes the property is present in pre-canonical content and is normalized
  away during canonicalization; `OccurrenceId` is never present in the ADR-0002
  typed model canonicalization operates on in the first place.
- **Treat the gap as requiring no ADR — resolve it by determination alone.**
  Rejected: ADR-0003 is ACCEPTED and immutable except by explicit supersession;
  the PBA-0161 read-only determination correctly found it could not itself
  perform this classification without exceeding its read-only, non-architecture-
  deciding mandate.
- **Full ADR-0003 re-litigation / broader identity-architecture ADR.** Rejected
  as disproportionate: no evidence surfaced during PBA-0161's determination or
  this drafting cycle indicates any other part of ADR-0003 is deficient: the
  identity layers (§4), the canonicalization table's existing rows (§6), the
  invariants (§7), and the lineage architecture (§9) all remain sound and
  unchallenged.
- **Leave the classification undetermined and let RFC-0004 hedge.** Rejected:
  PBA-0161 already tried a read-only characterization pass and correctly stopped
  at "insufficient authority, architecture work required" rather than
  manufacturing a hedge; a permanent hedge in RFC-0004 would leave §13's
  convergence guarantee permanently unfalsifiable, which is a worse outcome than
  a small, targeted architecture decision.

## Effect on RFC-0004 (stated for a future remediation cycle; RFC-0004 not edited here)

Once this ADR is **ACCEPTED** (not merely PROPOSED — see Governance note below),
a future RFC-0004 remediation cycle would be permitted to state:

- **§13 (cross-surface convergence):** the convergence guarantee may be stated
  **unconditionally** with respect to canonical/revision-identity content,
  because `OccurrenceId` differences never affect it (this ADR's ruling 2 and
  4). The single-shared-translation-implementation architecture §13 already
  establishes remains necessary for the convergence claim's *testability*, but
  the specific gap this ADR closes (whether `OccurrenceId` order could silently
  break it) is resolved: it cannot, because `OccurrenceId` is outside the
  identity computation entirely.
- **§14.2 (`OccurrenceId` minting):** unaffected in substance — the assigner,
  uniqueness, and lifecycle requirements remain exactly as remediated under
  PBA-0159, now with an explicit citation available (this ADR) for why they were
  always safely independent of revision identity.
- **§14.3 (submission-order-derived occurrence order):** may remain exactly as
  written — the submission-order-source policy is confirmed to be a pure
  authoring-surface/product-level choice with **zero** revision-identity
  consequence, removing the need for any hedge or caveat about identity effects.
- **Revision-equivalence despite differing occurrence handles/order:** may be
  stated as a direct, citable consequence of this ADR (ruling 4) rather than an
  assumption.

## Validation / Acceptance Criteria

1. ADR-0003's own text (§1–§17) is unedited and unreopened except for this one
   documented, narrow supersession.
2. No new ID/CX/PX/INV row is added to ADR-0003 §6's table; no existing row's
   meaning changes.
3. ADR-0002 and ADR-0004 are cited, not amended or reopened.
4. `OccurrenceId`'s exclusion from revision identity is stated as an explicit,
   citable ruling — not left as an inference — closing the RFC-0004 PBA-0160
   gap.
5. No `src/**`, `tests/**`, Cargo, or toolchain change is proposed or implied.
6. No canonicalization ordering rule, lineage semantics, revision-token format,
   duplicate-handling rule, provenance classification, evaluator semantics,
   serialization decision, or N-24/Q-08/F-8/broader-N-25 resolution is made.

---

> ADRs are immutable once Accepted — supersede, never edit. This ADR is
> currently **PROPOSED**. On acceptance, it becomes the authoritative narrow
> supersession of ADR-0003 for this one classification question; ADR-0003
> otherwise remains fully in force. Any future material change to this ruling
> must itself be recorded through a new ADR that explicitly supersedes this one.
