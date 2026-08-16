# Slice-G Implementation Report (PBA-0123 / ER-0102)

**Date:** 2026-08-17
**Authority:** PBA-0123 → ER-0102 (allocatted after PBA-0122/ARR-0040 synchronization)
**Governing baseline:** HEAD `1b87c8e` (Slice F closure + ARR-0040 PASS + Slice F CLOSED)
**Source commit:** `a66982f5e015be0353208f4c302d8544b07edfec`
**Push status:** fast-forward push to `origin/main` complete; `main == origin/main` (EQUAL)
**Working tree:** clean except untracked `?? IDEA.md` (excluded from all work)

## 1. PBA-0122 / ARR-0040 synchronization confirmation

PBA-0122 (review authorization) and ARR-0040 (review report) were already synchronized under §12(6) in the preceding governance step:

- PBA-0122 → ARR-0040 recorded in §5 (`docs/governance/identifier-ledger.md` line 200)
- ARR-0040 → PASS — CHECK 6 REMEDIATION VERIFIED recorded in §7 (line 291), with G-1 CLOSED, S-1 CLOSED, T-1 CLOSED, Check 6 independently verified, Slice F CLOSED
- §11 high-water: PBA-0122 / ARR-0040; next allocatable: PBA-0123 / ARR-0041 / ER-0102

No reuse of PBA-0122 or ARR-0040. ARR-0041 (future review) remains unallocated.

## 2. PBA-0123 / ER-0102 allocation

- PBA-0123 allocated in §5 (line 201): Slice-1 Phase-5 Slice G (Checks 9, 11, 12) Implementation authorization → ER-0102
- ER-0102 allocated in §8 (line 390): Slice-1 Phase-5 Slice G (Checks 9, 11, 12) Implementation Report, authority PBA-0123
- §11 high-water updated: PBA → PBA-0123 / next PBA-0124; ER → ER-0102 / next ER-0103; ARR → ARR-0040 / next ARR-0041 (unchanged, review not allocated)

ARR-0041 deliberately NOT allocated — it is the expected next read-only review identifier, reserved for the separate independent review of Slice G.

## 3. Commit

- **Hash:** `a66982f5e015be0353208f4c302d8544b07edfec`
- **Title:** `PBA-0123 / ER-0102 — Slice G (Checks 9, 11, 12) Implementation`
- **Files:** 6 files changed, 1495 insertions(+), 7 deletions(-)
  - `src/checks/direction_conflict.rs` (new, 352 lines)
  - `src/checks/unreachable_rules.rs` (new, 392 lines)
  - `src/checks/contradictory_rules.rs` (new, 404 lines)
  - `src/checks/guard_analysis.rs` (new, 328 lines)
  - `src/checks/mod.rs` (registration + documentation, 10 insertions)
  - `docs/governance/identifier-ledger.md` (PBA-0123/ER-0102 allocation + ER-0101 repair, 16 ±)
- No `IDEA.md` included.

## 4. Pushed branch / sync state

- Fast-forward push to `origin/main` complete: `1b87c8e..a66982f main -> main`
- `main == origin/main` (EQUAL)
- Repository remains PRIVATE (`gh api repos/fanoryu/Ordina` → `private`)

## 5. Exact files changed

| Path | Action | Role |
|---|---|---|
| `src/checks/direction_conflict.rs` | new | Check 9 implementation + tests |
| `src/checks/unreachable_rules.rs` | new | Check 11 implementation + tests |
| `src/checks/contradictory_rules.rs` | new | Check 12 implementation + tests |
| `src/checks/guard_analysis.rs` | new | shared Check-layer §4.10 guard-analysis helper |
| `src/checks/mod.rs` | modified | module registration + documentation |
| `docs/governance/identifier-ledger.md` | modified | PBA-0123/ER-0102 allocation; ER-0101 repaired |
| `docs/engineering/slice-1-implementation-plan.md` | modified | status: Slice G implemented |

## 6. Check-9 semantics

**Population:** every unordered pair of Direction rules.

**PASS condition (§5.1 check 9):** For every unordered pair of Direction rules, either their guards do NOT overlap (§4.10) OR the declared precedence ordinals differ.

**Defect category:** `DIRECTION_PRECEDENCE_TIE` (C9, ordinal 14, ADR-0004 check 9).

**Multiplicity:** exactly ONE defect per offending unordered pair.

