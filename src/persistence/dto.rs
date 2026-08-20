//! Crate-private serialization DTO for the Dogfood envelope — ADR-0009 §4.
//!
//! This module is the **only** place `serde` is used in `ordina`, and it is used
//! solely for a private data-transfer mirror of the canonical model. Opaque
//! handles are transported as their raw `u64` (read via the model's `pub(crate)`
//! `identity_handle()` accessors, reconstructed via the public
//! `::new(OpaqueValue::new(h))` constructors), so **no** public raw-handle
//! accessor exists and the opacity boundary to `ordina-desktop` is intact.
//!
//! The JSON produced here is a **transport/restore** form, explicitly not a
//! revision-identity canonicalization (ADR-0008 remains authoritative; identity
//! is recomputed on load — ADR-0009 §9). `ExactNumber` is transported as an
//! explicit decimal numerator/denominator string pair — never a float
//! (ADR-0005 §7.2; the numeric crates' `serde` feature stays disabled).

use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

use crate::model::condition::{Condition, ConditionClassification};
use crate::model::config::ConfigurationParameter;
use crate::model::declared::{Declared, DeclaredCategory};
use crate::model::derivation::{CandidateDerivation, CandidateRole, DerivationExpression};
use crate::model::direction::{Direction, DirectionRule};
use crate::model::evidence::{EvidenceDeclaration, EvidenceTypeTag};
use crate::model::guard::{Atom, ComparisonOperator, Guard};
use crate::model::literal::Literal;
use crate::model::opaque::{
    DimensionSymbol, EnumValue, LineageId, OccurrenceId, OpaqueValue, PrecedenceOrdinal,
    VersionToken,
};
use crate::model::strategy::Strategy;
use crate::numeric::ExactNumber;

use super::EnvelopeLoadError;

// --------------------------------------------------------------------------
// Generic declaration-state mirrors.
// --------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub(super) enum DeclaredDto<T> {
    #[serde(rename = "absent")]
    Absent,
    #[serde(rename = "present")]
    Present(T),
}

impl<T> DeclaredDto<T> {
    fn from_model<M>(d: &Declared<M>, f: impl Fn(&M) -> T) -> Self {
        match d {
            Declared::Absent => DeclaredDto::Absent,
            Declared::Present(v) => DeclaredDto::Present(f(v)),
        }
    }

