# RFC-0001: Strategy Formation and Authoring for User Zero

- **Status:** ACCEPTED
- **Author:** Principal Engineer
- **Created:** 2026-08-12
- **Decision date:** 2026-08-12
- **Accepted by:** Product Board, PBR-0003
- **Authorizing artifact:** PBA-0009 (dispositions of N-19, N-20, N-21; Q-16, Q-11b in-scope)
- **Discovery basis:** WR-0004

> **Subordinate to** Product Constitution **v1.0 — ACTIVE — RATIFIED** and to the
> applicable Tier-2 Product Specifications: `trading-decision-model.md`,
> `validation-model.md`, `ux-constitution.md`, `user-zero.md`,
> `system-context.md`. This RFC may refine, but may not contradict, any of them.
> It is a **product-and-contract** RFC: it defines *what a Strategy is* and *the
> constraints on how it is authored*, not how it is represented, stored, or drawn.

---

## 1. Summary

A **Strategy** is the trader-owned object that lets Ordina validate a proposed
decision deterministically. This RFC defines the minimum conceptual **Strategy
contract**, the boundary between what the Strategy owns and what other documents
own, the rule for classifying conditions as blocking or temporary (Q-11b), the
criterion for sufficient Direction logic (N-21), a named pre-use **Strategy
Well-Formedness** gate (N-19), the configuration-versus-revision boundary (N-20),
the revision/reproducibility model, and a **hybrid authoring model** by which
User Zero — a beginner who must not be required to write technical code — can
form a Strategy without Ordina originating it.

It selects no representation, tooling, UI, provider, or threshold.

## 2. Problem Statement

READY requires a named Strategy (`validation-model.md` §6), and Strategy
Validation derives Direction and evaluates ordered evidence (`validation-model.md`
§2, §6.2). But User Zero "does not yet have a mature strategy" and "must not be
required to write technical code" (`user-zero.md` §3, PBD-0002 D11). Nothing yet
defines *what a Strategy must contain*, *how a beginner authors one*, or *how the
system knows a Strategy is even usable*. Building validation before this is
settled risks a rules surface no User Zero can populate (Risk R-12). This RFC
closes that gap at the product/contract level.

## 3. Goals

1. Define the minimum conceptual Strategy contract.
2. Assign ownership for every contract concept without duplicating normative text.
3. Resolve Q-11b (condition classification).
4. Define sufficient-Direction criteria (N-21).
5. Define Strategy Well-Formedness as a named pre-use gate (N-19).
6. Define the configuration-vs-revision boundary (N-20).
7. Define the revision / historical-reproducibility model.
8. Resolve Q-10b to the extent needed for a Strategy to declare required evidence.
9. Recommend a product-level authoring model for User Zero (Q-16), with
   constitutional boundary guards.
10. Provide boundary compliance, risks, alternatives, acceptance criteria, and
    downstream architecture questions.

## 4. Non-Goals

This RFC does **not** define: a DSL; JSON/YAML/graph schema; database or storage
mechanism; rules-engine, compiler, or execution engine; programming language or
framework; UI screens or concrete interaction flows; market-data or broker
providers; deployment; freshness thresholds (Q-10a); actual trading-strategy
content; numerical trading thresholds; or risk limits. Status semantics remain
owned by the Validation Model; lifecycle semantics by the Trading Decision Model.

## 5. Governing Product Constraints

- **Four pillars** (C2): trader pre-commitment, deterministic derivation,
  traceable provenance, trader agency.
- **C3 / E1**: rules validate; AI explains. No AI, score, or probability on the
  status-resolution or Direction-derivation path.
- **C4 / E2**: every status and every calculated value carries provenance
  retained as validation runs.
- **C5 / E3**: historical decisions remain auditable; corrections preserve
  provenance; the past is never silently rewritten.
- **C6**: learning may propose rule changes for human review; never self-amends.
- **C8/C9/C10**: two-stage construction; Modify Inputs invalidates READY;
  pre-execution checkpoint.
