# First Runnable Slice — Implementation Contract — Revision 5

- **Title:** First Runnable Slice — Implementation Contract
- **Contract family:** First Runnable Slice Contract
- **Revision:** 5
- **Status:** **ACCEPTED** — normative.
- **Predecessor:** `first-runnable-slice-contract-r04.md` — Revision 4, accepted
  under **PBR-0016** (2026-08-16); **SUPERSEDED** by this revision on acceptance
  (2026-08-18) in governance provenance, its file preserved immutable.
- **Authority:** PBA-0139 (authoring); PBA-0142 (acceptance).
- **Author:** Principal Engineer
- **Tier:** 3 (engineering, normative)
- **Governed by:** PBD-0006 — Normative Engineering Contract Revision Convention
  (`docs/engineering/workflow.md`); Product Constitution v1.0 — ACTIVE — RATIFIED
- **Determination basis:** authored under PBA-0139, encoding the unit-identity
  design determined by the read-only cycle **PBA-0138 / ER-0113** (Phase-6
  `NotEvaluatedRecord` unit-identity determination), itself responding to the
  gap the independent read-only review of Phase-6 Slice 6B identified and an
  adversarial reconciliation confirmed against §6.2's unconditional text. That
  review was strictly read-only and made no repository mutation; no `ARR`
  identifier was allocated for it.
- **Review history:** authored under PBA-0139 / ER-0114 → bounded
  review-remediation PBA-0140 / ER-0115 → formal independent review PBA-0141 /
  **ARR-0047** (PASS — zero BLOCKER/MAJOR/MINOR/NOTE).
- **Accepted by:** Product Board, **PBR-0017**
- **Decision date:** 2026-08-18

> ## This revision is ACCEPTED and is the authoritative First Runnable Slice
> ## Contract for the `NotEvaluatedRecord` unit-identity surface.
>
> Accepted by the Product Board under **PBR-0017** on 2026-08-18 (acceptance
> authorization PBA-0142), following the review path PBD-0006 requires
> (PBA-0141 / ARR-0047 PASS, zero findings). It **supersedes Revision 4**,
> whose file (`first-runnable-slice-contract-r04.md`) remains an immutable
> historical artifact. This revision changes **only** the explicitly
> enumerated surface in §A and **incorporates the whole of Revision 4
> unchanged** otherwise (§B).

---

## A. Amendment — `NotEvaluatedRecord` unit identity (§6.2)

### A.1 Problem

Revision 4 (§C.4–§C.7) migrated the **defect** anchor (`DefectCore.offending`)
from lineage identity to occurrence identity specifically because a
construct's lineage may be `Absent` or duplicated. §C.11 of that revision
explicitly and deliberately left **"the not-evaluated representation"**
unchanged, so `NotEvaluatedRecord.construct` remained typed as `LineageId`.

§6.2 requires, unconditionally: "the **not-evaluated set** — one deterministic
record per suppressed check evaluation, each naming the check, the construct,
and the blocking defect identity." No accepted text carves out an exception
for a suppressed evaluation whose construct's lineage happens to be `Absent`.

A reachable state exists where this cannot be satisfied under the Revision-4
shape: `Strategy.lineage == Declared::Absent` (the envelope's own lineage,
independently and separately checked by Check 15) while a Strategy-level
check — 13B or 14B — is suppressed by an unrelated §5.2 prerequisite failure.
No `§5.2` edge ties Check 15 to 13B/14B (§C.11: "not gated on check 15"), so
both states are simultaneously reachable, and no `LineageId` exists to name
the suppressed unit.

A single-field replacement with `OccurrenceId` alone was evaluated and found
insufficient for the two **pairwise**-gated units §5.2 defines (check 9;
check 11's subsumption part) — a bare occurrence field cannot distinguish
"pair (A,B) suppressed" from "pair (A,C) suppressed" when both share the
same first-sorted member.

### A.2 Amended `NotEvaluatedRecord` shape

```rust
NotEvaluatedRecord {
    check: CheckNumber,
    construct: OccurrenceId,
    counterpart: Option<OccurrenceId>,
    blocking: DefectIdentity,
}
```

`LineageId` is no longer used for `NotEvaluatedRecord`'s unit identity.

### A.3 Normative semantics

**Single-unit suppressed evaluation** — every check whose §5.2 unit of work is
one construct or the Strategy as a whole (checks 2 (reference/atom), 3, 4, 5,
6, 8, 10, 11 (contradiction), 12, 13A, 13B, 14A, 14B):

- `construct` = that unit's own canonical occurrence identity (§C.1 —
  mandatory, non-optional, unique within one canonical Strategy input, so
  never subject to the absence/duplication problem lineage identity has).
