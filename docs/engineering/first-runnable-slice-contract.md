# First Runnable Slice — Implementation Contract

- **Status:** ACCEPTED
- **Accepted by:** Product Board, PBR-0011
- **Decision date:** 2026-08-13
- **Tier:** 3 (engineering, normative)
- **Authority:** PBA-0015 (authoring); PBR-0011 (acceptance)
- **Author:** Principal Engineer
- **Governed by:** Product Constitution v1.0 — ACTIVE — RATIFIED
- **Conforms to:** ADR-0002 (ACCEPTED), ADR-0003 (ACCEPTED), ADR-0004 (ACCEPTED),
  RFC-0001 (ACCEPTED)
- **Resolves:** ER-0027 blockers **A1**, **A2**, **A3** — for Slice 1 only
- **Review history:** ARR-0007 → ER-0029 → ARR-0008 → ER-0030 → ARR-0009 →
  ER-0031 → PBR-0011 (ACCEPTED)

> This document decides **nothing about architecture**. It selects a **subset** of
> the already-accepted canonical Strategy semantic model (ADR-0002) and the
> already-accepted Well-Formedness architecture (ADR-0004) that Slice 1 will
> implement, and states the authoritative meaning of that subset precisely enough
> to implement without invention. It selects **no** programming language,
> framework, package manager, parser, serialization format, schema language,
> solver, prover, hash algorithm, database, persistence mechanism, provider,
> broker integration, or UI.

---

## 0. Reading Order and Precedence

If any statement in this document conflicts with ADR-0002, ADR-0003, ADR-0004,
RFC-0001, or the Product Constitution, **the accepted document wins and this
document is defective**. Report the conflict; do not implement around it.

Two normative distinctions are load-bearing throughout and must never be
collapsed:

1. **INADMISSIBLE construct ≠ UNUSABLE Strategy instance** (ADR-0004 §4). A
   construct *type* that cannot satisfy decidability / termination / tractability
   admission is INADMISSIBLE and never enters the fragment. A concrete Strategy
   *instance* of an *admitted* construct that violates a mandatory invariant is
   **UNUSABLE**.
2. **OUT OF SLICE 1 ≠ INADMISSIBLE.** Everything this document defers is deferred
   for **scope and minimality reasons only** (PBA-0015 §7). A Slice-1 deferral is
   **not** a ruling that the construct is inadmissible under ADR-0004 §4, and it
   creates no precedent against admitting it in a later slice. Where a construct
   is deferred *because* it would breach the ADR-0004 §4 tractability envelope in
   its general form, this document says so explicitly.

A third distinction is restated because Slice 1 produces a verdict:
**UNUSABLE is not READY / WAIT / DO NOT TRADE.** UNUSABLE is a Strategy-usability
verdict owned by Well-Formedness. READY / WAIT / DO NOT TRADE are runtime
Decision Validation statuses owned by the Validation Model and are entirely
outside Slice 1.

---

## 1. Slice-1 Scope

### 1.1 What Slice 1 is

```
CANONICAL STRATEGY (already-canonical, in-memory)
        ↓
STRATEGY WELL-FORMEDNESS CHECKER
        ↓
DETERMINISTIC WF RESULT
        ├── WELL-FORMED
        └── UNUSABLE + DEFECT SET (+ not-evaluated metadata)
```

Slice 1 is a **total, pure, deterministic function** from an admitted canonical
Strategy structure to a Well-Formedness result. Total: it returns a result for
every input in the admitted structural domain, including adversarial ones — it
never fails to answer (ADR-0004 §12). Pure: no I/O, no clock, no randomness, no
ambient state, no side effects, no mutation of its input (ADR-0002 §10).

### 1.2 What Slice 1 is not

Natural-language authoring; authoring UI; authoring-to-canonical translation;
runtime market evaluation; READY / WAIT / DO NOT TRADE resolution; broker or
MetaTrader integration; persistence; database or storage; serialization;
canonicalization; revision hashing or revision identity computation; networking;
deployment; authentication; AI inference.

Slice 1 **consumes** the ordering position of ADR-0003 §5 — it *is* step 1
(`Well-Formed`) of `Well-Formed → canonicalize → identity`. It does not perform
steps 2 or 3 and takes no canonicalization ownership from ADR-0003.

---

## 2. A3 — Direct Canonical Input Boundary

**Ruling (PBA-0015 §3, scoped to Slice 1):** the Slice-1 checker consumes an
**already-canonical in-memory Strategy structure directly**. No authoring
transformation, parsing, decoding, or translation occurs inside Slice 1.

Consequences, all Slice-1-scoped:

- The checker's input domain is the typed structure defined in §3. How that
  structure comes to exist is **outside** Slice 1.
- Test fixtures construct the structure directly in memory.
- **No parser, serialization format, schema language, or file format is selected
  or implied.** "In-memory structure" is a semantic statement about the input
  domain, not a representation choice.
- The checker performs **no** normalization of its input beyond the analysis-local
  operations §4 explicitly authorizes. It must not canonicalize, reorder, resolve
  defaults, or rewrite the input structure (ADR-0003 owns canonicalization;
  ADR-0002 §10b forbids implicit defaults).
- **N-25 remains OPEN.** This ruling defines no general authoring-to-canonical
  translation contract and must not be cited as one. It expires as precedent at
  the Slice-1 boundary.

**Ordered opaque domains are part of the input (m-2).** Where this contract
requires an opaque domain to carry a deterministic total order (§3.3.1), **both**
the domain's values **and** that contract-valid deterministic total order are
constitutive of the already-canonical input domain supplied to the checker. The
checker never selects, derives, or infers such an order. Consequently, two
implementations that receive **the same input** necessarily receive the **same**
domain ordering, which is what makes the §8 obligation-8 equivalence test
well-posed. **No serialization or encoding of that ordering is defined or
implied** — its presence is a semantic property of the input domain, not a
representation choice.

**Input-domain precondition.** The caller supplies a structure whose *shape*
conforms to §3. Shape violations that make the structure uninterpretable as a
Strategy at all (for example, a construct kind that is not in the §3 inventory)
are handled by §5 check 16 as `UNKNOWN_CONSTRUCT` — **fail closed** (ADR-0002
§10b(2), ADR-0003 §6.1). The checker never best-effort-interprets an unknown
construct and never ignores one.

---

## 3. A1 — Slice-1 Admitted Construct Inventory

### 3.1 Admission summary

Every construct named in PBA-0015 §4 is dispositioned. **ADMIT** means Slice 1
implements it. **DEFER** means out of Slice 1 (see §0 point 2).

| # | Construct | Slice 1 | Reason |
|---|---|---|---|
| 1 | Strategy identity / semantic-model version | **ADMIT** | ADR-0002 §10b(1) requires an explicit semantic-model version on every canonical model; without it fail-closed evolution semantics cannot exist and check 16 has nothing to verify. Strategy-level lineage identity is required to anchor every defect. |
| 2 | Condition identity | **ADMIT** | ADR-0003 §9 lineage identity is the anchor for defect identity (ADR-0004 §10) and rule-level provenance (C4). Without it defects cannot be identified deterministically. |
| 3 | Static condition classification | **ADMIT** | RFC-0001 §10 / Q-11b core product semantics; check 3 is mandatory, cheap, and demonstrates a real product invariant end to end. |
| 4 | Evidence reference | **ADMIT** | Required for any predicate to say anything; the reference-integrity check (check 2) is meaningless without it. |
| 5 | Typed literal | **ADMIT** | The other operand of every admitted comparison; carries the dimension that makes check 14 real. |
| 6 | Typed evidence declaration | **ADMIT** | The typing environment every reference resolves against (RFC-0001 §7 "required market evidence"). Without it typing and dimension checking are impossible. |
| 7 | Comparison predicate | **ADMIT** (restricted, §3.6) | The atomic unit of Direction and condition logic (ADR-0002 §10). |
| 8 | Boolean composition | **ADMIT — conjunction only** (§3.6) | Conjunction of interval/equality atoms keeps overlap, subsumption, and contradiction analysis a bounded compositional traversal. General disjunction/negation is **deferred** — see §3.6 and §7.1. |
| 9 | Ordered Direction rules | **ADMIT** (§3.7) | ADR-0002 §10a is the central analyzable construct of the model; Direction coverage/conflict/determinism (checks 8–10) are the reason Well-Formedness exists as a gate. |
| 10 | Explicit Direction outcome | **ADMIT** | BUY / SELL (`glossary.md`). Required for check 10. |
| 11 | Candidate Entry / SL / TP derivation | **ADMIT** (restricted, §3.8) | RFC-0001 §7; makes checks 13 and 14 real rather than vacuous, and exercises dimensional semantics — the one place ADR-0003 §6 unit rules bite in Slice 1. |
| 12 | Configuration parameter + declared bounds | **ADMIT** (restricted, §3.9) | Required for checks 4 and 5. Check 5 (ADR-0003 §6.2 structure-preservation) is a mandatory semantic invariant that would otherwise be untestable. |
| 13 | Applicability constraints | **DEFER** | Applicability is a **filter on the trader-selected instrument** (RFC-0001 §7, §16) evaluated at runtime against an instrument. **No accepted document defines the instrument-attribute domain** the filter constrains. Admitting applicability into Slice 1 would require inventing that product domain — forbidden by PBA-0015 §9 and the Constitution's boundary discipline. Check 17 is deferred with it. Not an inadmissibility ruling. |
| 14 | Required-evidence declarations | **ADMIT** | Same construct as row 6; the declaration set *is* the required-evidence declaration (RFC-0001 §7). |