- **D11 / User Zero**: a Strategy must express ordered conditions without
  requiring User Zero to write technical code; no push into long-term dependency
  on externally provided strategies (WN-0002).

## 6. Terminology

Terms follow `glossary.md`. This RFC introduces, for Product Board adoption:

- **Strategy Well-Formedness** — a deterministic, rule-based pre-use gate that
  decides whether a Strategy is *validly authored and therefore usable*. Distinct
  from Decision Validation (§12). Board-resolved as a product concept (N-19).
- **Strategy Revision** — an immutable identified version of a Strategy; any
  material change creates a new one (§13).
- **Operative rule** — a rule that participates in Direction derivation, evidence
  evaluation, condition classification, or candidate-value derivation. Operative
  rules must be explicit, deterministic, inspectable, and trader-confirmed.
- **Condition classification** — the static blocking/temporary label a Strategy
  attaches to each condition (§10).

## 7. Strategy Contract

A **Strategy** is a trader-owned, named, revision-identified decision plan that
deterministically maps validated market evidence to a Direction, candidate trade
parameters, and a set of classified conditions the Validation Model resolves into
a status. The minimum contract:

**Identity & metadata**
- identity; human-readable name; purpose/thesis (**informational only** — must
  never reach status resolution or Direction); revision identity; change
  provenance.

**Applicability**
- instrument applicability as a **filter** on the trader-selected instrument
  (never a selector — §16).

**Evidence requirements**
- required Market Context; required market evidence; evidence relationships and
  ordered-evidence requirements; per-item provenance requirements; a **reference**
  to the applicable freshness policy (not thresholds).

**Ordered rules** (the D11 logic)
- Context, Location, Sequence, Timing, Confirmation rules.

**Direction**
- deterministic Direction derivation; the Strategy's sufficient-Direction
  criteria as instantiated (§11, §15).

**Conditions**
- blocking-condition declarations; temporary-condition declarations (§10). Each
  condition carries exactly one static classification.

**Candidate values**
- deterministic derivation of candidate Entry, Stop Loss, Take Profit (consumed
  by Preliminary Trade Construction, `trading-decision-model.md` §5).

**Configuration**
- permitted configuration parameters and, for each, the **declared adjustable
  bounds** and whether it is trader-adjustable (§14).

**Integrity**
- the provenance metadata the Product Model requires (C4); the Well-Formedness
  requirements it must satisfy (§12); its revision/reproducibility obligations
  (§13).

## 8. Ownership Boundaries

No concept below restates another owner's normative content.

| Concept | Owner |
|---|---|
| Identity, name, purpose/thesis, revision id, change provenance | **Strategy** |
| Instrument applicability (filter) | **Strategy** (boundary-guarded, §16) |
| Required context / evidence (declaration) | **Strategy declares** |
| Evaluation of context / evidence | **Validation Model** |
| Context / Location / Sequence / Timing / Confirmation rules | **Strategy** |
| Direction derivation logic | **Strategy** |
| Direction *is a Strategy-Validation output* | **Validation Model** (§2.1) |
| Sufficient-Direction *criterion* (general) | **This RFC / Validation-Model-adjacent** (§15) |
| Incomplete-direction *runtime behavior* | **Validation Model** (`STRATEGY_INCOMPLETE_DIRECTION_LOGIC`, §2.1/§4) |
| Condition classification (declaration) | **Strategy declares** |
| Status resolution (blocking→DO NOT TRADE, temporary→WAIT) | **Validation Model** |
| Candidate Entry/SL/TP derivation logic | **Strategy** |
| Construction stages & gating | **Trading Decision Model** (§5, §7) |
| Data availability vs. validity behavior | **Validation Model** (§6.1) |
| Freshness policy & thresholds | **Validation Model / Q-10a** |
| Risk limits, Risk Profile | **Risk Profile** (not the Strategy) |
| Provenance retention mechanism | **Engineering** (E2) |
| Well-Formedness *concept & taxonomy* | **This RFC (product concept)** |
| Well-Formedness *checking mechanism* | **Engineering** |
| Revision identity & reproducibility *requirement* | **Strategy / Product Model** |
| Revision *storage* | **Engineering / deferred (architecture)** |
| Persistence, representation, tooling | **Deferred — architecture/ADR** |

