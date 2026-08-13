# Glossary

- **Tier:** 2 (supporting)
- **Status:** PROPOSED (governed by Product Constitution v1.0 — ACTIVE — RATIFIED)
- **Last updated:** 2026-08-07

Controlled vocabulary. If a term is used in code or UI, it is defined here, and
code and UI use the same word. Vocabulary drift precedes product drift (R-02).

## Core terms

| Term | Definition |
|------|------------|
| **Proposed Decision** | A decision the trader submits for validation, comprising the trader-selected **instrument**, named **Strategy**, and predefined **Risk Profile**. Direction is derived by the Strategy (§2.1); Entry / Stop Loss / Take Profit are calculated, not supplied. Always trader-originated (Constitution C2). |
| **Validation** | Deterministic evaluation of a Proposed Decision across the three validation dimensions — Strategy, Trader Readiness, Risk — producing exactly one status. |
| **Validation Dimension** | One of the three independent evaluators: Strategy Validation, Trader Readiness Validation, Risk Validation (`validation-model.md` §2). |
| **Strategy Validation** | The dimension consuming strategy rules, market context, required market evidence, and (when required) historical evidence; derives Direction; evaluates ordered evidence. |
| **Trader Readiness Validation** | The dimension consuming trader-provided readiness information (and, if later authorized, approved behavioural evidence). Source open (Q-15). |
| **Risk Validation** | The dimension evaluating a decision against the Risk Profile, account/exposure state, and Preliminary Trade Construction values. "Veto authority" is emphasis, not a separate rule (PBD-0002 D9). |
| **READY** | All three validation dimensions pass and the minimum required evidence is valid. |
| **WAIT** | No blocking failure, but one or more required temporary conditions are incomplete. |
| **DO NOT TRADE** | A blocking condition in any validation dimension has failed. |
| **Blocking Constraint** | A constraint that influences status. |
| **Informational Evidence** | Context that explains but does not independently determine status. |
| **Direction** | BUY / SELL, derived deterministically by the selected Strategy; an input to construction, not a calculated output. If the Strategy lacks sufficient direction logic → DO NOT TRADE (`STRATEGY_INCOMPLETE_DIRECTION_LOGIC`). |
| **Strategy** | A trader-owned, explicitly named decision plan defined before execution, specifying required conditions, evidence, sequence, and when execution is and is not permitted. Must express ordered conditions without technical code (PBD-0002 D11). Authoring mechanism open (Q-16). |
| **Strategy Rule** | An individual rule within a Strategy. |
| **Risk Profile** | The trader's stable, predefined risk policy — a **source of constraints** (e.g. risk per trade, max daily loss, max trades per day, max concurrent exposure, minimum RR, max position size, consecutive-loss stop, leverage limits). Distinct from Strategy: a Strategy defines how a setup becomes valid; a Risk Profile defines whether and how much risk is permitted. |
| **Risk Constraint** | An individual limit sourced from the Risk Profile. Failure is blocking. |
| **Trader Readiness** | The trader's own state as a validation input. Sourced from Self Assessment (blocking) plus Objective Readiness Evidence (informational). Resolved by PBD-0003; `validation-model.md` §2.2. |
| **Self Assessment** | The trader's own declaration of readiness — the **primary, blocking** input to Trader Readiness Validation. Only the trader may make it; Ordina never infers or replaces it. A reported blocking condition (revenge trading, FOMO, emotional instability, inability to follow the Strategy, loss of discipline) yields DO NOT TRADE, which no confirmation may override. |
| **Objective Readiness Evidence** | Supplementary, **informational** readiness signals (e.g. trading behaviour, journal, sleep, wearables, calendar). May warn or prompt confirmation; may never determine status, override Self Assessment, or reach status resolution. No integration authorized. |
| **Readiness Confirmation** | A neutral, non-persuasive prompt shown when Self Assessment is ready but Objective Readiness Evidence raises a concern. Applies only to informational-evidence conflict; never an override of a blocking failure. |
| **Market Context** | Current market conditions, consumed within Strategy Validation. Requires market data (Q-03 resolved). |
| **Market Readiness** | *Retired.* Not a canonical validator; market conditions are evaluated within Strategy Validation (PBD-0001 D4). Recorded so the term is not reintroduced. |
| **Preliminary Trade Construction** | Deterministic pre-resolution computation of candidate values to support Risk Validation. Non-executable; does not imply READY; not a recommendation (Constitution C8). |
| **Final Calculated Trade Construction** | Deterministic computation of the executable parameters after READY; a concrete realization of a validated Strategy, not a recommendation (Constitution C8; `trading-decision-model.md` §7). |
| **Candidate value** | An output of Preliminary Trade Construction (candidate Entry / SL / TP / RR / Position Size), distinct from a final calculated value. |
| **Trader Review** | The post-construction step where the trader may Accept, Decline, or Modify Inputs (`trading-decision-model.md` §8). |
| **Modify Inputs / Revised Decision** | Modification of trader-owned inputs only; invalidates READY and requires complete re-validation (Constitution C9). Calculated outputs may never be directly overridden while retaining READY. |
| **Final Revalidation Checkpoint** | The pre-execution recheck of the three dimensions; not a fourth dimension (Constitution C10). |
| **Minimum Required Evidence** | The evidence a status requires. Universal floor in `validation-model.md` §6; per-strategy additions defined by the Strategy. |
| **Review** | Retrospective comparison of a Decision against its actual outcome; evaluates decision integrity, not profit. Principle 7. |
| **Outcome** | What actually happened after a Decision. Recorded separately; never rewrites the Decision (Constitution §6). |
| **Learning** | Pattern surfacing, review insight, and proposal of rule changes for human review. Never self-amending (Constitution §6). |
| **"Why?" Path** | The explanation route behind every status, grounded in evidence available at validation time. May summarize, connect, and explain that evidence. Has **no decision authority** (`ux-constitution.md` §2). |
| **`STRATEGY_INCOMPLETE_DIRECTION_LOGIC`** | Conceptual reason for a DO NOT TRADE arising from a Strategy lacking sufficient direction logic. Conceptual, **not** an implementation code. |
| **Market Data Source** | The external system providing market information required by Strategy Validation. Inbound to Ordina; no provider/protocol selected (`system-context.md` §2.A). |
| **Broker / Execution Venue** | The external system that receives an authorized execution instruction from Ordina after all product gates pass. Outbound from Ordina; no broker/API/custody model selected (`system-context.md` §2.B). |
| **Account & Exposure State** | A required inbound information flow consumed by Risk Validation, whose origin is undetermined (may later be broker, another account system, trader-provided, or another authorized source). Not modeled as a mandatory separate external system (N-07 open; `system-context.md` §3.2). |
| **Execution Authorization** | The single explicit Trader authorization for execution, given via Trader Review → **Accept**. It does not immediately transmit; the Final Revalidation Checkpoint runs automatically after it. No second Trader confirmation is canonical (PBD-0004 D4; `trading-decision-model.md` §8). |
| **Authorized Execution Instruction** | The outbound flow Ordina may transmit to the Broker / Execution Venue **only** after Accept, the Final Revalidation Checkpoint, and confirmed READY (`system-context.md` §4.E, §5). |

