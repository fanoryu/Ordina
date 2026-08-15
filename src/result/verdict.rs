//! The Well-Formedness verdict — Slice Contract §6.1.

/// The binary Well-Formedness verdict (§6.1).
///
/// Two states, **no gradations**. `Unusable` is a *checker* verdict about a
/// Strategy's static Well-Formedness; §6.1 forbids ever presenting it as a
/// runtime trading status (`READY` / `WAIT` / `DO NOT TRADE`). A closed
/// authoritative category, matched exhaustively with no catch-all arm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// The defect set is empty (§6.2).
    WellFormed,
    /// At least one defect was found (§6.2).
    Unusable,
}
