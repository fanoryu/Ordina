# First Runnable Slice — Implementation Contract — Revision 3

- **Title:** First Runnable Slice — Implementation Contract
- **Contract family:** First Runnable Slice Contract
- **Revision:** 3
- **Status:** **PROPOSED** — Product Board review required. **No normative force.**
- **Predecessor:** `first-runnable-slice-contract-r02.md` — Revision 2, accepted
  under PBR-0014 (2026-08-14). Revision 2 remains the authoritative contract until
  and unless this revision is accepted; on acceptance it becomes **SUPERSEDED** in
  governance provenance, its file preserved immutable.
- **Authority:** PBA-0082 (authoring). Acceptance authority is a future PBR, not
  yet issued.
- **Author:** Principal Engineer
- **Tier:** 3 (engineering, normative once accepted)
- **Governed by:** PBD-0006 — Normative Engineering Contract Revision Convention
  (`workflow.md`); Product Constitution v1.0 — ACTIVE — RATIFIED
- **Review history:** authored under PBA-0082 following the ER-0074 stop and the
  ER-0075 clarification review; independent review not yet performed.
- **Accepted by:** *(not accepted)*
- **Decision date:** *(not accepted)*

> ## This revision is PROPOSED and carries NO normative force.
>
> Revision 2 (`first-runnable-slice-contract-r02.md`) remains the authoritative
> First Runnable Slice Contract. This revision changes **only** the explicitly
> enumerated surfaces in §C and **incorporates the whole of Revision 2 unchanged**
> otherwise (§D). Its sole substantive purpose is to make the **check-1
> required-property ownership** — and its boundary with checks 15 and 16 —
> explicit, resolving the ambiguity ER-0075 identified.

---

## A. Supersession declaration

PBD-0006 requires three things to be kept apart, and this section keeps them
apart.

**Historical identity.** Revision 2 is preserved permanently as its own file,
`first-runnable-slice-contract-r02.md`, and is never substantively rewritten.
This revision does not edit it. Revision 1 (`first-runnable-slice-contract.md`)
likewise remains its own immutable file.

**Current normative authority.** **Revision 2 is the contract** until the Product
Board accepts this revision under a future PBR. Nothing in this file has normative
force while its status is PROPOSED or IN REVIEW.

**Supersession act.** Only a Product Board acceptance (a future PBR) can make this
revision authoritative and Revision 2 superseded. Authoring does not; neither does
review, an execution report, implementation usage, or Git history. This follows
PBD-0006's acceptance-authority rule.

---

## B. Normative boundary — what this revision changes

