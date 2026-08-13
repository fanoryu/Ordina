# Engineering Workflow

- **Tier:** 3
- **Status:** PROPOSED (governed by Product Constitution v1.0 — ACTIVE — RATIFIED)
- **Last updated:** 2026-08-07

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
- Every doc states tier, status, and last-updated date.
- Tier 5 documents (README, CONTRIBUTING, CLAUDE.md) contain **no normative
  content** — pointers only. A rule that exists only there does not exist.