**Prerequisites (§5.1):** A pair is evaluated only when computable — both rules' guards present and analyzable under check 1, check 2, check 6 (resolved references, valid declared types, type-consistent atoms), and both rules' precedence ordinals present. A non-computable pair is suppressed **for that pair only**.

**Implementation:** iterates unordered pairs via `for i in 0..rules.len()` / `for b in &rules[i+1..]` (unordered, deterministic). For each pair: both guards must be `Declared::Present` and `analyze_guard` must return `Some`; both precedence ordinals must be `Declared::Present`; then `guards_overlap` (§4.10 per-reference overlap) && `precedence_a == precedence_b` → emit.

**Anchor/discriminator (Revision 4 §C.7):** anchor = lower `OccurrenceId` (`a.occurrence.min(b.occurrence)`); `COUNTERPART` = higher `OccurrenceId` (`Discriminator::Counterpart(counterpart)`). Anchor chosen by supplied occurrence order, not discovery order — symmetric, deterministic. A rule with absent lineage is still representable through its occurrence identity; check 9 does NOT take check 15 as a prerequisite.

**Overlap semantics:** `guards_overlap` quantifies `RefDomain::overlaps` over the union of both guards' referenced lineages, treating a lineage referenced by only one guard as the other's unconstrained (full declared) domain — per accepted §4.10.

## 7. Check-11 semantics

**Two accepted populations (§5.1 check 11):**

### 7a. `UNREACHABLE_CONTRADICTORY_GUARD` (C11, ordinal 16)

- **Population:** one per contradictory Direction-rule guard.
- **PASS condition violated:** a Direction rule whose own guard is contradictory (some referenced lineage's constraint domain empty, §4.10).
- **Anchor:** that rule's `OccurrenceId`.
- **Discriminator:** `NONE`.
- **Prerequisite:** that rule's guard present and analyzable (check 1, 2, 6 for that rule); otherwise suppressed for that rule.

### 7b. `UNREACHABLE_SUBSUMED_RULE` (C11, ordinal 17)

- **Population:** one per valid `(R, S)` where `S` subsumes `R`.
- **PASS condition violated:** rule `R` is subsumed by a single rule `S` of **strictly higher** precedence: `D(R, e) ⊆ D(S, e)` (§4.10) for every referenced evidence item `e`.
- **Anchor:** `R`'s `OccurrenceId` (the subsumed/offending rule), regardless of occurrence order.
- **Discriminator:** `COUNTERPART` = `S`'s `OccurrenceId`.
- **Prerequisite:** both guards present and analyzable and both precedence ordinals present; `S`'s precedence strictly higher than `R`'s; non-computable pair suppressed locally.
- **Multiplicity:** multiple distinct subsumers remain multiple distinct defects — NOT collapsed (enforced by `assert_ne!(defects[0].cmp(&defects[1]), Ordering::Equal)` in `multiple_distinct_subsumers_remain_distinct`).

**Implementation (unreachable_rules.rs):** iterates rules; for each, contradiction part checks `is_contradictory(&domains)` and emits `UnreachableContradictoryGuard`/`NONE`. Subsumption part iterates other rules; requires `other_precedence > rule_precedence`, both guards present and analyzable, then `guard_subsumes(strategy, &other_domains, &rule_domains)` (i.e. `D(S,e) ⊇ D(R,e)` ⇔ `D(R,e) ⊆ D(S,e)`).

## 8. Check-12 semantics

**Population:** one `CONTRADICTORY_CONDITION_GUARD` per contradictory condition guard.

**Defect category:** `CONTRADICTORY_CONDITION_GUARD` (C12, ordinal 18, ADR-0004 check 12).

**PASS condition (§5.1 check 12):** no condition guard is contradictory (§4.10).

**Anchor:** the condition's `OccurrenceId`.
**Discriminator:** `NONE`.

**Disjointness:** contradictory **Direction-rule** guards are check 11's `UNREACHABLE_CONTRADICTORY_GUARD`; check 12 owns **conditions only**.

**Prerequisites (§5.1):** scoped to the condition — requires that condition's guard present and analyzable (check 1 for guard presence; check 2 and check 6 for every declaration that condition's guard references). A non-computable condition (unresolved reference, `TYPE_MISMATCH`, malformed referenced declaration) is **suppressed for that condition only** and emits nothing; it does NOT emit check 2's or check 6's categories. Another valid condition is still evaluated independently.

