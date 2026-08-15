# First Runnable Slice — Implementation Contract — Revision 4

- **Title:** First Runnable Slice — Implementation Contract
- **Contract family:** First Runnable Slice Contract
- **Revision:** 4
- **Status:** **PROPOSED** — Product Board review required. **No normative force.**
- **Predecessor:** `first-runnable-slice-contract-r03.md` — Revision 3, accepted
  under PBR-0015 (2026-08-15). Revision 3 **remains authoritative** until and
  unless the Product Board accepts this revision under a future PBR. This revision
  does not edit it.
- **Authority:** PBA-0091 (authoring). No acceptance authority allocated.
- **Author:** Principal Engineer
- **Tier:** 3 (engineering, normative once accepted)
- **Governed by:** PBD-0006 — Normative Engineering Contract Revision Convention
  (`workflow.md`); Product Constitution v1.0 — ACTIVE — RATIFIED
- **Determination basis:** authored under PBA-0091 encoding the integrated
  defect-identity design determined by the read-only cycles
  **PBA-0088 / ER-0080** (canonical construct-occurrence identity determination),
  **PBA-0089 / ER-0081** (Revision-4 authoring STOP identifying the class-level
  and COUNTERPART blockers), and **PBA-0090 / ER-0082** (integrated class-level
  and counterpart defect-identity determination — *REVISION 4 SUFFICIENT*).
- **Review history:** authored under PBA-0091. **No independent review (ARR) and
  no acceptance (PBR) has occurred.** PBD-0006 requires independent review →
  remediation if necessary → independent re-review if remediated → Product Board
  acceptance before this revision can become authoritative.

> ## This revision is PROPOSED. It has no normative force.
>
> Nothing in this file is authoritative. **Revision 3
> (`first-runnable-slice-contract-r03.md`) is the contract** until the Product
> Board accepts this revision under a future PBR. This revision changes **only**
> the explicitly enumerated surfaces in §C and **incorporates the whole of
> Revision 3 unchanged** otherwise (§D). It introduces a **canonical
> construct-occurrence identity** and migrates defect anchoring and canonical
> ordering to it, so that malformed canonical input — a construct whose lineage is
> Absent, two distinct occurrences sharing one lineage, or two otherwise-identical
> occurrences — still yields deterministic, non-colliding per-occurrence defect
> identities. It introduces **no** new construct, **no** new defect category,
> **no** new check, **no** prerequisite-graph edge, and **no** change to any check's
> defect population.

---

## A. Supersession declaration

PBD-0006 requires three things to be kept apart, and this section keeps them apart.

**Historical identity.** Revision 3 is preserved permanently as its own file,
`first-runnable-slice-contract-r03.md`, and is never substantively rewritten. This
revision does not edit it. Revision 2 (`first-runnable-slice-contract-r02.md`) and
Revision 1 (`first-runnable-slice-contract.md`) likewise remain their own immutable
files.

**Current normative authority.** **Revision 3 is the contract** until the Product
Board accepts this revision under a future PBR. Nothing in this file has normative
force while its status is PROPOSED or IN REVIEW.

**Supersession act.** Only a Product Board acceptance (a future PBR) can make this
revision authoritative and Revision 3 superseded. Authoring does not; neither does
review, an execution report, implementation usage, or Git history. This follows
PBD-0006's acceptance-authority rule.

---

## B. Normative boundary — what this revision changes

This revision changes **only enumerated surfaces**, not the whole contract. Per
PBD-0006, the unchanged surfaces are substantively identical to Revision 3's (and,
through Revision 3 §D, to Revision 2's and Revision 1's), and the authoring report
proves that by diff (Revisions 1–3 are byte-unchanged).

**The enumerated surfaces this revision changes** — all within the defect-identity
and canonical-ordering architecture of §3.3.1, §6.3, and §6.4:

1. A new **canonical construct-occurrence identity** opaque domain and its
   supplied total order (§C.1), added to the §3.3.1 opaque-domain architecture.
2. **§6.3 defect identity**: the offending-construct anchor becomes the occurrence
   identity, with explicit anchor rules for ordinary, Strategy-level, class-level,
   and pairwise defects (§C.4–§C.7).
3. **§6.4.2 discriminators**: the `COUNTERPART` payload migrates from lineage
   identity to occurrence identity, and one new kind `LINEAGE` (kind 7) is added
   (§C.7, §C.8).