**Additionally deferred** (not enumerated in PBA-0015 §4, listed so the boundary
is closed): ordered-evidence relations (Context / Location / Sequence / Timing /
Confirmation as typed ordered relations, ADR-0002 §2 item 3) — deferred because
their structural validity check (check 7) requires an ordering-relation and
temporal-semantics model that Slice 1 does not need to demonstrate a
Well-Formedness decision, and because their *satisfaction* is runtime-only;
freshness-policy references (ADR-0003 §6, thresholds are Q-10a); purpose/thesis,
names, descriptions, authoring provenance, timestamps, tool metadata (all **PX**
under ADR-0003 §6 — non-operative, excluded from identity, and irrelevant to
Well-Formedness); change provenance; human-facing revision labels; revision
identity itself (ADR-0003 layer 2 — computed *after* Well-Formedness, §5 there).

### 3.2 The fragment's compliance with ADR-0002 §10c and ADR-0004 §4

Every admitted construct satisfies all four admission requirements:

- **Non-Turing-complete.** The structure is a finite, acyclic, typed tree. There
  are no functions, no bindings, no references between rules, no self-reference.
- **Decidable.** Every mandatory analysis in §5 is decided by finite case analysis
  over declared elements.
- **Terminating.** Every analysis is a single bounded traversal or a bounded
  pairwise comparison over finite declared collections; there is no fixpoint, no
  iteration to convergence, no search.
- **Tractability-bounded** (ADR-0004 §4). Every mandatory analysis is at most
  **quadratic** in the declared construct size (the count of declared evidence
  items, configuration parameters, conditions, Direction rules, atoms, and
  derivation nodes). The quadratic terms are the explicit pairwise Direction-rule
  comparisons of checks 9 and 11; every other check is linear. **No analysis
  enumerates an undeclared or implicit combinatorial state space** — in
  particular, nothing in §5 enumerates the up-to-`2^k` implicit regions induced by
  `k` declared boundaries, which ADR-0004 §4 names as the paradigm case of an
  inadmissible analysis. §7.1 and §7.2 record exactly which general constructs
  were deferred to hold this line.

**Prohibited across the whole fragment** (ADR-0002 §10 determinism prohibitions,
restated as implementation constraints and not weakened): nondeterministic
primitives or random values; ambient wall-clock, calendar, or implicit
current-time access; undeclared external state; dynamically loaded predicates;
user-defined executable functions; arbitrary code execution; recursion;
unbounded loops or unbounded iteration; dynamic evaluation (`eval`-like
constructs); side effects; external I/O. Any temporal information a Strategy
needs must enter as **declared evidence**, never through checker state.

### 3.3 Common properties of every operative node

Every **operative node** — condition, Direction rule, candidate derivation, and
the Strategy envelope itself — carries a **lineage identity** (ADR-0003 §9 layer
3). Slice-1 requirements on lineage identity:

- It is an **opaque value**. No generation scheme, encoding, format, or identifier
  technology is selected (ADR-0003 §9 explicitly selects none).
- It supports **exact equality**.
- It is drawn from a domain equipped with a **deterministic total order** (§3.3.1).
  This order carries **no semantics**; it exists solely so §6.4 defect ordering is
  total and reproducible across conforming implementations.
- It is **unique within one Strategy structure**. Duplicates are a check 2 defect.
- It is **not revision-qualified inside Slice 1**, because Slice 1 runs *before*
  revision identity exists (ADR-0003 §5). See §6.3.

### 3.3.1 Opaque value domains and their orders

Slice 1 uses **five** opaque value domains. For each of them:

- the domain is opaque — **no encoding, format, or representation is selected**;
- values support **exact equality**;
- the domain is **equipped with a deterministic total order**, supplied as part of
  the input domain together with the values themselves (§2).

| # | Domain | Declared in | What its order is for |
|---|---|---|---|
| 1 | Lineage identities | §3.3 | Ordering only — **carries no semantics** (§6.4 key 2). |
| 2 | Dimension symbols | §4.6 | Ordering only — **carries no semantics** (§6.4.3 tie-break). |
| 3 | `ENUM` categorical values | §3.5 | Ordering only — **carries no semantics** (§6.4.3). |
| 4 | Version tokens | §3.4.1, §6.2 | Neither ordering nor semantics — **equality only**; listed here because the domain is opaque. |
| 5 | **Precedence ordinals** | §3.7 | **Semantically operative** — see the rule below. |

**Precedence ordinals are the one exception, and the distinction matters (m-1).**
The orders on domains 1–3 exist solely to make §6.4 total and mean nothing about
the Strategy. The order on **precedence ordinals is operative Strategy semantics**:
it determines which Direction rule resolves (§4.9), what "lowest precedence" means
in check 8, what "strictly higher precedence" means in check 11, and what a tie is
in check 9. It is therefore **required to be a deterministic total order supplied
with the input domain exactly as domains 1–3 are**, and it is **never** taken from
an implementation language's native numeric or collation ordering, from collection
position, or from any convention. Two conforming checkers compare precedence
ordinals identically because they receive the same order, not because they agree on
a representation. This restates no precedence semantics; §3.7 and §4.9 are
unchanged.

Any two conforming Slice-1 checkers operating over the same input domain use the
**same** orders and therefore agree. Because every order arrives with its domain,
no implementation ever chooses one, and **no locale-dependent, language-dependent,
filesystem, source-position, serialization-specific, hash-based, generated, or
otherwise unspecified lexical convention is used or permitted anywhere** in this
contract.

### 3.4 The Strategy envelope

Declared properties, all required:

| Property | Domain | Notes |
|---|---|---|
| lineage identity | opaque, §3.3 | Anchors Strategy-level defects. |
| semantic-model version | opaque version token, exact equality | ADR-0002 §10b(1). Its one supported value is **fixed by this contract** — see §3.4.1. Any other value, or absence, is a check 16 defect (`UNSUPPORTED_SEMANTIC_MODEL_VERSION`). No version *syntax* is selected. |
| evidence declarations | finite collection, §3.5 | May be empty only if no reference exists. |
| configuration parameters | finite collection, §3.9 | May be empty. |
| conditions | finite collection, §3.10 | May be empty. |
| Direction rules | finite ordered-by-declared-ordinal collection, §3.7 | Must be non-empty (check 8). |
| candidate derivations | exactly three, roles Entry / Stop Loss / Take Profit, §3.8 | Each role exactly once. |

**Prohibited:** any property not in this table (→ `UNKNOWN_CONSTRUCT`, check 16,
fail closed); any implicit or absent-but-defaulted value (ADR-0002 §10b — the
input is already canonical, so defaults are already resolved explicitly).

### 3.4.1 The supported semantic-model version constant

**The single semantic-model version supported by Slice 1 is the opaque token
`SM-1`.** This value is **declared by this contract**, not selected by an
implementation. Every conforming Slice-1 checker supports **exactly** this value
and no other. A Strategy declaring any other token, or none, is UNUSABLE under the
check 16 rule `UNSUPPORTED_SEMANTIC_MODEL_VERSION`. Two conforming
implementations therefore cannot support different tokens and cannot disagree on
the verdict for the same input.

`SM-1` is an **opaque identifier compared by exact equality** (§3.3.1). This
declaration defines **no** version syntax, numbering scheme, ordering, comparison
rule beyond equality, serialization, wire encoding, schema, parser behaviour, or
persistence representation, and it establishes **no** precedent for how any future
semantic-model version is named. Widening the supported set, or supporting a
second token, is a **checker-semantics version change** (§6.2) and requires a
change to this contract.

This constant concerns the **semantic-model** version only (ADR-0002 §10b). It is
**not** the runtime evaluator-semantics version: **N-24 remains OPEN** and
continues to own that decision (ADR-0004 §8, §9).

### 3.5 Evidence declaration

**Responsibility split (M-1).** Evidence declarations are the only construct with
**type-dependent** properties, and the two checks that could both claim them are
separated here, once, authoritatively:

- **Check 1 owns unconditional structural presence only.** A property is check 1's
  responsibility **iff** it is marked **Unconditional** below.
- **Check 6 exclusively owns all type-dependent evidence-declaration validity** —
  every property marked **Type-dependent**, including its presence, its absence,
  and its content. Check 1 **never** reports a type-dependent property, whether
  missing, present-when-prohibited, or malformed.

| Property | Required by | Domain |
|---|---|---|
| lineage identity | **Unconditional** (check 1) | opaque, §3.3 |
| declared type | **Unconditional** (check 1) | exactly one of `NUMERIC`, `BOOLEAN`, `ENUM` |
| declared dimension | **Type-dependent** (check 6 **only**) | present iff type is `NUMERIC`; an opaque dimension symbol (§4.6) |
| declared value set | **Type-dependent** (check 6 **only**) | present iff type is `ENUM`; a finite, non-empty set of distinct opaque categorical values |

**The authoritative assignment is therefore unambiguous.** A `NUMERIC` declaration
with no dimension yields **exactly one** defect: `INVALID_EVIDENCE_DECLARATION`
under check 6. It yields **no** `MISSING_REQUIRED_PROPERTY`. Identically, an `ENUM`
declaration with no value set, with an empty value set, or with duplicate values
yields **exactly one** `INVALID_EVIDENCE_DECLARATION` per violation under check 6
and nothing under check 1. No conforming checker may report such a fault under
check 1, and none may report it under both.

**Prohibited:** a dimension on a non-`NUMERIC` declaration; a value set on a
non-`ENUM` declaration; an empty `ENUM` value set; duplicate values within one
`ENUM` value set; any type outside the three named. **All of these are check-6
violations** (§5.1 check 6, mapped to their properties in §5.1.1). Evidence
declarations carry **no** freshness threshold, no provider, no source, and no
relationship to another evidence item in Slice 1 (all deferred, §3.1).

This changes **no** admitted evidence-declaration model: the same properties, the
same domains, and the same validity rules apply. Only the *reporting ownership* is
now stated.

### 3.6 Predicate fragment (guards)

