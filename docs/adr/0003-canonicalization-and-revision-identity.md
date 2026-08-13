# ADR-0003: Canonicalization & Revision Identity

- **Status:** ACCEPTED
- **Date:** 2026-08-12
- **Accepted by:** Product Board, PBR-0007
- **Decision date:** 2026-08-12
- **Deciders:** Principal Engineer; accepted by Product Board
- **Related:** ADR-0002 (ACCEPTED, immutable), RFC-0001 (ACCEPTED), ARW-0002
  (discovery), PBA-0016 (authorization); reviews ARR-0003 → ER-0018 → ARR-0004
  (PASS w/ minor) → ER-0019
- **Governed by:** Product Constitution v1.0 — ACTIVE — RATIFIED

> This ADR decides **how a Strategy revision is identified** and **what
> canonicalization must normalize, preserve, or exclude**. It does **not** select
> a serialization format, a hash/digest algorithm, storage/persistence, the
> evaluator or its version scheme (N-24), the Well-Formedness mechanism (ADR-0004),
> authoring translation (N-25), or any retention/privacy/legal mechanism. It does
> not reopen ADR-0002's Hybrid Typed Semantic Model or its decidability-admission
> invariant (ADR-0002 §10c). **Semantic identity ≠ serialization identity.**

---

## 1. Context

ADR-0002 selected a Hybrid Typed Semantic Model with no implicit defaults, an
explicit semantic-model version, unknown-constructs-fail-closed, and the invariant
that historical Strategy revisions retain their original semantic interpretation
(§10b). It named four identity notions conceptually and deferred their mechanics
here (§13). RFC-0001 §14 distinguishes in-bounds configuration (a Decision Input)
from operative change (a new revision), and requires decisions to bind the exact
revision. C4/C5 require rule-level provenance and auditable, non-silently-rewritten
history; R-04 records the auditability-vs-erasure tension.

The problem: **two semantically equivalent Strategy artifacts must not acquire
different revision identities for irrelevant representational reasons, and every
semantically operative change must produce a distinct revision identity.** This
ADR fixes the identity architecture and the canonicalization invariants; ARW-0002
is its discovery basis.

## 2. Decision

**We will identify a Strategy revision by a content-addressed semantic identity
computed as a deterministic function of the Strategy's canonical semantic form.**
A human-facing revision label may accompany it but is **non-authoritative** and is
bound to the content-addressed identity.

- Identity is derived from the **canonical semantic form** (the ADR-0002 typed
  model after Well-Formedness and canonicalization), **never** from serialization
  bytes.
- The identity binds the **semantic-model version** and the **canonicalization
  version** (§6, §7).
- The **concrete digest/hash algorithm is NOT selected here.** The architecture
  requires content-addressability — deterministic, collision-resistant, stable,
  reproducible across conforming implementations. The specific algorithm is a
  mechanism for a later finalization/implementation decision, unless Architecture
  Review shows a commitment is required for correctness.

## 3. Decision Drivers

Reproducibility (C5), rule-level provenance (C4), independence from serialization/
storage/timestamp/tool, collision resistance and content-addressability, the
config-vs-revision rule (RFC §14), the erasure tension (R-04), and compatibility
with ADR-0002's evolution invariants.

## 4. Identity Layers

Five layers, kept distinct; the "Authoritative for" column states purpose.

| # | Layer | Definition | Authoritative for |
|---|---|---|---|
| 1 | **Semantic Identity** | The equivalence relation: two artifacts mean the same iff their canonical semantic forms are equal. | Defining *sameness* of meaning. |
| 2 | **Strategy Revision Identity** | The content-addressed name of one accepted revision, derived from the canonical semantic form. | **Reproducibility; decision binding; the authoritative revision identifier.** |
| 3 | **Stable Rule/Node Lineage Identity** | Persistent identity of an operative rule/node across revisions and semantically-irrelevant edits. | **Provenance, explanation, evolution tracking** (§9). |
| 4 | **Serialization Identity** | Identity of a particular byte encoding. | Nothing normative — **must never** drive layers 1–3. |
| 5 | **Human-Facing Revision Label** | An optional human-readable revision name/number. | Human reference only; **non-authoritative**, bound to layer 2. |