## 9. Authoring Model

Three layers, kept separate (WR-0004, accepted by PBA-0009):

- **A — Product semantics:** the operative decisions the trader must make (ordered
  rules, Direction logic, condition classifications, candidate-value derivations,
  applicability, configuration). **Owned by this RFC.**
- **B — Authoring interaction:** how Ordina helps the trader express layer A.
  **Product-level constraints owned by this RFC; concrete UI owned by the UX
  Constitution / design.**
- **C — Machine representation:** how layer A is stored/encoded. **Deferred —
  architecture/ADR.**

**Recommendation — a hybrid authoring model.** Any assistive mechanism (guided
wizard/questionnaire, template, visual builder, natural-language assistance) may
help the trader *express* a Strategy, subject to constitutional guards:

1. the **trader remains the Strategy author and owner**;
2. every operative rule becomes **explicit**;
3. every operative rule is **inspectable**;
4. every operative rule is **deterministic**;
5. every operative rule is **confirmed by the trader** before it becomes
   authoritative;
6. assistive natural-language text is **never** the authoritative Strategy — the
   explicit deterministic rules are;
7. AI assistance may **clarify or translate expression** but gains **no decision
   authority** (C3);
8. Ordina **must not** silently supply discretionary Strategy content.

**Templates** may exist only as explicit, inspectable starting points the trader
deliberately adopts and owns — never as a strategy Ordina assigns, and never in a
way that fosters long-term dependency (WN-0002).

Rationale for hybrid over the alternatives is in §18. This RFC recommends the
*approach*; it does not specify the UI.

## 10. Q-11b — Condition Classification

**Resolution (design baseline, PBA-0009 §3):**

- every Strategy condition has **exactly one** declared classification —
  *blocking* or *temporary*;
- classification is set **at authoring time** and is **static** for that Strategy
  revision;
- **runtime evidence determines condition satisfaction, not condition class**; a
  condition may not compute its own classification from runtime evidence (a
  genuine "temporary early, blocking later" need is expressed as **two declared
  conditions**, not one self-mutating one);
- changing a classification requires a **Strategy revision** (§13, §14).

Resolution into status remains the Validation Model's:

```
unmet temporary condition   → Validation Model may resolve WAIT
unmet blocking condition    → Validation Model may resolve DO NOT TRADE
```

The Strategy **declares** classifications; it does **not** redefine READY / WAIT /
DO NOT TRADE. Well-Formedness (§12) rejects unclassified, multiply-classified,
contradictory, ambiguous-precedence, and (where deterministically detectable)
structurally unreachable or invalid rule combinations.

## 11. Direction Derivation

- Direction is a deterministic function of validated evidence to **exactly one**
  directional outcome (BUY/SELL or the instrument-appropriate equivalent), derived
  solely from explicit Strategy rules — no AI, score, or opinion (C3/E1).
- **"No Direction yet"** — a well-formed Strategy awaiting required temporary
  evidence — is a legitimate **runtime WAIT**, not a defect.
- **"Structurally insufficient Direction logic"** — the Strategy can never derive
  a Direction for some reachable required state — is a **Well-Formedness failure**
  (`STRATEGY_INCOMPLETE_DIRECTION_LOGIC` at runtime is the last-line guard, but it
  should be caught pre-use).
- **Conflicting Direction rules** (same relevant state yields BUY and SELL): an
  **authoring-invalid** condition rejected by Well-Formedness (primary), and
  **runtime-blocking** as defense in depth.
