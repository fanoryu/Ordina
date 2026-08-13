# System Context

- **Tier:** 2 — Normative Product Specification
- **Version:** 0.1
- **Status:** PROPOSED — NOT RATIFIED
- **Last updated:** 2026-08-12
- **Governing Constitution:** Product Constitution v1.0 — ACTIVE — RATIFIED
- **Authoritative basis:** PBD-0004 (adopts SCW-01 Findings 1–7)

> **Purpose.** This document defines the **external product boundary** of Ordina:
> who and what Ordina interacts with, the information that crosses the boundary,
> what Ordina is responsible for, and what it explicitly is not. It is a
> product-boundary artifact, **not** an architecture. It selects no provider,
> protocol, technology, or mechanism; unknowns are stated as unknown.
>
> **Ownership boundary.** This document owns external actors, external systems,
> boundary information flows, and the responsibility boundary. It **references**
> the Validation Model for status semantics and the Trading Decision Model for
> lifecycle and execution-authorization semantics, and restates neither. See §8.

---

## 1. Direct Actors

The sole direct human actor in the current product boundary is the **Trader**.

- **Mentor** is **not** a direct actor in the current scope. Mentor may remain
  represented as contextual metadata as defined in `user-zero.md` §4.
- **Admin / operator** is **not** a direct actor in the current scope.

These are **current-scope** statements. They do not prohibit a future Product
Board decision introducing additional actor roles. No other actor is modeled.

## 2. External Systems

Two external systems are part of the current boundary. Both are external services
Ordina depends on or communicates with; Ordina does not provide or replace them
(§7).

### 2.A Market Data Source

- **Role:** provides the market information required by Strategy Validation
  (`validation-model.md` §2).
- **Direction:** inbound to Ordina.
- **Not selected, and out of scope for this document:** provider, vendor, broker
  feed, exchange, API, protocol, ingestion architecture.

### 2.B Broker / Execution Venue

- **Role:** receives an authorized execution instruction from Ordina after all
  applicable product gates pass (§4.E, §5).
- **Direction:** outbound from Ordina.
- **Not selected, and out of scope for this document:** broker, API, order
  protocol, authentication model, credential model, custody mechanism,
  integration architecture.

### 2.C Not a mandatory external system

- **Account and Exposure State** is **not** modeled as a distinct mandatory
  external system. It is an inbound information flow whose origin is undetermined
  (§3, N-07).
- **Objective Readiness Sources** (wearables, sleep, calendar, and similar) are
  **not** current external systems. They are modeled only as *potential future
  external sources* and are not current dependencies (§3, §5, N-16).

## 3. Inputs / Inbound Information Flows

### 3.1 From the Trader

- selected **Instrument**;
- selected **Strategy**;
- **Risk Profile**;
- **Self Assessment** (the blocking Trader Readiness input, `validation-model.md`
  §2.2);
- explicit **execution authorization through Accept**, when applicable (§5).

### 3.2 From external boundaries

- **Market Data** (from the Market Data Source, §2.A).
- **Account and Exposure State** — a **required inbound information flow** consumed
  by Risk Validation (`validation-model.md` §2). Its **origin is undetermined**:
  it may later originate from the Broker / Execution Venue, another account
  system, Trader-provided data, or another authorized source. The actual source
  and integration remain **open (N-07)**.

### 3.3 Potential future optional input

- **Objective Readiness Evidence.** If ever introduced, it remains **informational
  only** and **cannot independently determine** Trader Readiness status, produce
  DO NOT TRADE, or override Self Assessment (`validation-model.md` §2.2). Its
  consent, privacy, jurisdiction, and data-dependency questions remain **open
  (N-16)**. No such source is a current dependency.

This document defines **no** schema, format, transport, refresh cadence, or
synchronization for any input.

## 4. Outputs / Outbound Information Flows

### To the Trader

**4.A Decision Status** — `DO NOT TRADE`, `WAIT`, `READY`. Status semantics are
owned by `validation-model.md` and referenced here, not redefined.