Semantic Identity (1) defines the equivalence; Strategy Revision Identity (2) is
its content-addressed representative and the authoritative identifier; Lineage
Identity (3) is orthogonal and serves provenance; Serialization Identity (4) and
the Label (5) are never authoritative for meaning or revision.

## 5. Canonical Form — precondition and ordering

The canonical semantic form is produced by the ordering
**Well-Formed → canonicalize → identity**:

1. a Strategy must first pass **Strategy Well-Formedness** (mechanism owned by
   **ADR-0004**; this ADR only *depends on* the gate, invariant I9);
2. the Well-Formed typed model is **canonicalized** per §6;
3. the content-addressed **revision identity** is computed over that canonical
   form (§7).

This ADR does not design step 1's mechanism.

## 6. Canonicalization Model

Each category is classified as **ID** (identity-bearing semantic content),
**CX** (canonicalized-away representational difference), **PX** (preserved but
excluded from identity — non-operative metadata), or **INV** (invalid/unadmitted).
An implementation may **not** silently re-decide these placements (see the
closure rule at the end of this section, N-29).

**Canonicalization is compositional / recursive (corrects ARR-0003 F-6).**
Children/members are canonicalized **before** their parent container. For
**unordered collections**, membership comparison, deterministic ordering, and
deduplication are performed **over the already-canonicalized member forms** — so
two members that share a canonical form are treated as the same member, and a set
has exactly one canonical representation.

| Category | Class | Treatment |
|---|---|---|
| Field/property ordering | **CX** | Deterministic ordering applied; ordering carries no meaning. |
| Unordered collections | **CX → ID(members)** | Order-independent; the member set is identity-bearing. |
| Semantically ordered collections (Sequence/Timing/Confirmation) | **ID** | Order **is** meaning (D11); preserved and identity-bearing. |
| Explicit / default / absent values | **CX → ID(resolved value)** | No implicit defaults (ADR-0002 §10b); the canonical explicit resolved value is identity-bearing; a canonical absent-marker where truly absent. |
| Numeric representation (1.0 / 1.00 / 1e0) | **CX → ID(value)** | Normalized to a canonical value form; representation is not meaning; the value is identity-bearing. |
| Semantically operative precision | **ID (conditional)** | Identity-bearing **only when semantically operative** (§N-28 rule); if so, represented explicitly and included; never inferred from presentation. |
| Units | **ID (conditional)** | Identity-bearing **only when semantically operative**; if so, explicit/declared. **Dimension is identity-bearing** and normalization occurs **only within the same declared dimension**; dimensionally-distinct quantities must never canonicalize together; canonicalization introduces **no lossy rounding or conversion**; any tolerance/precision/quantization/rounding that affects semantics is itself explicit and identity-bearing (corrects ARR-0003 F-7). Never inferred from formatting. |
| Applicability constraints | **ID (when operative)** | Operative filter on the trader-selected instrument (RFC-0001 §7); identity-bearing. |
| Required evidence declarations | **ID** | What evidence the Strategy requires (RFC-0001 §7); identity-bearing. |
| Evidence relationships / ordering | **ID** | Ordered/related evidence semantics (D11); identity-bearing. |
| Freshness-policy references | **ID (when operative)** | A reference that affects validation semantics; identity-bearing when operative. Concrete thresholds remain Q-10a / Validation-Model territory, not the Strategy's. |
| Sufficient-Direction criteria | **ID (when Strategy-declared / operative)** | If a Strategy declares its own sufficient-Direction criteria (ADR-0002 §15), they are identity-bearing. |
| Change provenance | **PX** (unless a field is explicitly operative) | Preserved, excluded from identity — unless a specific provenance field is explicitly part of operative Strategy semantics, in which case it is ID. |
| Enum/categorical | **CX(spelling) → ID(value)** | Canonical spelling/case; the categorical value is identity-bearing. |
| Whitespace / presentation | **CX** | Removed/normalized. |
| Rule/node identifiers | **CX (cosmetic) / ID (reference-bearing)** | A cosmetic/internal rename alone does not fork revision identity; identifiers that carry references are resolved to a canonical structural form (§9, N-26). |
| Inter-node references | **ID** | Structural meaning; identity-bearing via canonical structural resolution. |
| Configuration structure | **ID** | The declared parameter set/shape is identity-bearing. |
| Configuration bounds | **ID** | Declared permitted bounds are identity-bearing. |
| In-bounds configuration values | **PX (excluded from revision identity)** | A Decision Input (RFC §14); excluded from revision identity; bound at the decision level. |
| Semantic-model version | **ID** | Identity-bearing (ADR-0002 §10b; I5). |
| Canonicalization version | **ID** | Identity-bearing (N-27; I6). |
| Evaluator-version references | **PX / deferred** | **Not** part of revision identity (N-24); bound at decision/audit-record level (§13). |
| Unknown constructs | **INV** | Fail closed (ADR-0002 §10b/§10c); never admitted → no revision identity. |
| Operative provenance | **ID** | Rule-level provenance that is semantically operative (which rule produces what) is structural and identity-bearing (C4). |
| Authoring provenance | **PX** | Who/how authored — preserved, excluded from identity. |
| Timestamps | **PX** | Preserved, excluded (I4). |
| Authoring-tool metadata | **PX** | Preserved, excluded (I4). |
| Names / descriptions / other non-operative metadata | **PX** | Preserved, excluded; purpose/thesis is non-operative (RFC/ADR-0002). |

