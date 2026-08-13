# ADR-0001: Record architecture decisions

- **Status:** Accepted
- **Date:** 2026-08-05
- **Deciders:** Principal Engineer (pending product-owner countersign)
- **Related:** —

## Context

Ordina starts from zero. The most expensive mistakes in a long-lived product are
not bad code — code is replaceable — but architectural decisions whose *reasoning*
was never recorded. Two years on, nobody remembers whether a constraint was a
considered trade-off or an accident, so nobody dares change it.

This project also has an unusually strict product identity (Constitution §2). That
identity survives only if the reasoning behind boundary-adjacent decisions is
written down and citable.

## Decision

We will record every architecturally significant decision as an ADR in
`docs/adr/`, numbered sequentially, using `0000-template.md`. A decision is
architecturally significant if it is costly to reverse, affects a product
boundary, or constrains future options across more than one component.

Substantial features are preceded by an RFC in `docs/rfc/`. RFCs debate; ADRs
record. Not every RFC produces an ADR, and not every ADR needs an RFC.

## Consequences

### Positive
- Decisions become reviewable and challengeable by people who were not present.
- Constitution boundary calls acquire precedent, so they stay consistent.
- Onboarding reads as a narrative rather than archaeology.

### Negative
- Real overhead per decision. If applied to trivial choices it becomes ceremony,
  people route around it, and the archive rots (R-07).

### Neutral
- Requires calibration on "architecturally significant." When in doubt, discuss
  it in the RFC and skip the ADR.

## Reversibility

Cheap. Abandoning the practice costs nothing but the loss of future records.

## Alternatives Rejected

- **No formal record; rely on commit messages and PR discussion.** Rejected:
  discussion lives in a platform we do not control and is not indexed by topic.
- **A single evolving ARCHITECTURE.md.** Rejected: it records the current state
  but destroys the history of *why*, which is the part worth keeping.
