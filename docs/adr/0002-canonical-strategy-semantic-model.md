# ADR-0002: Canonical Strategy Semantic Model

- **Status:** ACCEPTED
- **Date:** 2026-08-12
- **Accepted by:** Product Board, PBR-0006 (2026-08-12)
- **Deciders:** Principal Engineer; accepted by Product Board
- **Related:** RFC-0001 (ACCEPTED), ARW-0001 (discovery), PBA-0012 (authorization),
  ARR-0001 → ER-0014 → ARR-0002 (PASS) → ER-0015 (final bounded remediation)
- **Governed by:** Product Constitution v1.0 — ACTIVE — RATIFIED

> This ADR selects a **semantic-model family** for an Ordina Strategy and resolves
> **N-23** (the expressiveness/decidability boundary). It selects **no**
> serialization format, evaluator, persistence, or canonicalization algorithm —
> those are explicitly deferred (§Downstream). **Semantic Model ≠ Serialization
> Format.**

---

## 1. Context

RFC-0001 (ACCEPTED) defines the Strategy contract, the static per-condition
classification model (Q-11b), the sufficient-Direction criterion (N-21), the
named Strategy Well-Formedness pre-use gate (N-19), and the revision/
reproducibility model. ARW-0001 derived the architectural requirements and
compared five semantic-model families without selecting one, and surfaced the
central lever: **static Well-Formedness completeness is a function of how
expressive the rule language is**, and constraining expressiveness is also a
security property.

This ADR is the root architecture decision (ARW-0001 §14, decision 1). Everything
downstream — canonicalization, Well-Formedness mechanism, evaluation,
serialization, persistence — depends on it. We must choose a semantic model that
is expressive enough for RFC-0001 yet deterministic, statically analyzable,
inspectable, provenance-preserving, canonicalizable, and secure against arbitrary
executable logic.

What we know: the RFC contract (§7 there); the four-pillar boundary (C2);
rules-validate-AI-explains (C3/E1); provenance and reproducibility (C4/C5). What
we do not know and must not assume: platform (Q-06), persistence, freshness
thresholds (Q-10a), and the concrete evaluator.

## 2. Decision

**We will represent an Ordina Strategy as a Hybrid Typed Semantic Model: a
typed, declarative composition of analyzable rule constructs over a deliberately
constrained, non-Turing-complete expression fragment.**

Concretely, the model is composed of typed elements — not free text and not
general code:

1. **A declarative Strategy envelope** carrying identity, revision identity,
   applicability (as a filter), declared evidence requirements with ordered
   relationships, provenance references, and freshness-policy references.
2. **Typed condition nodes**, each with a **stable rule identity** and **exactly
   one static classification** (`BLOCKING` or `TEMPORARY`), evaluating a bounded
   typed predicate over declared evidence.
3. **Ordered-evidence constructs** (Context, Location, Sequence, Timing,
   Confirmation) expressed as explicit, typed ordered relations between condition
   nodes — a directed structure where sequence/timing/confirmation demand it —
   so that ordering is part of the model, not an implicit convention.
4. **Direction derivation as an exhaustive, analyzable mapping** governed by the
   **Direction-resolution invariant** (§10a): for every reachable state that
   requires Direction, the declared Direction rules resolve deterministically to
   **exactly one** outcome, achieved only through mutually exclusive predicates or
   an explicit strict total order over co-applicable Direction rules.
   "Decision-table-style" is descriptive
   language for this construct, **not** its definition. Coverage and conflict are
   analyzable over the **declared logical partitions**, not by enumerating raw
   evidence values (§10, §7).
5. **Candidate-value derivations** (Entry, Stop Loss, Take Profit) as bounded,
   deterministic typed expressions — including bounded numeric comparison,
   threshold comparison, range membership, and deterministic bounded arithmetic
   over declared evidence and configuration — with stable identity for provenance.
   These operate over **continuous** declared evidence without requiring it to be
   discretized (§10).
