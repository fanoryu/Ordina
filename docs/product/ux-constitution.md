# UX Constitution

- **Tier:** 2 — Normative Product Specification
- **Status:** PROPOSED (governed by Product Constitution v1.0 — ACTIVE — RATIFIED)
- **Last updated:** 2026-08-07
- **Subordinate to:** `constitution.md`

---

## 1. Primary Decision View

The primary interface exposes only:

```
READY
WAIT
DO NOT TRADE
```

**No confidence percentage, probability, AI score, or rating appears in the
primary decision view.** This is absolute, not a default.

## 2. Explainability

Every status must support a **"Why?" path** (Constitution C4).

The "Why?" path must remain **grounded in evidence available at validation time**.
It may summarize, connect, and explain that evidence. It may **not**:

- introduce new decision inputs;
- change READY, WAIT, or DO NOT TRADE;
- originate a new trade proposal;
- produce or suggest an instrument, direction, entry, stop, or target outside
  explicit strategy logic;
- present model opinion as if it were a deterministic rule result.

AI-assisted explanation is permitted. **Explanation has no decision authority.**

Inference is not prohibited as such — summarizing and connecting evidence is the
point of the "Why?" path. What is prohibited is inference acquiring decision
authority, or being presented as a rule result.

## 3. Progressive Disclosure

The primary interface is minimal. Evidence and technical detail appear only when
requested.

## 4. Screen Time

Ordina should **reduce** screen time rather than maximize engagement
(Principle 9). A user seeing WAIT should be able to close Ordina and return to
daily life.

Any feature whose value depends on the user remaining in the application is in
conflict with this document.

## 5. Forbidden Surfaces

Derived from §1 and Constitution §2. Confidence, probability, score, rating, and
likelihood are prohibited as:

- primary decision statuses;
- user-facing substitutes for READY, WAIT, or DO NOT TRADE;
- persuasive claims about profitability;
- mechanisms that can override deterministic validation.

Also prohibited as a characterization of a proposed decision in any user-facing
surface: predicted or projected outcome; signal or recommendation framing;
any guarantee of profit.

**These terms are not prohibited in internal technical contexts.** They may be
used for model evaluation, statistical research, testing, observability, and
data-quality analysis, provided they cannot determine or alter an Ordina status.
See `docs/glossary.md` §"Vocabulary by context".

## 5A. Preliminary Construction Display

Preliminary Trade Construction values (`trading-decision-model.md` §5) **must not**
be presented to the trader as a trade plan, an executable proposal, or a
recommendation. They exist only to support Risk Validation. The product computes
values it must not surface as actionable. Whether and how any preliminary value
appears at all is constrained by §1 and §3, and the boundary against progressive
disclosure is a tracked open item (N-05).

## 5B. Modification Affordance

The interface **must not** offer direct editing of calculated outputs — Direction,
Entry, Stop Loss, Take Profit, Position Size, Risk–Reward Ratio — as a
READY-preserving action (Constitution C9). Changing a calculated value can be
surfaced only as changing the input or rule that produces it, which invalidates
READY and triggers complete re-validation.

## 5C. Provenance Display

Calculated parameters may not be presented without identifying their rule
provenance (Constitution C4). How much provenance appears in the primary view
versus on request must respect §1 (minimalism) and §3 (progressive disclosure);
the precise split is a tracked open item (N-05).

## 5D. Notification Language

A WAIT → READY notification communicates only that the trader's predefined
validation conditions are now satisfied. It **must not** use persuasive or
urgency-inducing language (`trading-decision-model.md` §10). Ordina informs; the
trader decides.

## 5E. Navigation Neutrality

Neutral navigation tools — search, watchlists, recently viewed instruments — must
not present an instrument as one the trader should trade, and must not
independently originate a trade opportunity (Constitution C2; PBD-0001 D1).
Whether such surfaces are decision-facing for the purpose of this document is a
tracked open item (N-06).

## 5F. Readiness Warning and Confirmation

When Objective Readiness Evidence raises a concern while Self Assessment reports
ready (`validation-model.md` §2.2), any readiness warning and confirmation
request **must be neutral and non-persuasive** — it states the concern and asks,
without urging a choice (cf. §5D). It is subject to §1 (single primary status)
and §3 (progressive disclosure). The confirmation applies only to an
informational-evidence conflict and **must never** be presented as a way to
proceed past a blocking DO NOT TRADE (Constitution C1). The exact presentation is
an open design question (N-12, N-13); the neutrality and the boundary are not.

Strategy satisfaction shown alongside a DO NOT TRADE arising from Trader Readiness
must remain explanatory in the "Why?" path and must never render as a second
primary status (PBD-0003; §1).

## 6. Relationship to Metrics

Principle 9 and Constitution §5 mean engagement-shaped metrics (session length,
daily actives as a success measure, notification-driven return) are not valid
justifications for UX change. Thirty consecutive days of WAIT — and a user who
opens Ordina rarely — can be a successful outcome.

## 7. Open Questions

Platform and form factor are unresolved (Q-06). This document constrains the
interface regardless of the platform eventually chosen.