A **guard** is the predicate attached to a condition (§3.10) or a Direction rule
(§3.7). The Slice-1 guard grammar is deliberately narrow:

> **guard** ::= a finite (possibly empty) **conjunction** of **atoms**
> **atom** ::= `evidence-reference` `operator` `typed-literal`

- An **empty conjunction** is the **always-applicable** guard. It is the only way
  to write a total guard, and §3.7 requires exactly one Direction rule to use it.
- **Admitted operators**, by referent type:
  - `NUMERIC`: `<`, `<=`, `>`, `>=`, `=`, `!=`
  - `BOOLEAN`: `=`, `!=`
  - `ENUM`: `=`, `!=`
- **Both operands are typed.** The left operand is always an evidence reference;
  the right operand is always a typed literal.

**Prohibited forms** (each is a Slice-1 scope boundary, not an inadmissibility
ruling, except where noted):

| Prohibited | Why deferred |
|---|---|
| Disjunction (`OR`) | Deciding overlap, subsumption, and contradiction over disjunctive guards is not expressible as a bounded compositional traversal in the general case; it invites exactly the implicit-region enumeration ADR-0004 §4 forbids. **This one is a tractability boundary, not mere scope.** A future slice may admit a *bounded, explicitly-declared* disjunctive form only if its required analyses stay within the §4 envelope. |
| Negation (`NOT`) over a guard | Same reason: negating a conjunction yields a disjunction. Atom-level negation is already available through the complementary operators (`!=`, `<` vs `>=`), which is why nothing is lost at Slice-1 expressiveness. |
| Nested guards / guard trees | Reduces to the two rows above. |
| evidence-vs-evidence comparison | Two-variable constraints leave the per-reference-domain fragment; overlap and subsumption then require relational reasoning rather than per-reference intersection, breaking the linear/quadratic bound. Deferred pending a construct whose analysis stays inside the §4 envelope. |
| Arithmetic inside a guard | Deferred; arithmetic exists in Slice 1 only in candidate derivations (§3.8), where no overlap/coverage analysis is performed over it. |
| Ordering operators (`<`, `<=`, `>`, `>=`) on `BOOLEAN` or `ENUM` | No accepted document declares an ordering over those domains; inventing one would be inventing semantics. Type defect (check 2 typing / check 14). |
| Any reference to a configuration parameter | See §3.9 — structurally forbidden by check 5. |
| Any reference to a Direction outcome, another rule, or a candidate value | Would create inter-rule references and reachability semantics Slice 1 does not model. |

### 3.7 Direction rules

A Direction rule declares:

| Property | Domain |
|---|---|
| lineage identity | opaque, §3.3 |
| precedence ordinal | a value from a totally-ordered domain; **explicitly declared**, never inferred from position |
| guard | §3.6 |
| Direction outcome | exactly one of `BUY`, `SELL` (`glossary.md`) |

**Precedence is declared, not positional.** ADR-0002 §10a permits exactly-one
resolution through mutual exclusivity *or* an explicit strict total order over
co-applicable rules. Slice 1 requires the order to be **explicitly declared** so
that ties are **authorable and therefore detectable** — if precedence were taken
from collection position, ties would be structurally impossible and check 9 would
be vacuous, which would not honour ADR-0004 check 9.

**Mandatory total rule.** Exactly one Direction rule must carry the empty
(always-applicable) guard, and it must carry the **lowest precedence** of all
declared rules. This is how Slice 1 makes Direction coverage (check 8) tractably
decidable — see §5 check 8 and §7.1 for the full justification.

**Prohibited:** zero Direction rules; zero or more than one always-applicable
rule; an always-applicable rule that is not lowest-precedence; a rule declaring
zero or more than one outcome; an outcome outside `{BUY, SELL}`; duplicate
precedence ordinals (check 9); any outcome derived rather than declared.

### 3.8 Candidate-value derivations

Exactly three, one each for roles `ENTRY`, `STOP_LOSS`, `TAKE_PROFIT`.

**Property table (M-3).** Stated as a table, in this declared top-to-bottom order,
so the §6.4.2 PROPERTY discriminator terminates for this construct kind exactly as
it does for every other. All three are **unconditional** (check 1). These are the
construct's **existing** properties — no semantic property is added here.

| Property | Required by | Domain |
|---|---|---|
| lineage identity | **Unconditional** (check 1) | opaque, §3.3 |
| role | **Unconditional** (check 1) | exactly one of `ENTRY`, `STOP_LOSS`, `TAKE_PROFIT` |
| derivation expression | **Unconditional** (check 1) | one expression per the grammar below |

Consequently the reachable case ARR-0008 identified — a derivation missing **both**
its lineage identity **and** its expression — is fully ordered: the two check-1
defects tie on §6.4 keys 1–3 and are separated by key 4, the PROPERTY discriminator,
as position 1 before position 3. This alters **no** ADR-0004 defect-identity
semantics; it supplies the ordering table that §6.4.2 already presumed.

Each derivation's **derivation expression** is:

> **expr** ::= `typed-literal`
>            | `evidence-reference`
>            | `configuration-parameter-reference`
>            | `expr` `+` `expr`
>            | `expr` `-` `expr`
>            | `expr` `*` `dimensionless-literal`

Finite, acyclic, non-recursive; evaluation and dimensional analysis are one
bottom-up traversal, linear in node count.

**Prohibited:** division; exponentiation; any other operator; multiplication where
neither operand is a dimensionless literal; comparison or boolean operators;
conditional/branching expressions; references to conditions, Direction rules, or
other candidate derivations; a missing role; a duplicated role.

Division is deferred not for tractability but for **semantics**: division requires
deciding divide-by-zero behaviour and a dimension algebra (dimension division),
neither of which any accepted document defines, and both of which would be
invented product/architecture semantics.

### 3.9 Configuration parameters

| Property | Domain |
|---|---|
| lineage identity | opaque, §3.3 |
| declared type | `NUMERIC` only in Slice 1 |
| declared dimension | required; opaque dimension symbol (§4.6) |
| declared lower bound | a numeric literal of the same dimension |
| declared upper bound | a numeric literal of the same dimension |
| adjustable flag | boolean; `true` = trader-adjustable Decision Input (RFC-0001 §14) |

**Structural restriction (this is how check 5 is enforced by construction).** In
Slice 1, a configuration-parameter reference may appear in **exactly one
position**: as a leaf of a candidate-value derivation expression (§3.8). It may
**not** appear in a guard, a condition classification, a Direction outcome, a
precedence ordinal, an evidence declaration, or anywhere in the envelope. This
directly realizes ADR-0003 §6.2: a Slice-1 configuration value can parameterize
only a **declared quantity inside a fixed operative structure**, and is
structurally incapable of changing a classification, adding or removing a rule,
altering Direction topology, selecting a branch, or changing evidence structure.

**Prohibited:** non-`NUMERIC` parameters (deferred; only numeric quantities are
needed to demonstrate the invariant); a parameter reference in any position other
than the one above; bounds of a different dimension from the parameter; an absent
bound. **In-bounds configuration *values* are not part of the Slice-1 input at
all** — they are Decision Inputs bound at decision time (ADR-0003 §6/I7, RFC-0001
§14), and Slice 1 has no decision. Only the **declared structure and bounds**,
which are identity-bearing, are present.

### 3.10 Conditions

| Property | Domain |
|---|---|
| lineage identity | opaque, §3.3 |
| classification | exactly one of `BLOCKING`, `TEMPORARY` |
| guard | §3.6 |

**Prohibited:** zero or more than one classification; any classification value
outside the two named; any runtime-mutable or evidence-dependent classification
(RFC-0001 §10 — classification is static for a revision); any ordering relation
to another condition (deferred, §3.1).

---

## 4. A2 — Slice-1 Canonical Semantic Contract

This section is the **authoritative meaning** of every construct admitted in §3.
It is a conforming instance, restricted to the Slice-1 fragment, of the
**Canonical Semantic Contract** established by ADR-0004 §9 — the object that both
the Well-Formedness checker and any future runtime evaluator must realize
congruently. It is not an evaluator, a parser, a library, a language, or a
serialization format (ADR-0004 §9.1).

**Congruence obligation (ADR-0004 §9.2).** Any future runtime evaluator that
evaluates a Slice-1-admitted construct **must** interpret it exactly as specified
here. Checked semantics ≠ evaluated semantics is architecture-forbidden. **N-24
remains OPEN** and continues to own the evaluator-semantics *version* scheme;
nothing here defines one.

### 4.1 Type rules

- Every evidence declaration has exactly one declared type; every reference to it
  has that type. Types are **not** inferred, coerced, widened, or promoted.
- An atom is **well-typed** iff the literal's type is identical to the referent's
  declared type, the operator is admitted for that type (§3.6), and — for
  `NUMERIC` — the literal's declared dimension is identical to the referent's
  declared dimension (§4.6).
- An `ENUM` literal is well-typed iff its value is a member of the referent's
  declared value set. A value outside the set is a **type defect**, not a
  never-satisfied atom — Slice 1 does not silently reinterpret it.
- A derivation expression is well-typed iff every leaf is `NUMERIC` and every
  operator's dimensional rule (§4.6) is satisfied.
- There is **no** universal or null type, and **no** null, absent, missing, `NaN`,
  or infinite value anywhere in the fragment.

### 4.2 Numeric semantics

- Numeric values are **exact decimal rational** values. There is no binary
  floating-point semantics, no rounding, no precision loss, and no
  implementation-defined numeric behaviour anywhere in Slice 1. This is a
  *semantic* requirement (ADR-0003 §6 forbids canonicalization introducing lossy
  rounding or conversion); **no numeric library or representation is selected.**
- There is no minimum or maximum magnitude, no overflow, and no underflow.
- `1.0`, `1.00`, and `1e0` denote the **same value**. Representation is not
  meaning (ADR-0003 §6, `CX → ID(value)`). Because Slice-1 input is already
  canonical, the checker compares **values**, never representations.