6. **Declared configuration** with typed parameters, **permitted bounds**, and an
   explicit adjustable/non-adjustable flag distinguishing Decision-Input
   configuration from revision-bearing semantics (RFC §14).

This hybrid is chosen because ARW-0001 §5 showed **no single pure family is
dominant**: ordered evidence pulls toward graph/tree, Direction sufficiency pulls
toward decision-table, and inspectability pulls toward declarative. The hybrid
composes the right construct for each requirement inside one typed model, at the
cost of higher structural complexity (§7 Negative).

## 3. Decision Drivers

- **RFC-0001 expressiveness** — must express Context/Location/Sequence/Timing/
  Confirmation, ordered evidence, static BLOCKING/TEMPORARY classification,
  deterministic Direction, candidate Entry/SL/TP, applicability, configuration
  and bounds, provenance and freshness references, and precedence.
- **Determinism** (C3/E1) — evaluation and Well-Formedness must be model-free and
  deterministic.
- **Static analyzability** (N-19, §15 of RFC) — the model must make Direction
  coverage/conflict, contradictions, unreachable rules, and precedence ambiguity
  analyzable to the decidable extent.
- **Provenance** (C4) — rule-level attribution for Direction, condition outcomes,
  and each candidate value.
- **Reproducibility & revision** (C5) — canonicalizable identity; stable rule IDs.
- **Inspectability** (RFC §9) — a human (User Zero) and a reviewer must be able to
  read the operative rules.
- **Security** — a Strategy artifact must not be a general-purpose program.

## 4. Considered Alternatives

Per ARW-0001 §5 (semantic families; serialization excluded):

1. **Structured declarative model.** Highly inspectable, low complexity, good
   diff — but weak at ordered/sequence logic and Direction coverage analysis.
2. **AST / expression-tree model.** Strong determinism, static analysis, and
   rule identity — but weaker raw inspectability and no first-class Direction
   coverage structure.
3. **Directed rule graph.** Strong for sequence/reachability and precedence,
   strong node-level provenance — but high complexity and weaker inspectability.
4. **Decision-table / rule-table model.** Excellent Direction coverage/conflict/
   completeness analysis and inspectability — but weak at temporal/sequence
   semantics (a hard requirement, D11).
5. **Hybrid typed semantic model.** Composes the strengths of 1/3/4 — expressive,
   deterministic, analyzable, inspectable, provenance-rich — at the cost of the
   highest structural complexity and evolution-governance burden.

No additional family was materially distinct enough to add.

## 5. Consequences

### 6. Positive Consequences

- **Every RFC-0001 requirement is expressible** (§Compliance) — ordered evidence
  via typed ordered relations; Direction via an exhaustive mapping; classification
  as a typed static attribute.
- **Well-Formedness becomes tractable** — the decision-table Direction construct
  gives coverage/conflict analysis; typed nodes give exactly-one-classification
  and reference checks; the directed ordered structure gives reachability/
  precedence analysis (§7).
- **Provenance is structural** — stable rule/node identities allow attribution of
  Direction, condition outcomes, and candidate values (C4) without reconstruction.
- **Canonicalizable** — a typed model normalizes cleanly (explicit defaults,
  stable identities, deterministic ordering), enabling ADR-0003 to define
  identity/hashing.
- **Secure by construction** — a constrained, non-Turing-complete fragment cannot
  express arbitrary or malicious computation (§10, N-23).
- **Authoring-surface neutral** — many surfaces can translate into the one typed
  artifact (§11 of PBA; RFC §9).

### 7. Negative Consequences / Trade-offs

- **Highest structural complexity** of the five families. The model has several
  construct types; implementation and governance are heavier than a single-family
  choice.
- **Evolution governance burden** — adding construct types risks semantic drift;
  the model needs disciplined, semver-style evolution rules (deferred detail).
- **Expressiveness ceiling** — the constrained fragment (§10) will refuse some
  strategies a trader could imagine (e.g., arbitrary computed indicators requiring
  unbounded or non-deterministic computation). This is intentional but must be
  communicated; it interacts with R-24 (over/under-expressiveness) and R-E.