    fn into_model<M>(
        self,
        f: impl FnOnce(T) -> Result<M, EnvelopeLoadError>,
    ) -> Result<Declared<M>, EnvelopeLoadError> {
        match self {
            DeclaredDto::Absent => Ok(Declared::Absent),
            DeclaredDto::Present(v) => Ok(Declared::Present(f(v)?)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) enum DeclaredCategoryDto<T> {
    #[serde(rename = "absent")]
    Absent,
    #[serde(rename = "one")]
    One(T),
    #[serde(rename = "multiple")]
    Multiple { first: T, second: T, rest: Vec<T> },
    #[serde(rename = "out_of_domain")]
    OutOfDomain,
}

impl<T> DeclaredCategoryDto<T> {
    fn from_model<M>(d: &DeclaredCategory<M>, f: impl Fn(&M) -> T) -> Self {
        match d {
            DeclaredCategory::Absent => DeclaredCategoryDto::Absent,
            DeclaredCategory::One(v) => DeclaredCategoryDto::One(f(v)),
            DeclaredCategory::Multiple {
                first,
                second,
                rest,
            } => DeclaredCategoryDto::Multiple {
                first: f(first),
                second: f(second),
                rest: rest.iter().map(&f).collect(),
            },
            DeclaredCategory::OutOfDomain => DeclaredCategoryDto::OutOfDomain,
        }
    }

    fn into_model<M>(
        self,
        f: impl Fn(T) -> Result<M, EnvelopeLoadError>,
    ) -> Result<DeclaredCategory<M>, EnvelopeLoadError> {
        Ok(match self {
            DeclaredCategoryDto::Absent => DeclaredCategory::Absent,
            DeclaredCategoryDto::One(v) => DeclaredCategory::One(f(v)?),
            DeclaredCategoryDto::Multiple {
                first,
                second,
                rest,
            } => {
                let first = f(first)?;
                let second = f(second)?;
                let rest = rest.into_iter().map(&f).collect::<Result<_, _>>()?;
                DeclaredCategory::Multiple {
                    first,
                    second,
                    rest,
                }
            }
            DeclaredCategoryDto::OutOfDomain => DeclaredCategory::OutOfDomain,
        })
    }
}

// --------------------------------------------------------------------------
// Opaque-handle transport (raw u64 — read/reconstruct crate-internally only).
// --------------------------------------------------------------------------

fn lineage_to(id: LineageId) -> u64 {
    id.identity_handle()
}
fn lineage_from(h: u64) -> LineageId {
    LineageId::new(OpaqueValue::new(h))
}
fn occurrence_to(id: OccurrenceId) -> u64 {
    id.identity_handle()
}
fn occurrence_from(h: u64) -> OccurrenceId {
    OccurrenceId::new(OpaqueValue::new(h))
}
fn dimension_from(h: u64) -> DimensionSymbol {
    DimensionSymbol::new(OpaqueValue::new(h))
}
fn enum_from(h: u64) -> EnumValue {
    EnumValue::new(OpaqueValue::new(h))
}
fn version_from(h: u64) -> VersionToken {
    VersionToken::new(OpaqueValue::new(h))
}
fn precedence_from(h: u64) -> PrecedenceOrdinal {
    PrecedenceOrdinal::new(OpaqueValue::new(h))
}

// --------------------------------------------------------------------------
// Closed-category enum mirrors (explicit stable tags, ADR-0009 §4).
// --------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub(super) enum ComparisonOperatorDto {
    #[serde(rename = "less")]
    Less,
    #[serde(rename = "less_or_equal")]
    LessOrEqual,
    #[serde(rename = "greater")]
    Greater,
    #[serde(rename = "greater_or_equal")]
    GreaterOrEqual,
    #[serde(rename = "equal")]
    Equal,
    #[serde(rename = "not_equal")]
    NotEqual,
}

impl ComparisonOperatorDto {
    fn from_model(op: ComparisonOperator) -> Self {
        match op {
            ComparisonOperator::Less => Self::Less,
            ComparisonOperator::LessOrEqual => Self::LessOrEqual,
            ComparisonOperator::Greater => Self::Greater,
            ComparisonOperator::GreaterOrEqual => Self::GreaterOrEqual,
            ComparisonOperator::Equal => Self::Equal,
            ComparisonOperator::NotEqual => Self::NotEqual,
        }
    }
    fn into_model(self) -> ComparisonOperator {
        match self {
            Self::Less => ComparisonOperator::Less,
            Self::LessOrEqual => ComparisonOperator::LessOrEqual,
            Self::Greater => ComparisonOperator::Greater,
            Self::GreaterOrEqual => ComparisonOperator::GreaterOrEqual,
            Self::Equal => ComparisonOperator::Equal,
            Self::NotEqual => ComparisonOperator::NotEqual,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) enum ConditionClassificationDto {
    #[serde(rename = "blocking")]
    Blocking,
    #[serde(rename = "temporary")]
    Temporary,
}

impl ConditionClassificationDto {
    fn from_model(c: ConditionClassification) -> Self {
        match c {
            ConditionClassification::Blocking => Self::Blocking,
            ConditionClassification::Temporary => Self::Temporary,
        }
    }
    fn into_model(self) -> ConditionClassification {
        match self {
            Self::Blocking => ConditionClassification::Blocking,
            Self::Temporary => ConditionClassification::Temporary,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) enum DirectionDto {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

impl DirectionDto {
    fn from_model(d: Direction) -> Self {
        match d {
            Direction::Buy => Self::Buy,
            Direction::Sell => Self::Sell,
        }
    }
    fn into_model(self) -> Direction {
        match self {
            Self::Buy => Direction::Buy,
            Self::Sell => Direction::Sell,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) enum EvidenceTypeTagDto {
    #[serde(rename = "numeric")]
    Numeric,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "enum")]
    Enum,
}

impl EvidenceTypeTagDto {
    fn from_model(t: EvidenceTypeTag) -> Self {
        match t {
            EvidenceTypeTag::Numeric => Self::Numeric,
            EvidenceTypeTag::Boolean => Self::Boolean,
            EvidenceTypeTag::Enum => Self::Enum,
        }
    }
    fn into_model(self) -> EvidenceTypeTag {
        match self {
            Self::Numeric => EvidenceTypeTag::Numeric,
            Self::Boolean => EvidenceTypeTag::Boolean,
            Self::Enum => EvidenceTypeTag::Enum,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) enum CandidateRoleDto {
    #[serde(rename = "entry")]
    Entry,
    #[serde(rename = "stop_loss")]
    StopLoss,
    #[serde(rename = "take_profit")]
    TakeProfit,
}

impl CandidateRoleDto {
    fn from_model(r: CandidateRole) -> Self {
        match r {
            CandidateRole::Entry => Self::Entry,
            CandidateRole::StopLoss => Self::StopLoss,
            CandidateRole::TakeProfit => Self::TakeProfit,
        }
    }
    fn into_model(self) -> CandidateRole {
        match self {
            Self::Entry => CandidateRole::Entry,
            Self::StopLoss => CandidateRole::StopLoss,
            Self::TakeProfit => CandidateRole::TakeProfit,
        }
    }
}

// --------------------------------------------------------------------------
// ExactNumber — decimal num/den strings, fail-closed reconstruction (§4.5).
// --------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub(super) struct ExactNumberDto {
    num: String,
    den: String,
}

impl ExactNumberDto {
    fn from_model(n: &ExactNumber) -> Self {
        let r = n.as_ratio();
        Self {
            num: r.numer().to_string(),
            den: r.denom().to_string(),
        }
    }
    fn into_model(self) -> Result<ExactNumber, EnvelopeLoadError> {
        let num: BigInt = self
            .num
            .parse()
            .map_err(|_| EnvelopeLoadError::RationalOutOfDomain)?;
        let den: BigInt = self
            .den
            .parse()
            .map_err(|_| EnvelopeLoadError::RationalOutOfDomain)?;
        ExactNumber::from_ratio_exact(num, den).map_err(|_| EnvelopeLoadError::RationalOutOfDomain)
    }
}

// --------------------------------------------------------------------------
// Literal / DerivationExpression.
// --------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub(super) enum LiteralDto {
    #[serde(rename = "numeric")]
    Numeric {
        value: ExactNumberDto,
        dimension: u64,
    },
    #[serde(rename = "boolean")]
    Boolean(bool),
    #[serde(rename = "enum")]
    Enum(u64),
}

impl LiteralDto {
    fn from_model(l: &Literal) -> Self {
        match l {
            Literal::Numeric { value, dimension } => Self::Numeric {
                value: ExactNumberDto::from_model(value),
                dimension: dimension.identity_handle(),
            },
            Literal::Boolean(b) => Self::Boolean(*b),
            Literal::Enum(e) => Self::Enum(e.identity_handle()),
        }
    }
    fn into_model(self) -> Result<Literal, EnvelopeLoadError> {
        Ok(match self {
            Self::Numeric { value, dimension } => Literal::Numeric {
                value: value.into_model()?,
                dimension: dimension_from(dimension),
            },
            Self::Boolean(b) => Literal::Boolean(b),
            Self::Enum(e) => Literal::Enum(enum_from(e)),
        })
    }
}

#[derive(Serialize, Deserialize)]
pub(super) enum DerivationExpressionDto {
    #[serde(rename = "literal")]
    Literal(LiteralDto),
    #[serde(rename = "evidence_ref")]
    EvidenceReference(u64),
    #[serde(rename = "config_ref")]
    ConfigurationReference(u64),
    #[serde(rename = "add")]
    Add {
        left: Box<DerivationExpressionDto>,
        right: Box<DerivationExpressionDto>,
    },
    #[serde(rename = "subtract")]
    Subtract {
        left: Box<DerivationExpressionDto>,
        right: Box<DerivationExpressionDto>,
    },
    #[serde(rename = "multiply")]
    Multiply {
        operand: Box<DerivationExpressionDto>,
        scalar: LiteralDto,
    },
}

impl DerivationExpressionDto {
    fn from_model(e: &DerivationExpression) -> Self {
        match e {
            DerivationExpression::Literal(l) => Self::Literal(LiteralDto::from_model(l)),
            DerivationExpression::EvidenceReference(id) => {
                Self::EvidenceReference(id.identity_handle())
            }
            DerivationExpression::ConfigurationReference(id) => {
                Self::ConfigurationReference(id.identity_handle())
            }
            DerivationExpression::Add { left, right } => Self::Add {
                left: Box::new(Self::from_model(left)),
                right: Box::new(Self::from_model(right)),
            },
            DerivationExpression::Subtract { left, right } => Self::Subtract {
                left: Box::new(Self::from_model(left)),
                right: Box::new(Self::from_model(right)),
            },
            DerivationExpression::Multiply { operand, scalar } => Self::Multiply {
                operand: Box::new(Self::from_model(operand)),
                scalar: LiteralDto::from_model(scalar),
            },
        }
    }
    fn into_model(self) -> Result<DerivationExpression, EnvelopeLoadError> {
        Ok(match self {
            Self::Literal(l) => DerivationExpression::Literal(l.into_model()?),
            Self::EvidenceReference(id) => {
                DerivationExpression::EvidenceReference(lineage_from(id))
            }
            Self::ConfigurationReference(id) => {
                DerivationExpression::ConfigurationReference(lineage_from(id))
            }
            Self::Add { left, right } => DerivationExpression::Add {
                left: Box::new(left.into_model()?),
                right: Box::new(right.into_model()?),
            },
            Self::Subtract { left, right } => DerivationExpression::Subtract {
                left: Box::new(left.into_model()?),
                right: Box::new(right.into_model()?),
            },
            Self::Multiply { operand, scalar } => DerivationExpression::Multiply {
                operand: Box::new(operand.into_model()?),
                scalar: scalar.into_model()?,
            },
        })
    }
}

// --------------------------------------------------------------------------
// Guard / Atom.
// --------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub(super) struct AtomDto {
    evidence: u64,
    operator: ComparisonOperatorDto,
    literal: LiteralDto,
}

impl AtomDto {
    fn from_model(a: &Atom) -> Self {
        Self {
            evidence: a.evidence.identity_handle(),
            operator: ComparisonOperatorDto::from_model(a.operator),
            literal: LiteralDto::from_model(&a.literal),
        }
    }
    fn into_model(self) -> Result<Atom, EnvelopeLoadError> {
        Ok(Atom::new(
            lineage_from(self.evidence),
            self.operator.into_model(),
            self.literal.into_model()?,
        ))
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct GuardDto {
    atoms: Vec<AtomDto>,
}

impl GuardDto {
    fn from_model(g: &Guard) -> Self {
        Self {
            atoms: g.atoms.iter().map(AtomDto::from_model).collect(),
        }
    }
    fn into_model(self) -> Result<Guard, EnvelopeLoadError> {
        let atoms = self
            .atoms
            .into_iter()
            .map(AtomDto::into_model)
            .collect::<Result<_, _>>()?;
        Ok(Guard::from_atoms(atoms))
    }
}

// --------------------------------------------------------------------------
// Constructs.
// --------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub(super) struct EvidenceDeclarationDto {
    occurrence: u64,
    lineage: DeclaredDto<u64>,
    declared_type: DeclaredCategoryDto<EvidenceTypeTagDto>,
    declared_dimension: DeclaredDto<u64>,
    declared_values: DeclaredDto<Vec<u64>>,
}

impl EvidenceDeclarationDto {
    fn from_model(e: &EvidenceDeclaration) -> Self {
        Self {
            occurrence: occurrence_to(e.occurrence),
            lineage: DeclaredDto::from_model(&e.lineage, |l| lineage_to(*l)),
            declared_type: DeclaredCategoryDto::from_model(&e.declared_type, |t| {
                EvidenceTypeTagDto::from_model(*t)
            }),
            declared_dimension: DeclaredDto::from_model(&e.declared_dimension, |d| {
                d.identity_handle()
            }),
            declared_values: DeclaredDto::from_model(&e.declared_values, |vs| {
                vs.iter().map(|v| v.identity_handle()).collect()
            }),
        }
    }
    fn into_model(self) -> Result<EvidenceDeclaration, EnvelopeLoadError> {
        Ok(EvidenceDeclaration {
            occurrence: occurrence_from(self.occurrence),
            lineage: self.lineage.into_model(|h| Ok(lineage_from(h)))?,
            declared_type: self.declared_type.into_model(|t| Ok(t.into_model()))?,
            declared_dimension: self
                .declared_dimension
                .into_model(|h| Ok(dimension_from(h)))?,
            declared_values: self
                .declared_values
                .into_model(|vs| Ok(vs.into_iter().map(enum_from).collect()))?,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct ConfigurationParameterDto {
    occurrence: u64,
    lineage: DeclaredDto<u64>,
    dimension: DeclaredDto<u64>,
    lower_bound: DeclaredDto<LiteralDto>,
    upper_bound: DeclaredDto<LiteralDto>,
    adjustable: DeclaredDto<bool>,
}

impl ConfigurationParameterDto {
    fn from_model(c: &ConfigurationParameter) -> Self {
        Self {
            occurrence: occurrence_to(c.occurrence),
            lineage: DeclaredDto::from_model(&c.lineage, |l| lineage_to(*l)),
            dimension: DeclaredDto::from_model(&c.dimension, |d| d.identity_handle()),
            lower_bound: DeclaredDto::from_model(&c.lower_bound, LiteralDto::from_model),
            upper_bound: DeclaredDto::from_model(&c.upper_bound, LiteralDto::from_model),
            adjustable: DeclaredDto::from_model(&c.adjustable, |b| *b),
        }
    }
    fn into_model(self) -> Result<ConfigurationParameter, EnvelopeLoadError> {
        Ok(ConfigurationParameter {
            occurrence: occurrence_from(self.occurrence),
            lineage: self.lineage.into_model(|h| Ok(lineage_from(h)))?,
            dimension: self.dimension.into_model(|h| Ok(dimension_from(h)))?,
            lower_bound: self.lower_bound.into_model(LiteralDto::into_model)?,
            upper_bound: self.upper_bound.into_model(LiteralDto::into_model)?,
            adjustable: self.adjustable.into_model(Ok)?,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct ConditionDto {
    occurrence: u64,
    lineage: DeclaredDto<u64>,
    classification: DeclaredCategoryDto<ConditionClassificationDto>,
    guard: DeclaredDto<GuardDto>,
}

impl ConditionDto {
    fn from_model(c: &Condition) -> Self {
        Self {
            occurrence: occurrence_to(c.occurrence),
            lineage: DeclaredDto::from_model(&c.lineage, |l| lineage_to(*l)),
            classification: DeclaredCategoryDto::from_model(&c.classification, |k| {
                ConditionClassificationDto::from_model(*k)
            }),
            guard: DeclaredDto::from_model(&c.guard, GuardDto::from_model),
        }
    }
    fn into_model(self) -> Result<Condition, EnvelopeLoadError> {
        Ok(Condition {
            occurrence: occurrence_from(self.occurrence),
            lineage: self.lineage.into_model(|h| Ok(lineage_from(h)))?,
            classification: self.classification.into_model(|k| Ok(k.into_model()))?,
            guard: self.guard.into_model(GuardDto::into_model)?,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct DirectionRuleDto {
    occurrence: u64,
    lineage: DeclaredDto<u64>,
    precedence: DeclaredDto<u64>,
    guard: DeclaredDto<GuardDto>,
    outcome: DeclaredCategoryDto<DirectionDto>,
}

impl DirectionRuleDto {
    fn from_model(d: &DirectionRule) -> Self {
        Self {
            occurrence: occurrence_to(d.occurrence),
            lineage: DeclaredDto::from_model(&d.lineage, |l| lineage_to(*l)),
            precedence: DeclaredDto::from_model(&d.precedence, |p| p.identity_handle()),
            guard: DeclaredDto::from_model(&d.guard, GuardDto::from_model),
            outcome: DeclaredCategoryDto::from_model(&d.outcome, |o| DirectionDto::from_model(*o)),
        }
    }
    fn into_model(self) -> Result<DirectionRule, EnvelopeLoadError> {
        Ok(DirectionRule {
            occurrence: occurrence_from(self.occurrence),
            lineage: self.lineage.into_model(|h| Ok(lineage_from(h)))?,
            precedence: self.precedence.into_model(|h| Ok(precedence_from(h)))?,
            guard: self.guard.into_model(GuardDto::into_model)?,
            outcome: self.outcome.into_model(|o| Ok(o.into_model()))?,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct CandidateDerivationDto {
    occurrence: u64,
    lineage: DeclaredDto<u64>,
    role: DeclaredDto<CandidateRoleDto>,
    expression: DeclaredDto<DerivationExpressionDto>,
}

impl CandidateDerivationDto {
    fn from_model(c: &CandidateDerivation) -> Self {
        Self {
            occurrence: occurrence_to(c.occurrence),
            lineage: DeclaredDto::from_model(&c.lineage, |l| lineage_to(*l)),
            role: DeclaredDto::from_model(&c.role, |r| CandidateRoleDto::from_model(*r)),
            expression: DeclaredDto::from_model(&c.expression, DerivationExpressionDto::from_model),
        }
    }
    fn into_model(self) -> Result<CandidateDerivation, EnvelopeLoadError> {
        Ok(CandidateDerivation {
            occurrence: occurrence_from(self.occurrence),
            lineage: self.lineage.into_model(|h| Ok(lineage_from(h)))?,
            role: self.role.into_model(|r| Ok(r.into_model()))?,
            expression: self
                .expression
                .into_model(DerivationExpressionDto::into_model)?,
        })
    }
}

// --------------------------------------------------------------------------
// Strategy.
// --------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub(super) struct StrategyDto {
    occurrence: u64,
    lineage: DeclaredDto<u64>,
    semantic_model_version: DeclaredDto<u64>,
    evidence_declarations: DeclaredDto<Vec<EvidenceDeclarationDto>>,
    configuration_parameters: DeclaredDto<Vec<ConfigurationParameterDto>>,
    conditions: DeclaredDto<Vec<ConditionDto>>,
    direction_rules: DeclaredDto<Vec<DirectionRuleDto>>,
    candidate_derivations: DeclaredDto<Vec<CandidateDerivationDto>>,
}

impl StrategyDto {
    pub(super) fn from_model(s: &Strategy) -> Self {
        Self {
            occurrence: occurrence_to(s.occurrence),
            lineage: DeclaredDto::from_model(&s.lineage, |l| lineage_to(*l)),
            semantic_model_version: DeclaredDto::from_model(&s.semantic_model_version, |v| {
                v.identity_handle()
            }),
            evidence_declarations: DeclaredDto::from_model(&s.evidence_declarations, |vs| {
                vs.iter().map(EvidenceDeclarationDto::from_model).collect()
            }),
            configuration_parameters: DeclaredDto::from_model(&s.configuration_parameters, |vs| {
                vs.iter()
                    .map(ConfigurationParameterDto::from_model)
                    .collect()
            }),
            conditions: DeclaredDto::from_model(&s.conditions, |vs| {
                vs.iter().map(ConditionDto::from_model).collect()
            }),
            direction_rules: DeclaredDto::from_model(&s.direction_rules, |vs| {
                vs.iter().map(DirectionRuleDto::from_model).collect()
            }),
            candidate_derivations: DeclaredDto::from_model(&s.candidate_derivations, |vs| {
                vs.iter().map(CandidateDerivationDto::from_model).collect()
            }),
        }
    }

    pub(super) fn into_model(self) -> Result<Strategy, EnvelopeLoadError> {
        Ok(Strategy {
            occurrence: occurrence_from(self.occurrence),
            lineage: self.lineage.into_model(|h| Ok(lineage_from(h)))?,
            semantic_model_version: self
                .semantic_model_version
                .into_model(|h| Ok(version_from(h)))?,
            evidence_declarations: self.evidence_declarations.into_model(|vs| {
                vs.into_iter()
                    .map(EvidenceDeclarationDto::into_model)
                    .collect()
            })?,
            configuration_parameters: self.configuration_parameters.into_model(|vs| {
                vs.into_iter()
                    .map(ConfigurationParameterDto::into_model)
                    .collect()
            })?,
            conditions: self
                .conditions
                .into_model(|vs| vs.into_iter().map(ConditionDto::into_model).collect())?,
            direction_rules: self
                .direction_rules
                .into_model(|vs| vs.into_iter().map(DirectionRuleDto::into_model).collect())?,
            candidate_derivations: self.candidate_derivations.into_model(|vs| {
                vs.into_iter()
                    .map(CandidateDerivationDto::into_model)
                    .collect()
            })?,
        })
    }
}

// --------------------------------------------------------------------------
// Envelope DTO.
// --------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub(super) enum MintHighWaterDto {
    #[serde(rename = "next")]
    Next(u64),
    #[serde(rename = "exhausted")]
    Exhausted,
}

#[derive(Serialize, Deserialize)]
pub(super) struct EnvelopeDto {
    pub(super) format_version: u32,
    pub(super) binding: u64,
    pub(super) supported: u64,
    pub(super) strategy: Option<StrategyDto>,
    pub(super) mint_high_water: MintHighWaterDto,
}
