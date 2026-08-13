# Trading Decision Model

- **Tier:** 2 — Normative Product Specification
- **Version:** 0.2
- **Status:** PROPOSED (governed by Product Constitution v1.0 — ACTIVE — RATIFIED)
- **Last updated:** 2026-08-07
- **Subordinate to:** `constitution.md`
- **Supersedes:** the conceptual Trading Decision Model v0.1 (WR-0001), which was
  an analysis artifact and never a repository document.

> **Ownership boundary.** This document owns the Ordina **decision lifecycle**:
> the stages, their order, and their responsibilities. It **references** the
> Validation Model for validation dimensions, status definitions, status
> resolution, evidence requirements, and blocking/temporary semantics, and must
> **not** duplicate or redefine those. Where lifecycle and status semantics meet,
> the Validation Model governs.

---

## 1. Provenance

This model is the reconciliation of Product Discovery evidence into a single
lifecycle. Its authority derives from approved Product Board Decisions, not from
the workshop notes directly:

- **PBD-0001** — decision inputs, canonical validation decomposition, boundary
  basis, Calculated Trade Construction after READY, Trader Review, evidence
  sequencing.
- **PBD-0002** — two-stage Trade Construction, modification scope, incomplete
  direction handling, final revalidation checkpoint, canonical ordering.

Workshop Notes WN-0001 through WN-0006 remain historical Product Discovery
records (RECORDED — NOT ADOPTED). Their findings are adopted only to the extent
incorporated by the decisions above.

## 2. Trader Inputs

The trader selects, and pre-commits to, three inputs:

- **Instrument** — the trader selects it. Ordina never independently selects,
  ranks, or recommends an instrument within the core decision-validation
  workflow (Constitution C2; PBD-0001 D1).
- **Strategy** — a named, explicit decision plan defined before execution.
- **Risk Profile** — the trader's stable, predefined risk policy, distinct from
  the Strategy (see `validation-model.md` for the dimension; `glossary.md` for
  the definition).

Entry, Stop Loss, and Take Profit are **not** trader inputs. They are calculated
(§5, §7).

## 3. Direction

Direction (BUY / SELL) is an **output of Strategy Validation**, derived only
through explicit deterministic logic defined in the trader-selected Strategy
(PBD-0001 D2). It is an **input to** Preliminary Trade Construction, not a value
that construction produces.

If the selected Strategy lacks sufficient explicit direction logic, Ordina must
not infer a direction. The result is **DO NOT TRADE** with the conceptual reason
`STRATEGY_INCOMPLETE_DIRECTION_LOGIC`. This is a blocking, **non-temporary**
condition: waiting for market changes cannot repair an incomplete Strategy
definition (PBD-0002 D7). Status classification is owned by
`validation-model.md` §4.

## 4. Canonical Lifecycle

```
Trader selects Instrument, Strategy, and Risk Profile
    ↓
Strategy Validation                         → derives Direction (§3)
    ↓
Preliminary Trade Construction (§5)
    ↓
Risk Validation  +  Trader Readiness Validation
    ↓
Final Status Resolution                     (validation-model.md §4)
    ↓
READY
    ↓
Final Calculated Trade Construction (§7)
    ↓
Trader Review (§8)
    ↓
Final Revalidation Checkpoint (§9)
    ↓
Execution or Decline
    ↓
Review
    ↓
Learning
```

WAIT and DO NOT TRADE keep execution controls locked. A WAIT decision may remain
pending and be re-evaluated as temporary conditions change; a WAIT → READY
transition may trigger a neutral notification (§10).

## 5. Preliminary Trade Construction

Occurs after sufficient Strategy Validation evidence exists but **before** final
status resolution. It exists solely to give Risk Validation the values it needs,
resolving the ordering dependency between risk constraints (which reference a
Risk–Reward Ratio and a position size) and the values that express them
(PBD-0002 D3).

It is derived deterministically from the selected Strategy, validated market
evidence, the Direction produced by Strategy Validation (§3), and the predefined
Risk Profile. It may calculate **only**:

- candidate Entry;
- candidate Stop Loss;
- candidate Take Profit;
- preliminary Risk–Reward Ratio;
- preliminary Position Size;
- monetary and percentage Risk;
- margin and exposure impact.