This preserves RFC-0001 §14: in-bounds config values are excluded from revision
identity while their declared structure and bounds remain identity-bearing.

### 6.1 Classification closure rule (corrects ARR-0003 F-5, N-29)

**Every canonical Strategy property MUST carry exactly one declared classification
(ID / CX / PX / INV) before it may participate in revision-identity derivation.**
An unknown or unclassified property **fails closed** (it is treated as INV and the
Strategy does not receive a revision identity — consistent with ADR-0002 §10b).
An implementation may **not** decide an identity-affecting classification ad hoc.
Future extension fields are **not** silently classified; they require an explicit
semantic-model/version decision (ADR-0002 §10b evolution invariants).

**Operative rule-content default (closes ARR-0004 MINOR-1).** All **operative
semantic rule-content** not otherwise enumerated in the table above is **ID
(identity-bearing)** — this includes, at minimum, **condition classification**,
**Direction logic**, and **candidate-value derivations**. This default is scoped
**only** to operative semantic rule-content; it does **not** reclassify
presentation, authoring/non-operative provenance, contextual/non-operative
metadata, or invalid content, whose CX / PX / INV placements above are unchanged.
The default satisfies the closure rule for core operative properties without
altering the ID / CX / PX / INV taxonomy: a genuinely *unknown* construct still
fails closed (INV), while known operative rule-content is unambiguously ID.

### 6.2 Configuration cannot restructure operative semantics (corrects ARR-0003 F-2)

An **in-bounds configuration value** that remains a **Decision Input** may
parameterize only declared **quantities or thresholds** inside a **fixed operative
Strategy structure**. Changing such a value MUST NOT:

- change a BLOCKING/TEMPORARY classification (which is static — ADR-0002 §2
  item 2 / RFC-0001 Q-11b);
- add or remove a rule;
- add or remove an evidence relation;
- alter Direction topology;
- select a different rule graph / branch structure;
- change applicability structure;
- change required-evidence structure;
- change the semantic role of a condition.

If a configurable property is *capable* of changing any operative structure above,
that property is **identity-bearing Strategy semantics, not an ordinary Decision
Input**, and changing it requires a **new revision**. This preserves RFC-0001
§14 and ADR-0002's static-classification invariant; it is a soundness condition
for I7 (a value change excluded from revision identity must be genuinely
structure-preserving). No implementation mechanism is selected.

## 7. Revision-Identity Invariants

- **I1 — Deterministic:** revision identity is a deterministic function of the
  canonical semantic content.
- **I2 — Equivalence (canonical/representational scope; corrects ARR-0003 F-1):**
  two Strategies receive the **same** revision identity iff they have the **same
  canonical semantic form** under the canonicalization rules of §6. This collapses
  *irrelevant representational differences* (ordering, whitespace, numeric
  representation, resolved defaults). It does **not** normalize arbitrary logical
  equivalence: logically-equivalent predicates written with materially different
  semantic structures **may** produce different revision identities — e.g.
  `price > 100`, `100 < price`, and `NOT (price <= 100)` are **not** required to
  collapse to one revision identity. This is intentional. Revision identity
  identifies **canonical authored semantics**, not a theorem-proved equivalence
  class of all logically-equivalent expressions. ADR-0003 introduces no predicate
  theorem prover or logical-normal-form architecture.
