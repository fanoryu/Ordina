# ADR-0004: Strategy Well-Formedness Evaluation

- **Status:** ACCEPTED
- **Date:** 2026-08-12
- **Accepted by:** Product Board, PBR-0009
- **Decision date:** 2026-08-13
- **Deciders:** Principal Engineer; accepted by Product Board
- **Related:** ADR-0002 (ACCEPTED, immutable), ADR-0003 (ACCEPTED, immutable),
  RFC-0001 (ACCEPTED), ARW-0003 (discovery), PBA-0023 (authorization); reviews
  ARR-0005 → ER-0023 → ARR-0006 (PASS) → ER-0024
- **Governed by:** Product Constitution v1.0 — ACTIVE — RATIFIED

> This ADR decides **how Ordina determines that a canonical Strategy is
> Well-Formed** — the deterministic, model-free, pre-use gate whose failure means
> **UNUSABLE**. It selects **no** parser, expression language, solver, theorem
> prover, evaluator, serialization, persistence, language, framework, provider, or
> UI, and resolves none of N-22 / N-24 / N-25 / Q-06 / Q-08 / F-8. It does not
> reopen ADR-0002 or ADR-0003. **UNUSABLE is not READY / WAIT / DO NOT TRADE.**

---

## 1. Context

RFC-0001 §12 (N-19) makes Strategy Well-Formedness a named product-level pre-use
gate: a Strategy that fails it is **UNUSABLE** and may not enter the runtime
Decision Validation lifecycle; Ordina reports defects and never repairs them.
ADR-0002 fixes the invariants Well-Formedness must satisfy: deterministic and
model-free (§12/E1/C3); only constructs whose every required WF analysis is
**decidable** may enter the canonical model (§10c); Direction sufficiency/
coverage/conflict are static WF concerns; and WF is one of the four separated
layers (syntactic / structural / semantic WF / runtime). ADR-0003 fixes the
artifact-level ordering **Well-Formed → canonicalize → identity** (§5) and makes
WF a precondition to revision identity (I9).

What remains, and what this ADR decides, is the **architecture that performs the
check**: the checking-model family, how §10c's admission boundary is
operationalized (including tractability), the mandatory check set, the evaluation
ordering, the deterministic result/diagnostic model, the checker-semantics
version, and the checker/runtime congruence property. ARW-0003 is the discovery
basis and the PBA-0023 rulings on W-1…W-5 are binding here.

## 2. Decision

**We will evaluate Strategy Well-Formedness with a staged compositional
validation pipeline: an ordered set of dependency-respecting stages that compose
typed/structural checking, graph/reference validation, decision-table Direction
analysis, and compositional semantic validation, producing a deterministic,
reproducible, multi-defect result whose authoritative outcome is
Well-Formed or UNUSABLE.**

The pipeline is:

- **deterministic and model-free** — no AI, score, or probability on the WF path
  (C3/E1); the same canonical content and the same checker-semantics version
  (§11) always yield the same verdict and defect set;
- **non-bypassable** — a Strategy cannot reach canonicalization, revision
  identity, or runtime without passing WF (ADR-0003 I9; §15);
- **bounded** — every admitted construct's required analyses are not only
  decidable but guaranteed to terminate within a declared bounded-analysis model
  (§4, W-3);
- **congruent** — WF analyzes the same canonical semantics a conforming runtime
  evaluator will use (§12, W-4);
- **implementation-independent** — it selects no technology (§17).

The alternatives analysis (§16) independently justifies this family; the evidence
supports it, so no STOP is raised.

## 3. Architectural Properties Preserved

Deterministic evaluation; model-free WF; **WF failure = UNUSABLE**, distinct from
READY/WAIT/DO NOT TRADE (runtime statuses owned by the Validation Model); no
AI/score authority on the WF path; non-bypassable gate; decidable canonical
construct semantics (ADR-0002 §10c); bounded/tractable required analyses (§4);
deterministic and reproducible diagnostics (§10); implementation independence;
compatibility with ADR-0003 revision identity (WF is its precondition, I9).

## 4. Construct-Admission Model (operationalizes ADR-0002 §10c; W-3)

Each construct is dispositioned into exactly one of four classes:

