# Validation Model

- **Tier:** 2 — Normative Product Specification
- **Version:** 0.2
- **Status:** PROPOSED (governed by Product Constitution v1.0 — ACTIVE — RATIFIED)
- **Last updated:** 2026-08-07
- **Subordinate to:** `constitution.md`

> **Ownership boundary.** This document owns **status semantics** — validation
> dimensions, status definitions, resolution, evidence rules, and
> blocking/temporary vocabulary. The decision **lifecycle** (stage order, Trade
> Construction stages, Trader Review, notification, execution) is owned by
> `trading-decision-model.md`, which references this document and does not
> restate it.

---

## 1. Input

The trader selects and pre-commits to three inputs (`trading-decision-model.md`
§2):

1. the **instrument** (trader-selected; Ordina never selects, ranks, or
   recommends one — Constitution C2);
2. a named **Strategy**;
3. a predefined **Risk Profile**.

Entry, Stop Loss, and Take Profit are **not** trader inputs. They are calculated
by Trade Construction after validation (`trading-decision-model.md` §5, §7).
Direction is derived by the Strategy, not supplied by the trader (§2.1).

## 2. Validation Dimensions

The canonical model has **three** validation dimensions (PBD-0001 D4). The
previous five-input formulation is **not** retained as five parallel validators;
its concepts are reclassified below. "Market Readiness" is **not** a canonical
validator — market conditions are evaluated within Strategy Validation.

1. **Strategy Validation** consumes: strategy rules; market context; required
   market evidence; historical evidence when the selected Strategy requires it.
   It derives Direction (§2.1) and evaluates ordered evidence (§5.3).
2. **Trader Readiness Validation** consumes: the trader's **Self Assessment**
   (the blocking input) and **Objective Readiness Evidence** (informational only,
   §5, if later authorized). Sourcing is resolved — see §2.2.
3. **Risk Validation** consumes: the trader's Risk Profile; account and exposure
   state; the Preliminary Trade Construction values needed to evaluate risk
   (`trading-decision-model.md` §5). Risk Validation holds "veto authority" only
   in the sense already defined by §4 — any blocking failure in any dimension
   yields DO NOT TRADE. It carries **no** higher procedural rank than the other
   dimensions (PBD-0002 D9).

The three dimensions are independent in evaluation and combined only at final
status resolution (§4).

### 2.1 Direction

Direction (BUY / SELL) is an **output of Strategy Validation**, derived only
through explicit deterministic logic in the selected Strategy. If the Strategy
lacks sufficient direction logic, Ordina must not infer one; the result is
**DO NOT TRADE**, reason `STRATEGY_INCOMPLETE_DIRECTION_LOGIC` (§4). This is a
blocking, non-temporary condition — waiting cannot repair an incomplete Strategy
definition.

### 2.2 Trader Readiness sourcing

Resolved by PBD-0003 (adopts WN-0009). Trader Readiness has two inputs of
**different authority**:

- **Self Assessment — the blocking input.** The trader is the primary authority
  on their own mental and behavioural state; Ordina never infers or replaces this
  declaration (Constitution C2, trader-agency pillar). If the trader explicitly
  reports a blocking condition — for example revenge trading, uncontrolled FOMO,
  emotional instability, inability to follow the Strategy, or acknowledged loss of
  discipline — Trader Readiness fails and the status is **DO NOT TRADE**. **No
  confirmation flow may override this blocking failure** (Constitution C1).
- **Objective Readiness Evidence — informational only.** Potential future sources
  (trading behaviour, journal history, sleep information, wearable data, calendar
  context, other approved signals) are **informational evidence** (§5). They
  **may** trigger explanation, a neutral warning, or a confirmation request; they
  **may not** independently fail Trader Readiness, independently produce DO NOT
  TRADE, override Self Assessment, reach deterministic status resolution, or allow
  a model-generated score or probability to determine status (Constitution C3;
  Engineering Constitution E1). **No integration with any such source is
  authorized.**

**Conflict and confirmation.** When Self Assessment = ready but Objective
Readiness Evidence = concern, Ordina may request an explicit, neutral,
non-persuasive confirmation, after which the trader remains the final decision
maker. Confirmation applies **only** to informational-evidence conflict; it is
never an override path for a blocking failure. Interaction detail is owned by
`ux-constitution.md`; lifecycle placement by `trading-decision-model.md`.

**Single status.** If Strategy Validation passes but Trader Readiness fails, the
overall status is **DO NOT TRADE**. Ordina may explain that the Strategy
conditions were satisfied, but that remains explanatory only and must never
appear as a competing READY or a second primary status (`ux-constitution.md` §1).

## 3. Output

Exactly one of:

| Status | Meaning |
|---|---|
| **READY** | All three validation dimensions pass and the minimum required evidence is available. |
| **WAIT** | Validation cannot yet pass because a potentially temporary condition is incomplete. |
| **DO NOT TRADE** | A blocking condition in any validation dimension has failed (including incomplete Strategy direction logic). |

These statuses validate decision quality based on information available at the
time. They never imply or guarantee a profitable outcome (Constitution §5).

## 4. Deterministic Status Resolution

Evaluated in this order:

```
Any blocking failure in any validation dimension
    → DO NOT TRADE

No blocking failure, but one or more required temporary
conditions are incomplete
    → WAIT

All three validation dimensions pass and all required
evidence is valid
    → READY
```