- **Coverage analysis is over declared logical partitions** — Direction coverage/
  conflict analysis operates over the finite set of predicate boundaries the
  Strategy declares. This does **not** require discretizing continuous evidence:
  bounded comparison, range membership, and bounded arithmetic over continuous
  price/indicator/distance evidence remain expressible (§10). Authors of
  strategies over continuous quantities must declare explicit predicate boundaries
  (e.g., thresholds/ranges), which they do naturally.

### 8. Rejected Alternatives

- **Structured declarative only** — rejected: cannot cleanly express ordered
  evidence (D11) or make Direction coverage analyzable.
- **Decision-table only** — rejected: cannot express temporal/sequence semantics
  (D11) — a hard RFC requirement.
- **AST-only / graph-only** — rejected as *sole* model: each is strong on one axis
  but weak on inspectability or Direction-coverage; the hybrid subsumes their
  strengths.
- **Turing-complete / scripting rule language** — rejected on determinism,
  decidability, and security grounds (§10). This is the most important rejection.

## 9. Constitutional / RFC Compliance

- **C2 (four pillars)** — the model encodes trader-authored, deterministic,
  provenanced rules; applicability is a filter (never a selector); nothing here
  originates a trade or instrument.
- **C3 / E1** — the model is data, not a model-backed component; evaluation over it
  is deterministic; no AI/score on the path.
- **C4** — stable rule identities provide rule-level provenance for status,
  Direction, and calculated values.
- **C5** — the typed model is canonicalizable → stable revision identity; supports
  reproducibility and no-retroactive-mutation.
- **RFC-0001** — §7 contract fully expressible; §10 static classification is a
  typed static attribute; §12 Well-Formedness is supported and kept **distinct**
  from runtime Decision Validation (this ADR does not touch status semantics,
  owned by the Validation Model); §15 sufficient-Direction is analyzable via the
  Direction mapping.
- **Ownership** — status resolution remains the Validation Model's; lifecycle the
  Trading Decision Model's; this ADR owns only the Strategy semantic model.

## 10. N-23 Resolution

**Resolved: the canonical Strategy model is intentionally non-Turing-complete and
bounded to a decidable/analyzable fragment.** A Strategy artifact **describes
deterministic decision semantics; it is not a general-purpose executable
program.**

> **Traceability.** This N-23 resolution is the authoritative architecture
> decision. Where the discovery artifact ARW-0001 used looser wording about
> "discretizing continuous quantities," this ADR clarifies and supersedes that
> wording for decision purposes. ARW-0001 is not modified; the discovery artifact
> does not override this ADR.

### Raw evidence domain vs. logical outcome partition (corrects ARR-0001 B-1)

The finiteness/analyzability requirement applies to the **logical Direction-outcome
partition** induced by the Strategy's declared predicates and their explicit
boundaries — **not** to the underlying evidence quantity.

- **Raw or derived declared evidence may use continuous or otherwise large
  domains** — price, indicator values, ratios, distances, timestamps, and
  calculated numeric values. **The model does NOT require global discretization of
  these underlying evidence domains** to permit static Direction analysis.
- The constrained expression fragment **supports** deterministic constructs over
  such evidence: bounded numeric comparison; threshold comparison; range
  membership; equality where type-valid; and deterministic bounded arithmetic over
  declared evidence and declared configuration — **but only within the decidable
  fragment defined below (§10c)**.
- **Direction coverage and conflict analysis operates over the finite set of
  declared logical partitions/boundaries** the predicates induce — never by
  enumerating or globally discretizing every possible numeric evidence value.

This preserves RFC-0001's ordinary User-Zero strategies — price-vs-level,
distance-vs-threshold, SNR/retest, timing, confirmation, and deterministic
candidate-value calculations. No statement in this ADR requires continuous market
evidence itself to be discretized.

### Determinism prohibitions (corrects ARR-0001 M-1)

To guarantee deterministic semantics (C3/E1), the fragment **prohibits**:

- nondeterministic primitives, including random values;
- ambient wall-clock or implicit current-time access;
- undeclared external state;
- dynamically loaded executable predicates;
- arbitrary user-defined executable functions/predicates;
- evaluator-specific implicit semantic behavior;
- arbitrary external code execution;
- recursion;
- unbounded loops / unbounded iteration;
- unrestricted dynamic evaluation (`eval`-like constructs);
- arbitrary side effects / I/O.

**Time, session, calendar, or similar temporal information** needed by a Strategy
must enter through **declared evidence** (or another explicitly declared semantic
input), never through hidden evaluator state. This is a **semantic-determinism**
requirement, not an implementation design; no evaluator mechanism or language is
selected here.

This boundary preserves determinism, static analyzability, Well-Formedness
quality, conflict detection, Direction-coverage analysis, provenance,
reproducibility, and security — while remaining, per PBA-0012 §2, **capable of
expressing every RFC-0001 requirement** (Context/Location/Sequence/Timing/
Confirmation, ordered evidence, BLOCKING/TEMPORARY, deterministic Direction,
candidate Entry/SL/TP, applicability, configuration and bounds, provenance and
freshness references, precedence).

Where a genuine strategy need exceeds the fragment, the answer is an **explicit,
governed extension of the typed model** (§10b), never an escape hatch to arbitrary
code.

### 10a. Direction-resolution invariant (corrects ARR-0001 M-2)

For every reachable state for which the Strategy requires Direction resolution,
the declared Direction rules **must resolve deterministically to exactly one
Direction outcome.** Exactly-one resolution may be achieved **only** through
unambiguous semantic structure:

1. mutually exclusive Direction predicates; or
2. an explicit **strict total order over co-applicable Direction rules** —
   deterministic ordering sufficient to select exactly one outcome for every
   overlap.

Therefore:

- overlapping/co-applicable Direction rules **without** a resolving strict order
  are a Strategy **Well-Formedness conflict**;
- contradictory Direction resolution is a Well-Formedness failure;
- a reachable required state for which **no** Direction outcome can be derived is
  **incomplete Direction logic** and makes the Strategy **UNUSABLE**;
- UNUSABLE remains distinct from READY / WAIT / DO NOT TRADE;
- runtime evidence may determine **which** declared branch is satisfied, but
  runtime evaluation **must not invent precedence or choose among ambiguous
  branches** — ambiguity is resolved (or rejected) statically at Well-Formedness,
  not at runtime.

Runtime status semantics are **not** defined here; they remain owned by the
Validation Model.

### 10b. Semantic-model evolution invariants (incorporates ARR-0001 m-1)

At this root decision the following invariants are established:

1. every canonical semantic model declares an **explicit semantic-model version**;
2. **unknown or unsupported semantic constructs fail closed** (never silently
   ignored or best-effort interpreted);
3. a model extension **must not silently change the meaning** of an
   already-defined construct;
4. **historical Strategy revisions retain the semantic interpretation under which
   they were valid** (load-bearing for C5 reproducibility);
5. semantic evolution must therefore be **explicit and governed**.

Migration algorithms, persistence, evaluator-version storage, serialization-version
encoding, and compatibility tooling are **downstream** and not decided here; N-24
authority is preserved.

### 10c. Decidability admission invariant (closes ARR-0002 D-1)

**The canonical Strategy semantic fragment may include only predicate and
derivation constructs for which every analysis required by Strategy
Well-Formedness is decidable under the fragment's declared semantics.**

"Deterministic bounded arithmetic" (§10) therefore does **not** admit arbitrary
mathematical expressions: an expression is a valid canonical construct only if the
Direction coverage, conflict, contradiction, and completeness checks over the
logical partitions it induces remain statically decidable under the fragment.
Consequently:

- an expression **outside** that supported decidable fragment is **not** deferred
  to runtime — it is **not a valid canonical Strategy construct** at all
  (rejected as ill-formed / unknown construct, §10b(2));
- the required Direction coverage/conflict/contradiction/completeness checks
  **remain statically decidable** by construction;