- Direction **provenance** — the rule(s) fired and the evidence consumed — is
  retained and exposed via the "Why?" path (C4).
- Direction is **reproducible** from {validated evidence + exact Strategy revision
  + rule fired}.

## 12. Strategy Well-Formedness

**Board-resolved as a named product-level concept and pre-use gate (N-19).**

Strategy Well-Formedness is a **deterministic, rule-based** (no AI, E1) check a
Strategy must pass **before** it may participate in the Decision Validation
lifecycle. Minimum failure taxonomy:

- missing Direction logic;
- insufficient Direction logic (§15);
- conflicting Direction logic;
- unclassified conditions;
- multiply-classified conditions;
- contradictory conditions;
- unreachable rules (where deterministically detectable);
- ambiguous precedence;
- missing required-evidence declarations;
- invalid references;
- missing Strategy revision identity;
- missing Product-Model-required provenance metadata.

**Failure meaning.** A Well-Formedness failure means *the Strategy is not
currently usable*. It is **not** WAIT and **not** DO NOT TRADE — those are runtime
Decision Validation statuses owned by the Validation Model. Keeping these separate
protects the status vocabulary (R-10) and the determinism boundary.

**Boundary.** Ordina **may report** the defects and **must not silently repair**
Strategy content (§16). The trader revises and resubmits the new revision for
Well-Formedness.

This RFC defines the concept and taxonomy only — **not** a compiler, tooling, DSL,
storage, or execution engine.

## 13. Revision / Reproducibility

Required invariants:

- every **material** Strategy change creates a **new revision** with immutable
  identity and change provenance;
- historical decisions remain **bound to the exact revision used**;
- a later revision **never** retroactively changes a prior decision (C5);
- a revised Strategy **cannot silently replace** the Strategy attached to an
  in-progress decision;
- provenance identifies the Strategy revision used in each decision;
- **historical reproducibility** is required at Product Model level.

Storage/event mechanisms are **deferred** (architecture/ADR).

## 14. Configuration vs Revision — N-20

**Resolution (delegated, PBA-0009 §1):**

- Adjusting an **authorized configuration value within its explicitly declared
  bounds** is a **Decision Input** (Modify Inputs, C9): it invalidates READY and
  triggers complete re-validation, but does **not** by itself create a new Strategy
  revision.
- The following are **Strategy revisions** (new immutable identity):
  - modifying the rule that defines a parameter;
  - modifying the permitted bounds of a parameter;
  - changing any condition classification;
  - changing Direction logic;
  - changing candidate-value derivation;
  - changing applicability;
  - changing any operative rule or ordered-evidence requirement.
- Historical decisions remain bound to the exact revision used regardless of later
  configuration or revision changes.

**Test:** if a change could alter *how the Strategy would classify, direct,
derive, or apply* for **any** future decision beyond selecting a value the
Strategy already authorized, it is a **revision**. Selecting a value within
pre-authorized bounds is a **Decision Input**. This RFC does not choose a
persistence architecture.

## 15. Sufficient Direction Logic — N-21

**Resolution (delegated, PBA-0009 §1):** a Strategy's Direction logic is
**sufficient** iff, for **every reachable validated-evidence state in which a
Direction is required**, the Strategy's explicit rules yield **exactly one**
Direction — or the decision resolves to a non-READY status for a reason **other
than** Direction.

The criterion must prevent:

- a reachable state requiring Direction but producing none due to structural
  incompleteness;
- the same relevant state producing conflicting Directions;
- discretionary AI/model opinion filling a missing Direction rule;
- silent repair of incomplete Direction logic.

It **distinguishes**:

- *no Direction yet because required temporary evidence is incomplete* → legitimate
  runtime **WAIT**;
- *Direction logic is structurally insufficient* → **Well-Formedness failure**.