## Vocabulary by context

Two distinct categories. Confusing them produces either a leaky product or
needlessly crippled engineering tooling.

### A. Forbidden in decision-facing UX

Never appear in any user-facing surface as a characterization of a Proposed
Decision or as a decision status.

| Term | Why |
|------|-----|
| **Signal** | Ordina is not a signal provider (Constitution §2). |
| **Recommendation** | Ordina validates the user's own proposal; it does not recommend (C2). |
| **Prediction / Forecast** | Ordina does not predict outcomes (Constitution §1). |
| **Guarantee** | Ordina does not guarantee profit (Constitution §1, §5). |

### B. Restricted — forbidden decision-facing, permitted internally

**Confidence, probability, score, rating, likelihood.**

Prohibited as:

- primary decision statuses;
- user-facing substitutes for READY, WAIT, or DO NOT TRADE;
- persuasive claims about profitability;
- mechanisms that can override deterministic validation.

Permitted internally for legitimate technical purposes — model evaluation,
statistical research, testing, observability, data-quality analysis — **provided
they cannot determine or alter an Ordina status** (Constitution C3; Engineering
Constitution E1).

The test is not the word. It is whether the quantity can reach a status or a
decision-facing surface.

## Note on "confirmation"

"Wait for confirmation" is a legitimate WAIT condition (`validation-model.md`
§5.1). "Confirmation" in that sense is a user-defined strategy condition, not an
Ordina-produced signal. The distinction matters and should be preserved in code.