- detected violations remain **Strategy Well-Formedness failures → UNUSABLE**;
- the **runtime fail-closed guard remains defense in depth** for implementation
  defects and genuinely runtime-only evidence satisfaction (§11) — it is **not** an
  escape hatch for undecidable canonical semantics.

This ADR selects **no** evaluator, solver, theorem prover, expression language,
concrete numeric theory, library, or technology; the precise admissible construct
set and its checking mechanism are owned by **ADR-0004** (Well-Formedness
mechanism), which must honour this invariant.

## 11. Static Analysis / Decidability Boundary

Per ARW-0001 §7, the model targets three tiers and claims no completeness it
cannot provide:

**Fully statically checkable:** required fields; valid references; exactly-one
condition classification; configuration-bounds validity; revision-identity
presence; provenance-metadata presence.

**Statically analyzable to a defined extent** (decidable because the fragment is
constrained and analysis is over the **declared logical Direction partitions**,
not raw evidence enumeration — §10): Direction conflicts; Direction coverage; rule
contradictions; unreachable rules; precedence ambiguity; ordered-rule structural
validity.

**Well-Formedness owns decidable violations (incorporates ARR-0001 n-2).** When a
structural or semantic property required by the canonical model is **decidable
under the constrained fragment**, a detected violation is handled by **Strategy
Well-Formedness** and therefore makes the Strategy **UNUSABLE** — it is not
deferred to runtime. The runtime fail-closed guard is **defense in depth, not a
substitute** for a static Well-Formedness check the architecture declares
decidable.

**Runtime-only:** whether declared market evidence actually occurs; live evidence
values; temporal conditions whose satisfaction depends on future data. These are
**not** Well-Formedness concerns; they belong to runtime Decision Validation
(Validation Model), which continues to own live evidence satisfaction and
READY / WAIT / DO NOT TRADE. The runtime fail-closed guard (e.g.,
`STRATEGY_INCOMPLETE_DIRECTION_LOGIC` → DO NOT TRADE) is **defense in depth** for
implementation defects and these genuinely runtime-only conditions — **not** a
fallback for undecidable canonical semantics: constructs whose required
Well-Formedness analyses are not statically decidable are excluded from the
canonical fragment (§10c), not admitted and deferred. This ADR does not redefine
Validation Model status semantics.

## 12. Evaluation Compatibility (no evaluator selected)

This ADR selects no evaluator. Any future evaluator MUST preserve: semantic
equivalence over this model; deterministic results; stable rule identity;
rule-level provenance; revision binding; and **evaluator-semantics versioning
compatibility** (N-24 remains a downstream decision, but the model is designed so
an evaluator version can be pinned alongside a revision).

## 13. Canonicalization Compatibility (no format/algorithm selected)

The typed model is designed to support later canonicalization (ADR-0003) with:
explicit defaults (no implicit values); stable identities; deterministic
normalization; deterministic ordering or order-independent identity; and
semantic-version evolution without silent drift. The canonical serialization and
hash algorithm are **not** decided here.

**Identity layers (incorporates ARR-0001 n-1) — conceptual distinction only.** The
model conceptually distinguishes four identity notions: **Strategy semantic
identity** (what the Strategy means), **Strategy revision identity** (which
immutable revision), **stable rule/node identity** (attribution of individual
operative rules for provenance), and **serialization identity** (the encoded
form). This ADR names these to keep them from being conflated; it does **not**
define canonical hashes, normalization, ordering normalization, revision-hash
construction, or serialization-identity algorithms — **ADR-0003 retains ownership**
of all canonicalization and revision-identity mechanics.

## 14. Security / Integrity Assessment

The model choice directly reduces several ARW-0001 §12 risks: **arbitrary code
execution** and **malicious imported Strategy behavior** are prevented by the
non-Turing-complete fragment (§10); **parser/evaluator ambiguity** and **implicit
defaults** are reduced by the typed model + canonicalization compatibility (§13);
**unstable rule identity** and **provenance loss** are addressed by stable rule
identities (§2); **rule mutation without revision** is addressed by the
config-vs-revision distinction (RFC §14) plus canonical identity. Deferred (not
decided here): authentication, signatures, cryptographic algorithms, tamper
protection mechanisms, evaluator-divergence enforcement.