"Reachable" and "required" are conceptual here; their deterministic detection is
an engineering concern. No implementation syntax or formal-verification tooling is
selected. Where full reachability analysis is not deterministically decidable, the
last-line runtime guard (`STRATEGY_INCOMPLETE_DIRECTION_LOGIC` → DO NOT TRADE)
applies; this RFC does not require solving undecidable cases at author time.

## 16. Boundary / Constitution Check

The proposed model must, and by construction does, prohibit designs where Ordina:

| Prohibited | Guard |
|---|---|
| selects the Instrument for the Trader | applicability is a filter on the trader's instrument, never a selector (§7, §8) |
| selects a Strategy without deliberate adoption | templates are adoptable/inspectable only; no auto-assignment (§9) |
| invents discretionary operative rules | every operative rule explicit, deterministic, trader-confirmed (§9) |
| silently converts NL into authoritative rules without confirmation | NL is assistive only; explicit rules are authoritative; trader confirms (§9) |
| fills missing Direction logic with AI/model judgment | Direction is rule-only; missing logic is a Well-Formedness failure (§11, §15) |
| lets probabilistic output determine Direction or status | C3/E1; no score on the path (§5, §11) |
| silently repairs an ill-formed Strategy | Well-Formedness reports, never repairs (§12) |
| changes condition classification at runtime | classification static per revision (§10) |
| retroactively applies revised logic to historical decisions | revision binding; no retroactive effect (§13) |

**§2.1 five-part boundary test:** *Origination* — the trader authors and owns the
Strategy; Ordina proposes at most, trader confirms. *Determinism* — all operative
logic is rule-based; AI confined to expression/explanation. *Provenance* — rules,
classifications, Direction, and candidate values are all provenanced. *Gating* —
Well-Formedness gates usability; status and construction gates are untouched.
*Restraint* — nothing here implies or guarantees an outcome. **No boundary failure
identified**, provided the §16 guards are normative in the accepted RFC.

## 17. Risks and Failure Modes

Carried internally (register alignment deferred — §9/PBA-0009):

- **R-A** — Authoring assistance drifts into Strategy origination (extends R-02,
  R-09). *Mitigation:* per-rule trader confirmation; explicit rules authoritative.
- **R-B** — Well-Formedness auto-repair temptation. *Mitigation:* report-only,
  constitutionally (§12).
- **R-C** — Condition-classification ambiguity reaches runtime (R-10 adjacent).
  *Mitigation:* Well-Formedness rejects ambiguity pre-use (§10, §12).
- **R-D** — Strategy revision / reproducibility gaps (C5). *Mitigation:* mandatory
  revision binding (§13).
- **R-E** — User-Zero authoring too simple to express D11 ordered evidence.
  *Mitigation:* the contract (§7) is the semantic floor; the *interaction* adapts,
  not the semantics.

## 18. Alternatives Considered

- **Structured manual authoring only.** Rejected as the primary model: high
  determinism but poor User-Zero fit (a beginner cannot reliably hand-author
  ordered-evidence rules) — retained as one expression path within the hybrid.
- **Natural-language strategies as authoritative.** Rejected: highest ambiguity
  and origination risk; violates the "explicit deterministic rules are
  authoritative" guard. NL is permitted only as assistance.
- **Template-only / Ordina-supplied strategies.** Rejected as primary: origination
  and long-term-dependency risk (WN-0002). Templates permitted only as adoptable,
  inspectable starting points.
- **Pure visual builder.** Viable but insufficient alone for User-Zero
  bootstrapping; folded into the hybrid.
- **AI-generated strategies.** Rejected: Ordina would originate the Strategy —
  direct C2 failure.

The **hybrid** model (assistive bootstrap → explicit, deterministic,
trader-confirmed, owned rules) dominates on the combined criteria (WR-0004 §5).

## 19. Open Questions / Deferred Decisions

- **Q-16, Q-11b** — addressed here as **proposed resolutions**; they are **not**
  written into the canonical open-question register by this RFC (that awaits RFC
  acceptance + an authorized register alignment).
