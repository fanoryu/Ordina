# Ordina — Product Constitution

- **Tier:** 1 — highest precedence
- **Version:** 1.0
- **Status:** ACTIVE — RATIFIED
- **Last updated:** 2026-08-12
- **Supersedes:** v0.5 Candidate (2026-08-12)
- **Ratified by:** Product Owner, POR-0001 (2026-08-12)

**Intended lifecycle**

```
v0.1 — Initial Foundation Draft
v0.2 — Proposed Product Foundation
v0.3 — Proposed Reconciled Product Model
v0.5 — Candidate for Ratification
v1.0 — Active
```

This document is now **v1.0 — ACTIVE — RATIFIED**. It was ratified by the Product
Owner via **POR-0001** (2026-08-12), advancing it from v0.5 Candidate. Its Product
Model and System Context completed the required reconciliation and review cycle:
Q-15 (trader-readiness sourcing) is resolved (PBD-0003); **System Context v0.1**
exists, passed formal review (SCR-0001), and is aligned across precedence,
lifecycle ownership, and the open-question register (PBA-0005); and Product Model
alignment is complete.

**Ratification did not change any product philosophy, principle, constraint,
boundary test, neutrality rule, learning boundary, or precedence semantics** — it
is a governance/status advancement only. Ratification does **not** mean the
product is implementation-ready, that architecture is selected, that RFC-0001 is
complete, that regulatory review is done, or that all Product Model questions are
closed. Deliberately tracked open questions remain open (§10).

**This is now the ACTIVE Tier-1 Product Constitution.** All lower-tier product and
engineering work must comply with it. Future changes to product identity,
constitutional boundaries, principles, or constraints require the formal
amendment process (§9). Lower-tier documents may refine it but may not contradict
it.
>
> **Ratification is a product-governance decision, not an architecture decision.
> It is not recorded by ADR.** It is recorded through this status and version
> header, the change history (§11), explicit Product Owner approval, and the
> corresponding repository commit.

---

## 1. Identity

Ordina is a **decision-validation platform for traders**.

**North Star**

> Better decisions create better traders.

**Product Thesis**

> We do not build a platform to make traders trade more.
> We build a platform to help traders trade only when they should.

Ordina does not predict outcomes and does not guarantee profit.

## 2. What Ordina Is Not

Product boundaries, not roadmap gaps. Crossing one changes product identity and
requires owner-ratified amendment.

- Not a trading bot.
- Not a signal provider.
- Not an AI predictor.
- Not an auto-trading platform.

### 2.1 The Boundary Test

Every proposed feature must pass all five. An RFC that cannot answer these is not
ready for review.

1. **Origination** — Does the trader select the instrument, and is every derived
   value (Direction, and any Preliminary or Final construction value) a
   deterministic consequence of a Strategy and Risk Profile the trader committed
   to before execution? Ordina must not independently invent an instrument,
   select a direction by discretionary opinion, or produce entry, stop loss, or
   take profit outside explicit strategy logic.
2. **Determinism** — Is the resulting status produced by rules, with AI confined
   to explanation, summary, organization, and learning support?
3. **Provenance** — Does every status support a "Why?" path backed by evidence
   produced during validation, **and** does every calculated parameter retain
   traceability to the rule that produced it?
4. **Gating** — Do all Ordina execution controls remain locked unless validation
   returns READY, and is Final Calculated Trade Construction gated on READY while
   Preliminary Trade Construction remains non-executable (§4, C8)?
5. **Restraint** — Does the feature avoid implying or guaranteeing an outcome?

A feature failing any test is not "out of scope for now." It is out of product.

**Note on what is *not* prohibited:** Ordina responding `WAIT` or `DO NOT TRADE`
to a trader-supplied decision, and deterministically calculating the trade
parameters implied by the trader's own validated Strategy, are core product
behaviour, not directional advice. The prohibited behaviour is **trade
origination** — inventing an instrument, choosing a direction by opinion, or
producing values outside explicit strategy logic.

## 3. Principles

Authoritative numbering. These are the principles; no other numbering scheme
exists in this repository.

1. Better decisions create better traders.
2. The platform validates decisions, never guarantees outcomes.
3. The best trade is sometimes no trade.
4. Every decision must be explainable.
5. Simplicity over information.
6. Evidence before execution.
7. Learning continues after execution.
8. The platform must protect the trader from impulsive decisions.
9. The platform should reduce screen time, not increase it.
10. Rules validate; AI explains.
11. Every trade must earn the right to be executed.
12. The simplest interface that preserves good decisions wins.