**4.B Explanation / Evidence / Provenance** — status reasons, relevant evidence,
decision provenance, and calculated-value provenance (Constitution C4). This does
not authorize any AI or probabilistic component to determine status
(Constitution C3).

**4.C Final Calculated Trade Construction** — when applicable: Direction, Entry,
Stop Loss, Take Profit, Position Size, Risk, Risk–Reward Ratio, and required
provenance. Construction rules are owned by `trading-decision-model.md` §7 and
referenced here, not redefined. No new trade parameter is introduced.

**4.D Review and Learning Output** — decision-integrity review; patterns and
insights; proposed rule changes **for human review**. Learning may not
autonomously rewrite Strategy or rules (Constitution §6).

### To the Broker / Execution Venue

**4.E Authorized Execution Instruction** — an outbound flow that may occur **only
after**:

```
Trader Accept
    ↓
Final Revalidation Checkpoint
    ↓
READY remains confirmed
    ↓
authorized execution instruction may be transmitted
```

## 5. Execution Authorization Boundary

Recorded precisely from PBD-0004 Decision 4; lifecycle detail is owned by
`trading-decision-model.md`.

**Accept** (Trader Review) is the **single canonical explicit Trader
authorization** for execution. Accept does **not** immediately transmit an order.
After Accept, the **Final Revalidation Checkpoint runs automatically**:

- if **READY** remains confirmed → Ordina may transmit the already-authorized
  execution instruction;
- if a **temporary** condition fails → **WAIT** → no transmission;
- if a **blocking** condition fails, or required data becomes untrustworthy →
  **DO NOT TRADE** → no transmission.

**No second Trader confirmation** after the checkpoint is part of the canonical
lifecycle. The checkpoint is an automatic safety gate between explicit Trader
authorization and external order transmission (Constitution C1, C10).

**Distinct interaction — do not conflate.** The **Trader Readiness Confirmation**
(`trading-decision-model.md` §8.1, `ux-constitution.md` §5F) is a separate,
informational-evidence-conflict interaction that occurs **before** the execution
authorization boundary. It applies only when Self Assessment is ready but
Objective Readiness Evidence raises a concern, and it is never an override of a
blocking failure. It is not the execution authorization, and Accept is not a
readiness confirmation.

## 6. Ordina Responsibility Boundary

At product level, Ordina is responsible for:

- receiving Trader-owned decision inputs;
- consuming required external market information;
- consuming Account and Exposure State for Risk Validation;
- deterministic validation according to the normative models;
- deterministic Preliminary and Final Trade Construction;
- maintaining required provenance;
- gating construction and execution;
- communicating Decision Status;
- explaining evidence and status;
- review and learning within existing constitutional limits;
- transmitting an explicitly authorized execution instruction **only** when READY
  remains confirmed.

This describes **product responsibility** only. It describes no internal
implementation mechanism.

## 7. Explicit External / Out-of-Scope Responsibilities

Ordina is **not** responsible for:

1. selecting or recommending which Instrument the Trader should trade;
2. producing a discretionary BUY / SELL decision where the selected Strategy lacks
   sufficient deterministic direction logic (`validation-model.md` §2.1);
3. making the final decision that the Trader should execute a trade;
4. automatically executing a trade without explicit Trader authorization;
5. guaranteeing profit, return, win rate, or any particular trading outcome;
6. replacing the Broker / Execution Venue as provider of execution services;
7. replacing the Market Data Source as provider of market information;
8. automatically rewriting Strategy or rules from outcomes or learning.

The Trader retains final decision authority.

## 8. Conceptual Boundary Diagram

Implementation-neutral. Arrows are information flows, not protocols. Dashed lines
denote optional/future relationships that are **not** current dependencies.