- **Semantically operative precision is identity-bearing when present** (ADR-0003
  §6, N-28). Slice 1 declares **no** operative precision construct; therefore no
  Slice-1 construct carries operative precision, and no quantization or tolerance
  exists. Comparison is **exact**. Approximate or tolerance-based comparison is
  prohibited — introducing an implicit epsilon would be inventing operative
  semantics.
- Arithmetic (`+`, `-`, `*`) is exact, total, associative and commutative in the
  ordinary mathematical sense over exact rationals.

### 4.3 Equality semantics

- `NUMERIC`: equality of exact value, **and only within an identical declared
  dimension** (§4.6). Cross-dimension equality is not `false` — it is a **type
  defect** (§4.6).
- `BOOLEAN`: ordinary two-valued equality.
- `ENUM`: equality of the canonical categorical value (ADR-0003 §6,
  `CX(spelling) → ID(value)`). Because Slice-1 input is already canonical,
  spelling/case is already canonical on arrival; the checker compares values, and
  performs **no** case folding or spelling normalization of its own.
- `!=` is the exact complement of `=` on the same well-typed operand pair.

### 4.4 Comparison semantics

- `<`, `<=`, `>`, `>=` are the ordinary total order on exact rationals, defined
  **only** between values of an identical declared dimension.
- They are **undefined** — and therefore a type defect — on `BOOLEAN` and `ENUM`.
  Slice 1 declares no ordering over those domains.

### 4.5 Boolean / guard semantics

- A guard is a **conjunction**: it holds exactly when **every** atom holds.
- The **empty conjunction holds always** (it is the always-applicable guard).
  This is the standard meaning of an empty conjunction and is stated explicitly so
  no implementation invents a different one.
- Atom order within a guard carries **no meaning** (ADR-0003 §6: field ordering is
  `CX`). Two guards with the same atom set are the same guard.
- Guard semantics are **two-valued**. There is no unknown, no third value, and no
  short-circuit-dependent meaning.

### 4.6 Dimensional / unit semantics

- A **dimension** is an **opaque symbol** compared by **exact equality only**.
- Slice 1 defines **no** unit conversion, **no** dimension algebra beyond the
  rules below, and **no** dimension hierarchy or compatibility relation. This is
  the minimal system consistent with ADR-0003 §6: *"normalization occurs only
  within the same declared dimension; dimensionally-distinct quantities must never
  canonicalize together; canonicalization introduces no lossy rounding or
  conversion."*
- A **dimensionless literal** carries the distinguished dimension symbol
  `DIMENSIONLESS`.
- Dimensional rules for derivation expressions (§3.8):
  - `a + b` and `a - b`: require `dim(a) = dim(b)`; result dimension is `dim(a)`.
  - `a * s` where `s` is a dimensionless literal: result dimension is `dim(a)`.
  - a literal or reference: its declared dimension.
- **All three candidate derivations must yield the same dimension as each other.**
  Entry, Stop Loss, and Take Profit are quantities of one kind. Slice 1 does not
  name that dimension (naming a price dimension would be inventing product
  vocabulary); it requires only that the three agree. Disagreement is a check 14
  defect.
- Any dimensional-rule violation is a defect. It is **never** repaired by
  conversion, scaling, or coercion.

### 4.7 Missing / unknown reference behaviour

- A reference whose target lineage identity is not declared in the Strategy
  structure is an **unresolved reference** → check 2 defect. It is never treated
  as absent-and-defaulted, never as false, never as skipped.
- A duplicate lineage identity within one Strategy makes references ambiguous →
  check 2 defect. The checker does **not** pick one.
- An unknown construct kind or an undeclared property → check 16
  `UNKNOWN_CONSTRUCT`, **fail closed** (ADR-0002 §10b(2), ADR-0003 §6.1). Never
  ignored, never best-effort interpreted.

### 4.8 Condition-classification semantics

- Each condition carries **exactly one static classification**, `BLOCKING` or
  `TEMPORARY`, for the Strategy revision (RFC-0001 §10 / Q-11b; ADR-0002 §2
  item 2).
- The classification is a **declared static attribute**. It is never derived from
  the guard, never influenced by evidence, and never mutable at runtime.
- **Slice 1 attaches no status meaning to either value.** How a BLOCKING or
  TEMPORARY condition resolves into DO NOT TRADE or WAIT is owned entirely by the
  Validation Model and is outside Slice 1. Slice 1 checks only that exactly one
  valid classification is declared.

### 4.9 Direction-rule and precedence semantics

- A Direction rule is **applicable** in a state exactly when its guard holds.
- Two rules are **co-applicable** iff some state satisfies both guards
  simultaneously (their guards **overlap** — §4.10).
- **Precedence** is the declared strict order over rules. Required properties
  (ADR-0004 check 9, ADR-0002 §10a): **strict/irreflexive**, **acyclic**, **total
  over any co-applicable set**, and **tie-free**. A declared total order over
  ordinals is automatically irreflexive and acyclic; the substantive Slice-1
  obligation is **tie-freeness over co-applicable rules**, which is check 9.
- **Resolution meaning:** in a state, the applicable rule of **highest**
  precedence determines the Direction outcome. Because §3.7 requires a
  lowest-precedence always-applicable rule, at least one rule is applicable in
  every state; because check 9 forbids co-applicable ties, the highest-precedence
  applicable rule is unique. Therefore exactly one outcome is derived in every
  state.
- **Runtime never invents or repairs precedence** (ADR-0002 §10a). Ambiguity is
  rejected statically, here.

### 4.10 Overlap, contradiction, and subsumption semantics

These three relations are the analytic core of Slice 1. All are defined through
one construction:

**Constraint domain of a guard.** For a guard `G` and each declared evidence item
`e`, the *constraint domain* `D(G, e)` is the set of values of `e` permitted by
the atoms of `G` that reference `e`:

- `NUMERIC`: the intersection of the half-lines and points the atoms impose — a
  set expressible as a single interval (open/closed/point) **minus a finite set of
  excluded points** contributed by `!=` atoms.
- `BOOLEAN`: a subset of `{true, false}`.
- `ENUM`: a subset of the declared value set.
- If `G` has no atom referencing `e`, `D(G, e)` is the **full declared domain** of
  `e` (unconstrained).

Because guards are conjunctions of single-reference atoms, the set of states a
guard admits is exactly the product of its per-reference constraint domains. That
product structure is what makes all three relations decidable by **per-reference**
comparison, in time linear in the number of declared evidence items and atoms —
and is precisely why disjunction and evidence-vs-evidence atoms are excluded
(§3.6).

- **Contradiction.** `G` is contradictory iff `D(G, e)` is **empty** for some `e`.
  A contradictory guard admits no state. Emptiness is decided per reference:
  a numeric interval with lower bound above upper bound, a point excluded by its
  own `!=`, an empty boolean subset, or an empty enum subset.
- **Overlap.** `G1` and `G2` overlap iff `D(G1, e) ∩ D(G2, e)` is **non-empty for
  every** `e`. Non-overlapping guards are **mutually exclusive** — ADR-0002 §10a's
  first resolution mechanism.
- **Subsumption.** `G2` is subsumed by `G1` iff `D(G2, e) ⊆ D(G1, e)` for
  **every** `e` — every state satisfying `G2` also satisfies `G1`.

**Numeric-domain note.** Interval intersection, emptiness, and containment on
exact rationals with finitely many excluded points are exact and decidable by
comparison of declared boundary values. **No solver, prover, decision procedure,
or constraint library is selected or implied** — these are ordinary comparisons
between declared literals, which is exactly why the fragment is bounded.

### 4.11 Configuration-bound semantics

- A parameter's bounds define a **closed interval** `[lower, upper]` in the
  parameter's declared dimension.
- **Valid** iff `lower <= upper` (equality permitted — a pinned parameter is
  well-formed) and both bounds share the parameter's dimension.
- Bounds are **identity-bearing declared structure** (ADR-0003 §6, I7). In-bounds
  *values* are Decision Inputs, excluded from revision identity, and absent from
  Slice 1 entirely (§3.9).
- The `adjustable` flag is declared. Slice 1 attaches no further meaning to it and
  performs no check on it beyond presence; its Decision-Input consequences are
  runtime/decision-level and out of scope.

### 4.12 Candidate-value derivation semantics

- Each derivation denotes an exact rational quantity in a declared dimension,
  computed by the ordinary meaning of `+`, `-`, `*` under §4.2 and §4.6.
- Derivations are **not evaluated** in Slice 1 — there are no evidence values and
  no configuration values to evaluate against. Slice 1 checks derivations
  **statically**: well-typedness, reference resolution, and dimensional
  consistency. This is deliberate and is the boundary between Well-Formedness and
  runtime.
- Derivations have no ordering relationship among themselves in Slice 1. Whether
  Stop Loss must sit below Entry for a BUY is a **product** question about
  candidate-value coherence that **no accepted document decides**; asserting it
  here would be inventing product semantics. See §7.3.

---

## 5. Slice-1 Mandatory Well-Formedness Checks

This section **maps** the admitted subset onto the mandatory check set already
required by ADR-0004 §5. It introduces **no new check** and **no new Well-Formedness
architecture**. Check numbers are ADR-0004's.

**Global rules, all inherited from ADR-0004 §11 and restated as implementation
obligations:**

- **Aggregate, never fail-fast.** All independently-computable mandatory defects
  are collected.
- **Per-check prerequisite gating, not stage membership.** A check is evaluated
  whenever **its own** prerequisites are satisfied. An unrelated failure must not
  suppress an independently-computable check.
- A check whose own prerequisites fail is **not evaluated** and is recorded as
  deterministic metadata `not-evaluated: prerequisite <defect-identity> failed` —
  **never** as an invented defect. A skipped check neither passes nor fails.
