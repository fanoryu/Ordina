# Engineering Workflow

- **Tier:** 3
- **Status:** PROPOSED (governed by Product Constitution v1.0 — ACTIVE — RATIFIED)
- **Last updated:** 2026-08-14

## The sequence

```
Discussion → RFC (if required) → Decision → ADR (if required)
          → Implementation Plan → Implementation → Review → Documentation Update
```

Implementation is never the first step.

## When each artifact is required

| Change | RFC | ADR |
|--------|-----|-----|
| Typo, comment, formatting | No | No |
| Bug fix within existing design | No | No |
| New feature within an accepted design | No | No |
| New user-facing capability | **Yes** | Only if it constrains future options |
| **Anything that changes status semantics or resolution** | **Yes** | **Yes** |
| **Anything touching the rules/AI boundary** | **Yes** | **Yes** |
| **Anything touching execution gating or the four checkpoints** | **Yes** | **Yes** |
| **Anything touching Trade Construction (Preliminary or Final) or its provenance** | **Yes** | **Yes** |
| **Anything touching the pre-execution revalidation checkpoint** | **Yes** | **Yes** |
| Anything touching a Constitution §2 boundary | **Yes** | **Yes** |
| Choice of language, framework, datastore, hosting, market-data provider, broker | Yes | **Yes** |
| Change to how decisions or corrections are recorded | Yes | **Yes** |
| Anything touching auth, PII, or credentials | Yes | **Yes** |
| Third-party dependency with lock-in or licence terms | Yes | **Yes** |
| An engineering exception to a higher-tier document | Yes | **Yes** |

When unsure: write the RFC. It is cheaper than the migration.

## Review standards

Checked in this order:

1. **Identity** — does it pass the Constitution §2.1 boundary test (Origination,
   Determinism, Provenance, Gating, Restraint)?
2. **Status integrity** — does it leave `validation-model.md` §4 resolution
   unchanged, or is the change explicitly RFC'd?
3. **Correctness** — does it do what the plan said?
4. **Simplicity** — is there a more boring version? (Principles 5, 12)
5. **Explainability** — is every user-visible assertion backed by evidence
   produced during validation? (C4)
6. **Documentation** — did the docs change in the same commit?

A reviewer may reject on (1) or (2) alone, without regard to code quality.
Product identity is not negotiable at review time.

## Definition of Done

- [ ] Behaviour matches the accepted plan
- [ ] Constitution §2.1 boundary test answered in the PR description
- [ ] Status semantics confirmed unchanged, or the change is RFC-approved
- [ ] Tests cover the failure modes named in the RFC, not just the happy path
- [ ] No model-backed component introduced on the status-resolution path (E1)
- [ ] Docs updated in the same change — never "in a follow-up"
- [ ] Risk register updated if the change introduced or retired a risk

## Documentation rules

- Docs change with the code, in the same commit (R-07).
- ADRs are immutable once Accepted. Supersede, never rewrite.
- Normative engineering contracts follow the same principle, through the
  successor convention below.
- Every doc states tier, status, and last-updated date.
- Tier 5 documents (README, CONTRIBUTING, CLAUDE.md) contain **no normative
  content** — pointers only. A rule that exists only there does not exist.

## Revising an accepted normative engineering contract

**Authority: PBD-0006** — Product Board determination, 2026-08-14.

This section governs an artifact class the repository had no convention for: a
document that is **normative and immutable once accepted**, is **not** an ADR or
RFC, carries **no** document-version lifecycle, and yet expressly permits a
future authorized successor. The First Runnable Slice Contract is the first
member. The convention below is reusable and is not an exception for that
document.

> **Supersede, never rewrite** — the same principle ADRs already follow, applied
> to a class that is not numbered like an ADR.

### The successor model

A revision is a **separate successor file**. The accepted predecessor is
preserved permanently as its own file and is never substantively rewritten.

In-place versioned rewriting is **not** the convention here, even though the
Product Constitution and the Trading Decision Model use it. Those documents
already carry version metadata and a lifecycle; a normative engineering contract
does not, so importing that pattern would redefine an accepted artifact's
lifecycle in the act of using it. Keeping predecessor and successor as distinct
files also gives stronger historical immutability. A separately governed artifact
family may still choose in-place versioning; it is simply not the default here.

### Identity and naming

A contract has a stable **family identity** — its existing human-readable name —
plus an explicit **revision number**. The accepted document keeps its current
filename and is **revision 1**; it is not renamed and no identifier is invented
for it retroactively.