- **N-20, N-21** — resolved within this RFC (§14, §15), subject to Board review.
- **N-19** — treated as Board-resolved (product concept).
- **N-22** — whether/how an "unusable Strategy" is surfaced to the trader is a UX
  question (UX Constitution / design).
- **Q-10a** — freshness thresholds remain open; the Strategy only *references* a
  policy.
- **Q-10b** — substantially resolved (Strategy declares required evidence, §7);
  final closure at register alignment.
- **Q-12b, Q-17, Q-04** — constrained (learning-proposed changes route through
  authoring + Well-Formedness + human review + revision; mentor-derived strategies
  are a trader-owned capture path; decision records must carry a revision id) but
  not resolved.

## 20. Acceptance Criteria

For a future implementation to be judged faithful to this RFC:

1. A Strategy exposes every §7 contract element; purpose/thesis never influences
   status or Direction.
2. Every condition carries exactly one static classification; runtime alters
   satisfaction only.
3. Direction is rule-derived, single-valued per reachable required state,
   provenanced, and reproducible.
4. Strategy Well-Formedness is a deterministic pre-use gate whose failure blocks
   lifecycle entry and is never expressed as READY/WAIT/DO NOT TRADE.
5. Ordina never repairs Strategy content; it reports and re-checks.
6. Every operative rule is explicit, inspectable, deterministic, and
   trader-confirmed; no assistive text is authoritative.
7. Material changes create revisions; decisions bind to the exact revision;
   history is reproducible and never retroactively altered.
8. Configuration-within-bounds is a Decision Input; rule/bounds/classification/
   Direction/derivation/applicability changes are revisions.
9. No AI, score, or probability appears on the status or Direction path.
10. Status semantics remain the Validation Model's; lifecycle the Trading Decision
    Model's; the Strategy restates neither.

## 21. Downstream Architecture Questions

For later RFC/ADR work — **not** decided here: machine representation (DSL /
structured data / graph); Well-Formedness checking mechanism and its decidability
limits; provenance and revision persistence (and the C5/R-04 erasure tension);
evidence/freshness policy binding (Q-10a); rules-engine and determinism-testing
approach; the UX of authoring and of "unusable Strategy" (N-22); learning's
rule-change proposal surface (Q-12b).

## 22. Recommendation

Accept this RFC as the product-and-contract basis for Strategy Formation. On
acceptance: (a) align the canonical open-question register (Q-16, Q-11b, Q-10b
resolved; N-19/N-20/N-21 recorded; N-22 opened); (b) add risks R-A…R-E to the
register; (c) treat the machine representation, Well-Formedness mechanism, and
persistence as the next RFC/ADR track. No architecture, tooling, or UI should be
selected until this contract is accepted.

## 23. Product Board Acceptance Record

Accepted by the Product Board via **PBR-0003** (2026-08-12). The Board accepted,
as authoritative within this RFC's scope: the hybrid authoring model and its
boundary (Q-16); the static per-condition classification model (Q-11b); the
Strategy's declaration of required evidence (Q-10b, with Q-10a remaining OPEN);
Strategy Well-Formedness as a named product-level pre-use gate whose failure means
**UNUSABLE** and is never READY/WAIT/DO NOT TRADE (N-19); the
configuration-vs-revision boundary (N-20); the sufficient-Direction criterion and
the *no-Direction-yet* vs *structurally-insufficient* distinction (N-21); the
revision/reproducibility model; the boundary guards (§16); and risks R-A…R-E for
later register write-through.

**Deferred, not done in the acceptance task:** the canonical open-question
register (Q-16/Q-11b/Q-10b resolution, N-19/N-20/N-21 recording, N-22 opening) and
the risk register (R-A…R-E) are **not** yet updated — both await a separately
authorized alignment. **N-22** (UX of an unusable Strategy) remains OPEN.
Representation, tooling, UI, and persistence remain deferred to the downstream
RFC/ADR track (§21).