- The **verdict is UNUSABLE iff at least one evaluated mandatory check produces at
  least one defect**; otherwise **WELL-FORMED**. A skipped check can never turn an
  invalid Strategy into Well-Formed.
- Defect **categories** below are **conceptual names**, not implementation codes
  (following `glossary.md`'s treatment of `STRATEGY_INCOMPLETE_DIRECTION_LOGIC`).

### 5.1 Check table

| # | ADR-0004 check | Slice 1 | Prerequisites | Deterministic PASS condition | Deterministic defect condition (category) |
|---|---|---|---|---|---|
| 1 | Required-field completeness | **APPLICABLE** — **unconditional presence only** (M-1) | none | Every construct present in the structure declares every property §3 marks **Unconditional** for its kind. **Type-dependent properties are outside check 1 entirely** — they belong exclusively to check 6 (§3.5). | Any **unconditionally** required property absent → `MISSING_REQUIRED_PROPERTY`, one defect per (construct, missing property), discriminated by the property (§6.4.2 kind 2). Never reported for a type-dependent property. |
| 2 | Stable identity / reference integrity | **APPLICABLE** | **Per unit of work, never Strategy-wide.** Uniqueness: none. Resolution of one reference: check 1 passes for the **referencing construct**. Well-typedness of one atom or derivation node: check 1 passes for the referencing construct **and** for the **single declaration that atom or node resolves to**. A defect in any *other* declaration is irrelevant and must not suppress it. | Every lineage identity in the structure is unique; every evidence reference resolves to a declared evidence item; every configuration reference resolves to a declared parameter; every atom and derivation node is well-typed per §4.1. | Duplicate lineage identity → `DUPLICATE_LINEAGE_IDENTITY` (one per duplicated identity). Unresolved reference → `UNRESOLVED_REFERENCE` (one per reference). Type violation → `TYPE_MISMATCH` (one per atom or derivation node). Enum literal outside declared value set → `ENUM_VALUE_NOT_DECLARED`. |
| 3 | Condition-classification completeness | **APPLICABLE** | check 1 passes for the condition | The condition declares exactly one classification, drawn from `{BLOCKING, TEMPORARY}`. | Zero, multiple, or out-of-domain classification → `INVALID_CONDITION_CLASSIFICATION` (one per condition). |
| 4 | Configuration bounds validity | **APPLICABLE** | check 1 passes for the parameter | `lower <= upper`, and both bounds share the parameter's declared dimension (§4.11). | `lower > upper` → `INVERTED_CONFIGURATION_BOUNDS`. Bound dimension ≠ parameter dimension → `CONFIGURATION_BOUND_DIMENSION_MISMATCH`. One defect per parameter per violation. |
| 5 | Configuration structure-preservation (ADR-0003 §6.2) | **APPLICABLE** | check 1 passes for the referencing construct | Every configuration-parameter reference occurs **only** as a leaf of a candidate-value derivation expression (§3.9). | A configuration reference in a guard, classification, Direction outcome, precedence ordinal, evidence declaration, or envelope position → `CONFIGURATION_IN_STRUCTURAL_POSITION` (one per offending reference). |
| 6 | Evidence-declaration integrity | **APPLICABLE** — **exclusive owner of all type-dependent declaration validity** (§3.5, M-1) | check 1 passes for the declaration (i.e. its **unconditional** properties — lineage identity and declared type — are present) | Type is one of the three admitted; dimension present iff `NUMERIC`; value set present, non-empty, and duplicate-free iff `ENUM`. | Each violation → `INVALID_EVIDENCE_DECLARATION` (one per declaration per violation), discriminated by the property mapped in §5.1.1. Check 1 reports **none** of these. |
| 7 | Evidence relationship / order validity | **DEFERRED — constructs outside Slice 1** | — | — | Ordered-evidence relations (Context / Location / Sequence / Timing / Confirmation) are not admitted in Slice 1 (§3.1), so no instance of this check exists. This is a **scope deferral**, not an inadmissibility ruling, and not a silent removal of a mandatory check. |
| 8 | Direction coverage | **APPLICABLE** (default-rule form) | **Only the properties check 8 actually consumes**: for every Direction rule, its **precedence ordinal** and its **guard** are present (check 1 for those two properties on that rule). Check 8 reads guards **only** to ask whether each is the syntactic empty guard — it performs **no** type, reference, overlap, or domain analysis — so it requires **neither check 2 nor check 6 on any rule**, and an unrelated type or reference defect in any rule must not suppress it. | Exactly one Direction rule declares the empty always-applicable guard, and it holds the lowest declared precedence (§3.7). Given §4.9, every reachable required state then resolves. | No always-applicable rule → `DIRECTION_COVERAGE_INCOMPLETE`. More than one → `MULTIPLE_TOTAL_DIRECTION_RULES`. An always-applicable rule that is not lowest-precedence → `TOTAL_RULE_NOT_LOWEST_PRECEDENCE`. Zero Direction rules → `DIRECTION_COVERAGE_INCOMPLETE`. |
| 9 | Direction conflict / precedence validity | **APPLICABLE** | check 1 and check 2 pass for **both** rules of the pair; check 6 passes for every evidence declaration both reference | For **every unordered pair** of Direction rules: either their guards do **not** overlap (mutually exclusive, §4.10), or their declared precedence ordinals **differ**. | Overlapping pair with **equal** precedence ordinals → `DIRECTION_PRECEDENCE_TIE`. **Exactly one defect is emitted per offending unordered pair, anchored deterministically (F-1):** compare the two rules' lineage identities under the contract-valid total order of §3.3.1; the rule that **sorts first** is the defect's **offending/anchoring construct** (§6.3 component 1, §6.4 key 2), and the **other** rule is the **COUNTERPART** discriminator (§6.4.2 kind 6). Because the anchor is chosen by that order and not by how the pair was reached, the rule is **symmetric in discovery order** — evaluating `(R_a, R_b)` or `(R_b, R_a)` yields the **same** defect, the **same** identity, and the **same** ordering position. Anchoring must **never** use collection position, traversal order, source position, serialization order, implementation-language ordering, generated ordering, hash ordering, or locale/collation ordering. Duplicate precedence ordinals among non-overlapping rules are **not** a defect — mutual exclusivity is an independently sufficient resolution mechanism under ADR-0002 §10a. |
| 10 | Direction determinism | **APPLICABLE** | check 1 passes for the rule | Every Direction rule declares **exactly one** outcome, drawn from `{BUY, SELL}`. | Zero, multiple, or out-of-domain outcome → `INVALID_DIRECTION_OUTCOME` (one per rule). Given checks 8, 9, and 10 together, §4.9 guarantees exactly-one resolution in every state — the ADR-0002 §10a invariant. |
| 11 | Unreachable operative rules | **APPLICABLE** (contradiction + pairwise subsumption) | **Contradiction part**, per rule: checks 1 and 2 pass for **that rule**, **and check 6 passes for every evidence declaration that rule's guard references** (M-2 — the constraint domain of §4.10 is derived from the declared domain, so an invalid declaration makes the analysis non-computable, not contradictory). **Pairwise-subsumption part**, per pair: checks 1 and 2 pass for **both** rules of the pair, **and check 6 passes for every evidence declaration those two rules reference**. Scoped per unit of work; never Strategy-wide. | The rule's guard is not contradictory (§4.10), **and** the rule is not subsumed by any single strictly-higher-precedence rule's guard. | Contradictory guard → `UNREACHABLE_CONTRADICTORY_GUARD`. Subsumed by a single higher-precedence rule → `UNREACHABLE_SUBSUMED_RULE` (one defect per subsumed rule, discriminated by the subsuming rule). **Detection criterion is pairwise only** — see §7.2. |
| 12 | Contradictory operative rules | **APPLICABLE** | Per condition: checks 1 and 2 pass for **that condition**, **and check 6 passes for every evidence declaration that condition's guard references** (M-2). An invalid declared domain — an empty `ENUM` value set above all — must **never** be used to derive a contradiction in a condition that is not itself defective. Scoped per unit of work; never Strategy-wide. | No **condition** guard is contradictory (§4.10). | Contradictory condition guard → `CONTRADICTORY_CONDITION_GUARD` (one per condition). (Contradictory *Direction*-rule guards are reported under check 11 as `UNREACHABLE_CONTRADICTORY_GUARD`; the two checks are kept disjoint so no single fault yields two defects for the same reason.) |
| 13 | Candidate-value derivation validity | **APPLICABLE** — evaluated as **two independent parts** | **13A (per-derivation validity)**, once per *present* derivation: check 1 and check 2 pass for **that derivation**. **13B (Strategy-level role completeness)**, once per Strategy: check 1 passes for the envelope's candidate-derivation collection. The two parts are **independently computable and neither gates the other**. | **13A:** the derivation's expression conforms to §3.8 — every leaf resolves and is `NUMERIC`, every operator is admitted, the expression is finite and acyclic. **13B:** each of the three roles (`ENTRY`, `STOP_LOSS`, `TAKE_PROFIT`) is present exactly once. | **13A:** prohibited operator or form → `INVALID_DERIVATION_FORM` (one per offending node). **13B:** missing role → `MISSING_CANDIDATE_DERIVATION` (one per missing role); duplicate role → `DUPLICATE_CANDIDATE_DERIVATION` (one per duplicated role). |
| 14 | Dimensional / unit consistency (ADR-0003 §6) | **APPLICABLE** — evaluated as **two independent parts** | **14A (per-derivation dimensional consistency)**, once per *present* derivation: **13A** passes for **that derivation**, and check 2 passes for **that derivation's** leaves. It depends on **no other derivation** and **not on 13B** — a missing or duplicated role elsewhere must not suppress it. **14B (cross-derivation agreement)**, once per Strategy: **13B** passes (all three roles present exactly once — otherwise there is no set of three dimensions to compare) **and 14A passes for all three derivations**. | **14A:** every `+`/`-` node has dimensionally identical operands; every `*` node has a dimensionless literal operand (§4.6). **14B:** all three derivations yield an identical dimension. | **14A:** operand dimension mismatch → `DIMENSION_MISMATCH` (one per offending node). **14B:** divergent candidate dimensions → `CANDIDATE_DIMENSION_DIVERGENCE` (one per Strategy). |
| 15 | Required provenance presence | **APPLICABLE** (reduced) | none | Every operative node (Strategy envelope, condition, Direction rule, candidate derivation) carries a lineage identity — the rule-level provenance anchor of ADR-0003 §9 / C4. | Missing lineage identity on an operative node → `MISSING_OPERATIVE_PROVENANCE` (one per node). Non-operative provenance (authoring, timestamps, tooling — all **PX**) is out of Slice 1 and is not checked. |
| 16 | Prerequisites for canonicalization / revision identity (ADR-0003 I9) | **APPLICABLE** (reduced) | none | The semantic-model version is declared and equals the contract-declared constant **`SM-1`** (§3.4.1); **every** construct kind and property in the structure is one this contract declares. | Absent or unsupported version → `UNSUPPORTED_SEMANTIC_MODEL_VERSION`. Any unknown construct kind or undeclared property → `UNKNOWN_CONSTRUCT` (one per occurrence), **fail closed** (ADR-0002 §10b(2), ADR-0003 §6.1). Slice 1 computes **no** canonical form and **no** revision identity; it establishes only the *preconditions* ADR-0003 I9 requires. |
| 17 | Applicability-instance validity | **DEFERRED — constructs outside Slice 1** | — | — | Applicability constraints are not admitted in Slice 1 (§3.1, row 13), so no instance of this check exists. **Scope deferral, not an inadmissibility ruling.** |

### 5.1.1 Check-6 violation → property mapping (m-3)

Check 6 emits one defect per violation per declaration and uses the **PROPERTY**
discriminator (§6.4.2 kind 2). Each violation maps to the **existing** §3.5
property it concerns; no property is invented to support ordering.

| Violation | Mapped property (§3.5 position) |
|---|---|
| declared type is not one of `NUMERIC` / `BOOLEAN` / `ENUM` | declared type (2) |
| `NUMERIC` declaration with no dimension | declared dimension (3) |
| dimension present on a `BOOLEAN` or `ENUM` declaration | declared dimension (3) |
| `ENUM` declaration with no value set | declared value set (4) |
| `ENUM` value set is empty | declared value set (4) |
| `ENUM` value set contains duplicate values | declared value set (4) |
| value set present on a `NUMERIC` or `BOOLEAN` declaration | declared value set (4) |

The mapping is total over the check-6 violation set and makes ordering
deterministic: two check-6 defects on one declaration are separated by the mapped
property's §3.5 position. Two violations cannot map to the same property on the
same declaration — the value-set violations are mutually exclusive (a value set
cannot be simultaneously absent, empty, and duplicate-bearing, and a value set
prohibited by type cannot also be required by it), as are the dimension
violations.

### 5.2 Prerequisite graph

**Governing invariant (the sole basis for every edge below).**

> **A defect may suppress a mandatory check evaluation only where that specific
> defect makes that specific evaluation non-computable under this contract.
> Unrelated defects must never suppress an independently-computable mandatory
> check.**

This is ADR-0004 §11 restated as an implementation obligation. It is the test
every edge below must satisfy, and the test any future edge must satisfy. The gate
is **per check, per unit of work** — never per stage, never per collection, and
never Strategy-wide unless the check itself is a Strategy-level property.

The **unit of work** for each check is the thing one evaluation produces a verdict
about: a single construct, a single reference, a single atom, a single derivation
node, a single rule pair, or — for genuinely Strategy-level properties (checks 8,
13B, 14B, 16, and the uniqueness part of check 2) — the Strategy. Prerequisites
attach to the unit of work, not to the check as a whole.

The full dependency set is small enough to state exhaustively:

```
check 1  (required fields)          →  no prerequisite
check 15 (lineage presence)         →  no prerequisite
check 16 (version / unknown)        →  no prerequisite
check 2  (uniqueness)               →  no prerequisite
check 3, 4, 6, 10                   →  check 1 on the same construct
check 2  (one reference resolves)   →  check 1 on the referencing construct
check 2  (one atom/node well-typed) →  check 1 on the referencing construct
                                       + check 1 on the ONE declaration it resolves to
check 5                             →  check 1 on the referencing construct
check 8                             →  check 1 for the precedence ordinal and guard
                                       of each Direction rule — nothing further
check 9, 11 (pairwise subsumption)  →  checks 1, 2 on BOTH rules of the pair
                                       + check 6 on the declarations THOSE two reference
check 11 (contradiction), 12        →  checks 1, 2 on the same construct
                                       + check 6 on the declarations THAT guard references
check 13A (per derivation)          →  checks 1, 2 on THAT derivation
check 13B (role completeness)       →  check 1 on the envelope's derivation collection
check 14A (per derivation)          →  13A on THAT derivation + check 2 on ITS leaves
check 14B (cross-derivation)        →  13B + 14A on all three derivations
```

Worked consequences, to make the gating unambiguous:

- A `TYPE_MISMATCH` in condition *C*'s guard suppresses check 12 **for condition C
  only**. It must not suppress check 12 for any other condition, must not suppress
  checks 9 or 11 for rule pairs that do not reference the offending construct, and
  must not suppress checks 3, 4, 6, 8, 10, 13, 14, 15, or 16 at all.
- A malformed evidence declaration *E* suppresses well-typedness checking **only
  for atoms and derivation nodes that resolve to *E***. Atoms resolving to any
  other declaration are checked normally.
- A type or reference defect in Direction rule *R* does **not** suppress check 8.
  Coverage is decided from precedence ordinals and guard-emptiness alone.
- A missing `TAKE_PROFIT` produces a 13B defect and suppresses only **14B**. The
  dimensional consistency of the present `ENTRY` and `STOP_LOSS` derivations is
  still checked under **14A**.
- An `ENUM` declaration *E* with an empty value set produces its own check-6
  defect. Contradiction analysis (checks 11 and 12) is **not evaluated** for the
  guards that reference *E*, and is recorded as not-evaluated metadata naming that
  check-6 defect. It must **never** be reported as a contradiction in those guards:
  the constraint domain of §4.10 is derived from the **declared** domain, so an
  invalid declared domain makes the analysis **non-computable**, not false.
  Conditions and rules whose guards do **not** reference *E* are analyzed normally.

**§5.1 and §5.2 are one normative prerequisite model (m-4).** Where the check
table and this graph describe the same edge they state it identically; neither
introduces Strategy-wide gating. If they are ever found to diverge, the contract is
defective and the divergence must be reported, not resolved by preference.

Each suppressed evaluation is recorded as `not-evaluated` metadata naming the
check, the unit of work, and the blocking defect's identity.

### 5.3 Complexity envelope (ADR-0004 §4 conformance)

Let `n` = declared evidence items, `p` = configuration parameters, `c` =
conditions, `r` = Direction rules, `a` = total atoms, `d` = total derivation
nodes.

| Check(s) | Bound |
|---|---|
| 1, 3, 4, 6, 10, 15, 16 | linear in structure size |
| 2 | linear in `a + d` with exact-equality identity lookup |
| 5 | linear in the number of references |
| 8 | linear in `r` |
| 12, and the contradiction part of 11 | linear in `a` |
| 9, and the subsumption part of 11 | `O(r² · n)` — explicit pairwise comparison over declared rules and declared references |
| 13, 14 | linear in `d` |

Worst case is **quadratic in the declared construct size**. Nothing enumerates an
implicit region space, a power set, or any undeclared combination. This is inside
the ADR-0004 §4 polynomial bounded-compositional-traversal envelope.

---

## 6. Result Contract

Realizes ADR-0004 §10 over the Slice-1 fragment.

### 6.1 Verdict

Binary: **WELL-FORMED** or **UNUSABLE**. No gradations. UNUSABLE is **never** a
runtime status and is never presented as READY / WAIT / DO NOT TRADE.

### 6.2 Result contents

1. the verdict;
2. the **defect set** — empty iff WELL-FORMED;
3. the **not-evaluated set** — one deterministic record per suppressed check
   evaluation, each naming the check, the construct, and the blocking defect
   identity;
4. the **checker-semantics version** the result is bound to (ADR-0004 §8).

**Checker-semantics version (ADR-0004 §8; M-4).** **The Slice-1 checker-semantics
version is the opaque token `WF-1`** — declared by this contract, identical for
every conforming Slice-1 implementation, and bound to every authoritative WF result
this contract defines. No implementation selects it, so two conforming checkers
cannot stamp different tokens for identical semantics, and a stored or exchanged
result's token identifies the authoritative semantics that produced it — which is
the reproducibility-identification property ADR-0004 §8 requires and the
drift-detection purpose it assigns (mitigating W-R4 / R-30-series checker drift).

`WF-1` denotes **exactly the authoritative checker semantics of this contract**.
Its scope is every semantic rule in §3, §4, §5, and §6.4 — construct admission,
mandatory-invariant semantics, verdict-affecting analysis, defect identity, defect
categories, and canonical ordering. **Any material change to authoritative Slice-1
Well-Formedness semantics requires a different checker-semantics version**; `WF-1`
must never be reused for altered semantics. It **encompasses and binds** the
applicable Canonical Semantic Contract version, which for Slice 1 is §4 of this
document, and it likewise binds the supported semantic-model constant `SM-1`
(§3.4.1) — widening that set is a `WF-1` change.

`WF-1` is an opaque identifier compared by exact equality (§3.3.1). This
declaration creates **no general versioning scheme** and defines **no** version
syntax, encoding, ordering, serialization, schema, or persistence. It is
conceptually distinct from the runtime evaluator-semantics version, which it does
not define: **N-24 remains OPEN.**

**Optional diagnostics** (ADR-0004 §5): Slice 1 emits **none**. If a later slice
adds any, they are advisory metadata and can **never** change the verdict.

### 6.3 Defect identity

Per ADR-0004 §10, the authoritative defect identity is a deterministic function of:

1. the **offending construct's lineage identity** (never a byte offset, line
   number, editor position, serialization location, or presentation location);