## 4. Constraints

Binding. Each derives directly from an owner decision.

- **C1 — Execution gating.** Ordina execution controls remain locked unless
  deterministic validation returns READY. A trader remains free to leave Ordina
  and use an external broker application; that manual action is outside Ordina's
  validated workflow.
- **C2 — Boundary basis and trade origination prohibited.** Ordina's boundary
  rests on four pillars: **trader pre-commitment, deterministic derivation,
  traceable provenance, and trader agency.** Ordina does not originate a trade
  when it calculates execution parameters that are deterministic consequences of
  the trader-selected instrument, the trader-selected Strategy, validated
  evidence, and the trader's predefined Risk Profile. Ordina must **never**:
  invent an instrument; use discretionary model opinion to select direction;
  produce Entry, Stop Loss, or Take Profit outside explicit strategy logic; alter
  a deterministic status through AI judgement; or present calculated parameters
  without identifying their rule provenance.
- **C3 — Rules validate; AI explains.** Rules determine validation. AI may
  explain, summarize, organize, and support learning. AI must never override a
  failed rule or independently change READY, WAIT, or DO NOT TRADE. No averaging,
  voting, probability score, or confidence threshold may override the
  deterministic status hierarchy.
- **C4 — Explainability without decision authority.** Every status must support a
  "Why?" path grounded in evidence available at validation time, **and** every
  calculated parameter — Preliminary or Final — must retain provenance to the
  strategy rule or risk rule that produced it. AI-assisted explanation is
  permitted; **explanation has no decision authority.** The binding limits are in
  `ux-constitution.md` §2.
- **C5 — Record integrity.** Historical decisions must remain auditable.
  Corrections must preserve provenance and must not silently rewrite the past.
  This does **not** mandate event sourcing, append-only physical storage, or any
  specific database architecture. Deletion or anonymization of personal data may
  still be required by user choice, retention policy, or applicable law.
- **C6 — Learning may not self-amend.** No single outcome may automatically
  rewrite strategy rules. See §6.
- **C7 — Data ownership and export.** The user's decision history is theirs.
  Basic export is mandatory before Ordina stores persistent data for external
  beta users. A mature export and deletion workflow is required before public
  release. Not a blocker for the local User Zero prototype.
- **C8 — Two-stage construction gate.** Trade Construction has two stages.
  **Preliminary Trade Construction** may occur before final status resolution to
  supply the values Risk Validation requires; it is non-executable, must not
  imply READY, must not unlock execution, and must not be presented as a
  recommendation. **Final Calculated Trade Construction** occurs only after
  validation returns READY. Both must be deterministic and carry rule provenance
  (C4). Lifecycle detail is owned by `trading-decision-model.md`.
- **C9 — Modification cannot bypass validation.** The trader may modify
  trader-owned inputs only. Calculated outputs may never be directly overridden
  while retaining READY. Any modification invalidates READY, creates a revised
  decision, and requires complete re-validation, including new Preliminary and
  Final construction. No modification inherits the prior READY status.
- **C10 — Pre-execution revalidation.** Immediately before execution, Ordina
  rechecks the three validation dimensions for conditions that may have changed
  since READY. This is not a fourth validation dimension. Execution remains
  locked until the resulting status is READY again.

## 5. Neutrality and Success

Ordina must remain neutral when:

- READY produces a losing trade;
- a user ignores DO NOT TRADE and profits;
- a user declines a READY trade that later would have profited.

Statuses validate decision quality based on information available at the time.
They never imply or guarantee a profitable outcome.

**Thirty consecutive days of WAIT can be a successful product outcome** if market
or trader conditions do not support a valid trade. Any metric, feature, or
growth argument that treats this as failure is in conflict with this Constitution.

A trader who **declines** a READY trade is treated neutrally. Decline is not a
failure and not a missed opportunity.

## 6. Learning

Learning **may**: surface patterns; generate review insights; compare validated
and manual behaviour; propose rule changes for human review.

Learning **may never**: automatically rewrite strategy rules; change thresholds
without explicit approval; rewrite historical decisions; treat one profitable or
losing outcome as sufficient evidence for change.

## 7. Scope Boundaries — MVP

A mentor is **not** a product entity. Mentor influence may be recorded as
contextual metadata.

Detailed MVP mentor scope is defined in `user-zero.md` §4, which is the single
source for it. This section states only the durable boundary.

## 8. Document Precedence