- **I3 — Sensitivity:** every semantically operative change (a change to the
  canonical semantic form) produces a different revision identity.
- **I4 — Independence:** independent of serialization formatting, storage location,
  timestamp, authoring tool, and other non-operative metadata.
- **I5 — Model-version binding:** the semantic-model version is identity-bearing.
- **I6 — Canonicalization-version binding:** the canonicalization version is
  identity-bearing; a change in canonicalization rules must not silently
  re-identify an existing revision.
- **I7 — Config exclusion:** in-bounds Decision-Input configuration values are
  excluded from revision identity, while their declared structure and bounds
  remain identity-bearing.
- **I8 — Reproducible** across conforming machines/implementations from the
  canonical form.
- **I9 — Well-Formed precondition:** only a Strategy that has passed Strategy
  Well-Formedness may receive an accepted canonical revision identity (depends on
  ADR-0004's gate; this ADR does not design it).
- **I10 — Erasure-survivable:** the revision identity may survive authorized
  physical erasure of canonical content as an audit tombstone (§8).

## 8. Historical Reproducibility vs Erasure

Two explicit states (this ADR defines only the identity invariant enabling them):

**Retained-content state.** While canonical Strategy content is legitimately
retained, the system must be capable of recovering/verifying the canonical
semantic form associated with a revision identity — subject to future persistence
and evaluator (N-24) decisions. Because identity is content-addressed, a recovered
form can be **verified** against the identity.

**Authorized post-erasure state.** After an authorized physical erasure of
canonical content: the content may no longer be recoverable; **full historical
reproduction is therefore no longer guaranteed**; the revision identity/tombstone
and the audit fact that a Decision referenced that revision may remain. **The loss
of reproduction must be explicit** — the system must not present a decision as
successfully reproducible when its underlying canonical content has been erased.

This ADR does **not** decide retention duration, legal basis, erasure workflow,
crypto-shredding, storage/database mechanism, or physical tombstone
implementation — all downstream under Q-08 / a persistence-privacy architecture.
This advances R-04 (identity half) without closing it.

## 9. Stable Rule/Node Lineage (N-26)

**Dual-identity architecture** (Board disposition): a **lineage identity** for
provenance/explanation/evolution, and the content-addressed **revision identity**
for reproducibility. A semantically-irrelevant rename of a cosmetic/internal
identifier must not by itself fork revision identity.

**Lineage must be revision-qualified (corrects ARR-0003 F-3).** Lineage identity
represents **continuity of authorship/history, not semantic sameness.** Any
provenance, audit, explanation, or rule reference that identifies a lineage
identity **in connection with operative semantics** MUST also bind the applicable
**Strategy revision identity** — conceptually the reference becomes
`(lineage identity, Strategy revision identity)`. A lineage identity surviving a
semantic modification MUST NOT imply that the rule's semantics are unchanged.
**Revision identity remains authoritative for reproducibility.**

Architecture-level behaviour:

| Event | Lineage identity | Revision identity |
|---|---|---|
| Unchanged rule across revisions | **Same** lineage identity | Revision identity unchanged if nothing operative changed |
| Semantically modified rule | **Same** lineage identity (the rule *evolves*); its content changes | New revision identity |
| Cosmetic/internal rename | Same lineage identity | **Unchanged** (I2) |
| Replacement (rule removed, new rule in its place) | **New** lineage identity for the replacement; old retained as tombstone | New revision identity |
| Deletion | Lineage identity **retained as tombstone** for historical provenance (C4) | New revision identity |
| Split (one rule → several) | Each output gets a **new** lineage identity; origin recorded in provenance | New revision identity |
| Merge (several → one) | Result gets a **new** lineage identity; origins recorded in provenance | New revision identity |
| **Copy / Clone** | The clone receives a **new** lineage identity; source lineage may be recorded in provenance. It must **not** share the source's lineage identity merely because initial semantics are identical. | New revision identity |
| **Move / Re-parenting** | Lineage identity **persists** through the move. Revision identity **changes** iff the move changes operative ordered relations, references, applicability, evidence relationships, Direction structure, or other identity-bearing semantics. A presentation-only relocation that changes no canonical semantic property does not itself require a revision. | Conditional |
| References to another node | Resolved to canonical structural form; reference target is identity-bearing | Part of revision identity |
| Historical provenance after deletion | Preserved via tombstoned lineage identity | Prior revisions and their decisions remain referable |

This preserves C4 rule-level provenance. No identifier-generation technology
(UUIDs, database keys, etc.) is selected.

## 10. Configuration-vs-Revision (RFC §14 preserved)

Revision identity is derived from **{ordered and unordered rules, condition
classifications, Direction logic, candidate-value derivations, applicability,
evidence requirements, configuration structure + bounds, semantic-model version,
canonicalization version, operative provenance}** and **excludes {in-bounds
configuration values, authoring provenance, timestamps, tool, names/descriptions,
non-operative metadata, evaluator-version references}**. Thus a value change within
declared bounds is a Decision Input (same revision identity); a change to bounds,
rules, classification, Direction logic, candidate derivation, applicability, or
evidence requirements forks the revision — exactly RFC-0001 §14. The exhaustive
placement is §6 (N-29).

## 11. N-24 Boundary

**Confirmed deferred.** The evaluator-semantics version is **not** part of Strategy
revision identity. The architecture requires the applicable **evaluator-semantics
version to be bound at the Decision / audit-record level**, so historical Decision
interpretation can later be reproduced correctly once N-24 is decided. This ADR
does **not** define the evaluator-version scheme and is **finalizable without
resolving N-24**.

## 12. ADR-0004 Boundary

This ADR does **not** design the Well-Formedness checking mechanism, expression
validation, Direction-coverage checking, or contradiction detection — all owned by
**ADR-0004**, bounded by ADR-0002 §10c. This ADR only *relies on* the Well-Formed
gate as a precondition (I9, §5) and does not specify how it is performed.

## 13. Security / Integrity

Required architecture properties (mechanisms deferred):

| Threat | Architecture property |
|---|---|
| Canonicalization ambiguity | Canonicalization must be **total and deterministic** — every admissible artifact has exactly one canonical form (R-25). |
| Equivalent-content identity divergence | Guaranteed absent by I2 + total canonicalization. |
| Semantically-different identity collision | Collision-resistant content-addressing over a canonical form capturing **all** operative content (§6 ID rows). |
| Parser differentials | Identity computed over the **semantic model** (post-parse), not raw bytes. |
| Numeric normalization ambiguity | Canonical numeric value form; operative precision explicit (N-28). |
| Unit ambiguity | Operative units explicit and normalized (N-28). |
| Rule-ID substitution | Reference targets identity-bearing via canonical structural resolution (§9). |
| Provenance stripping | Operative provenance is identity-bearing (§6). |
| Revision spoofing | Content-addressing: spoofing requires reproducing exact canonical content. |
| Unknown-field injection | Fail closed (INV, §6). |
| Semantic-model downgrade | Model version identity-bearing (I5) → detectable different identity. |
| Canonicalization-version drift | Canonicalization version identity-bearing (I6). |
| Identity/domain confusion | Identity input must be domain-separated (implementation note; not decided here). |

## 14. Consequences

**Positive:** deterministic, reproducible, serialization/storage/timestamp/tool-
independent identity; equivalent artifacts collide, operative changes fork;
config-vs-revision preserved; erasure-compatible auditability; strong integrity by
construction. **Negative / trade-offs:** correctness hinges on **total, deterministic
canonicalization** (R-25) — the hardest obligation; content-addressed identity is
not human-readable (mitigated by the bound Label); the dual-identity model adds
conceptual weight (§9). **Reversibility:** costly — revision identity underpins
decision binding and downstream ADRs; changing it later would invalidate authored
Strategies' identities, hence the conservative content-addressed choice and the
canonicalization-version binding.

## 15. Risks (recorded here; not written to the canonical register in this task)

- **R-25 — Canonicalization incompleteness.** A non-total/non-deterministic
  canonical form lets equivalent artifacts diverge in identity. *Highest-severity.*
  Mitigation: totality/determinism as an acceptance criterion.
- **R-26 — Canonicalization-version drift.** Changing canonicalization silently
  re-identifies revisions. Mitigation: I6 canonicalization-version binding.
- **R-27 — Erasure/reproducibility loss.** Erasure destroys reproduction.
  Mitigation: explicit post-erasure state (§8); identity-as-tombstone.
- **R-28 — Config/revision mis-placement.** In-bounds config folded into identity,
  or an operative change excluded. Mitigation: §6 exhaustive placement (N-29).
- **R-29 — Rule-ID instability.** Unstable rule identity breaks provenance.
  Mitigation: §9 dual-identity.

## 16. Deferred Decisions

Concrete serialization format; concrete hash/digest algorithm (unless review shows
it required); persistence/database/storage; evaluator implementation and its
semantics-version scheme (N-24); Well-Formedness mechanism and construct set
(ADR-0004); authoring-to-canonical translation (N-25); language/framework/provider;
and retention/legal/privacy mechanism (Q-08 / persistence-privacy architecture).

## 17. Validation / Acceptance Criteria

1. Five identity layers defined and non-collapsed (§4); semantic identity and
   serialization identity are not conflated.
2. Revision identity is deterministic from canonical semantic content only (I1),
   equivalent artifacts collide (I2), operative changes fork (I3).
3. Independence from serialization/storage/timestamp/tool/non-operative metadata
   (I4).
4. Binds semantic-model (I5) and canonicalization (I6) versions; excludes in-bounds
   config (I7).
5. Canonicalization specified as **total and deterministic** (R-25).
6. Operative-content placement is exhaustive and non-silently-decidable (§6, N-29).
7. Rule/node lineage identity specified (§9), with N-26 resolved.
8. Erasure-compatible: identity survives content erasure as tombstone; reproduction
   loss explicit (§8, I10).
9. Well-Formed → canonicalize → identity ordering stated as a dependency on
   ADR-0004; no Well-Formedness mechanism designed.
10. N-24 boundary explicit: evaluator-version bound at decision level, deferred;
    ADR finalizable without N-24.
11. No serialization/hash/storage/evaluator/language/framework/database/provider
    selected.
12. No ADR-0002 semantics reopened.

## 18. Acceptance Record

Accepted by the Product Board via **PBR-0007**, decision date **2026-08-12**,
after: initial architecture review (ARR-0003); ER-0018 bounded remediation closing
F-1…F-7; final review (ARR-0004, PASS with minor); and ER-0019 editorial
remediation closing MINOR-1/MINOR-2. All acceptance-blocking findings are closed;
F-8 (privacy/preimage/inference on erased low-entropy content) remains an
intentionally deferred downstream input and does not block acceptance.

**Accepted architecture:** content-addressed Strategy revision identity derived
from the canonical semantic form; deterministic, compositional/recursive
canonicalization; representational canonical equivalence (not arbitrary logical
equivalence); the ID / CX / PX / INV classification taxonomy with the operative
rule-content default (ID) and fail-closed closure; configuration
structure-preservation; semantic-model-version and canonicalization-version
binding; cross-machine reproducibility; Well-Formedness as a precondition to
revision identity; revision-qualified lineage semantics with explicit copy/clone
and move/re-parenting behaviour; dimension-safe numeric/unit canonicalization; and
the erasure-survivable revision-identity requirement.

**Deferred (unchanged by acceptance):** F-8 privacy/preimage observation; N-24
(evaluator-semantics version); serialization format; concrete hash algorithm;
persistence/storage; evaluator implementation/model; canonicalization
implementation; N-25 (authoring-to-canonical translation); retention/legal/privacy
mechanism. **ADR-0004** retains ownership of the Well-Formedness checking mechanism
and precise supported construct set.

---

> ADRs are immutable once Accepted — supersede, never edit. This ADR is now
> **ACCEPTED**; its decision text is immutable historical architecture record. Any
> future material architecture change must be recorded through a **new ADR that
> explicitly supersedes ADR-0003** — never a silent rewrite.