Preliminary Trade Construction:

- **must** be deterministic;
- **must** derive only from the sources named above;
- **is not** an executable trade plan;
- **must not** imply READY;
- **must not** unlock execution;
- **must not** be presented to the trader as a recommendation
  (`ux-constitution.md`);
- exists only to support Risk Validation.

## 6. Validation

Three independent dimensions, evaluated independently and combined only at final
status resolution. Their definitions, inputs, and resolution are owned by
`validation-model.md`:

- **Strategy Validation** — consumes strategy rules, market context, required
  market evidence, and historical evidence when the Strategy requires it; derives
  Direction (§3); evaluates ordered evidence (context, location, sequence,
  timing, confirmation).
- **Risk Validation** — consumes the Risk Profile, account and exposure state,
  and the Preliminary Trade Construction values (§5). Holds "veto authority" only
  in the sense already defined by the status hierarchy: any blocking failure in
  any dimension yields DO NOT TRADE. It carries no higher procedural rank
  (PBD-0002 D9).
- **Trader Readiness Validation** — consumes the trader's **Self Assessment**
  (the blocking input) and, if later authorized, **Objective Readiness Evidence**
  (informational only). A trader-reported blocking condition yields DO NOT TRADE,
  which no confirmation may override; objective evidence may warn but never
  determines status. Sourcing is resolved; see `validation-model.md` §2.2
  (PBD-0003). If Strategy Validation passes but Trader Readiness fails, the
  overall status is **DO NOT TRADE** and strategy satisfaction is explanatory
  only — never a second primary status.

Final status (READY / WAIT / DO NOT TRADE) is resolved by `validation-model.md`
§4. No averaging, voting, probability score, or confidence threshold may override
that hierarchy.

## 7. Final Calculated Trade Construction

Occurs **only after** Strategy Validation passes, Trader Readiness Validation
passes, Risk Validation passes, and final status resolves to READY (PBD-0002 D4).
It is the second and only executable construction stage; it is the concrete
realization of a Strategy that has already passed validation, not a
recommendation (PBD-0001 D8).

It may present:

- Direction;
- Entry;
- Stop Loss;
- Take Profit;
- Position Size;
- monetary and percentage Risk;
- Risk–Reward Ratio;
- **provenance for every calculated value** — the Strategy rule or risk rule that
  produced it (Constitution C4).

Final Construction must remain consistent with the Preliminary Construction used
during Risk Validation, except for explicitly permitted freshness, rounding,
tick-size, or broker-constraint adjustments that are themselves deterministic and
revalidated. The bounds of those adjustments are a tracked open item (P-03), as
is whether changed exposure requires a fresh Preliminary Construction (P-02).

## 8. Trader Review

After Final Calculated Trade Construction, the trader may (PBD-0002 D6):

- **Accept** — the **single canonical explicit Trader authorization for
  execution** (PBD-0004 D4). Accept preserves the current READY state subject to
  the Final Revalidation Checkpoint (§9); it does **not** itself immediately
  transmit an execution instruction. It is the one explicit execution
  authorization in the lifecycle — no second Trader confirmation is canonical.
- **Decline** — results in no execution. Treated neutrally; not a failure and not
  a missed opportunity (Constitution §5).
- **Modify Inputs** — the trader may change trader-owned inputs only: instrument,
  selected strategy, permitted strategy configuration, risk profile or a
  permitted risk parameter, or trader-provided readiness information.

The trader **may not** directly override calculated outputs while preserving
READY — strategy-derived Direction, calculated Entry, Stop Loss, Take Profit,
Position Size, or Risk–Reward Ratio. To obtain different calculated values, the
trader must change the input or rule that produces them.

Any modification:

- invalidates the existing READY state;
- creates a **revised decision**;
- requires complete re-validation;
- requires a new Preliminary Trade Construction and a new Final Calculated Trade
  Construction;
- must **not** inherit the prior READY status.

No modification may bypass validation.

### 8.1 Readiness confirmation

