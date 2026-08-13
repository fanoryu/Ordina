# Contributing to Ordina

> **Tier 5 — non-normative.** Process pointers only. Every rule referenced here
> lives in a higher-tier document, which governs.

## Before you write anything

1. Read the [Product Constitution](docs/product/constitution.md), then the
   [Validation Model](docs/product/validation-model.md) and
   [UX Constitution](docs/product/ux-constitution.md).
2. Read the [Engineering Workflow](docs/engineering/workflow.md) and check
   whether your change needs an RFC or an ADR.
3. Check [Open Questions](docs/product/open-questions.md). If your change depends
   on an open question, the answer comes first.

## The rule that matters most

Every change must pass the **boundary test in Constitution §2.1**. Answer all
five parts in the PR description. Do not paraphrase the test from memory — cite
the section and answer it as written there. A change that fails any part is
rejected regardless of code quality.

## Precedence

Lower-tier documents may refine higher-tier ones but may not silently contradict
them. An explicitly approved ADR may record a justified engineering exception —
it must name the higher-level principle involved and cannot amend product
identity, which requires product-owner ratification.

## Disagreement is expected

If you think a decision is wrong, say so, with reasoning. Agreement without
reasoning is a failure mode here. The venue is an RFC, or a new ADR that
supersedes the old one. We do not quietly edit history.

## Documentation

Docs ship with the change, in the same commit. "Docs to follow" is not accepted.

## Commits

- Present tense, imperative: "Add decision export", not "Added".
- Reference the RFC or ADR when one applies.
- One logical change per commit.