2. the **defect category**;
3. an **additional discriminator** where one construct can produce multiple
   defects of the same category — for example the missing property name (check 1),
   the offending atom, by its content (checks 2, 12), or the offending derivation
   node, by its path (checks 2, 13A, 14A), or the counterpart rule's lineage
   identity (checks 9, 11). The discriminator kinds and their values are defined
   in §6.4.2;
4. the **applicable Strategy revision identity, where one is applicable**.

**Boundary interpretation (dispositioned by ARR-0007 D-3).** ADR-0003 §5 fixes the
ordering `Well-Formed → canonicalize → identity`, so **at Slice-1 execution time a
revision identity does not yet exist** for a Strategy being checked for the first
time. Slice 1 therefore treats component (4) as a **caller-supplied optional
binding**: when the caller supplies an applicable revision identity (for example
when re-checking an already-identified revision), the defect identity is
revision-qualified exactly as ADR-0004 §10 requires; when none is applicable, the
identity is the unqualified triple (1)(2)(3). This reconciles ADR-0004 §10 with
ADR-0003 §5 without weakening either, and Slice 1 computes **no** revision
identity itself. **This interpretation was dispositioned ACCEPT WITH BOUNDED
CORRECTION by ARR-0007 (D-3); the correction is the rule immediately below.**