1. **Structurally checkable** — decided by shape/type/reference rules that always
   terminate.
2. **Semantically analyzable** — decidable over the declared logical partitions
   under the admitted fragment (ADR-0002 N-23), **and** bounded per below.
3. **Genuinely runtime-only** — a property that depends on **live evidence or
   runtime state** (whether declared evidence occurs; live values; temporal
   satisfaction). Excluded from WF.
4. **Inadmissible** — rejected, never deferred.

**Tractability admission (architectural, W-3; corrects ARR-0005 M-1).** A construct
is admissible only if every Well-Formedness analysis required for it is:

1. **decidable** under its declared semantics;
2. **terminating**; and
3. **tractability-bounded** — computable by a **bounded compositional traversal of
   the declared canonical construct**, whose work is bounded by a function that is
   at most **polynomial in the declared canonical construct size** (the count of
   declared nodes, rules, conditions, predicates, references, and declared
   partition boundaries).

Decidability + termination + finite structural boundedness are **necessary but not
sufficient**: a finite bound that grows combinatorially in the declared construct
(e.g., enumerating the up-to-`2^k` implicit region combinations induced by `k`
declared boundaries) is **not** tractability-bounded and is therefore
**inadmissible**. The admitted fragment must be constrained so every mandatory
analysis is expressible as a bounded compositional traversal — analysis proceeds
over the **declared** structure and the **declared** partition boundaries
themselves, and **must not enumerate an undeclared/implicit combinatorial state
space** (such as the power-set of partition regions). Where a required analysis can
only be expressed by such combinatorial enumeration, the construct is inadmissible,
**not** deferred to runtime and **not** merely UNUSABLE.

The admission decision is **deterministic and conformance-testable** from the
canonical construct and the applicable Canonical Semantic Contract (§9): two
independent conforming checkers, given the same canonical construct and contract,
decide admissibility identically. This growth-class rule is **architectural**, not
an implementation resource budget.

This ADR does **not** select implementation-specific resource budgets — no
wall-clock timeout, CPU/memory budget, worker/thread count, concrete complexity
threshold, solver configuration, or deployment quota. Those are implementation
decisions. The tractability bound governs the **shape and growth** of the required
analysis as a function of the declared construct, independent of how any
implementation schedules it.

**Runtime-only is not an escape hatch.** A property is class 3 only if it is
*genuinely* dependent on live evidence/state. An undecidable or architecturally
unbounded required WF analysis makes the construct **inadmissible** (class 4) —
it is never re-labelled runtime-only to avoid checking it.

**Admission vs instance validation are distinct (corrects ARR-0006 f-2).** Two
dispositions must not be conflated:

- **(A) Construct-language / semantic-fragment admission.** If a *construct type or
  semantic form* cannot satisfy the decidability / termination / tractability
  admission requirements above, it is **INADMISSIBLE** — it does not enter the
  admitted canonical Strategy fragment and is not deferred to runtime. This is a
  §4 concern, evaluated on the construct *type*, prior to Well-Formedness.
- **(B) Strategy instance Well-Formedness.** If the construct *is* admitted but a
  concrete Strategy instance violates a mandatory invariant — including an
  applicable statically-decidable contradiction — that is a **mandatory
  Well-Formedness defect → Strategy UNUSABLE** (§5), **not** "inadmissible."

A contradictory *instance* of an admitted construct is UNUSABLE; only an
un-admittable *construct type* is inadmissible.

## 5. Mandatory WF Check Model

The pipeline enforces the following **mandatory WF invariants** — any failure
yields **UNUSABLE**. Each is tagged with its admission class (§4).

