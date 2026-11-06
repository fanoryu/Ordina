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

**Dogfood-0 is complete.** Ordina has a Rust engine for deterministic Strategy
Well-Formedness checks and a Windows-tested desktop prototype for diagnosing,
correcting, and resuming a Strategy from a local JSON file.

The checker reports `Well-Formed` or `Unusable`. The desktop app shows issues
and any checks blocked by those issues. It computes a revision identity when the
Strategy is Well-Formed. Live trade-decision statuses (`READY`, `WAIT`,
`DO NOT TRADE`) are outside the scope of this prototype.

In the [first live Dogfood-0 session (ER-0154)](docs/governance/identifier-ledger.md?plain=1#L539)
on August 20, 2026, the Product Owner corrected a Strategy from `Unusable` to
`Well-Formed`, saved it locally, and reopened it successfully. A Blocking
condition added after the first restart was still present after a second save
and restart.

### Run the desktop example

From the repository root, with Rust installed through rustup, run this command
with a new local file path:

```sh
cargo run -p ordina-desktop -- ./ordina-demo.dogfood.json
```

1. Select **Begin correction**, then **Set semantic-model version to supported**.
2. Select **Confirm submission**, then **Submit (full re-check)**. The app should
   show **Well-Formed** and **No issues found**.
3. Select **Save to local envelope**, close the app, and rerun the same command
   to resume the saved Strategy.

See the [First Runnable Slice Contract](docs/engineering/first-runnable-slice-contract.md)
for the checker scope and [RFC-0005](docs/rfc/0005-dogfood-local-persistence.md)
for the local save and resume scope.

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

The Product Owner will select the next roadmap item after the Dogfood-0 closure
and the startup regression review. See the [closure record (ARR-0064)](docs/governance/identifier-ledger.md?plain=1#L389)
and the [open questions](docs/product/open-questions.md).
