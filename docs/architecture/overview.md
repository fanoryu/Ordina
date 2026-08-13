# Architecture Overview

- **Tier:** 3 (supporting)
- **Status:** INTENTIONALLY FREE OF IMPLEMENTATION ARCHITECTURE
- **Last updated:** 2026-08-07

There is no implementation architecture yet, and inventing one now would be
dishonest. A diagram drawn today acquires false authority merely by having been
written first.

This file records only constraints that follow **directly** from ratified-track
product decisions. It contains no mechanism.

The **external product boundary** — external actors, external systems, external
information flows, and the product/system boundary — is now specified at Product
Model level in `docs/product/system-context.md` (System Context v0.1, PROPOSED).
That document is the canonical source for the boundary; this file does not restate
it and infers no implementation architecture from it.

## Constraints already known

- **Deterministic validation path.** Status resolution must be reachable and
  testable without any model-backed component in the path (Constitution C3;
  Engineering Constitution E1).
- **Evidence retained during validation.** Explainability requires evidence
  captured as validation runs, not reconstructed afterwards (C4; E2).
- **Auditable correction.** Corrections preserve provenance and do not silently
  rewrite the past (C5; E3). **The mechanism is undecided** — this is not a
  commitment to event sourcing or append-only storage.
- **Structural execution gating, four checkpoints.** Incomplete-direction
  rejection, final resolution, the Final-Construction gate (READY), and the
  pre-execution checkpoint must be enforced by structure, not a UI check (C1, C8,
  C10; E4). Enforcement mechanism undecided.
- **Two-stage construction, deterministically reproducible.** Preliminary (pre-
  resolution, non-executable) and Final (post-READY) construction; Final may
  differ from Preliminary only by deterministic, revalidated adjustments (C8; E8).
  Mechanism undecided.
- **Calculated-value provenance retention.** Every Preliminary and Final value
  retains provenance to the rule that produced it (C4; E2). Mechanism undecided.
- **Pending decisions re-evaluated over time.** A WAIT decision may remain
  pending and be re-evaluated (E7). Mechanism undecided.
- **Deletion and export must be possible.** C7 staged by release phase. Any
  persistence approach must not make erasure architecturally impossible (R-04).

## Explicitly undecided

Database, persistence model, event-sourcing, language, framework, deployment
model, broker, market-data provider, platform, form factor. None may be inferred
from anything above.

## What must close before this file gains content

- Q-10a — freshness thresholds, an input to the rules engine (Q-11b condition
  classification and Q-15 trader-readiness sourcing are both RESOLVED; only
  Q-15's design questions N-10/N-12/N-13 remain)
- Q-06 — platform
- N-07 — source of account and exposure state
- P-02, P-03 — checkpoint re-construction and construction-adjustment bounds
- R-12 — strategy authoring, currently the unowned centre of the product
- `trading-decision-model.md` lifecycle stabilised (v0.2) — done for structure;
  tracked open items remain

Until then, this emptiness is the accurate description of the system.