Resolution is deterministic and rule-driven. AI plays no part in producing a
status (Constitution C3). **No averaging, voting, probability score, or
confidence threshold may override this hierarchy** (Constitution C3).

**Incomplete direction logic.** A Strategy that lacks sufficient explicit
direction logic (§2.1) is a **blocking, non-temporary** failure → DO NOT TRADE,
reason `STRATEGY_INCOMPLETE_DIRECTION_LOGIC`. It is never WAIT: no market change
can repair a Strategy definition. (`STRATEGY_INCOMPLETE_DIRECTION_LOGIC` is a
conceptual reason, not an implementation code.)

## 5. Constraint Vocabulary

There is **no category called "non-blocking rules."** Use exactly two terms:

- **Blocking constraint** — influences status.
- **Informational evidence** — explains context; does not independently determine
  status.

"Veto authority," where used of Risk Validation, is **emphasis, not a separate
resolution rule**. It means only what §4 already states: any blocking failure in
any dimension yields DO NOT TRADE (PBD-0002 D9).

### 5.1 WAIT conditions

WAIT applies only to temporary conditions that can reasonably resolve **without
fundamentally changing the proposed decision**. Examples:

- waiting for candle close;
- waiting for confirmation;
- waiting for a session;
- waiting for a news window to pass;
- temporary data unavailability.

### 5.2 DO NOT TRADE conditions

Blocking failures. Examples:

- trader-readiness failure;
- daily loss limit reached;
- unacceptable risk;
- strategy-rule violation;
- invalid position size;
- unreliable or invalid market data.

## 6. Minimum Required Evidence

There is **no universal historical sample-size threshold at this stage.** Each
strategy specification defines the evidence required for its own validation.

READY requires, at minimum:

- a trader-selected instrument;
- a named Strategy;
- a Strategy containing **sufficient explicit direction logic** (§2.1);
- evaluable blocking constraints across all three validation dimensions;
- sufficiently fresh and valid required market data;
- a defined Risk Profile;
- calculable and acceptable risk (evaluated against Preliminary Trade
  Construction values, `trading-decision-model.md` §5);
- available trader-readiness state.

Entry, Stop Loss, and Take Profit are **not** among the requirements: they are
calculated, not supplied (`trading-decision-model.md` §5, §7).

Historical performance evidence may become strategy-required, but it is not a
universal launch requirement for User Zero.

### 6.1 Data availability vs. data validity

A confirmed and binding distinction:

```
Required data temporarily unavailable from an otherwise valid source
    → WAIT

Data invalid, corrupted, inconsistent, unacceptably stale, or otherwise
untrustworthy
    → DO NOT TRADE
```

Availability is a temporary condition. Trustworthiness is a blocking constraint.

The **thresholds** that separate these — what counts as "sufficiently fresh" or
"unacceptably stale" per data type — remain open (Q-10a). The rule above is
settled; its numeric boundaries are not.

### 6.2 Evidence sequencing

Evidence is **not** an unordered checklist. When required by a Strategy,
validation must be capable of evaluating **context, location, sequence, timing,
and confirmation relationships** (PBD-0001 D11) — for example: trend established;
price reaches the required SNR area; retest occurs after reaching that area;
confirmation appears after the retest. Availability of evidence does not satisfy
a Strategy whose conditions require a particular order. A Strategy must be able to
express ordered conditions without requiring User Zero to write technical code
(see `user-zero.md` §3; Q-16).

## 6A. Evaluation Order and Trade Construction

Status resolution depends on Risk Validation, which consumes Preliminary Trade
Construction values. The evaluation therefore proceeds: Strategy Validation →
Preliminary Trade Construction → (Risk Validation + Trader Readiness Validation)
→ final resolution (`trading-decision-model.md` §4, PBD-0002 D5). Preliminary
Trade Construction is non-executable, must not imply READY, and exists only to
support Risk Validation (Constitution C8). The construction stages themselves are
owned by `trading-decision-model.md`.

## 6B. Final Revalidation Checkpoint

Immediately before execution, the three dimensions are re-checked for conditions
that may have changed since READY (Constitution C10; `trading-decision-model.md`
§9). This is **not** a fourth validation dimension. A temporary failure resolves
to WAIT; a blocking failure or untrustworthy data resolves to DO NOT TRADE, per
§4. Execution remains locked until the status is READY again.

## 7. Execution Gating

Ordina execution controls remain locked unless validation returns READY
(Constitution C1). Final Calculated Trade Construction is additionally gated on
READY (Constitution C8). When validation returns WAIT or DO NOT TRADE, execution
controls are locked.

## 8. Learning Boundary

Learning may propose rule changes for human review. It may never automatically
rewrite strategy rules, change thresholds without explicit approval, rewrite
historical decisions, or treat a single outcome as sufficient evidence for change
(Constitution §6).

## 9. Open Questions

Deliberately unresolved; see `open-questions.md`. Notably: what "sufficiently
fresh" means per data type (Q-10a); how a strategy specification declares its own
required evidence (Q-10b) and its blocking vs. temporary conditions (Q-11b); the
source of account and exposure state (N-07); and whether changed exposure at the
checkpoint requires a fresh Preliminary Construction (P-02). Trader Readiness
sourcing is resolved (§2.2); its **design** questions — Self Assessment
collection (N-10), conflict presentation (N-12), confirmation interaction (N-13) —
remain open. No answer is assumed here.