**Implementation (contradictory_rules.rs):** iterates conditions; for each, guard must be `Declared::Present` and `analyze_guard` must return `Some`; `is_contradictory(&domains)` → emit `ContradictoryConditionGuard`/`NONE`. Absent/duplicated condition lineage does not block anchoring (check 15's concern, not a prerequisite).

**Duplicate-identical atoms:** the constraint-domain construction is idempotent for duplicate-identical atoms, so duplicate atoms do NOT change the contradictory/satisfiable result (`duplicate_identical_atoms_do_not_alter_result` passes: contradictory-with-duplicates → 1 defect; satisfiable-with-duplicates → 0 defects). O-4 remains OPEN; `Guard.atoms` untouched; no global canonicalization, positional identity, or traversal identity.

## 9. Shared guard-analysis design

**File:** `src/checks/guard_analysis.rs` (new, 328 lines).

**Public API:**

- `RefDomain` — enum of `Numeric(NumericDomain)` / `Boolean(BooleanDomain)` / `Enum(EnumDomain)`, with `is_empty`, `overlaps`, `contains`, `unconstrained(decl)`.
- `analyze_guard(strategy, guard) -> Option<HashMap<LineageId, RefDomain>>` — builds per-reference constraint domains (§4.10). Returns `None` when the guard is not computable (any prerequisite failure).
- `is_contradictory(domains) -> bool` — any referenced domain empty.
- `guards_overlap(strategy, a, b) -> bool` — per-reference overlap over union of lineages.
- `guard_subsumes(strategy, outer, inner) -> bool` — per-reference containment `D(outer,e) ⊇ D(inner,e)`.

**Exactly-one declaration resolution:** `resolve_declaration` returns the single present declaration whose `Declared::Present(lineage)` matches the atom's `LineageId`; returns `None` for zero (absent → check 1's) or duplicate (ambiguous → check 2's `UNRESOLVED_REFERENCE`) matches.

**Check-6-valid declaration prerequisite:** the referent's `declared_type` must be `DeclaredCategory::One(EvidenceTypeTag)` (Numeric/Boolean/Enum); `Absent`/`Multiple`/`OutOfDomain` → `None` (check 6's domain).