4. **§6.4 canonical ordering**: key 2 orders by occurrence identity under the
   supplied occurrence order, and the totality proof is rewritten to rest on
   occurrence-identity uniqueness rather than checked lineage uniqueness (§C.9).
5. **§8 conformance**: a minimum obligation making the occurrence assignment and
   order interoperable (§C.10).
6. The **pairwise anchor wording** (§C.7): check 9's symmetric canonical selection
   moves from lineage order to occurrence order; check 11's asymmetric anchor stays
   the semantically-fixed subsumed rule (its identity representation moving from
   lineage to occurrence). No pairwise **semantic condition, population, or
   prerequisite** changes.

**Everything else is unchanged.** No check's defect **population** is altered; no
check **ownership** is reopened; §5.2 is untouched; §C.11 states this exhaustively.

---

## C. The canonical occurrence-identity amendment (substantive)

### C.1 Canonical construct-occurrence identity

Revision 3 §3.3.1 uses **five** opaque value domains, each opaque, equality-bearing,
and equipped with a deterministic total order supplied with the input. This revision
adds a **sixth**:

| # | Domain | Declared in | What its order is for |
|---|---|---|---|
| 6 | **Construct-occurrence identities** | §C.1 | Ordering only — **carries no product semantics** (§6.4 key 2; §C.9). |

**Every concrete construct occurrence in canonical Slice-1 input carries exactly
one occurrence identity** (`OccurrenceId`). Coverage is universal over every
construct that can be the subject or anchor of a mandatory defect:

- the Strategy envelope;
- every EvidenceDeclaration;
- every ConfigurationParameter;
- every Condition;
- every Direction rule;
- every CandidateDerivation.

The occurrence identity is:

- **mandatory and non-optional** — it exists for every construct occurrence,
  including a malformed one;
- **opaque** — no encoding, format, or representation is selected; in particular it
  is **not** a numeric index, collection position, source position, parser
  traversal order, or serialization order;
- **unique within one canonical Strategy input** — no two distinct construct
  occurrences share an occurrence identity;
- **caller-supplied** — part of the canonical input (§2), exactly as the values and
  their orders already are; the implementation computes none;
- **stable** for one canonical input;
- **distinct from lineage identity** (§C.2).

The canonical input also supplies a **deterministic total order over occurrence
identities**, exactly as §3.3.1 domains 1–3 supply their orders: total,
caller-supplied, and **never** derived by the implementation from token magnitude,
string representation, memory address, allocation order, hash iteration, collection
index, source position, parser traversal, or serialization order.

**Uniqueness is a canonical-input representation invariant, not a new
well-formedness check.** Canonical input construction is responsible for supplying a
valid occurrence-identity assignment and order. This revision creates **no** new
Slice-1 defect category for duplicate or missing occurrence identity, and adds
nothing to any check's population.

### C.2 Occurrence identity is not lineage identity

The two identities are semantically distinct and both persist.

**Occurrence identity** identifies one concrete occurrence in one canonical input.
It exists even when the occurrence's lineage is Absent, is duplicated, or when the
occurrence is otherwise malformed or byte-for-byte identical to another.

**Lineage identity** remains the domain-specific semantic identity of §3.3
unchanged, and continues to be the authority for, where applicable: check-2 lineage
**uniqueness** (`DUPLICATE_LINEAGE_IDENTITY`); evidence- and configuration-reference
**resolution**; lineage **equality**; the supplied **lineage order** where a rule
uses it; and the **lineage-bearing discriminator payloads** ATOM (referenced
lineage) and — now representing the *duplicated* class — LINEAGE (§C.8).

Occurrence identity is an **addressing and deterministic-conformance mechanism
only**. It does **not** replace lineage in any semantic role. In particular, this
revision does **not** migrate any semantic **reference** from lineage to occurrence
identity.

### C.3 Collection semantics are unchanged

Adding occurrence identity and a supplied occurrence order does **not** turn any
collection into a semantically ordered sequence. The accepted semantics of evidence
declarations, configuration parameters, conditions, Direction rules, and candidate
derivations (§3.4, §3.5, §3.7–§3.10, §4) are **unchanged**. The occurrence order
exists for defect identity, canonical defect ordering, and canonical tie-breaking
**only**; it never defines collection equality, never becomes precedence, never
becomes candidate role, and never promotes any incidental position — preserving the
§3.3.1 anti-incidental-position principle exactly. Because occurrence identity is an
addressing device orthogonal to semantic collection equality/order, it decides
nothing about set-versus-sequence semantics: **O-1 remains OPEN and independent**
(§C.11).