**Non-comparability across the revision-identity boundary (ARR-0007 D-3).**

> The unqualified and revision-qualified identities of the same underlying defect
> are **not interchangeable** and **MUST NOT be directly compared, equated, or
> deduplicated across that boundary.**

A consumer **must not** infer identity continuity merely because the underlying
semantic defect appears equivalent: an unqualified identity produced before a
revision identity existed and a revision-qualified identity produced by a later
re-check are **distinct identities**, and equality between them is undefined
rather than false. Identity comparison is well-defined **only within one
qualification state** — unqualified with unqualified, or revision-qualified with
revision-qualified under the **same** revision identity.

**No cross-boundary reconciliation or matching mechanism is defined here**, and
none may be inferred from this rule. Whether such a mechanism is ever wanted is a
downstream question this contract does not open. This rule alters neither
ADR-0003's ordering `Well-Formed → canonicalize → revision identity` nor
ADR-0004 §10's revision-qualified defect semantics.

### 6.4 Canonical defect ordering

A total, deterministic order, so two conforming checkers present defects
identically (ADR-0004 §10). Compare in order; the first difference decides:

1. **ADR-0004 check number**, ascending;
2. **offending construct's lineage identity**, under the total order on the
   lineage-identity domain (§3.3.1);
3. **defect category**, under the ordinal declared in §6.4.1, ascending;
4. **discriminator**, under the rule declared in §6.4.2.

Because lineage identities are unique within a Strategy (check 2) and the
discriminator distinguishes same-category defects on one construct (§6.3), this
order is **total** — no two distinct defects compare equal. The ordering keys use
**no** serialization position, file offset, presentation location, locale, host
language ordering, hash, or generated value.

#### 6.4.1 Declared total order over defect categories

This is the **complete** set of defect categories admitted by Slice 1. The
ordinals below **are** the order; they are declared here and are not derived from
the category names, so no lexical convention is involved.

| Ordinal | Category | Check |
|---|---|---|
| 1 | `MISSING_REQUIRED_PROPERTY` | 1 |
| 2 | `DUPLICATE_LINEAGE_IDENTITY` | 2 |
| 3 | `UNRESOLVED_REFERENCE` | 2 |
| 4 | `TYPE_MISMATCH` | 2 |
| 5 | `ENUM_VALUE_NOT_DECLARED` | 2 |
| 6 | `INVALID_CONDITION_CLASSIFICATION` | 3 |
| 7 | `INVERTED_CONFIGURATION_BOUNDS` | 4 |
| 8 | `CONFIGURATION_BOUND_DIMENSION_MISMATCH` | 4 |
| 9 | `CONFIGURATION_IN_STRUCTURAL_POSITION` | 5 |
| 10 | `INVALID_EVIDENCE_DECLARATION` | 6 |
| 11 | `DIRECTION_COVERAGE_INCOMPLETE` | 8 |
| 12 | `MULTIPLE_TOTAL_DIRECTION_RULES` | 8 |
| 13 | `TOTAL_RULE_NOT_LOWEST_PRECEDENCE` | 8 |
| 14 | `DIRECTION_PRECEDENCE_TIE` | 9 |
| 15 | `INVALID_DIRECTION_OUTCOME` | 10 |
| 16 | `UNREACHABLE_CONTRADICTORY_GUARD` | 11 |
| 17 | `UNREACHABLE_SUBSUMED_RULE` | 11 |
| 18 | `CONTRADICTORY_CONDITION_GUARD` | 12 |
| 19 | `INVALID_DERIVATION_FORM` | 13A |
| 20 | `MISSING_CANDIDATE_DERIVATION` | 13B |
| 21 | `DUPLICATE_CANDIDATE_DERIVATION` | 13B |
| 22 | `DIMENSION_MISMATCH` | 14A |
| 23 | `CANDIDATE_DIMENSION_DIVERGENCE` | 14B |
| 24 | `MISSING_OPERATIVE_PROVENANCE` | 15 |
| 25 | `UNSUPPORTED_SEMANTIC_MODEL_VERSION` | 16 |
| 26 | `UNKNOWN_CONSTRUCT` | 16 |

A category outside this table cannot arise: §5.1 is exhaustive. Adding one is a
checker-semantics version change (§6.2).

#### 6.4.2 Declared total order over discriminators

A discriminator has exactly one of six **kinds**. Compare the **kind ordinal**
first; if the kinds are equal, compare by that kind's rule.

| Kind ordinal | Kind | Used by | Comparison rule |
|---|---|---|---|
| 1 | **NONE** | any defect needing no discriminator | All equal. Reachable only when the preceding keys already distinguish the defects. |
| 2 | **PROPERTY** | checks 1, 4, 6 | The property's **declared position in the §3 property table for the offending construct's kind**, top to bottom, ascending. Both defects share a construct (key 2 tied) and therefore a kind, so one table always applies. |
| 3 | **ROLE** | check 13B | Declared order: `ENTRY` (1) < `STOP_LOSS` (2) < `TAKE_PROFIT` (3). |
| 4 | **ATOM** | checks 2, 12 where the offending sub-element is a guard atom | The triple *(referenced lineage identity, operator, literal)*, compared left to right: lineage identity by §3.3.1; operator by the declared order `<` (1) < `<=` (2) < `>` (3) < `>=` (4) < `=` (5) < `!=` (6); literal by §6.4.3. This is **content-based, not positional** — it does not depend on atom order, consistent with §4.5's rule that atom order carries no meaning. |
| 5 | **NODE-PATH** | checks 2, 13A, 14A where the offending sub-element is a derivation-expression node | The **path from the derivation's root**: the finite sequence of child positions taken to reach the node (left operand = 1, right operand = 2). Compare sequences element-wise ascending; where one is a proper prefix of the other, the **shorter sorts first**. This is a canonical semantic position under the accepted model — operand position is semantically operative for `-` and for the §4.6 dimensional rules — and is expressly permitted by ADR-0004 §10. |
| 6 | **COUNTERPART** | checks 9, 11 (subsumption) | The counterpart rule's lineage identity, by §3.3.1. |

#### 6.4.3 Declared total order over literals

Used only by discriminator kind 4. Compare the **type ordinal** first —
`NUMERIC` (1) < `BOOLEAN` (2) < `ENUM` (3) — then within a type:

- **`NUMERIC`:** by exact rational value, ascending (§4.2); if values are equal,
  by dimension symbol under §3.3.1.
- **`BOOLEAN`:** `false` (1) < `true` (2).
- **`ENUM`:** by categorical value under §3.3.1.

Cross-type comparison is unreachable in practice — key 2 having tied, both atoms
belong to one construct and, sharing a referent, one declared type — but the type
ordinal is declared so the order is total unconditionally.

---

## 7. Explicit Deferrals and the Reasoning Behind Them

### 7.1 Why Direction coverage takes the default-rule form

General Direction coverage asks whether the union of the declared rule guards
covers every reachable required state. Over conjunctive guards, each guard admits
a *box* in the product of declared evidence domains, and deciding whether a union
of boxes covers a region requires reasoning about the implicit regions the
declared boundaries induce — up to `2^k` of them for `k` boundaries. That is
**exactly** the analysis ADR-0004 §4 names as **not** tractability-bounded and
therefore inadmissible.

Slice 1 avoids it entirely by requiring a **lowest-precedence always-applicable
rule** (§3.7). Coverage then holds **by construction**, and check 8 verifies a
structural property in time linear in the rule count. This is faithful to
ADR-0002 §10a — the invariant is that every reachable required state resolves to
exactly one outcome, not that it must be proved by union reasoning.

The cost is honest and should be understood by the Board: a Strategy author who
wants coverage to arise from a partition of guards, with **no** catch-all rule,
cannot express that in Slice 1. Admitting that form later requires a construct
whose coverage analysis stays inside the §4 envelope — for example an explicitly
**declared** partition over a single reference, where coverage is checked against
the declared boundaries rather than an implicit region space. That is a future
slice's work and is **not** decided here.