| # | Check | Class |
|---|---|---|
| 1 | Required-field completeness | structural |
| 2 | Stable identity / reference integrity | structural |
| 3 | Condition-classification completeness (exactly one, static) | structural |
| 4 | Configuration bounds validity | structural |
| 5 | Configuration **structure-preservation** (config cannot restructure — ADR-0003 §6.2) | semantic |
| 6 | Evidence-declaration integrity | structural |
| 7 | Evidence relationship / order validity | semantic |
| 8 | **Direction coverage** (every reachable required state resolves) | semantic |
| 9 | **Direction conflict / precedence validity** (co-applicable Direction rules are resolved by mutual exclusivity **or** a precedence relation that is **strict/irreflexive, total over the co-applicable set, acyclic, and tie-free** — ADR-0002 §10a; corrects m-2). A cycle, an unresolved co-applicable pair, or an equal-precedence tie is a mandatory defect → UNUSABLE. Runtime never repairs or invents precedence. | semantic |
| 10 | **Direction determinism** (exactly one outcome) | semantic |
| 11 | Unreachable operative rules | semantic |
| 12 | Contradictory operative rules | semantic |
| 13 | Candidate-value derivation validity | semantic |
| 14 | Dimensional / unit consistency (ADR-0003 §6) | semantic |
| 15 | Required provenance presence | structural |
| 16 | Prerequisites for canonicalization / revision identity (ADR-0003 I9) | structural |
| 17 | **Applicability-instance validity** (corrects m-1, f-2). Given an **admitted** applicability construct (admission itself is a §4 concern — see below), the concrete Strategy instance must be **well-typed**, have **valid references**, be **semantically admissible**, and be **free of statically-decidable structural contradiction**. A contradictory or invalid admitted instance is a **mandatory WF defect → UNUSABLE** — an ordinary contradictory instance is **not** called "inadmissible" merely because its contradiction was detected. Runtime evidence satisfaction is **not** moved into WF. | structural / semantic |

**Optional diagnostics** (advisory only — e.g., style, unused declarations,
quality hints) may be produced but **must never change the authoritative
Well-Formed / UNUSABLE verdict.** They are diagnostic metadata, not WF invariants.

## 6. Evaluation Ordering

Staging is justified by genuine analysis dependencies (a later stage's analysis is
meaningless if a prerequisite fails), not preference:

```
Stage 1  structure / required fields                       (checks 1, 6, 15, 16)
Stage 2  references & types                                (check 2)
Stage 3  classification & configuration integrity          (checks 3, 4, 5)
Stage 4  evidence declarations & relationship/order        (checks 6*, 7)
Stage 5  Direction analysis (determinism, coverage,        (checks 8, 9, 10, 11, 12)
         conflict, reachability, contradiction)
Stage 6  candidate derivations & dimensional consistency   (checks 13, 14)
Stage 7  aggregate Well-Formedness result                  (§10)
```

(* evidence-declaration integrity is structural at Stage 1; its *relationship/order*
semantics are Stage 4.) Stage *n* may depend on invariants established by earlier
stages (e.g., Direction analysis presupposes valid references and classifications).
The conceptual stages may be refined at review; no implementation technology is
selected.

## 7. W-1 — Well-Formedness vs Canonicalization Boundary

**ADR-0003's artifact-level ordering is authoritative and unchanged:**
Well-Formed → canonicalize → revision identity. **Canonicalization of the Strategy
artifact is NOT a precondition for determining Well-Formedness**, and this ADR
takes no canonicalization ownership from ADR-0003.

Where a WF *analysis* requires normalizing sub-expressions to be deterministic
(e.g., comparing two Direction predicates for overlap/contradiction), the pipeline
may use a **local analysis normalization** that:

- is **deterministic**;
- is **local to the specific analysis** (a means of comparison inside a check);
- **does not** produce the Strategy revision identity;
- **does not** constitute canonical Strategy serialization;
- **does not** supersede or re-implement ADR-0003 canonicalization;
- **does not** invert the authoritative artifact-level ordering.

Local analysis normalization is explicitly a *different thing* from canonical
Strategy canonicalization (ADR-0003 §6): the former is an internal, per-analysis
comparison aid that yields no identity and no persisted form; the latter is the
authoritative artifact-level normalization owned by ADR-0003. It must **not** mutate
Strategy semantics, produce revision identity, produce canonical serialization, or
become a second canonicalization system.