```
                          ┌──────────────────────┐
        Market Data        │                      │
   ┌────────────────────▶ │                      │
   │   (inbound)           │                      │
┌──┴───────────────┐       │                      │
│ Market Data      │       │       ORDINA         │
│ Source (external)│       │  (product boundary)  │
└──────────────────┘       │                      │
                           │                      │
        Trader             │                      │      authorized execution
   ◀──────────────────────▶│                      │──────────instruction───────▶ ┌────────────────────┐
   (inputs; status,        │                      │      (outbound, only after   │ Broker /           │
    explanation,           │                      │       Accept + checkpoint +   │ Execution Venue    │
    construction,          │                      │       READY confirmed)        │ (external)         │
    review; Accept)        │                      │                               └────────────────────┘
                           │                      │
   Account & Exposure      │                      │
   State ────────────────▶ │                      │
   (inbound; origin        │                      │
    UNDETERMINED — N-07)   │                      │
                           │                      │
   Objective Readiness     │                      │
   Evidence ┈┈┈┈┈┈┈┈┈┈┈┈▶ │                      │
   (optional / future;     │                      │
    informational only)    └──────────────────────┘
```

**Summary of relationships:**

- **Trader ↔ Ordina** — bidirectional: Trader supplies inputs and the Accept
  authorization; Ordina returns status, explanation/provenance, Final Calculated
  Trade Construction, and review/learning.
- **Market Data Source → Ordina** — inbound market data.
- **Ordina → Broker / Execution Venue** — outbound authorized execution
  instruction, gated per §5.
- **Account & Exposure State → Ordina** — inbound, origin undetermined (N-07).
- **Objective Readiness Evidence ┈▶ Ordina** — optional/future, informational
  only (N-16); not a current dependency.

## 9. Cross-Document Ownership

No document may redefine another document's normative ownership.

| Document | Owns |
|---|---|
| **Product Constitution** | product identity; constitutional boundaries; precedence |
| **Trading Decision Model** | lifecycle; stage ordering; execution-authorization lifecycle |
| **Validation Model** | validation dimensions; status semantics; status resolution; evidence classification |
| **UX Constitution** | presentation and interaction constraints |
| **User Zero** | current target user; mentor-scope details |
| **System Context** (this document) | external actors, systems, and flows; the product boundary |

## 10. Open / Deferred and Resolved Items

Recorded here for traceability; **not** resolved by this document. The canonical
register is `open-questions.md` (not modified by this authoring task).

**Open / partially clarified:**

- **N-07** — actual source and integration of Account & Exposure State.
- **N-16** — Objective Readiness source privacy, consent, jurisdiction, data
  dependency, future integration design.
- **Q-04** — trade-record source.
- **Q-06** — platform / form factor.
- **Q-07** — future tenancy / sharing.
- **Q-08** — jurisdiction.
- **P-03** — broker-constraint adjustment bounds.
- **Q-17** — mentor-influence metadata.
- All other currently open questions unaffected by this document.

**Recorded as resolved (by PBD-0004):**

- **N-17** — RESOLVED. Accept is the explicit execution authorization; the Final
  Revalidation Checkpoint precedes transmission (§5).
- **N-18** — RESOLVED AT SYSTEM-CONTEXT MODELING LEVEL. Account & Exposure State is
  an inbound information flow of undetermined origin (§3.2); this does not close
  N-07.

## 11. Abstraction Boundary

This document selects and implies **no**: programming language, framework,
database, broker, market-data vendor, execution API, market-data API, protocol,
cloud provider, deployment model, architecture style, synchronization mechanism,
queue/event system, credential storage, persistence model, or data-retention
implementation. Unknown origins and mechanisms are stated as unknown and must not
be filled by assumption.

## 12. Follow-up (not performed here)

Recorded for the post–System-Context review, not actioned by this task:

- Constitution §8 precedence list should add `system-context.md` to Tier 2
  (Governance item G-04; **do not amend §8 during authoring**).
- `open-questions.md` should record the N-17 / N-18 resolutions and the PBD-0004
  Decision 11 reclassifications when an alignment amendment is authorized.