### 7.2 Why unreachability detection is pairwise

Complete unreachability detection asks whether a rule's guard is subsumed by the
**union** of all higher-precedence guards — the same union-of-boxes problem as
§7.1, with the same tractability verdict.

Slice 1 therefore defines check 11's detection criterion as: **contradictory
guard, or subsumption by a single strictly-higher-precedence rule.** This is
**deterministic, precisely specified, and identical across conforming
implementations**, which is what ADR-0004 §10 and §12 require. It is **sound but
incomplete**: a rule unreachable only through the combined effect of several
higher-precedence rules is not detected in Slice 1.

**This is an authored Slice-1 scoping of a mandatory check's detection criterion,
dispositioned ACCEPT by ARR-0007 (D-2) and recorded in §10.** It does not weaken any
verdict-safety property — every defect it reports is genuinely a defect, and no
undetected unreachable rule can make an otherwise-invalid Strategy pass any other
check. It does mean check 11 is not exhaustive in Slice 1, and the record should
say so plainly rather than imply completeness.

### 7.3 Candidate-value coherence is not a Slice-1 check

Whether Stop Loss must lie on a particular side of Entry for a given Direction is
a **product** question. No accepted document decides it. Slice 1 therefore checks
candidate derivations only for structural validity, reference resolution, and
dimensional consistency (checks 13, 14). Adding a coherence rule here would be
inventing product semantics, which PBA-0015 §9 forbids. **This is not raised as a
blocker** — Slice 1 is a complete and meaningful Well-Formedness decision without
it.

### 7.4 Complete deferral list

Ordered-evidence relations and check 7; applicability constraints and check 17;
disjunction and negation in guards; evidence-vs-evidence comparison; arithmetic in
guards; division and exponentiation in derivations; non-`NUMERIC` configuration
parameters; freshness-policy references; purpose/thesis, names, descriptions,
authoring provenance, timestamps, tool metadata; change provenance; human-facing
revision labels; canonicalization; revision identity; unit conversion and
dimension algebra; optional diagnostics; partition-based Direction coverage;
union-based unreachability; candidate-value coherence.

---

## 8. Conformance Obligations for the Slice-1 Implementation

An implementation conforms to this contract iff:

1. It implements exactly the construct inventory of §3 — no more, no less.
2. It implements the semantics of §4 exactly, including exact numeric semantics,
   dimension-equality-only, and the empty-conjunction rule.
3. It evaluates exactly the checks §5 marks APPLICABLE, with the stated
   prerequisites, PASS conditions, and defect conditions.
4. It aggregates defects, gates per check, and records not-evaluated metadata
   (§5, ADR-0004 §11).
5. It produces the result contract of §6, including the total defect ordering.
6. It is total, pure, deterministic, and side-effect-free (§1.1).
7. It performs no I/O, no persistence, no serialization, no canonicalization, and
   computes no revision identity.
8. Two independent conforming implementations, given the same input structure and
   the same checker-semantics version, produce the **same verdict, the same defect
   set, the same defect identities, the same defect ordering, and the same
   not-evaluated set** (ADR-0004 §10, §12).

Obligation 8 is the acceptance test for the whole slice.

---

## 9. Compliance Statement

| Source | Conformance |
|---|---|
| **Constitution C2** (four pillars) | The checker originates nothing; it reads a trader-authored Strategy and reports defects. Applicability — the one construct adjacent to instrument selection — is deferred. |
| **Constitution C3 / E1** | The checker is deterministic and model-free. No AI, no score, no probability, no confidence anywhere on the path. |
| **Constitution C4** | Every defect anchors to a lineage identity, giving rule-level provenance. |
| **Constitution C5** | Results are reproducible from {input structure, checker-semantics version}; the checker mutates nothing. |
| **RFC-0001 §12** | Detect, report, explain — **never silently repair**. No check in §5 rewrites, defaults, coerces, converts, or repairs any input. This directly honours R-21. |
| **RFC-0001 §10 / Q-11b** | Exactly one static classification per condition (check 3), never runtime-mutable. |
| **RFC-0001 §15 / N-21** | Structurally insufficient Direction logic is a Well-Formedness failure → UNUSABLE (checks 8–10), not WAIT and not DO NOT TRADE. |
| **ADR-0002 §10 / §10c** | Non-Turing-complete, decidable, no prohibited primitive; §3.2. |
| **ADR-0002 §10a** | Exactly-one Direction resolution via mutual exclusivity or declared strict order; ties rejected statically (§4.9, check 9). |
| **ADR-0002 §10b** | Explicit semantic-model version; unknown constructs fail closed (check 16); no implicit defaults. |
| **ADR-0003 §5** | Slice 1 *is* step 1 and performs neither step 2 nor step 3; canonicalization ownership untouched. |
| **ADR-0003 §6 / §6.2** | Dimension identity-bearing, no lossy conversion (§4.6); configuration structurally incapable of restructuring (§3.9, check 5). |
| **ADR-0003 §9 / I9** | Lineage identity on every operative node; Well-Formedness established as the precondition to identity without computing identity. |
| **ADR-0004 §4** | Four admission classes honoured; every analysis decidable, terminating, and tractability-bounded (§3.2, §5.3); constructs deferred for tractability reasons are named as such (§7.1, §7.2). |
| **ADR-0004 §5** | 15 of 17 mandatory checks applicable; 2 deferred solely because their constructs are outside Slice 1, explicitly and not silently. |
| **ADR-0004 §8, §9** | Checker-semantics version bound to every result; §4 is a conforming Canonical Semantic Contract instance for the admitted fragment; congruence obligation stated; N-24 untouched. |
| **ADR-0004 §10, §11** | Binary verdict, deterministic defect set, position-independent defect identity, total defect ordering, aggregate reporting, per-check prerequisite gating. |
| **ADR-0004 §12** | Total and robust to adversarial structure — malformed input yields UNUSABLE, never a crash and never a bypass. |
| **Glossary** | `Direction` = BUY / SELL; UNUSABLE kept distinct from READY / WAIT / DO NOT TRADE; no forbidden decision-facing vocabulary used. |

**No technology is selected by this document.** No programming language,
framework, package manager, parser, serialization format, schema language,
solver, prover, hash algorithm, database, persistence mechanism, cloud or
provider, broker API, MetaTrader integration, or UI framework.

**No accepted document is modified by this document.**

---

## 10. Board Dispositions of Record

Three matters were surfaced by ER-0028 and dispositioned by **ARR-0007**. None was
a coding blocker; all are now closed at review level and recorded here:

1. **§7.1 default-rule coverage form** (ARR-0007 D-1) — **ACCEPT**. The
   expressiveness cost is a known, accepted Slice-1 limit rather than a discovered
   surprise. General union-of-guards coverage remains out of scope.
2. **§7.2 pairwise unreachability criterion** (ARR-0007 D-2) — **ACCEPT**. Sound
   but incomplete by design, deterministic, and claiming no exhaustiveness. No
   solver, prover, or implicit-state-space enumeration is introduced.
3. **§6.3 defect-identity boundary interpretation** (ARR-0007 D-3) — **ACCEPT WITH
   BOUNDED CORRECTION**. The correction — non-comparability across the
   revision-identity boundary — is incorporated in §6.3.

ARR-0007 also raised three acceptance blockers, closed in this revision: the
canonical defect order is now fully declared (§6.4.1–§6.4.3); prerequisite gating
is scoped per unit of work under the §5.2 invariant; and the supported
semantic-model version is fixed by the contract as `SM-1` (§3.4.1).

**N-22, N-24, N-25, Q-06, Q-08, and F-8 remain OPEN and are untouched by this
document.**

---

## 11. Acceptance Record

Accepted by the Product Board via **PBR-0011**, decision date **2026-08-13**,
following the review sequence ARR-0007 → ER-0029 → ARR-0008 → ER-0030 →
ARR-0009 → ER-0031. All acceptance-blocking findings are closed: ARR-0007
**B-1/B-2/B-3** and the **D-3** bounded correction; ARR-0008 **M-1/M-2/M-3/M-4**
and **m-1…m-5**; ARR-0009 **F-1** and the §6.3 clerical correction. ER-0031 was
accepted as sufficient closure verification.

1. **The First Runnable Slice Contract is ACCEPTED.** It is now the normative
   engineering contract for Slice 1.
2. **Its substantive contract text is an immutable historical engineering
   contract.** The admitted construct inventory (§3), the Slice-1 Canonical
   Semantic Contract (§4), the mandatory check mapping and prerequisite model
   (§5), the result contract including defect identity and canonical ordering
   (§6), the deferrals (§7), and the conformance obligations (§8) are fixed as of
   this decision.
3. **Future material changes require an explicitly authorized successor or
   revision — never a silent rewrite.** Any change to authoritative Slice-1
   Well-Formedness semantics additionally requires a **new checker-semantics
   version**; `WF-1` must never be reused for altered semantics (§6.2).
4. **Accepted ADR boundaries remain authoritative.** §0's precedence rule stands:
   where this contract conflicts with ADR-0002, ADR-0003, ADR-0004, RFC-0001, or
   the Product Constitution, the accepted document wins and this contract is
   defective. Acceptance confers no authority to amend, reinterpret, or supersede
   any of them; this contract creates no ADR and supersedes nothing.
5. **Deferred decisions remain deferred.** **N-22**, **N-24**, **N-25**, **Q-06**,
   **Q-08**, and **F-8** remain OPEN and are not resolved by this acceptance, nor
   is any implementation technology selected (§9).

Acceptance records the decision only and changes no substantive contract
semantics.

---

> This contract is now **ACCEPTED**. Its decision text is an immutable engineering
> record. It remains an implementation contract, not an architecture decision — it
> creates no ADR, supersedes nothing, and conforms to every ACCEPTED document
> above.