When Self Assessment reports ready but Objective Readiness Evidence raises a
concern (`validation-model.md` §2.2), Ordina may present a neutral,
non-persuasive readiness warning and request explicit confirmation before
execution; the trader remains the final decision maker. This confirmation applies
**only** to an informational-evidence conflict on an otherwise-READY decision. It
is **never** an override of a blocking Trader Readiness failure — a
trader-reported blocking condition is DO NOT TRADE and execution stays locked
(Constitution C1). The precise interaction and its exact lifecycle placement are
open design questions (N-12, N-13); the boundary above is not.

## 9. Final Revalidation Checkpoint

Immediately before execution, Ordina performs a final revalidation checkpoint
(PBD-0002 D8). It is **not** a fourth validation dimension. It rechecks the
existing three dimensions for conditions that may have changed since READY, and
confirms:

- the decision still has READY status;
- required market data remains valid and sufficiently fresh;
- price movement remains within Strategy-defined tolerance;
- risk and exposure remain within Risk Profile limits;
- no new blocking condition exists;
- trader readiness remains valid;
- the decision has not been cancelled or revised.

The checkpoint **runs automatically after Accept** and **validates the standing
Trader authorization; it does not create a new authorization** (PBD-0004 D4). If
READY remains confirmed, Ordina may transmit the already-authorized execution
instruction to the Broker / Execution Venue (`system-context.md` §4.E, §5). If a
temporary condition fails → **WAIT**; if a blocking condition fails, or required
data becomes untrustworthy → **DO NOT TRADE**. In either case **no transmission
occurs**, and execution remains locked until the resulting status is READY again.
**No second Trader confirmation is canonical.** Status semantics are owned by
`validation-model.md`.

This execution authorization is distinct from the **Trader Readiness
Confirmation** (§8.1), which occurs earlier and applies only to informational
readiness-evidence conflicts; it is never execution authorization.

## 10. Notification

A WAIT decision may remain pending and be re-evaluated as temporary conditions
change. A WAIT → READY transition may trigger a **neutral** notification whose
sole content is that the trader's predefined validation conditions are now
satisfied. It must not use persuasive or urgency-inducing language
(`ux-constitution.md`). Ordina informs; the trader decides.

The following remain open for later product definition (PBD-0002 D10, not
resolved here): maximum number of pending decisions; pending-decision lifetime;
expiration rules; re-validation frequency; notification frequency and
suppression. No technical mechanism is selected.

## 11. Execution, Review, and Learning

- **Execution** — controls unlock only at a confirmed READY (§9). The trader
  remains the final decision maker (Constitution C1).
- **Review** — evaluates whether the trader followed their own validated
  Strategy, not whether the trade produced profit (Constitution §5; Principle 7).
- **Learning** — may propose rule changes for human review; may never self-amend
  (Constitution §6).

## 12. Invariants

This model preserves, and must never violate:

1. Rules validate; AI explains (Constitution C3).
2. No AI or probabilistic mechanism may alter a status.
3. The trader selects the instrument; Ordina never originates one.
4. Every derived Direction and calculated parameter has deterministic rule
   provenance.
5. Preliminary Trade Construction is non-executable and does not imply READY.
6. Final Calculated Trade Construction occurs only after READY.
7. Calculated outputs cannot be directly overridden while retaining READY.
8. Any input modification invalidates READY and requires complete re-validation.
9. Final revalidation is not a fourth validation dimension.
10. Execution remains locked unless READY is confirmed.
11. The trader remains the final decision maker; Decline is neutral.

## 13. Tracked Open Items

Recorded, not resolved (see `open-questions.md`): P-02 (fresh Preliminary
Construction on changed exposure during checkpoint), P-03 (bounds of
broker-constraint adjustments), N-05 (provenance display vs. progressive
disclosure), N-06 (neutral-navigation surface boundaries), N-07 (source of
account and exposure state), N-09 (concurrency across strategies and instruments),
N-10 (Self Assessment collection and cadence), N-12 (conflict presentation), N-13
(confirmation interaction and lifecycle placement), N-15 (self-assessment
gaming), N-16 (readiness-data privacy and dependency), Q-10a (freshness/validity
thresholds), Q-11b (declaration of blocking vs. temporary conditions), Q-16
(strategy formation and authoring). Trader Readiness **sourcing** is resolved
(Q-15, PBD-0003); only its design questions remain.
