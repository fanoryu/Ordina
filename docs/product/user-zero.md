# User Zero

- **Tier:** 2 — Normative Product Specification
- **Status:** PROPOSED (governed by Product Constitution v1.0 — ACTIVE — RATIFIED)
- **Last updated:** 2026-08-07
- **Subordinate to:** `constitution.md`

---

## 1. Who

The initial user is a **beginner trader** who understands basic concepts but:

- often follows a mentor without an independent process;
- is vulnerable to FOMO;
- does not yet have a mature strategy;
- wants greater discipline;
- currently captures screenshots that rarely become reusable knowledge.

## 2. Intended Transformation

From:

```
Market → Mentor → Trade → Screenshot → Lost
```

To:

```
Market → Evidence → Decision → Trade if validated → Review → Knowledge
```

## 3. Design Implications

Stated only where they follow directly from §1–§2:

- The user **does not yet have a mature strategy**, yet a named strategy is
  required for READY (`validation-model.md` §6). How User Zero arrives at a first
  strategy specification is an open question (Q-16), not an assumed feature.
  Strategy authoring must **not** be assumed to be technical rule entry. A
  Strategy must be able to express ordered conditions (context, location,
  sequence, timing, confirmation) **without requiring User Zero to write technical
  code** (PBD-0002 D11; `validation-model.md` §6.2). This is a binding constraint
  on any future authoring approach, not a design choice.
- FOMO vulnerability is the concrete case Principle 8 (protect from impulsive
  decisions) exists to address.
- Screenshots-that-become-nothing is the concrete failure that Principle 7
  (learning continues after execution) replaces.

## 4. Mentor Scope

> **This section is the single detailed source for MVP mentor scope.**
> Constitution §7 states the durable boundary and refers here. Do not restate
> these exclusions elsewhere.

For the MVP, a mentor is **not a product entity**. Mentor influence may be
recorded as contextual metadata. Ordina will not include:

- mentor accounts;
- mentor marketplace;
- copy trading;
- automatic mentor-signal ingestion;
- mentor ranking.

Automatic mentor-signal ingestion would additionally violate Constitution C2
(trade origination).

## 5. Prototype Scope

For the local User Zero prototype, basic export is **not** a blocker
(Constitution C7). It becomes mandatory before Ordina stores persistent data for
external beta users.
