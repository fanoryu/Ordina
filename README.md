# Ordina

**A decision-validation platform for traders.**

> **North Star:** Better decisions create better traders.
>
> **Thesis:** We do not build a platform to make traders trade more. We build a
> platform to help traders trade only when they should.

Ordina does not predict outcomes and does not guarantee profit.

---

> **This document is Tier 5 — non-normative.** It is a front door, not a source
> of truth. Where it summarizes, the linked document governs. A rule that exists
> only here does not exist.

## Project status

**Foundation phase. No production code exists, by design.**

The Product Constitution is **v1.0 — ACTIVE — RATIFIED** (Product Owner, POR-0001,
2026-08-12).

The first architecture and engineering decisions are now accepted:

| Artifact | Status |
|----------|--------|
| [Product Constitution](docs/product/constitution.md) v1.0 | **ACTIVE — RATIFIED** (POR-0001) |
| [RFC-0001 — Strategy Formation and Authoring](docs/rfc/0001-strategy-formation-and-authoring.md) | **ACCEPTED** (PBR-0003) |
| [ADR-0001 — Record architecture decisions](docs/adr/0001-record-architecture-decisions.md) | **ACCEPTED** |
| [ADR-0002 — Canonical Strategy Semantic Model](docs/adr/0002-canonical-strategy-semantic-model.md) | **ACCEPTED** (PBR-0006) |
| [ADR-0003 — Canonicalization & Revision Identity](docs/adr/0003-canonicalization-and-revision-identity.md) | **ACCEPTED** (PBR-0007) |
| [ADR-0004 — Strategy Well-Formedness Evaluation](docs/adr/0004-strategy-well-formedness-evaluation.md) | **ACCEPTED** (PBR-0009) |
| [First Runnable Slice Contract](docs/engineering/first-runnable-slice-contract.md) | **ACCEPTED** (PBR-0011) |

Accepted architecture does not mean the product is implementation-ready. **No
implementation technology is selected** — language, framework, parser,
serialization, persistence, database, hash algorithm, evaluator, deployment, and
UI all remain open and each requires its own RFC and ADR. Regulatory review is not
done, and tracked open questions remain open, including Q-06 (platform), which
gates stack selection. The architecture overview stays deliberately free of
implementation architecture until those close.

## Document precedence

1. [Product Constitution](docs/product/constitution.md)
2. Normative Product Specifications — [Trading Decision Model](docs/product/trading-decision-model.md) · [Validation Model](docs/product/validation-model.md) · [UX Constitution](docs/product/ux-constitution.md) · [User Zero](docs/product/user-zero.md) · [System Context](docs/product/system-context.md)
3. [Engineering Constitution](docs/engineering/constitution.md)
4. [RFCs](docs/rfc/) and [ADRs](docs/adr/)
5. README.md, CONTRIBUTING.md, CLAUDE.md

Within Tier 2, the Trading Decision Model owns the decision lifecycle and the
Validation Model owns status semantics.

Lower documents may refine higher documents but may not silently contradict them.

## Supporting documents

| Document | Purpose |
|----------|---------|
| [Open Questions](docs/product/open-questions.md) | What is unanswered, and what it blocks. |
| [Risk Register](docs/product/risk-register.md) | Live technical and product risks. |
| [Engineering Workflow](docs/engineering/workflow.md) | RFC/ADR gating, review standards, Definition of Done. |
| [Architecture Overview](docs/architecture/overview.md) | Known constraints only. No mechanism. |
| [Glossary](docs/glossary.md) | Controlled vocabulary, including forbidden terms. |

## How we work

```
Discussion → RFC → Decision → ADR → Implementation Plan → Implementation → Review → Docs
```

We never skip to implementation. Speed is not preferred over architecture.

## Next milestone

The pre-implementation architecture and engineering-contract chain is complete
through the accepted First Runnable Slice Contract, which fixes what the first
runnable slice is: a deterministic Strategy **Well-Formedness** checker that takes
an already-canonical Strategy and returns Well-Formed, or UNUSABLE with a
deterministic defect set. It selects no technology.

Remaining before implementation can begin:

- **Git baseline** — the repository is not yet under version control.
- **Implementation authorization**, including how language selection is reconciled
  with **Q-06** (platform), which the register records as blocking stack selection.
  Language, framework, and datastore choices each require an RFC **and** an ADR
  (`docs/engineering/workflow.md`).

The Tier-2 Normative Product Specifications and the Tier-3 engineering documents
remain **PROPOSED**; ratification of the Constitution did not auto-promote them.