1. **Product Constitution** (this document)
2. **Normative Product Specifications** — `trading-decision-model.md`,
   `validation-model.md`, `ux-constitution.md`, `user-zero.md`,
   `system-context.md`
3. **Engineering Constitution** — `docs/engineering/constitution.md`
4. **RFCs and ADRs**
5. **README.md, CONTRIBUTING.md, CLAUDE.md** (non-normative)

Within Tier 2, `trading-decision-model.md` owns the decision **lifecycle**,
`validation-model.md` owns **status semantics**, and `system-context.md` owns the
**external product boundary** — external actors, external systems, information
flows, and external/out-of-scope responsibilities. Each references the others and
never restates their normative content.

Lower documents may refine higher documents but may not silently contradict them.
An explicitly approved ADR may document a justified engineering exception, but it
must identify the higher-level principle involved and cannot amend product
identity without product-owner ratification.

## 9. Amendment and Ratification

**Amendment:** RFC → discussion → decision → amendment, with version increment.
Amendments to §1, §2, §4, or §5 require explicit Product Owner sign-off.

**Ratification and amendment of this document are product-governance acts and are
not recorded by ADR.** ADRs record architecture decisions and may document
justified engineering exceptions (§8); they cannot amend or ratify product
identity. The governance record is the version header, §11, Product Owner
approval, and the repository commit.

**Ratified.** This Constitution was ratified by the Product Owner via **POR-0001**
(2026-08-12) and is now **v1.0 — ACTIVE**. It is authoritative and binding
precedent. Subsequent changes to §1, §2, §4, or §5 require explicit Product Owner
sign-off through the amendment process above; ADRs cannot amend product identity.

## 10. Open Questions

Tracked in `docs/product/open-questions.md`. Legal and regulatory
characterization is tracked in `docs/product/risk-register.md` and is explicitly
**not** settled by this document.

## 11. Change History

| Version | Date | Status | Summary | Approved by |
|---------|------|--------|---------|-------------|
| 0.1 | 2026-08-05 | Superseded | Initial draft reconstructed from the product brief. | — (not approved) |
| 0.2 | 2026-08-05 | Proposed | Reconciled against the Sessions 1–8 product-discovery record. Added North Star, Thesis, authoritative principles 1–12, constraints C1–C7, five-part boundary test, neutrality and success, learning boundaries, precedence. | Pending Product Owner |
| 0.3 | 2026-08-07 | Proposed | Reconciled Product Model (WR-0003, executed per PBA-0001). Re-founded the boundary on four pillars (C2); extended C4 to calculated-value provenance; added C8 two-stage construction gate, C9 modification, C10 pre-execution revalidation; added Decline neutrality; added `trading-decision-model.md` to Tier 2. Adopts the findings of WN-0001–WN-0006 **only** as incorporated by PBD-0001 and PBD-0002; unincorporated hypotheses remain RECORDED — NOT ADOPTED. | Pending Product Owner |
| 0.3 (rev.) | 2026-08-12 | Proposed | Incorporated PBD-0003 (Trader Readiness sourcing; adopts WN-0009) per PBA-0002, **without** a version change — v0.3 is still unratified. Self Assessment is the blocking readiness input; Objective Readiness Evidence is informational only; confirmation may never override a blocking failure (detail in `validation-model.md` §2.2 and `trading-decision-model.md`, governed by existing C1/C3 — no new constraint added). Q-15 resolved; removed as a v0.5 prerequisite, leaving System Context v0.1. | Pending Product Owner |
| 0.5 | 2026-08-12 | Candidate for Ratification | Advanced to Candidate for Ratification per PBA-0006. System Context v0.1 authored (PBA-0004), reviewed (SCR-0001), and added to Tier 2 (§8); N-17 / N-18 alignment completed and written through the open-question register; Product Model alignment completed (PBA-0005). **No product philosophy, execution semantics, or validation semantics were changed by the version advancement itself** — governance/version advancement only. Ratification (v1.0 ACTIVE) still requires explicit Product Owner approval, not yet granted. | Pending Product Owner |
| 1.0 | 2026-08-12 | Active — Ratified | **Ratified by the Product Owner via POR-0001.** Advanced from v0.5 Candidate to ACTIVE. This is the ratification decision itself — **no Product Principle, no C1–C10 semantics, no boundary test, no neutrality rule, no learning boundary, and no precedence semantics were changed.** Lower-tier documents were not auto-promoted. Deliberately tracked open questions remain open. | **Product Owner (POR-0001)** |
