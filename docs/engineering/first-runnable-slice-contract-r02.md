# First Runnable Slice — Implementation Contract — Revision 2

- **Title:** First Runnable Slice — Implementation Contract
- **Contract family:** First Runnable Slice Contract
- **Revision:** 2
- **Status:** **ACCEPTED** — normative
- **Predecessor:** `first-runnable-slice-contract.md` — Revision 1, accepted under
  PBR-0011 (2026-08-13); **SUPERSEDED** by this revision on acceptance (its file
  remains an immutable historical artifact)
- **Authority:** PBA-0059 (authoring); PBA-0063 (acceptance)
- **Author:** Principal Engineer
- **Tier:** 3 (engineering, normative)
- **Governed by:** PBD-0006 — Normative Engineering Contract Revision Convention
  (`workflow.md`); Product Constitution v1.0 — ACTIVE — RATIFIED
- **Review history:** ARR-0019 (PASS WITH MINOR REMEDIATION REQUIRED — M-1, M-2)
  → remediation ER-0061 → ARR-0020 (PASS — READY FOR PRODUCT BOARD ACCEPTANCE)
- **Accepted by:** Product Board, **PBR-0014**
- **Decision date:** 2026-08-14

> ## This revision is ACCEPTED and is the authoritative First Runnable Slice Contract.
>
> Accepted by the Product Board under **PBR-0014** on 2026-08-14, following the
> review path PBD-0006 requires. It supersedes Revision 1, whose file
> (`first-runnable-slice-contract.md`) remains an immutable historical artifact
> and is not edited to record the supersession — the relationship is recorded
> here and in the identifier ledger, per PBD-0006.
>
> **Normative boundary (ARR-0020 B-2).** The substantive checker-contract content
> this acceptance makes normative is **§C's §2.1** — the addition to the accepted
> Revision-1 §2 — together with the whole of Revision 1 §§0–11 as incorporated
> unchanged by §D. The Revision-2 wrapper sections **§A, §B, §E, §F and §G are
> governance and provenance material**, not checker-contract semantics: they
> record how this revision came to be and what it changed, and bind no
> implementation.

---

## A. Supersession declaration

PBD-0006 requires three things to be kept apart, and this section keeps them
apart.

**Historical identity.** Revision 1 is preserved permanently as its own file,
`first-runnable-slice-contract.md`, and is never substantively rewritten. This
revision does not edit it, and does not mark it superseded. Its acceptance record
(PBR-0011), its review history, and its substantive text stand exactly as
accepted.

**Current normative authority.** **This revision is now the contract.** It was
accepted by the Product Board under PBR-0014 on 2026-08-14, and is authoritative
for all Slice-1 behaviour. Revision 1 governed until that acceptance; where this
document and Revision 1 differ, this revision now governs.

**Supersession act.** Product Board acceptance recorded through **PBR-0014** made
this revision authoritative and Revision 1 superseded. No lesser act did so — not
authoring, not review, not an execution report, not implementation usage, not
convention-by-practice, and not Git history. This followed PBD-0006's
acceptance-authority rule and Revision 1 §11.3's requirement that future material
changes proceed through "an explicitly authorized successor or revision — never a
silent rewrite".

**Form of supersession.** This revision supersedes Revision 1 **only in the
enumerated surface below**, incorporating the whole remainder of Revision 1
unchanged. It is not a full replacement.

---

## B. Revision scope

**Changed surface — exactly one:**

> **§2 — A3 Direct Canonical Input Boundary**, which gains one new subsection,
> **§2.1 Distinguished members of opaque domains**.

**Unchanged surface — everything else**, incorporated by reference and
substantively identical to Revision 1:

§0 (reading order and precedence) · §1 (Slice-1 scope) · §2's existing text ·
§3 (admitted construct inventory) · §4 (Slice-1 Canonical Semantic Contract) ·
§5 (mandatory Well-Formedness checks) · §6 (result contract) · §7 (explicit
deferrals) · §8 (conformance obligations) · §9 (compliance statement) · §10
(Board dispositions of record) · §11 (acceptance record, which on acceptance
would be superseded by this revision's own acceptance record).

**No existing sentence of Revision 1 is deleted, rewritten, reordered, or
reinterpreted by this revision.** §2.1 is purely additive.

Revision 1 §11.2 fixes the substantive text of §§3–8 as immutable. This revision
respects that boundary exactly: it changes nothing in §§3–8, not even a
cross-reference. §2.1 is drafted to be **self-applying** — it quantifies over
"every member of an opaque domain that this contract names as a distinguished
semantic constant", so it reaches §3.4.1's `SM-1` and §4.6's `DIMENSIONLESS`
without either section needing a word added to it.

---

## C. The normative addition

The following subsection is added to §2, immediately after the existing
**Input-domain precondition** paragraph.

---

### 2.1 Distinguished members of opaque domains

**The gap this closes.** §3.3.1 makes an opaque domain's **values** and its
**deterministic total order** constitutive of the already-canonical input, and
the m-2 paragraph above says so in exactly those two terms. But this contract
also *names* particular members of two of those domains as **distinguished
semantic constants** — `DIMENSIONLESS` (§4.6) and `SM-1` (§3.4.1) — and mandatory
checks depend on recognizing them: check 14A must decide whether a `*` node's
scalar is dimensionless, and check 16 must compare a declared semantic-model
version against `SM-1`.

Knowing a domain's values and their order does not say **which** value is the
distinguished one. These domains are opaque, so no encoding is available to read;
equality is the only operation §4.6 admits over dimension symbols and the only
one §3.3.1 admits over version tokens; and the orders on domains 1–3 are declared
to carry **no semantics**, so position within them identifies nothing. A
conforming checker therefore could not identify a distinguished member from the
input as previously defined. This subsection supplies the missing constitutive
element, and nothing else.

**The rule.**

> For every member of an opaque domain that this contract names as a
> **distinguished semantic constant**, the already-canonical input additionally
> supplies an **authoritative binding** from that named constant to **exactly
> one** admitted value of the corresponding domain. The binding is
> **constitutive of the canonical input**, exactly as the domain's values and its
> total order are.

Where an opaque domain has distinguished members, its canonical semantic input
therefore consists of:

1. the domain's admitted opaque values;
2. the contract-valid deterministic total order over them, **where this contract
   requires that domain to carry one**; and
3. the authoritative distinguished-member bindings this contract requires for
   that domain.

As with the order, **no serialization or encoding of a binding is defined or
implied** — its presence is a semantic property of the input domain, not a
representation choice.

**The checker consumes the binding; it never produces one.** A conforming
implementation must **not** identify a distinguished member by:

- manufacturing an identity, or creating one when the binding is absent;
- inferring it from a name, spelling, label, or display form — in particular, a
  dimension symbol is **not** dimensionless because something calls it
  `"DIMENSIONLESS"`;
- inferring it from lexical form or lexical order;
- inferring it from declaration order, collection position, or source position;
- inferring it from host-language representation, an integer encoding, an enum
  discriminant, a hash, or any other incidental representation;
- inferring it from first, last, or any other position in the supplied total
  order;
- selecting it through a default, sentinel, or fallback convention.

The identity used by every semantic rule in this contract is **the identity the
canonical input supplies**, and no other.

**The distinguished members Slice 1 requires.** The table below is **normative**,
and is the **exhaustive and definitive enumeration** of the distinguished
semantic members Slice 1 requires a binding for. It is **not** illustrative and
**not** a convenience summary: appearing in this table is what places a constant
within the binding rule above, and a constant Slice 1 requires a binding for
appears here and nowhere else. It lists exactly the contract-declared
distinguished members Slice 1 currently requires — no more.

Membership is decided by this table, **not** by the wording each declaring
section happens to use. A constant listed here falls within the rule **however
its declaring section designates it** — whether as a *distinguished* symbol
(§4.6), a constant *declared by this contract* (§3.4.1), a *contract-declared
constant* (check 16), or by any other wording by which the contract itself
uniquely names the semantic constant. The incidental vocabulary of the declaring
section neither adds a constant to this rule nor removes one from it. In
particular, `SM-1` is within the rule on the same footing as `DIMENSIONLESS`,
notwithstanding that §3.4.1 does not use the word "distinguished".

| Distinguished constant | Domain (§3.3.1) | Declared in | Consumed by |
|---|---|---|---|
| `DIMENSIONLESS` | dimension symbols (domain 2) | §4.6 | §4.6's scaling rule; §3.8's `expr * dimensionless-literal`; check 14A |
| `SM-1` | version tokens (domain 4) | §3.4.1 | check 16 |

For `DIMENSIONLESS`: the bound value is an admitted **dimension symbol**, drawn
from the same domain as every other dimension in the input, and equality against
it is the exact `DimensionSymbol` equality §4.6 already requires. The
implementation does not choose which dimension is the dimensionless one. Its
position in the supplied total order is whatever that order assigns and carries
**no** additional meaning (§3.3.1, domain 2 — "ordering only").

For `SM-1`: the bound value is an admitted **version token**, and comparison with
a Strategy's declared semantic-model version remains **exact equality** (§3.4.1).
This introduces **no** order over version tokens; §3.3.1 gives that domain
"neither ordering nor semantics — equality only", and that is unchanged.

**Validity of the binding set.** A supplied binding set is valid only when:

1. every distinguished member this contract requires has **exactly one** binding;
2. each bound value is an **admitted member** of the corresponding domain;
3. no required binding is absent;
4. no required binding is duplicated or conflicting;
5. two distinct distinguished constants do not bind to the same opaque value
   unless this contract explicitly declares them equivalent — and it declares no
   such equivalence.

**A malformed binding set is a canonical-input precondition failure, not a
defect.** If any condition above fails, the supplied object is **not canonical
Slice-1 semantic input**, and therefore lies outside the domain of the
authoritative `WF-1` checker. It produces no verdict, because there is nothing
well-formed enough to reach a verdict about.

**This does not weaken the checker's totality.** §1.1 scopes totality to "every
input in the **admitted structural domain**", and §8 obligation 6 incorporates
§1.1 by reference; an object that fails a canonical-input precondition is
**outside** that admitted domain, exactly as a shape violation under §2's
input-domain precondition is. Excluding such an object from the authoritative
`WF-1` checker therefore contradicts neither §1.1's totality nor obligation 6 —
totality was never a claim over arbitrary objects, only over admitted canonical
input, and this binding requirement is one more condition on what makes an object
admitted.

This is the treatment the contract already gives the other constitutive elements
of the input domain: no defect category exists for a supplied order that is not a
deterministic total order either, and none is invented here. Accordingly the
checker must **not**:

- manufacture a missing binding;
- choose between conflicting bindings;
- repair an out-of-domain binding;
- fall back to an implementation default;
- report a newly invented defect category — **§6.4.1's twenty-six categories
  remain complete and unchanged**;
- reinterpret a malformed binding as `UNKNOWN_CONSTRUCT` under check 16, which
  concerns unknown **construct kinds and undeclared properties** (§3.4, §4.7),
  not domain bindings;
- treat it as any other Well-Formedness defect.

**What this subsection does not do.** It selects no representation, encoding,
serialization, schema, parser, or persistence for a binding or for any bound
value. It adds no construct to §3, no rule to §4, no check to §5, and no category
to §6.4.1. It changes no existing semantic rule — it supplies the identity
information those rules already presume. It is not ambient implementation
configuration: a binding arrives **with the input**, per input, and never from a
compiled-in constant, an environment, a file, or a process-global.

---

## D. Everything else, incorporated unchanged

Every section of Revision 1 other than §2 is incorporated into this revision
**verbatim and unchanged**, and §2 is incorporated unchanged apart from the
addition of §2.1 above.

**No substantive surface of §§3–8 is altered.** In particular, and without
limiting the general statement: the admitted construct inventory and the five
opaque domains (§3); the type, numeric, equality, comparison, guard, dimensional,
reference, classification, precedence, overlap, configuration-bound and
derivation semantics (§4); the fifteen applicable mandatory checks, their
deterministic PASS and defect conditions, the check-6 property mapping and the
prerequisite graph (§5); the verdict, result contents, checker-semantics version
`WF-1`, defect identity, the twenty-six defect categories and the canonical
defect ordering (§6); the explicit deferrals (§7); and the conformance
obligations (§8) all stand exactly as accepted.

The precedence rule of §0 is likewise unchanged: where this contract conflicts
with ADR-0002, ADR-0003, ADR-0004, RFC-0001, or the Product Constitution, the
accepted document wins and this contract is defective. This revision creates no
ADR and supersedes nothing but its own predecessor, and then only on acceptance.

---

## E. Change register

The complete, auditable set of differences from Revision 1.

**Substantive changes — one:**

| # | Surface | Change | Nature |
|---|---|---|---|
| 1 | §2 | New subsection **§2.1 Distinguished members of opaque domains** | **Addition only.** No existing §2 sentence altered. |

**Lifecycle and metadata differences — not substantive:**

| Surface | Difference |
|---|---|
| Header | Revision, Status (PROPOSED), Predecessor, Contract family, Governed by (PBD-0006), Authority (PBA-0059), empty acceptance fields |
| Banner | Non-authoritative notice |
| §A–§G | This revision's own wrapper — supersession declaration, scope, change register, lifecycle. **Not contract text**, and would not form part of the accepted contract's normative body beyond the acceptance record §11 would carry. |

**Everything not listed above is unchanged.**

**Verification mechanism.** Because §2.1 is purely additive and §§3–8 are not
touched at all, substantive identity is provable directly: the predecessor file
is byte-identical to its accepted state — blob `b0fbf73a` at both the
pre-authoring baseline and after this revision was authored — and this revision
reproduces no §§3–8 text that could drift from it. Byte-identity of the
predecessor plus additivity of the change is a stronger guarantee than a
normalized section-by-section comparison of two reproduced copies, and it removes
the transcription risk such a reproduction would carry.

---

## F. What this revision deliberately does not do

- **It does not change `WF-1`.** The checker-semantics version is unchanged and
  is not incremented, renamed, or reinterpreted. §6.2 scopes `WF-1` to "every
  semantic rule in §3, §4, §5, and §6.4"; this revision changes none of them. It
  supplies identity information that existing rules already presume, so no
  determinate `WF-1` result changes for any input that was already canonical
  under the clarified definition. This revision makes **no** general versioning
  policy, and **N-24 remains OPEN**.
- **It adds no defect category.** §6.4.1's twenty-six remain complete.
- **It adds, removes, and reorders no mandatory check**, and repurposes none.
- **It implements nothing.** Checks 14A, 16 and 6 are unimplemented, as is all of
  Slice-1 Phase 2B. Acceptance unblocks Phase 2B (§G) but authorizes no
  implementation.
- **It resolves no deferred decision.** Q-06, Q-08, N-22, N-24, N-25 and F-8
  remain **OPEN**. It selects no platform, language, framework, UI technology,
  persistence, database, serialization format, parser, schema language,
  deployment target, hosting provider, broker, market-data provider, evaluator
  implementation, evaluator-semantics version scheme, hash algorithm, CI/CD
  platform, or release packaging model. In particular, resolving the `SM-1`
  identity binding does **not** resolve N-24.
- **It resolves no carried observation.** O-1, O-2 and O-3 stand; O-1 must still
  be settled before check 6.
- **It amends, reinterprets, and supersedes no other accepted document.**
  ADR-0002, ADR-0003, ADR-0004, ADR-0005, RFC-0001, RFC-0002 and the Product
  Constitution are untouched and remain authoritative over this contract per §0.

---

## G. Lifecycle and review path

This revision is **ACCEPTED**. Under PBD-0006 the path it completed was:

1. authoring — PBA-0059 / ER-0060;
2. independent read-only review — PBA-0060 / **ARR-0019** (PASS WITH MINOR
   REMEDIATION REQUIRED — M-1, M-2);
3. bounded remediation — PBA-0061 / **ER-0061** (closed M-1 and M-2 inside §2.1);
4. final read-only re-review — PBA-0062 / **ARR-0020** (PASS — READY FOR PRODUCT
   BOARD ACCEPTANCE);
5. Product Board acceptance — PBA-0063 / **PBR-0014**, 2026-08-14.

**On that acceptance:**

- this revision became authoritative and Revision 1 became superseded;
- **F-3 is CLOSED** — the normative distinguished-member identity mechanism now
  exists, so the checker receives authoritative bindings for `DIMENSIONLESS` and
  `SM-1` and manufactures neither;
- the latent `SM-1` identification gap affecting check 16 is normatively
  resolved (check 16 itself remains unimplemented);
- **Slice-1 Phase 2B is UNBLOCKED — awaiting implementation authorization.**
  Unblocking is not an implementation authorization; no Phase-2B code is
  authorized by this acceptance.

Current-state phase status is tracked in `slice-1-implementation-plan.md`.