## 15. Downstream Decisions (explicitly deferred)

- **ADR-0003** — canonicalization and revision hashing / identity.
- **ADR-0004** — Strategy Well-Formedness evaluation mechanism.
- **Later RFC** — authoring-to-canonical translation contract (and N-25).
- **Later ADR** — execution/evaluation model (and N-24 evaluator versioning).
- **Later ADR** — serialization format.
- **Later ADR** — revision/provenance persistence, **gated by Q-06** and touching
  the C5/R-04 erasure tension.

These must not be collapsed into this ADR.

## 16. Risks

- **Complexity risk** — the hybrid's construct variety could invite inconsistent
  implementation; mitigated by governed evolution rules (ADR-0003+ / this model's
  own change discipline).
- **Expressiveness-boundary friction** (R-24 / R-E) — the fragment will refuse
  some strategies; mitigated by governed extension, never by arbitrary code.
- **Coverage-analysis dependence on declared boundaries** — Direction coverage is
  decidable over the predicate boundaries a Strategy declares; continuous evidence
  is **not** discretized. The only modeling obligation is that Direction predicates
  declare explicit boundaries (thresholds/ranges), which authors do naturally.
- Existing R-20…R-24 remain relevant; this ADR does not modify the register
  (deferred to an authorized alignment).

## 17. Reversibility

**Costly.** The semantic model is the root artifact; changing families later would
invalidate downstream ADRs and any authored Strategies. This is why it is decided
first, in isolation, and why the constrained-fragment boundary (§10) is set
conservatively — it is far cheaper to *widen* a governed typed model later than to
retract a Turing-complete one.

## 18. Validation / Acceptance Criteria

A faithful realization of this ADR must:

1. Represent every RFC-0001 §7 contract element as typed model elements.
2. Carry exactly one static classification per condition; never runtime-mutable
   within a revision.
3. Express ordered evidence (Context/Location/Sequence/Timing/Confirmation) as
   explicit typed ordered relations.
4. Represent Direction derivation as an analyzable exhaustive mapping supporting
   coverage and conflict analysis over enumerable outcomes.
5. Carry stable rule identities enabling rule-level provenance for Direction,
   condition outcomes, and candidate Entry/SL/TP.
6. Contain no Turing-complete or side-effecting constructs (§10).
7. Keep Well-Formedness (structural + semantic) distinct from runtime Decision
   Validation, and restate no status semantics.
8. Be canonicalization-ready (explicit defaults, stable identities, deterministic
   normalization) without prescribing the format.
9. Distinguish adjustable Decision-Input configuration from revision-bearing
   semantics.
10. Select no serialization, persistence, evaluator, language, or provider.

---

## Acceptance Record

Accepted by the Product Board via **PBR-0006** (2026-08-12), following ARR-0001,
the ER-0014 remediation, ARR-0002 (PASS), and the ER-0015 final bounded
remediation. All acceptance-blocking findings (ARR-0001 B-1/M-1/M-2/m-1/n-1/n-2/
n-3; ARR-0002 D-1/D-2/D-3) are closed. Acceptance recorded the decision only and
changed no substantive architecture semantics.

Downstream ownership is preserved: **ADR-0003** owns canonicalization,
normalization, and revision-identity mechanics; **ADR-0004** owns the
Well-Formedness checking mechanism and the precise supported construct set
satisfying §10c. Serialization format, evaluator implementation/model, N-24, N-25,
persistence/storage, and language/framework/database/provider remain deferred.

> ADRs are immutable once Accepted — supersede, never edit. This ADR is now
> **ACCEPTED**; its decision text is immutable historical architecture record. Any
> future decision that materially changes or supersedes it must be recorded through
> a new ADR that explicitly supersedes this one.
