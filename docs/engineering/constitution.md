# Engineering Constitution

- **Tier:** 3
- **Status:** PROPOSED (governed by Product Constitution v1.0 — ACTIVE — RATIFIED)
- **Last updated:** 2026-08-07
- **Subordinate to:** Product Constitution (Tier 1) and Normative Product
  Specifications (Tier 2)

> This document governs *how we build*. It does not restate product rules. Where
> a product rule has engineering consequences, it is **cited**, never copied — a
> copy drifts, a citation does not.

---

## 1. Engineering Invariants

**E1 — Deterministic validation boundary.**
Constitution C3 ("Rules validate; AI explains") is an architectural boundary, not
only a policy. Whatever components eventually exist, status resolution must be
reachable, reviewable, and testable without any model-backed component in the
path. No AI component may sit on the path that produces READY, WAIT, or DO NOT
TRADE.

AI-assisted explanation *is* permitted — but on the explanation path only.
**Explanation has no decision authority** (`ux-constitution.md` §2). Internal use
of confidence, probability, scores, or ratings is permitted for model evaluation,
research, testing, observability, and data-quality analysis, provided no such
quantity can reach status resolution or a decision-facing surface
(`glossary.md` §B). **No averaging, voting, probability score, or confidence
threshold may override the deterministic status hierarchy** (Constitution C3) —
this is a structural requirement, not only a policy. The same rule governs
**Objective Readiness Evidence** (`validation-model.md` §2.2): behavioural,
wearable, or model-derived readiness signals are informational and must be
structurally unable to fail Trader Readiness, produce DO NOT TRADE, override Self
Assessment, or reach status resolution.

**E2 — Explainability and provenance are a build-time obligation.**
Constitution C4 requires every status to be explainable from evidence available
at validation time, **and** every calculated parameter — Preliminary or Final —
to retain provenance to the rule that produced it. Both evidence and provenance
must be retained *as validation and construction run*, not reconstructed
afterwards. Reconstruction after the fact is not explanation.

**E3 — Auditable correction.**
Constitution C5 requires that corrections preserve provenance and never silently
rewrite the past. This is a requirement on observable behaviour, **not** a
mandate for event sourcing, append-only physical storage, or any particular
database. The mechanism is undecided and must not be assumed.

**E4 — Gating is structural.**
The product has **four** structural checkpoints that must be enforced by system
structure, not by a UI check alone: rejection of a Strategy with incomplete
direction logic; final status resolution; the Final Construction gate (READY,
Constitution C8); and the pre-execution revalidation checkpoint (Constitution
C10). None may depend on a model-backed component. How each is enforced is
undecided.

**E5 — Simplicity.**
Principle 5 (simplicity over information) and Principle 12 apply to code and
documents as much as to interfaces. Prefer the boring design a new engineer
understands in a day.

**E6 — Reversibility.**
Prefer decisions that are cheap to undo. Where a decision is effectively
permanent, it requires an ADR that says so explicitly.

**E7 — Pending decisions require re-evaluation over time.**
A WAIT decision may remain pending and must be re-evaluable as temporary
conditions change (`trading-decision-model.md` §10). This is a product
requirement with architectural consequence; **no mechanism is selected**, and
none may be inferred (cf. E3).

**E8 — Construction is deterministically reproducible.**
Preliminary and Final Trade Construction must be deterministically reproducible
from their inputs. Final Construction may differ from the Preliminary values used
during Risk Validation **only** by explicitly permitted, revalidated adjustments
(freshness, rounding, tick-size, broker constraint). Bounds are a tracked open
item (P-03). No mechanism is selected.

## 2. Exceptions

An explicitly approved ADR may document a justified engineering exception. It
must:

1. identify the higher-level principle or constraint involved;
2. state why compliance is not achievable as specified;
3. state the scope and expected lifetime of the exception.

An ADR **cannot** amend product identity. That requires product-owner
ratification (Constitution §9).

## 3. Testing Posture

- Tests must cover the failure modes named in the governing RFC, not only the
  happy path.
- Status resolution (`validation-model.md` §4) is the highest-value test surface
  in the product; it must be exhaustively testable in isolation.
- No test may depend on a model-backed component to assert a status.

## 4. Documentation Posture

- Docs change with the code, in the same commit.
- ADRs are immutable once Accepted — supersede, never rewrite.
- Every document states tier, status, and last-updated date.

## 5. What This Document Does Not Decide

Deliberately undecided, and not to be inferred from anything above: database,
storage or persistence model, event-sourcing, language, framework, deployment
model, broker, market-data provider, platform, or form factor.

Each requires an RFC and an ADR (`workflow.md`).
