# RFC-NNNN: <Title>

- **Status:** Draft | In Review | Accepted | Rejected | Withdrawn | Superseded by RFC-NNNN
- **Author:**
- **Created:**
- **Decision date:**

## Summary

One paragraph. What is being proposed, in plain language.

## Motivation

What problem does this solve? Who has it? What is the evidence it exists?
"It would be nice" is not motivation.

## Constitution Check

Answer each. An RFC that cannot answer these is not ready for review.

**Boundary test (Constitution §2.1):**

- **Origination** — does the trader select the instrument, with every derived
  value a deterministic consequence of a pre-committed Strategy and Risk Profile,
  and Ordina never inventing an instrument, choosing direction by opinion, or
  producing values outside explicit strategy logic? 
- **Determinism** — is the status produced by rules, with AI confined to
  explanation, and no averaging/voting/score overriding the hierarchy? 
- **Provenance** — does every status support a "Why?" path, and does every
  calculated parameter retain rule provenance? 
- **Gating** — do execution controls remain locked unless READY, with Final
  Construction gated on READY and Preliminary Construction non-executable? 
- **Restraint** — does this avoid implying or guaranteeing an outcome? 

**Further:**

- Which principles (1–12) does this serve?
- Which constraints (C1–C10) does it touch?
- Which of the three validation dimensions (Strategy, Trader Readiness, Risk) does
  it touch?
- **Does this change any status semantics or the resolution model
  (`validation-model.md` §4), or any lifecycle stage
  (`trading-decision-model.md`)?** If yes, say so explicitly and expect
  heightened scrutiny.
- Does it comply with `ux-constitution.md`, in particular §1, §5, and §5A–§5F?

If any answer is "no" or "unclear", stop and state the conflict explicitly.
Proposing around a conflict is not permitted; propose an amendment instead.

## Detailed Design

The actual proposal. Include the data it touches and its failure modes.

## Alternatives Considered

At least two, each with why it was rejected. An RFC with no alternatives has not
been thought about.

## What This Makes Harder

Every design closes doors. Name the ones this closes.

## Risks

New entries for `docs/product/risk-register.md`, or updates to existing ones.

## Open Questions

What is still unresolved, and does it block acceptance? Reference
`docs/product/open-questions.md` where applicable.