- `counterpart` = `None`.

**Pairwise suppressed evaluation** — checks 9 and 11's subsumption part, whose
§5.2 unit of work is an unordered pair of Direction rules:

- Reuses §C.7's existing, unmodified canonical pair-orientation rule exactly:
  the pair member whose occurrence identity **sorts first** under the
  supplied occurrence order is `construct`; the other member's occurrence
  identity is `counterpart = Some(...)`.
- A pair `(A, B)` and the same pair discovered as `(B, A)` normalize to the
  identical `(construct, counterpart)` value — no new symmetry rule is
  introduced beyond the one §C.7 already establishes for `DefectCore`'s own
  `Counterpart` discriminator payload.

**Explicitly unchanged:**

- **Cardinality.** Exactly one `NotEvaluatedRecord` per suppressed evaluation
  (§6.2), restored rather than weakened by this amendment.
- **Blocker semantics.** `blocking: DefectIdentity` and its meaning are
  unchanged. Unit identity (`construct`/`counterpart`) and blocker identity
  remain independent fields; PBA-0134 / ER-0109 **Ruling A** (transitive
  root-blocker propagation, canonical-minimum selection under the existing
  §6.4 `DefectIdentity`/`DefectCore` order) is unchanged and requires no
  redesign.
- **Set semantics.** PBA-0134 / ER-0109 **Ruling B** (the not-evaluated
  collection is set-valued; no canonical order is defined or introduced for
  `NotEvaluatedRecord`) is unchanged.
- **No synthetic or sentinel identity** is introduced anywhere by this
  amendment (§C.1/§C.10 opacity preserved).
- **No `Discriminator` reuse.** `Discriminator`'s seven kinds (§6.4.2) encode
  defect-category-specific payloads (property position, role, atom, node-path,
  counterpart-for-a-defect, lineage) with no applicability to naming a
  suppressed *check evaluation's* unit — `counterpart: Option<OccurrenceId>`
  is the minimal sufficient representation and imports no unrelated semantics.
- **No new ordering rule.** Occurrence-identity comparison reuses the existing
  supplied occurrence total order (§C.1, §C.9) verbatim.

### A.4 Adversarial cardinality proof (informative)

For an occurrence order `A < B < C`, three pairs `(A,B)`, `(A,C)`, `(B,C)`
under this amendment yield the distinct triples `(A, Some(B))`,
`(A, Some(C))`, `(B, Some(C))` — no collision. `(A,B)` and `(B,A)` both
normalize to `(A, Some(B))` — one record, matching the unordered-pair
semantics §5.2 already assigns to checks 9 and 11's subsumption part.

---

## B. What changed, and what did not

**Changed (enumerated).** `NotEvaluatedRecord`'s unit-identity fields only:
`construct: LineageId → OccurrenceId`, plus a new `counterpart:
Option<OccurrenceId>` field, with the normative semantics of §A.3.

**Deliberately unchanged — the whole of Revision 4 otherwise, incorporated in
full**, including without limitation: every check's defect population and
ownership; §5.1, §5.2, §5.3 in full; the 26 defect categories and their
ordinals; `DefectCore`'s occurrence-anchor migration and pairwise anchor rules
(§C.1–§C.9 of Revision 4); the PROPERTY, ROLE, ATOM, NODE-PATH, and
COUNTERPART discriminator semantics; the §6.4.3 literal order; revision
qualification (§6.3); `WF-1` and the result invariant; the deferral of checks
7 and 17; every §3 and §4 semantic. O-1/O-2/O-3, O-4, and Q-06/Q-08/N-22/N-24/
N-25/F-8 remain OPEN, untouched by this proposal.

**Accepted; not yet implemented.** This revision is accepted and normative for
the §A surface (PBR-0017), but makes **no implementation change itself**.
`src/result/**` remains unmodified until a separately authorized bounded
remediation follows this acceptance.