### C.4 §6.3 defect identity — generalized anchor

Revision 3 §6.3 defines the authoritative defect identity as a deterministic
function of (1) the offending construct's **lineage identity**, (2) the defect
category, (3) a discriminator, and (4) the applicable revision identity where one
is applicable. This revision changes **component (1) only**: the anchor becomes the
defect's canonical **occurrence identity**. Components (2), (3), and (4), and the
entire revision-qualification boundary of Revision 3 §6.3 (unqualified vs qualified
non-comparability, differing-revision non-comparability, caller-supplied optional
revision identity), are **unchanged**.

The anchor is defined by the defect's semantic subject:

- **Ordinary occurrence-level defect** (checks 1, 3, 4, 5, 6, 10, 13A, 14A, 15) —
  anchor = the **offending construct's** occurrence identity.
- **Strategy-level defect** (§C.6) — anchor = the **Strategy envelope's** occurrence
  identity.
- **Class-level defect** (§C.5, §C.6) — anchor = the **Strategy envelope's**
  occurrence identity, with the offending **class** carried by the discriminator.
- **Pairwise defect** (§C.7) — anchor = the pair member whose occurrence identity
  **sorts first** under the supplied occurrence order; the other member is carried
  by the COUNTERPART discriminator.

No additional subject form is introduced.

### C.5 `DUPLICATE_LINEAGE_IDENTITY` — envelope anchor + LINEAGE discriminator

The accepted check-2 population is **unchanged**:

> `DUPLICATE_LINEAGE_IDENTITY` — **one defect per duplicated lineage identity.**

Because the semantic subject is a **lineage-identity class** and not a single
occurrence, its identity under the generalized model is:

- **anchor** = the Strategy **envelope** occurrence identity;
- **category** = `DUPLICATE_LINEAGE_IDENTITY`;
- **discriminator** = `LINEAGE(LineageId)` carrying the **duplicated** lineage
  identity, ordered under the supplied §3.3.1 lineage order (§C.8).

Two distinct duplicated classes — say `L4` and `L7` — therefore produce **two
distinct defects** that share the envelope anchor and category but differ in their
LINEAGE discriminator. This preserves the one-per-duplicated-lineage population
exactly and introduces **no** per-occurrence duplication of the defect. Occurrence
order plays **no** role here; in particular no single duplicate occurrence is
designated "the offender."

### C.6 Class-level and Strategy-level anchors

**`DUPLICATE_CANDIDATE_DERIVATION`.** Population unchanged (**one per duplicated
role**); anchor = **envelope** occurrence identity; discriminator = the **existing
ROLE** kind carrying the duplicated role. This is the role-class analogue of §C.5.

**Strategy-level principle.** *A defect whose semantic subject is the Strategy as a
whole anchors on the Strategy envelope occurrence identity.* This applies to
`CANDIDATE_DIMENSION_DIVERGENCE` (14B, "one per Strategy") and to the check-8
collection-level coverage defects (`DIRECTION_COVERAGE_INCOMPLETE`,
`MULTIPLE_TOTAL_DIRECTION_RULES`). Where an accepted category is instead tied to a
specific offending **member** (for example `TOTAL_RULE_NOT_LOWEST_PRECEDENCE`, tied
to the offending always-applicable rule), that **member's** occurrence identity is
the anchor. **No defect population changes.** A Strategy-level defect with no
category-specific discriminator uses the existing **NONE** kind (§6.4.2 kind 1);
because each such category yields at most one defect per Strategy, keys 1–3 already
distinguish it and NONE is reachable-safe.

### C.7 Pairwise defects — anchor selection + COUNTERPART

The two pairwise categories carry the COUNTERPART discriminator, but their
offending-construct anchors are selected differently, because their inherited
semantic relations differ: `DIRECTION_PRECEDENCE_TIE` (check 9) is **symmetric**,
whereas `UNREACHABLE_SUBSUMED_RULE` (check 11) is **asymmetric**. Revision 4 does
**not** state a single universal pairwise anchor rule; it states one rule per
category, and in **both** the COUNTERPART payload is the **other** member's
occurrence identity (§C.8).

**Check 9 — `DIRECTION_PRECEDENCE_TIE` (symmetric).** The inherited rule anchors a
tie by a **canonical selection** over the pair — the two rules are peers and neither
is inherently the offender (§5.1 check 9, F-1). Revision 4:

1. compares the two participating rules' occurrence identities under the supplied
   occurrence order;
2. makes the rule whose occurrence identity **sorts first** the **anchor**;
3. carries the **other** rule's occurrence identity in **COUNTERPART**.

This **replaces** the Revision-3/F-1 wording that made the canonical selection by
**lineage** order — which is not total when a rule's lineage is Absent or
duplicated. The replacement is **canonical selection only**: symmetric in discovery
order (evaluating `(R_a, R_b)` or `(R_b, R_a)` yields the same anchor, identity, and
ordering position) and invariant under order-preserving relabelling of occurrence
identities. The precedence-tie semantic condition, the pair population, and the §5.2
prerequisites are unchanged.

**Check 11 — `UNREACHABLE_SUBSUMED_RULE` (asymmetric).** The inherited population and
role assignment are **fixed by semantics**, not by any canonical order: *"one defect
per **subsumed** rule, discriminated by the **subsuming** rule"* (§5.1 check 11). The
subsumed (unreachable) rule is the defective construct; the subsuming
higher-precedence rule is not. Revision 4 therefore:

1. anchors the defect on the **subsumed rule's** occurrence identity — the offending
   construct — **regardless** of how the two occurrence identities compare under the
   supplied occurrence order;
2. carries the **subsuming rule's** occurrence identity in **COUNTERPART**.