**Verdict-affecting normalization is contract-bound (corrects ARR-0005 M-3).** Any
local analysis normalization capable of changing **construct admission (§4)**, a
**mandatory WF verdict**, **mandatory defect existence**, **Direction
overlap/conflict/coverage**, or **mandatory defect identity or ordering** MUST be
normatively specified by, and version-bound to, the applicable **Canonical Semantic
Contract** (§9) and **checker-semantics version** (§8). Two conforming checkers are
**not** free to choose different verdict-affecting normalizations: at a given
checker-semantics version, all conforming checkers apply the same contract-specified
normalization and therefore reach the same authoritative verdict, defect set, and
defect ordering. Only local transformations **provably incapable** of affecting any
authoritative WF result may remain non-authoritative implementation detail. If a
conforming implementation can perform the required analyses without any
sub-expression normalization, it may; the property that matters is contract-bound,
cross-implementation-deterministic analysis.

## 8. W-2 / W-11 — Checker-Semantics Version

The Well-Formedness result depends on the canonical content, the semantic-model
version (ADR-0002), and a **Well-Formedness-checker semantics version** — a change
to the checker's semantics can change a verdict. This ADR establishes the
checker-semantics version as an architectural concern and requires that **a WF
result be bound to the checker-semantics version that produced it** so the result
is reproducible (mitigates W-R4).

**The checker-semantics version is conceptually DISTINCT from the runtime
evaluator-semantics version tracked by N-24** (per the PBA-0023 W-2 ruling). This
ADR owns the *checker* version; **N-24 remains OPEN and owns the runtime evaluator
version.** This ADR does **not** resolve N-24, and does not select the version
syntax, persistence format, serialization, or schema — only the architecture-level
requirement that the version exist and bind to the result.

**Version scope (corrects ARR-0005 checker-version gap).** The checker-semantics
version MUST cover **every semantic rule whose change can alter an authoritative WF
result**, including at minimum: construct-admission rules; tractability/admission
semantics (§4); mandatory-invariant semantics (§5); verdict-affecting local
normalization (§7); Direction-analysis semantics; applicability analysis (§5);
defect aggregation and prerequisite/dependency rules (§11); defect identity (§10);
and canonical defect ordering (§10). A change to **any** such authoritative
semantics requires an applicable checker-semantics version change. The
checker-semantics version is a conforming realization of the Canonical Semantic
Contract (§9) and remains conceptually distinct from N-24.

**Contract-version coupling (corrects ARR-0006 f-1).** The checker-semantics
version **encompasses and binds the applicable Canonical Semantic Contract
version**: a change to the applicable Contract that can alter **any** authoritative
Well-Formedness result **necessarily requires a checker-semantics version change.**
The Contract and the checker-semantics version are therefore **not** two
independent semantic authorities — the Contract is the authoritative *meaning* and
the checker-semantics version is a versioned realization *of a specific Contract
version*, so the reproducibility inputs (§10) unambiguously bind the applicable
Contract semantics. This defines no N-24 evaluator-version scheme; **N-24 remains
OPEN**.

## 9. Canonical Semantic Contract & Checker / Runtime Congruence (W-4; corrects ARR-0005 M-2)

### 9.1 Canonical Semantic Contract

We introduce a named architecture-level concept: the **Canonical Semantic
Contract** — the **authoritative operational semantic contract for every admitted
canonical Strategy construct**. It governs, where applicable:

- predicate meaning; comparison semantics; numeric semantics;
- dimensional/unit semantics; ordering and precedence;
- evidence relationships; Direction derivation/resolution;
- candidate-value derivation; configuration interpretation; defaults;
- unknown/extension-construct behaviour;
- any other construct semantics capable of affecting Well-Formedness **or** runtime
  interpretation.

The Canonical Semantic Contract is **not** an evaluator implementation, a parser, a
shared library, a VM, a solver, a programming language, or a serialization format.
It is the **meaning** that any conforming component must honour, decided at the
architecture level; its concrete expression, storage, and versioning syntax are
downstream and unselected here.

### 9.2 Congruence invariant

Both:

1. the **ADR-0004 checker-semantics version**, and
2. the future **N-24 evaluator-semantics version**,

MUST be **conforming realizations of the same applicable Canonical Semantic
Contract.** A conforming Well-Formedness checker and a conforming runtime evaluator
therefore interpret every admitted construct **congruently**; a Strategy must
**never** pass Well-Formedness under semantics incompatible with those used by
runtime evaluation. **Checked semantics ≠ evaluated semantics is
architecture-forbidden.**