A successor file is named `<contract-family-name>-rNN.md`, zero-padded to two
digits. The first successor to `first-runnable-slice-contract.md` is therefore
`first-runnable-slice-contract-r02.md`.

**No new ledger namespace is created.** A revision needs no globally allocated
identifier: it is identified by family name, revision number, its predecessor,
and the `PBR` that accepted it — all of which already exist. A namespace would be
symmetry for its own sake.

### Revision numbering

Revisions increase monotonically by one within a family, are never reused, and
carry no meaning beyond sequence. A proposal that is rejected or withdrawn keeps
the revision number it was issued, which is then not reused.

Revision numbers are **not** coupled to `WF-1` or any other checker-semantics
version, evaluator-semantics version, application or release version, ADR or RFC
number, or any `PBA`/`PBR`/`ARR`/`ER` number.

### Required metadata

A proposed successor states, at minimum: **Title**, **Contract family**,
**Revision**, **Status**, **Predecessor**, **Authority** (the authoring PBA),
**Author**, **Tier**, **Governed by**, and **Review history**. On acceptance it
additionally records **Accepted by** (the PBR) and **Decision date**.

It must state plainly whether it supersedes the predecessor **in full**, or
changes only **explicitly enumerated surfaces** while incorporating the remainder
unchanged. Where it is the latter, the unchanged surfaces must be substantively
identical to the predecessor's, and the authoring report proves that by diff.

No separate template file is introduced — the fields above are the template, and
adding a file would create an artifact family the repository does not otherwise
need.

### Status lifecycle

```
PROPOSED → IN REVIEW → ACCEPTED
              ↓
   REJECTED / WITHDRAWN            ACCEPTED → SUPERSEDED
```

| Status | Meaning |
|---|---|
| **PROPOSED** | Authorized and authored. **No normative force.** |
| **IN REVIEW** | Undergoing required independent review. **No normative force.** |
| **ACCEPTED** | Product Board acceptance recorded under a `PBR`. **Normative.** |
| **REJECTED** | The Board declined it. Never became normative. |
| **WITHDRAWN** | Withdrawn before acceptance. Never became normative. |
| **SUPERSEDED** | Was ACCEPTED, later replaced by an accepted successor. Its contents remain immutable. |

These six reuse the vocabulary the ADR and RFC templates already carry rather
than inventing a parallel one; no smaller set preserves all six distinctions.

### Review path

1. The Product Board authorizes authoring, via `PBA`.
2. The Principal Engineer authors the proposed successor.
3. A separately authorized `PBA` commissions an independent, read-only review.
4. The review is recorded as an `ARR`.
5. BLOCKER / MAJOR / MINOR findings are remediated before acceptance.
6. Remediation occurs only under separate authorization.
7. Re-review occurs where required.
8. The Product Board separately authorizes acceptance.
9. Acceptance is recorded as a `PBR`.
10. Only then is the successor normative and the predecessor superseded.

This mirrors the discipline accepted artifacts already follow, without pretending
the contract is an ADR or an RFC.

### Acceptance authority

> **Product Board acceptance recorded through a `PBR` is the only act that makes
> a proposed successor authoritative.**

A `PBA` authorizes work. An `ARR` records review. An `ER` records execution.
**None of them makes proposed contract text normative**, and neither does a
documentation edit, implementation usage, convention-by-practice, or Git history.
A successor stays non-normative until its `PBR`.

### Supersession semantics

- **Historical immutability** — the predecessor's accepted substantive text is
  preserved permanently and is never rewritten. Clerical acceptance or
  supersession metadata may be recorded only where an existing immutable-artifact
  rule already permits it; otherwise the relationship is recorded on the
  successor and in the ledger, not by editing history.
- **Normative authority** — after acceptance, new work follows the successor.
- **Provenance** — the successor names its predecessor and the acceptance act
  that caused supersession.

### Current application — First Runnable Slice Contract

`first-runnable-slice-contract.md` is the **predecessor and current revision**
(revision 1), ACCEPTED under PBR-0011, and its substantive contents remain
authoritative and unmodified.

The distinguished-member clarification directed by PBA-0057 — binding
`DIMENSIONLESS` and `SM-1` as constitutive canonical input — requires a successor
under this convention. **No successor exists.** Consequently **F-3 remains
OPEN / BLOCKED** and **Slice-1 Phase 2B remains blocked**.

This section establishes only *how* such a revision becomes an artifact and
acquires authority. It does not restate, reconsider, or supply the substantive
semantics PBA-0057 already directed.