Occurrence order does **not** select the check-11 offender; it only positions the
resulting defect among others through §6.4 key 2 (the subsumed rule's occurrence).
This changes **only** the identity *representation* of the inherited check-11
semantics (subsumed-rule anchor: lineage → occurrence; subsuming-rule COUNTERPART:
lineage → occurrence). It does **not** change which rule is defective, the
one-per-subsumed-rule population, or the §5.2 prerequisites.

> **Worked check-11 example.** Rule *A* is subsumed by a single strictly
> higher-precedence rule *B*. The defect anchors on `OccurrenceId(A)` with
> COUNTERPART `OccurrenceId(B)` — **whether `OccurrenceId(B) < OccurrenceId(A)` or
> `OccurrenceId(A) < OccurrenceId(B)`**. The offending construct is always the
> subsumed rule *A*; occurrence order never promotes the subsuming rule *B* to the
> anchor.

**COUNTERPART payload migration.** Because a Direction rule's lineage absence is a
check-15 fault (Revision 3 §C.1) and check 9/11 do **not** take check 15 as a
prerequisite (§5.2), a pairwise defect is reachable while a member's lineage is
Absent, making the Revision-3 COUNTERPART payload (the counterpart rule's **lineage
identity**) unrepresentable. This revision therefore changes the COUNTERPART payload
domain from **`LineageId` to `OccurrenceId`**: COUNTERPART now identifies the other
member **occurrence**. The counterpart occurrence identity is always present, so the
defect is representable for absent-lineage and duplicate-lineage pairs alike. No
§5.2 prerequisite is added to preserve the old payload; overlap and subsumption
(§4.10) depend on **guards**, not lineage, so the analysis remains computable and no
otherwise-computable defect is suppressed.

### C.8 Discriminator model — seven kinds

Revision 3 §6.4.2 defines six discriminator kinds. This revision makes exactly two
changes and preserves everything else:

- **COUNTERPART** (kind 6) — retains its kind ordinal; **payload domain changes from
  `LineageId` to `OccurrenceId`**; compared by exact occurrence-identity equality and
  ordered by the supplied occurrence order.
- **LINEAGE** (new **kind 7**) — payload is one `LineageId`; ordered by the supplied
  §3.3.1 lineage order; used by `DUPLICATE_LINEAGE_IDENTITY` (§C.5). Assigning it the
  new **final** ordinal leaves the accepted kinds 1–6 **un-renumbered**.

The resulting discriminator model:

| Kind ordinal | Kind | Used by | Payload / comparison |
|---|---|---|---|
| 1 | NONE | any defect needing no discriminator | all equal |
| 2 | PROPERTY | checks 1, 4, 6 | §3 property position, ascending (**unchanged**) |
| 3 | ROLE | checks 13B (incl. `DUPLICATE_CANDIDATE_DERIVATION`) | `ENTRY` < `STOP_LOSS` < `TAKE_PROFIT` (**unchanged**) |
| 4 | ATOM | checks 2, 12 | (referenced **lineage**, operator, literal) (**unchanged**) |
| 5 | NODE-PATH | checks 2, 13A, 14A | derivation-root path (**unchanged**) |
| 6 | COUNTERPART | checks 9, 11 | counterpart **occurrence** identity, supplied occurrence order (**payload changed**) |
| 7 | **LINEAGE** | check 2 (`DUPLICATE_LINEAGE_IDENTITY`) | one **lineage** identity, supplied lineage order (**new**) |

NONE, PROPERTY, ROLE, ATOM, and NODE-PATH are **unchanged** in meaning and order.
ATOM's referenced-lineage payload is **not** migrated — it is a semantic reference,
not an occurrence anchor. NODE-PATH is **not** conflated with occurrence identity.

### C.9 §6.4 canonical ordering and totality

Canonical defect ordering compares in order; the first difference decides:

1. **ADR-0004 check number**, ascending;
2. **offending construct's occurrence identity**, under the supplied occurrence
   total order (§C.1);
3. **defect category**, under the §6.4.1 ordinal, ascending;
4. **discriminator**, under §6.4.2 as amended (§C.8).

The revision-qualification / non-comparability rules of §6.3 are unchanged, as is
the §6.4.1 category ordinal table and the §6.4.3 literal order.

**Totality proof (rewritten).** Revision 3's proof rested on *"lineage identities
are unique within a Strategy (check 2)."* That premise is invalid on the malformed
inputs Slice 1 exists to diagnose: uniqueness is a **checked** property that may
fail, and check 2 does not gate the checks that produce per-occurrence defects. The
new proof rests instead on the **canonical-input invariant that occurrence
identities are unique** (§C.1). For any two **distinct** conforming defects, the
keys separate them:

- **Two identical malformed siblings** (same content, absent or duplicate lineage):
  distinct occurrence identities ⇒ **key 2** differs.
- **Two duplicate-lineage classes** `L4`, `L7`: same check 2, same envelope anchor
  (key 2 tied), same category ord (key 3 tied) ⇒ **key 4** differs
  (`LINEAGE(L4)` ≠ `LINEAGE(L7)`).
- **Two duplicate candidate-role classes**: same envelope anchor, same category ⇒
  **key 4** differs (ROLE).
- **Pairwise defect on absent-lineage rules**: the anchor is the first member's
  occurrence identity and COUNTERPART is the other member's occurrence identity —
  both present ⇒ the defect is **representable**, and two such defects differ at key
  2 (distinct anchors) or, sharing an anchor, at key 4 (distinct COUNTERPART
  occurrence).
- **Pairwise defect on duplicate-lineage rules**: the members are distinct
  occurrences ⇒ distinct anchor / COUNTERPART occurrence, so identity is
  deterministic.

No two distinct conforming defects compare equal; the order is **total**. The
ordering keys use no serialization position, file offset, presentation location,
locale, host-language ordering, hash, or generated value.

### C.10 §8 conformance obligation

Add to the §8 conformance obligations the minimum rule making occurrence identity
interoperable:

> For one canonical Slice-1 input, all conforming implementations consume the
> **same** occurrence-identity assignment and the **same** supplied occurrence total
> order, and therefore agree on occurrence-identity equality and ordering, on every
> defect's occurrence anchor, on the LINEAGE and COUNTERPART discriminator orderings,
> and on the canonical defect ordering.

This strengthens, and is subsumed by, Revision 3 §8 obligation 8 (identical
verdict, defect set, defect identities, defect ordering, not-evaluated set). No
serialization, token encoding, or handle representation is prescribed; the occurrence
token remains opaque and carries no semantics (§C.1). Conforming behaviour is
invariant under order-preserving relabelling of occurrence identities, exactly as for
the existing opaque domains.

### C.11 Deliberate preservations

This revision changes defect **anchoring and canonical ordering** only. It preserves,
without change:

- **Check-2 populations**, including `DUPLICATE_LINEAGE_IDENTITY` = one per
  duplicated lineage identity, and lineage-uniqueness checking itself.
- **§5.2 prerequisite graph** — no edge added, changed, or removed; pairwise checks
  are **not** gated on check 15; no otherwise-computable defect is suppressed;
  not-evaluated semantics are unchanged.
- **Revision-3 ownership matrix** exactly: operative-node lineage absence → check 15
  exclusively; envelope semantic-model-version absence → check 16 exclusively;
  EvidenceDeclaration and ConfigurationParameter lineage absence → check 1; candidate
  derivation lineage → check 15, role/expression → check 1; no operative-lineage dual
  reporting; the §C.3 check-1 property matrix.
- **Revision qualification** — qualified vs unqualified non-comparability,
  differing-revision non-comparability, and caller-supplied optional revision
  identity, all unchanged.
- **The 26 defect categories** and their §6.4.1 ordinals; the PROPERTY, ROLE, ATOM,
  and NODE-PATH discriminator semantics; the §6.4.3 literal order; the result
  invariant and WF-1; the not-evaluated representation.
- **Open questions** — **O-1** (still gating check 6), **O-2**, **O-3** untouched;
  `EvidenceDeclaration.declared_values` set/sequence semantics unchanged. **Q-06,
  Q-08, N-22, N-24, N-25, F-8** remain OPEN.
- **Check 8** remains an **APPLICABLE** mandatory Phase-5 check — this revision
  neither removes, defers, reschedules, nor implements it.

---

## D. Incorporation of Revision 3 unchanged

Except for the enumerated surfaces in §C, this revision **incorporates the whole of
Revision 3** — and, through Revision 3 §D, the whole of Revision 2 and Revision 1 —
**unchanged and by reference**. Where §C and the incorporated text could be read to
differ on the enumerated defect-identity/ordering surfaces, §C governs (once this
revision is accepted). On **every other matter** the incorporated text governs
exactly as it does today, word for word. No construct, type rule, numeric or
dimensional semantic, guard or constraint-domain semantic (§4.10), check population,
prerequisite-graph edge (§5.2), complexity bound (§5.3), or deferral is altered by
this revision.

---

## E. What changed, and what did not

**Changed (enumerated).** A canonical construct-occurrence identity domain and its
supplied total order (§C.1); the §6.3 defect anchor from lineage identity to
occurrence identity, with explicit ordinary/Strategy/class/pairwise anchor rules
(§C.4–§C.7); the COUNTERPART discriminator payload from lineage identity to
occurrence identity (§C.7, §C.8); a new LINEAGE discriminator kind 7 (§C.8); the
§6.4 key-2 ordering domain and the rewritten totality proof (§C.9); the pairwise
anchor wording — check 9 symmetric occurrence-order selection, check 11 asymmetric
subsumed-rule anchor (§C.7); and a §8 conformance obligation (§C.10).

**Deliberately unchanged.** Every check's defect population; every check ownership
(Revision 3 §C in full); §5.2; §5.3; the 26 categories and their ordinals; the
PROPERTY/ROLE/ATOM/NODE-PATH discriminator semantics; the literal order; revision
qualification; WF-1 and the result invariant; the deferral of checks 7 and 17; and
every §3 and §4 semantic. O-1/O-2/O-3 and Q-06/Q-08/N-22/N-24/N-25/F-8 remain OPEN.

**O-A proposed disposition.** This revision's occurrence-identity semantics
normatively address the defect-anchoring gap tracked as O-A, covering **both** absent
lineage and duplicate lineage. However, while this revision remains **PROPOSED**, O-A
is **not** finally CLOSED, Revision 3 remains authoritative, and Phase 5A remains
blocked. Authoritative observation status is not changed here.

**Not decided here; not implemented here.** This revision does not accept itself,
does not reauthorize Phase-5A, and makes **no** implementation change. It states only
the **semantic consequences** a later, separately authorized remediation must honour:

- **Phase 1 (CLOSED).** The runtime model must make each construct occurrence's
  canonical occurrence identity available to mandatory-check evaluation, and the
  caller must supply the occurrence total order. The exact Rust representation (a
  stored non-optional occurrence field per construct, or a canonical invocation
  context keyed by occurrence, or a semantics-equivalent mechanism) is **not** chosen
  here; the smallest consistent implementation is selected by a bounded post-closure
  Phase-1 remediation.
- **Phase 4 (CLOSED).** Defect identity and canonical ordering must use occurrence
  identity as the offending-construct anchor. The expected bounded result-model
  remediation is `DefectCore.offending: LineageId → OccurrenceId`, plus the new
  `LINEAGE` discriminator, the COUNTERPART payload migration to occurrence identity,
  the discriminator-ordering update, first-occurrence pairwise selection, and the
  corresponding tests — or a semantics-equivalent representation. **No Rust source is
  changed by this revision.**