This defines the object the two components are congruent *with* — the Canonical
Semantic Contract — without selecting the evaluator, a shared library, or the
evaluator-semantics-version scheme. **N-24 remains OPEN** and continues to own the
evaluator-semantics *versioning* decision; this ADR fixes only that both versions
realize one contract and that incongruence is forbidden.

## 10. Result and Diagnostic Model

Architecture-level result contract:

- **Authoritative outcome:** binary **Well-Formed** or **UNUSABLE** (no gradations
  of validity; UNUSABLE is never a runtime status).
- **Deterministic defect set:** same canonical content + same checker-semantics
  version ⇒ identical defect set.
- **Deterministic defect identity (corrects m-3; f-3).** The authoritative defect
  identity is a **deterministic function of (offending canonical semantic construct
  identity — and, where structurally needed, its canonical semantic position/path
  under the accepted semantic model — defect category, applicable Strategy revision
  identity)**. It **must not** depend on source-file byte offset, line number,
  editor position, serialization-specific location, or presentation location; the
  construct identity is the ADR-0003 canonical lineage/rule identity, never a
  physical/document location. No serialization/path encoding is selected here.
  Where a **single construct can produce multiple distinct defects of the same
  category**, an **explicitly-defined additional discriminator** applies (e.g., an
  ordinal over the construct's declared sub-elements). Consequently: the same authoritative
  defect under the same revision → the **same** identity; different offending
  constructs → **distinguishable**; different categories → **distinguishable**; a
  revision change → **revision-qualified** identity; and copy/clone behaviour is
  consistent with ADR-0003 lineage/revision semantics (a clone's new lineage yields
  distinct revision-qualified defect identities). Verdict-affecting local
  normalization (§7) is contract-bound, so conforming checkers **must not disagree**
  on the authoritative offending construct.
- **Offending construct / location:** each defect anchors to the offending
  semantic construct, using ADR-0003 **lineage/rule identity**,
  **revision-qualified** where the reference is to operative semantics (ADR-0003
  §9).
- **Multiple simultaneous defects** supported.
- **Deterministic / canonical ordering rule** for the defect set (a total,
  deterministic order so two conforming checkers present defects identically).
- **Reproducibility:** the result is reproducible from {canonical content,
  semantic-model version, checker-semantics version} — where the checker-semantics
  version encompasses and binds the applicable Canonical Semantic Contract version
  (§8, §9), so the applicable Contract semantics are unambiguously part of the
  reproducibility inputs.
- **Checker-semantics-version binding** (§8).

UI presentation is **not** defined here; **N-22 remains OPEN** and owns it.

## 11. W-5 — Multi-Defect Aggregation

The architecture prefers **maximum deterministic diagnostic information** without
performing speculative analysis on invalid prerequisites. The blocking criterion is
**per-check prerequisite satisfaction, not stage membership (corrects m-4):**

- **A mandatory check is evaluated whenever all of *that check's own* semantic
  prerequisites are satisfied.** Failure of an unrelated check, or a defect in an
  unrelated construct, **must not** suppress an independently-computable mandatory
  check — even if the two happen to sit in the same or different conceptual stages.
- **All independently-computable mandatory defects are collected** (not fail-fast),
  yielding the complete deterministic defect set.
- **A dependent check whose own prerequisites fail is not evaluated** and is
  recorded deterministically as **diagnostic metadata**
  (`not-evaluated: prerequisite <defect-id> failed`), **not** as an invented
  defect — a skipped check neither passes nor fails.
- The **seven conceptual stages (§6) remain organizational/dependency groups**, but
  **stage membership alone is not the blocking criterion**; the operative gate is
  the specific check's prerequisites.
- The **authoritative verdict** is UNUSABLE if any **evaluated** mandatory invariant
  fails; skipped checks can **never** turn an invalid Strategy into Well-Formed.
- Aggregation is deterministic (§10 defect identity and canonical ordering), so the
  same canonical content + checker-semantics version yields the same defect set,
  identities, ordering, and skipped-check metadata.

No UX is designed.

## 12. Security / Integrity

Architecture-level properties (mechanisms deferred):

| Threat | Property |
|---|---|
| Malformed canonical artifacts | The pipeline rejects deterministically and must be robust to adversarial structure — a malformed artifact yields UNUSABLE, never a crash or bypass. |
| Validation bypass | WF is a **non-bypassable** gate; no path to canonicalization/identity/runtime skips it (ADR-0003 I9 + structural enforcement). |
| Pathological but decidable inputs | Bounded-analysis admission (§4, W-3): unbounded-analysis constructs are inadmissible → no resource-exhaustion vector. |
| Cross-implementation checker divergence | WF semantics specified deterministically enough that conforming checkers agree (a determinism/totality requirement, analogous to canonicalization). |
| Nondeterministic diagnostics | Deterministic defect set + canonical ordering (§10). |
| Reference / identity spoofing | Reference-integrity and identity checks (checks 2, 15, 16). |
| Configuration-based semantic restructuring | Check 5 enforces ADR-0003 §6.2. |
| Checker / evaluator semantic divergence | Congruence invariant (§9). |

No security product or implementation technology is selected.

## 13. Consequences

**Positive:** heterogeneous WF obligations are all covered by one coherent,
deterministic, explainable, reproducible pipeline that mirrors ADR-0002's hybrid
model and ADR-0003's compositional canonicalization; the UNUSABLE gate is
non-bypassable and technology-neutral; the bounded-analysis admission closes the
resource-exhaustion vector at the architecture level. **Negative / trade-offs:**
the staged compositional pipeline is the most complex of the candidate families
(§16); the boundedness and congruence properties impose real constraints on what
the future expression fragment and evaluator may do; a checker-semantics version
adds a reproducibility obligation. **Reversibility:** moderately costly — WF is
the gate every Strategy passes, and downstream (ADR-0003 identity, runtime
evaluation) depends on its verdict; changing the family later would require
re-validating existing Strategies, hence the conservative, property-based choice.

## 14. Alternatives Considered

| Family | Why not selected as the whole architecture |
|---|---|
| **Typed constraint checking only** | Strong for structural/type/reference checks; cannot, alone, decide Direction coverage/conflict/determinism (checks 8–10, 12). |
| **Graph / relationship validation only** | Strong for references, reachability, ordered relations (checks 2, 7, 11); no first-class Direction-coverage analysis. |
| **Decision-table coverage/conflict only** | The natural fit for Direction coverage/conflict/completeness (checks 8–10), matching ADR-0002's decision-table Direction construct; weak for temporal/sequence/structural checks. |
| **Compositional semantic validation only** | Fits the typed model and derivations (checks 13–14); on its own lacks a dedicated Direction-coverage technique and clear staging. |
| **Staged compositional pipeline (SELECTED)** | Composes the strengths of all of the above as dependency-ordered stages; covers every mandatory check; deterministic, explainable, reproducible; mirrors ADR-0002/0003. Cost: highest structural complexity (§13). |

No single pure family covers the heterogeneous mandatory check set; the hybrid
staged/compositional pipeline is superior for the accepted semantic model.

## 15. Risks (recorded here; not written to the canonical register in this task)

- **W-R1 — WF/runtime semantic divergence.** A Strategy passes WF but the
  evaluator interprets it differently. *Mitigation:* the §9 congruence invariant;
  shared canonical semantic model and admitted construct set.
- **W-R2 — Intractable-but-decidable analysis.** Pathological decidable Strategies
  exhaust resources. *Mitigation:* the §4 bounded-analysis admission; such
  constructs are inadmissible.
- **W-R3 — Cross-implementation checker divergence.** Different checkers disagree
  on WF. *Mitigation:* deterministic WF specification + canonical defect ordering
  (§10, §12).
- **W-R4 — Checker-semantics-version drift.** A checker change silently changes WF
  verdicts. *Mitigation:* checker-semantics version bound to the result (§8).
- **W-R5 — WF bypass.** A Strategy reaches identity/runtime without passing WF.
  *Mitigation:* non-bypassable gate (§12; ADR-0003 I9).

## 16. Deferred Decisions

Not selected/resolved here: parser, expression language, solver, theorem prover,
programming language, framework, serialization format, concrete hash/digest
algorithm, persistence/database, evaluator implementation, deployment model,
provider/cloud, UI, authoring-to-canonical translation. Not resolved: **N-22**
(WF-failure UX), **N-24** (evaluator-semantics version), **N-25** (authoring
translation), **Q-06** (platform), **Q-08** (jurisdiction), **F-8** (privacy/
preimage). ADR-0003 retains ownership of canonicalization/revision identity;
ADR-0002 retains the semantic model and §10c admission invariant.

## 17. Validation / Acceptance Criteria

1. One checking-model family selected (staged compositional pipeline) and
   independently justified (§14).
2. WF failure = **UNUSABLE**, never READY/WAIT/DO NOT TRADE; four-layer separation
   preserved.
3. §10c operationalized (§4): structurally-checkable / semantically-analyzable /
   runtime-only / **inadmissible**, with **decidability + bounded-analysis**;
   undecidable or unbounded required analyses → inadmissible, never deferred.
4. Boundedness is **architectural**, not an implementation resource budget (§4).
5. Mandatory WF check set specified (§5); mandatory-invariant vs optional-diagnostic
   split explicit; diagnostics never change the verdict.
6. Deterministic, reproducible result with stable defect category/identity,
   offending-construct location (revision-qualified), multi-defect support, and a
   canonical defect ordering (§10); bound to the checker-semantics version (§8).
7. Checker/runtime **congruence** established as a property without selecting the
   evaluator (§9); N-24 not resolved.
8. W-1 resolved without ownership inversion (§7); ADR-0003 ordering and
   canonicalization ownership intact.
9. Evaluation ordering justified by dependencies; multi-defect policy defined
   (§6, §11).
10. Non-bypassable, deterministic, bounded, congruent (§12).
11. No parser/language/solver/prover/serialization/persistence/evaluator/UI
    selected; N-22/N-24/N-25/Q-06/Q-08/F-8 preserved.
12. ADR-0002/0003 not reopened.

## 18. Acceptance Record

Accepted by the Product Board via **PBR-0009**, decision date **2026-08-13**,
after ARR-0005 (independent review), ER-0023 (bounded remediation closing
M-1/M-2/M-3 and m-1…m-4), ARR-0006 (PASS — READY FOR ACCEPTANCE), and ER-0024
(editorial remediation closing f-1/f-2/f-3). o-1 (enforcement mechanism /
checker-version persistence) remains an intentionally deferred downstream item and
does not block acceptance.

**Accepted architecture:** the **staged compositional Strategy Well-Formedness
evaluation** pipeline; the construct-admission boundary with **decidability /
termination / tractability** admission (the polynomial / bounded-compositional-
analysis envelope, no combinatorial enumeration); the mandatory Well-Formedness
invariants and the mandatory-vs-diagnostic split; the **INADMISSIBLE construct vs
UNUSABLE Strategy-instance** distinction; the **Canonical Semantic Contract**;
**checker-semantics versioning bound to the applicable Canonical Semantic Contract
version**; verdict-affecting normalization constraints; deterministic Direction
coverage / conflict / precedence analysis; applicability-instance validation;
deterministic defect identity and canonical ordering; per-check
prerequisite/dependency gating; cross-implementation authoritative Well-Formedness
determinism; and the boundary between Well-Formedness and runtime Decision
Validation.

**Explicitly preserved:** UNUSABLE is not READY / WAIT / DO NOT TRADE; the
ADR-0003 ordering **Well-Formed → canonicalize → revision identity**; ADR-0002 and
ADR-0003 ownership; runtime evidence satisfaction remains runtime-only.

**Deferred (unchanged by acceptance):** **N-22** (WF-failure UX), **N-24**
(evaluator-semantics version), **N-25** (authoring translation), **Q-06**
(platform), **Q-08** (jurisdiction), **F-8** (privacy/preimage); and the
implementation choices — parser, solver/prover, expression language, programming
language/framework, serialization format, concrete hash algorithm,
persistence/database, evaluator implementation, deployment/provider, UI. ADR-0002
retains the semantic model and §10c; ADR-0003 retains canonicalization/revision
identity.

---

> ADRs are immutable once Accepted — supersede, never edit. This ADR is now
> **ACCEPTED**; its decision text is immutable historical architecture record. Any
> future material architecture change must be recorded through a **new ADR that
> explicitly supersedes ADR-0004** — never a silent rewrite.
