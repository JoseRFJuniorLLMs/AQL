//! AST (Abstract Syntax Tree) for AQL v2.0
//! Represents the parsed structure of an AQL program.

use crate::types::*;
use serde::{Deserialize, Serialize};

/// A complete AQL program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// A statement can be simple, chain, parallel, atomic, or reactive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Simple(SimpleStatement),
    Chain(ChainStatement),
    Parallel(ParallelStatement),
    Atomic(AtomicBlock),
    Watch(WatchStatement),
    Explain(ExplainStatement),
}

/// A simple verb + subject + qualifiers statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleStatement {
    pub verb: Verb,
    pub subject: Subject,
    pub qualifiers: Vec<Qualifier>,
    pub condition: Option<WhenClause>,
    pub else_stmt: Option<Box<SimpleStatement>>,
}

/// THEN chain: step1 THEN step2 THEN ...
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainStatement {
    pub steps: Vec<SimpleStatement>,
}

/// Parallel: step1 AND step2 AND ... [THEN join]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelStatement {
    pub branches: Vec<SimpleStatement>,
    pub join_step: Option<SimpleStatement>,
}

/// ATOMIC { ... } transactional block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomicBlock {
    pub statements: Vec<Statement>,
}

/// WATCH/SUBSCRIBE reactive statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchStatement {
    pub subject: Subject,
    pub trigger: WatchTrigger,
    pub reaction: Box<Statement>,
}

/// EXPLAIN wraps another statement for provenance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainStatement {
    pub inner: Box<SimpleStatement>,
}

/// WHEN condition clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhenClause {
    pub field: String,
    pub op: CompOp,
    pub value: ConditionValue,
}

/// Condition value (right side of WHEN).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionValue {
    Float(f64),
    Int(i64),
    Str(String),
}

// ── The 13 Cognitive Verbs ────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Verb {
    // v1.0 core
    Recall,
    Resonate,
    Reflect,
    Trace,
    Imprint,
    Associate,
    Distill,
    Fade,
    // v2.0 geometric
    Descend,
    Ascend,
    Orbit,
    // v2.0 altered states
    Dream,
    Imagine,
}

impl std::fmt::Display for Verb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Recall => "RECALL",
            Self::Resonate => "RESONATE",
            Self::Reflect => "REFLECT",
            Self::Trace => "TRACE",
            Self::Imprint => "IMPRINT",
            Self::Associate => "ASSOCIATE",
            Self::Distill => "DISTILL",
            Self::Fade => "FADE",
            Self::Descend => "DESCEND",
            Self::Ascend => "ASCEND",
            Self::Orbit => "ORBIT",
            Self::Dream => "DREAM",
            Self::Imagine => "IMAGINE",
        };
        write!(f, "{s}")
    }
}

/// Side effects implicitly triggered by each verb.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SideEffect {
    BoostAccessedNodes,
    CreateTemporalEdge,
    RecordAccessPattern,
    RecordResonancePattern,
    BoostPathNodes,
    CreatePatternNode,
    LinkSourceEpisodes,
    AssociateToSessionContext,
    BoostLinkedNodes,
    RecordFadeEvent,
}

impl Verb {
    /// Returns the implicit side effects for this verb.
    pub fn side_effects(&self) -> Vec<SideEffect> {
        match self {
            Self::Recall => vec![
                SideEffect::BoostAccessedNodes,
                SideEffect::CreateTemporalEdge,
                SideEffect::RecordAccessPattern,
            ],
            Self::Resonate => vec![
                SideEffect::BoostAccessedNodes,
                SideEffect::RecordResonancePattern,
            ],
            Self::Reflect => vec![SideEffect::RecordAccessPattern],
            Self::Trace => vec![
                SideEffect::BoostPathNodes,
                SideEffect::CreateTemporalEdge,
            ],
            Self::Imprint => vec![
                SideEffect::AssociateToSessionContext,
                SideEffect::BoostLinkedNodes,
            ],
            Self::Associate => vec![
                SideEffect::CreateTemporalEdge,
                SideEffect::BoostLinkedNodes,
            ],
            Self::Distill => vec![
                SideEffect::CreatePatternNode,
                SideEffect::LinkSourceEpisodes,
            ],
            Self::Fade => vec![SideEffect::RecordFadeEvent],
            Self::Descend | Self::Ascend | Self::Orbit => vec![
                SideEffect::BoostAccessedNodes,
                SideEffect::RecordAccessPattern,
            ],
            Self::Dream => vec![
                SideEffect::CreatePatternNode,
                SideEffect::BoostAccessedNodes,
            ],
            Self::Imagine => vec![SideEffect::RecordAccessPattern],
        }
    }
}

// ── Subjects ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Subject {
    Text(String),
    TypeFilter(EpistemicType),
    TypeWithContent {
        etype: EpistemicType,
        content: String,
    },
    SelfRef,
    AgentRef(String),
    ResultsRef {
        index: Option<usize>,
    },
    LastDream,
    DelegateResult,
    TraceRange {
        from: String,
        to: String,
    },
    About(Box<Subject>),
}

// ── Qualifiers ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Qualifier {
    // v1.0
    Confidence(f32),
    Recency(RecencyDegree),
    Depth(u8),
    Within(ContextScope),
    As(EpistemicType),
    Linking(LinkTarget),
    Novelty(NoveltyDegree),
    Limit(u32),

    // v2.0 geometric
    Magnitude(f32, f32),
    Curvature(CurvatureDegree),
    Radius(f32),

    // v2.0 affective
    Valence(ValenceSpec),
    Arousal(ArousalSpec),
    Mood(MoodState),

    // v2.0 epistemic
    Evidence(u32),

    // v2.0 multi-agent
    WithAgent(String),
    ToAgent(String),
    Policy(ConflictPolicy),
}

/// Link target for ASSOCIATE/LINKING.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkTarget {
    Text(String),
    ResultsRef { index: Option<usize> },
    SelfRef,
}