**Type/operator/dimension compatibility (check 2):** for each atom: NUMERIC referent → `Literal::Numeric` with dimension matching `declared_dimension`; BOOLEAN referent → `Literal::Boolean` with operator `Equal`/`NotEqual`; ENUM referent → `Literal::Enum` with operator `Equal`/`NotEqual`. Any mismatch → `None` (check 2's `TYPE_MISMATCH`).

**Existing `semantics::constraint` APIs consumed, not modified:** `NumericDomain::from_constraints`, `BooleanDomain::from_constraints`, `EnumDomain::from_constraints`, `is_empty`, `overlaps`, `contains`, `unconstrained`. No `src/semantics/**` change.

**No other check executed:** the guard analysis does not call `check_1`, `check_2`, or `check_6` — it reconstructs prerequisite state locally from the guard and declarations. No Phase-6 orchestration, not-evaluated record, verdict, or canonical sorting.

**No defect-level deduplication:** no `Vec<DefectCore>` dedup anywhere.

**No positional/traversal identity:** domains built from atoms; duplicate-identical atoms are idempotent; no collection index, traversal order, or source position participates.

## 10. Prerequisite / non-computability treatment

All three checks implement **explicit prerequisite-gating (§5.2 classification C)**, reconstructed locally:

- Check 9: pair suppressed if either guard non-computable or either precedence absent (`continue`).
- Check 11 contradiction: rule suppressed if its guard non-computable.
- Check 11 subsumption: pair suppressed if either guard non-computable or either precedence absent, or if `S` precedence not strictly higher.
- Check 12: condition suppressed if its guard non-computable; another valid condition still evaluated.

No suppression of a computable pair/condition. No emitting of check 2/6 categories.

## 11. Anchors / discriminators

| Check | Defect | Anchor | Discriminator |
|---|---|---|---|
| 9 | `DIRECTION_PRECEDENCE_TIE` | lower `OccurrenceId` | `COUNTERPART` = higher `OccurrenceId` (kind 6) |
| 11 | `UNREACHABLE_CONTRADICTORY_GUARD` | rule's `OccurrenceId` | `NONE` |
| 11 | `UNREACHABLE_SUBSUMED_RULE` | subsumed rule's `OccurrenceId` | `COUNTERPART` = subsuming rule's `OccurrenceId` (kind 6) |
| 12 | `CONTRADICTORY_CONDITION_GUARD` | condition's `OccurrenceId` | `NONE` |

All anchors occurrence-based (Revision 4 §C.1/§C.7). Absent/duplicated lineage does not block anchoring (check 15's concern, not a prerequisite).

## 12. O-4 treatment

O-4 remains **OPEN / BOARD-NOTED / prospective**:

- `Guard.atoms` is still a `Vec` (Representation unchanged — `git diff --stat HEAD -- src/model/guard.rs` empty).
- Check 2 unchanged.
- No global guard canonicalization.
- The constraint-domain construction is idempotent for duplicate-identical atoms, so guard set semantics (§3.6/§4.5) are honored locally without resolving O-4 globally.
- The implementation conforms Checks 9/11/12 without touching the representation.

## 13. Check-2 freeze

- `src/checks/reference_integrity.rs` unchanged from HEAD.
- Check 2 not modified; its parallel divergence (content-identical duplicate atoms → multiple defects) remains frozen until separately authorized.
- `git diff --stat HEAD -- src/checks/reference_integrity.rs` empty.

## 14. Adversarial identity results

| Scenario | Expected | Result |
|---|---|---|
| 9c: overlapping equal-precedence pair | 1 defect, anchor=lower occ, COUNTERPART=higher occ | PASS |
| 9d: reversed storage order | identical identity | PASS |
| 9e: three overlapping equal-precedence rules | 3 pairwise defects, all distinct | PASS |
| 9g: unresolved/type-invalid pair | 0 defects, no panic | PASS |
| 11c: higher-precedence subsumer | 1 defect, anchor=R occ, COUNTERPART=S occ | PASS |
| 11d: equal/lower precedence, higher-but-not-subsuming | 0 defects | PASS |
| 11e: two distinct subsumers | 2 defects, distinct COUNTERPARTs, distinct identities | PASS |
| 11f: anchor stays subsumed occurrence even when subsumer sorts first | anchor=R occ | PASS |
| 11g: absent-lineage rule still subsumed | anchored by occurrence | PASS |
| 11h: non-computable pair suppressed | 0 defects | PASS |
| 12a/b/c: contradictory NUMERIC/BOOLEAN/ENUM condition | 1 defect each, NONE | PASS |
| 12d: satisfiable condition | 0 defects | PASS |
| 12e: duplicate-identical atoms | contradictory→1, satisfiable→0 (unchanged) | PASS |
| 12f: TYPE_MISMATCH suppresses only that condition; other contradictory condition still emitted | 1 defect (the valid one) | PASS |
| 12g: unresolved reference suppresses only that condition | 1 defect (the valid one) | PASS |
| 12h: invalid (Multiple) declaration suppresses only that condition | 1 defect (the valid one) | PASS |
| 12i: absent lineage does not block anchoring | 1 defect, anchored by occurrence | PASS |
| identity: two contradictory conditions distinct | 2 defects, non-equal | PASS |

## 15. Tests added

**Check 9:** 7 tests (`direction_conflict.rs` test module):
- `non_overlapping_equal_precedence_is_not_a_tie`
- `overlapping_different_precedence_is_not_a_tie`
- `overlapping_equal_precedence_is_one_tie`
- `reversed_storage_order_is_identical_identity`
- `three_equal_precedence_overlapping_rules_yield_three_ties`
- `absent_lineage_rules_still_representable`
- `non_computable_pair_is_suppressed_locally`

**Check 11:** 8 tests (`unreachable_rules.rs`):
- `contradictory_direction_guard_emits_contradictory_guard`
- `satisfiable_direction_guard_emits_nothing`
- `higher_precedence_subsumer_emits_subsumed_rule`
- `equal_or_lower_precedence_never_subsumes`
- `multiple_distinct_subsumers_remain_distinct`
- `subsumed_rule_is_always_the_anchor`
- `absent_lineage_rule_still_subsumed`
- `non_computable_pair_is_suppressed`

**Check 12:** 10 tests (`contradictory_rules.rs`):
- `contradictory_numeric_condition_guard`
- `contradictory_boolean_condition_guard`
- `contradictory_enum_condition_guard`
- `satisfiable_condition_guard_emits_nothing`
- `duplicate_identical_atoms_do_not_alter_result`
- `type_mismatch_suppresses_only_that_condition`
- `unresolved_reference_suppresses_only_that_condition`
- `invalid_declaration_suppresses_only_that_condition`
- `absent_lineage_condition_still_anchored`
- `two_contradictory_conditions_are_distinct`

## 16. Final verification counts

- `cargo fmt --check`: clean
- `cargo check`: clean
- `cargo clippy --all-targets -- -D warnings`: 0 diagnostics
- `cargo test`:
  - **347 unit tests, 0 failures**
  - 1 integration test, 0 failures
  - 5 doctests, 0 failures
  - 14 compile-fail doctests, 0 failures
- Total: **347 + 1 + 5 + 14 = 367 test results, 0 failures**

## 17. Prior-check freeze proof

All prior checks byte-identical to HEAD (pre-Slice-G baseline `1b87c8e`):

| Check | Path | Status |
|---|---|---|
| Check 1 | `src/checks/required_property.rs` | unchanged |
| Check 2 | `src/checks/reference_integrity.rs` | unchanged |
| Check 3 | `src/checks/condition_classification.rs` | unchanged |
| Check 4 | `src/checks/configuration_bounds.rs` | unchanged |
| Check 5 | `src/checks/configuration_position.rs` | unchanged |
| Check 6 | `src/checks/evidence_integrity.rs` | unchanged (pre-existing remediation; not a Slice-G change) |
| Check 8 | `src/checks/direction_coverage.rs` | unchanged |
| Check 10 | `src/checks/direction_determinism.rs` | unchanged |
| Check 15 | `src/checks/operative_provenance.rs` | unchanged |
| Check 16 | `src/checks/semantic_model_version.rs` | unchanged |

Verification: `git diff --stat HEAD -- src/checks/` shows ONLY `mod.rs` (registration) modified; all check files unchanged. `git diff --stat HEAD -- src/model/ src/result/ src/numeric/ src/semantics/` shows NO change (the only `src/result` change history is the pre-existing Check-6 remediation in `discriminator.rs`, which is NOT part of this Slice-G commit — `1a1a978` is a prior commit, `a66982f` does not touch `discriminator.rs`).

## 18. Forbidden-path proof

Slice-G commit `a66982f` modifies **only**:

- `src/checks/**` (4 new check modules + `guard_analysis.rs` helper + `mod.rs` registration)
- `docs/governance/identifier-ledger.md` (PBA-0123/ER-0102 allocation; ER-0101 repaired)
- `docs/engineering/slice-1-implementation-plan.md` (status)

**No change to:**
- `src/model/**`
- `src/result/**`
- `src/numeric/**`
- `src/semantics/**`
- any prior check implementation
- `Cargo.toml` / toolchain
- the accepted contract (`docs/engineering/first-runnable-slice-contract-r04.md`) or ADR/RFC artifacts

The helper consumes (does not modify) `src/semantics::constraint` public APIs.

## 19. Phase-6 boundary

No Phase-6 machinery introduced:

- No scheduler / orchestrator
- No prerequisite executor
- No suppression framework
- No global `NotEvaluatedRecord` orchestration
- No verdict computation
- No qualification
- No canonical global defect sorting
- No cross-check execution engine

Within-check traversal only (unordered pairs in Check 9; per-rule/per-pair in Check 11; per-condition in Check 12). Prerequisites reconstructed locally, not via an external gating service.

## 20. Post-write allocator

**§11 high-water:**

| Namespace | High-water | Next allocatable |
|---|---|---|
| PBA | **PBA-0123** | **PBA-0124** |
| PBR | PBR-0016 | **PBR-0017** |
| ARR | **ARR-0040** | **ARR-0041** |
| ER | **ER-0102** | **ER-0103** |
| PBD | PBD-0006 | **PBD-0007** |

**Used this cycle:** PBA-0123 → ER-0102.
**Reserved (NOT allocated):** ARR-0041 — the next read-only review identifier.
**Future review PBA:** not allocated.

## 21. Exact next permitted governance action

**Separate independent read-only review of Slice G (Checks 9, 11, 12), under ARR-0041.**

This is the expected next action after this push. It must be:

- a new PCAOB-style review (not implementation);
- authorized under the next unallocated PBA (PBA-0124), allocated only at that review's start;
- ARR-0041 allocated as the review identifier;
- a full independent reproduction + semantic reconstruction (not resting on these green tests);
- a STOP-worthy review: if the semantic reconstruction disagrees with the implementation, the review must STOP and report.

**Do NOT perform or preallocate that review in this cycle.**

---

**End of Slice-G implementation report.** Slice G is implemented, recorded, committed, pushed, and verified. Slice F is closed. The next permitted action is the independent read-only review of Slice G under ARR-0041.