This revision changes **only enumerated surfaces**, not the whole contract. Per
PBD-0006, the unchanged surfaces are substantively identical to Revision 2's, and
the authoring report proves that by diff (Revision 2's file is byte-unchanged).

**The enumerated surfaces this revision clarifies** — all within the check-1 /
check-15 / check-16 required-property ownership boundary of §5.1, and the §3
property tables that feed the §6.4.2 PROPERTY discriminator:

1. Ownership of **operative-node lineage-identity absence** (§C.1).
2. Ownership of **envelope semantic-model-version absence** (§C.2, codification).
3. The **complete check-1 required-property matrix** for every construct kind
   (§C.3).
4. A narrow **correction to the §3.8 candidate-derivation** text (§C.4).
5. Preservation of the **presence-versus-validity** boundary (§C.5).

**Everything else is unchanged.** No other check's normative population is
altered; §C.6 states this exhaustively. This revision resolves an ownership
ambiguity; it introduces no new construct, no new defect category, no new check,
and no new deferral.

---

## C. The check-1 ownership clarification (substantive)

### C.1 Operative-node lineage absence is exclusively a check-15 fault

For **every operative node** — the Strategy envelope, a condition, a Direction
rule, and a candidate derivation (§3.3) — a **missing lineage identity** produces
**exactly one** defect:

> `MISSING_OPERATIVE_PROVENANCE`, under **check 15** (§5.1 check 15).

It does **not** additionally produce `MISSING_REQUIRED_PROPERTY` under check 1.
**There is no dual reporting for operative-node lineage absence.** Check 15 is the
**exclusive** owner of operative-node lineage-identity absence.

This resolves the ER-0075 ambiguity: where Revision 2's §3.8 table marked
candidate-derivation lineage "Unconditional (check 1)" while §3.4/§3.7/§3.10 did
not mark their operative-node lineage at all, and check 15 uniformly claimed
operative-node lineage, this revision settles the boundary in favour of check 15
uniformly. It changes **no** other lineage rule: lineage remains opaque (§3.3),
supports exact equality, carries a supplied deterministic total order used only by
§6.4, and its **uniqueness** remains a **check 2** concern (`DUPLICATE_LINEAGE_IDENTITY`)
— none of which this revision touches.

Note that an **evidence declaration is not an operative node** (§3.3). Its lineage
absence is therefore **not** governed by this rule; it remains a check-1
`MISSING_REQUIRED_PROPERTY` (§C.3, evidence declaration position 1), because §3.5
marks it "Unconditional (check 1)" and no check-15 ownership applies.

### C.2 Envelope semantic-model-version absence is exclusively a check-16 fault (codification)

An **absent** envelope `semantic-model version` produces **exactly one** defect:

> `UNSUPPORTED_SEMANTIC_MODEL_VERSION`, under **check 16** (§5.1 check 16, §3.4.1).

It does **not** produce a check-1 `MISSING_REQUIRED_PROPERTY`. Check 1 does not
report a missing semantic-model version.

This is **codification of already-accepted text**, not a new choice: Revision 2
§3.4 already states of that property "Any other value, **or absence**, is a check
16 defect (`UNSUPPORTED_SEMANTIC_MODEL_VERSION`)," and §5.1 check 16 already names
"absent or unsupported version." This revision only makes the check-1 exclusion
explicit so no implementer infers a second defect. `SM-1` is unchanged; **N-24
remains OPEN**.

### C.3 The authoritative check-1 required-property matrix

For each construct kind, the table below states every §3 property, its §3
top-to-bottom table position (the §6.4.2 PROPERTY-discriminator position), and the
mandatory check that owns the property's **absence**. **Check 1 emits
`MISSING_REQUIRED_PROPERTY` for absence only where the "Check 1" column says
YES.** Positions are the §3 table order and are preserved from Revision 2; no
property is renumbered.

Phase-5 check 1 requires no inference from any implementation representation
(`Declared<T>`, `DeclaredCategory<T>`, field order, or source comments): this
matrix is authoritative.

**Strategy envelope (§3.4)**

| Pos | Property | Absence owned by | Check 1 emits? |
|---|---|---|---|
| 1 | lineage identity | check 15 | **No** (§C.1) |
| 2 | semantic-model version | check 16 | **No** (§C.2) |
| 3 | evidence declarations | check 1 | **Yes** |
| 4 | configuration parameters | check 1 | **Yes** |
| 5 | conditions | check 1 | **Yes** |
| 6 | Direction rules | check 1 | **Yes** |
| 7 | candidate derivations | check 1 | **Yes** |

A **present-but-empty** collection is a different state from an **absent** one:
"zero Direction rules" is a **check 8** condition and role completeness of present
derivations is **check 13B** — neither is check 1. Check 1 owns only the
collection's **absence**.

**Evidence declaration (§3.5)** — *not an operative node.*

| Pos | Property | Absence owned by | Check 1 emits? |
|---|---|---|---|
| 1 | lineage identity | check 1 | **Yes** |
| 2 | declared type | check 1 | **Yes** (absence only) |
| 3 | declared dimension | check 6 | **No** |
| 4 | declared value set | check 6 | **No** |

Declared-type **absence** is check 1; a present declared type that is
out-of-domain (or multiple) is **check 6** (§5.1.1). The type-dependent dimension
and value set — presence, absence, and content — are **check 6 exclusively**
(§3.5 M-1), unchanged.

**Direction rule (§3.7)** — *operative node.*

| Pos | Property | Absence owned by | Check 1 emits? |
|---|---|---|---|
| 1 | lineage identity | check 15 | **No** (§C.1) |
| 2 | precedence ordinal | check 1 | **Yes** |
| 3 | guard | check 1 | **Yes** |
| 4 | outcome | check 10 | **No** |

Outcome zero/multiple/out-of-domain is **check 10** (`INVALID_DIRECTION_OUTCOME`),
unchanged.

**Candidate derivation (§3.8)** — *operative node.*

| Pos | Property | Absence owned by | Check 1 emits? |
|---|---|---|---|
| 1 | lineage identity | check 15 | **No** (§C.1, §C.4) |
| 2 | candidate role | check 1 | **Yes** |
| 3 | expression | check 1 | **Yes** |

**Configuration parameter (§3.9)** — *not an operative node.*

| Pos | Property | Absence owned by | Check 1 emits? |
|---|---|---|---|
| 1 | lineage identity | check 1 | **Yes** |
| 2 | declared type | — (NUMERIC-only, structural; absence unrepresentable) | **No** |
| 3 | declared dimension | check 1 | **Yes** |
| 4 | declared lower bound | check 1 | **Yes** |
| 5 | declared upper bound | check 1 | **Yes** |
| 6 | adjustable flag | check 1 | **Yes** |

A configuration parameter is not an operative node (§3.3), so its lineage absence
is check 1's, not check 15's. Declared type is NUMERIC-only and structurally
implied (§3.9); its absence is not a representable state and manufactures no
check-1 population. A **present-but-invalid** bound (inverted, or wrong dimension)
is **check 4**, unchanged.

**Condition (§3.10)** — *operative node.*

| Pos | Property | Absence owned by | Check 1 emits? |
|---|---|---|---|
| 1 | lineage identity | check 15 | **No** (§C.1) |
| 2 | classification | check 3 | **No** |
| 3 | guard | check 1 | **Yes** |

Classification zero/multiple/out-of-domain is **check 3**
(`INVALID_CONDITION_CLASSIFICATION`), unchanged.

### C.4 Candidate-derivation correction (narrow)

Revision 2 §3.8 marks candidate-derivation lineage identity "Unconditional (check
1)" and includes an ordering example stating that a candidate derivation missing
**both** its lineage identity and its expression yields "the two **check-1
defects**," separated by the PROPERTY discriminator "position 1 before position
3." Under §C.1 that example is corrected:

> A candidate derivation missing **both** its lineage identity and its expression
> yields **two** defects: one **check-15** `MISSING_OPERATIVE_PROVENANCE` for the
> absent lineage identity, and one **check-1** `MISSING_REQUIRED_PROPERTY` for the
> absent expression (§3.8 property position 3). The lineage absence does **not**
> also produce a check-1 defect.

The two defects remain fully ordered under §6.4, now separated by **key 1 (ADR-0004
check number)** — check 1 before check 15 — rather than by the PROPERTY
discriminator. The §3.8 property **positions are unchanged**: lineage identity
remains position 1, role position 2, expression position 3; only the **ownership**
of lineage absence moves from check 1 to check 15. No unrelated property is
renumbered. This corrects **only** the affected §3.8 marking and its ordering
example; every other statement of §3.8 (the expression grammar, the three roles,
role and expression check-1 ownership) is unchanged.

### C.5 Presence versus validity is preserved

Check 1 detects the **absence** of properties assigned to it (the "Yes" rows of
§C.3). It does **not** absorb a **present-but-invalid** state owned by another
mandatory check. This revision preserves, without change:

- evidence declared-type **absent** → check 1; **out-of-domain / multiple** → check 6;
- evidence dimension / value set (presence, absence, content) → check 6;
- condition classification zero / multiple / out-of-domain → check 3;
- Direction outcome zero / multiple / out-of-domain → check 10;
- semantic-model version absent / unsupported → check 16;
- configuration bounds present-but-invalid (inverted, dimension mismatch) → check 4.

Check 1 is not broadened into a general property-validity check.

### C.6 Explicit ownership marking

For the constructs whose Revision-2 §3 tables carried no explicit check-ownership
column (§3.4 envelope, §3.7 Direction rule, §3.9 configuration parameter, §3.10
condition), the §C.3 matrix **is** the explicit marking: future implementers read
check-1 ownership from §C.3, not from any implementation artifact. This revision
adds no new column to the preserved Revision-2 tables and reformats nothing;
§C.3 supplies the authoritative ownership that §6.4.2 presumes.

---

## D. Incorporation of Revision 2 unchanged

Except for the enumerated surfaces in §C, this revision **incorporates the whole
of Revision 2** — and, through Revision 2 §D, the whole of Revision 1 §§0–11 — 
**unchanged and by reference**. Where §C and the incorporated text could be read
to differ **on the enumerated check-1/15/16 ownership surfaces**, §C governs (once
this revision is accepted). On **every other matter** the incorporated Revision-2
text governs exactly as it does today, word for word.

No construct, type rule, numeric or dimensional semantic, guard or constraint-domain
semantic (§4.10), result-contract rule (§6, including the 26 defect categories,
the 6 discriminator kinds, the literal order, defect identity, and canonical
ordering), prerequisite-graph edge (§5.2), complexity bound (§5.3), or deferral
(checks 7 and 17; §3.1) is altered by this revision.

---

## E. What changed, and what did not

**Changed (enumerated).** The check-1 required-property ownership boundary with
checks 15 and 16, made explicit as the §C.3 matrix; the exclusive assignment of
operative-node lineage absence to check 15 (§C.1); the codified exclusive
assignment of semantic-model-version absence to check 16 (§C.2); and the narrow
§3.8 candidate-derivation correction (§C.4).

**Deliberately unchanged.** Check 2 (including lineage uniqueness), checks 3, 4,
5, 6, 8, 9, 10, 11, 12, 13A/13B, 14A/14B; check 15 apart from its stated
operative-lineage exclusivity; check 16 apart from its stated version-absence
exclusivity; the deferral of checks 7 and 17; §6 in full; §5.2/§5.3; and every §3
and §4 semantic. **Check 8 remains an APPLICABLE mandatory Phase-5 check** (§5.1
check 8) — this revision neither removes, defers, nor reschedules it. The open
questions **O-1** (still gating check 6), **O-2**, and **O-3** are untouched, as
are **Q-06, Q-08, N-22, N-24, N-25, F-8**.

**Not decided here.** This revision does not accept itself, does not reauthorize
Phase-5A check 1, and does not correct the Phase-1 model documentation comments
that become stale under this ruling. The complete model-comment currency
inventory (ER-0075, ARR-0026) is:

- **`src/model/strategy.rs`** — the envelope description that has check 1 report a
  missing lineage identity (with check 15 "additionally"), and that reads "one
  defect per missing property" across all seven envelope properties. Under §C.1
  and §C.2 an absent envelope lineage is check 15 only and an absent version is
  check 16 only; check 1 reports neither.
- **`src/model/condition.rs`** — the condition description stating an absent
  lineage identity is "check 15's `MISSING_OPERATIVE_PROVENANCE` **and** check
  1's `MISSING_REQUIRED_PROPERTY`". Under §C.1 it is check 15 only.
- **`src/model/derivation.rs`** — the `CandidateDerivation` doc-comment describing
  a derivation missing both its lineage identity and its expression as yielding
  "**the two check-1 defects** … separated by the PROPERTY discriminator," and the
  related statement that check 1 owns candidate-derivation lineage absence. Under
  §C.1 and §C.4 the lineage absence is a **check-15** defect and only the
  expression absence is a **check-1** defect; the two are separated by §6.4 key 1
  (check number), not the PROPERTY discriminator.

By contrast, `src/model/direction.rs` (rule lineage → check 15; check 1 =
precedence and guard), `src/model/evidence.rs`, and `src/model/config.rs` already
match this ruling and require no correction. These currency items are bounded,
**documentation-comment-only** fixes for a separate authorized cycle **after
acceptance**; **no executable Phase-1 model semantic change** is implied or
authorized.
