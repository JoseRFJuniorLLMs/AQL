//! Core types for AQL — Agent Query Language v2.0
//! Epistemic types, recency, novelty, scope, curvature, affect.

use serde::{Deserialize, Serialize};

/// The five epistemic knowledge types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EpistemicType {
    Belief,
    Experience,
    Pattern,
    Signal,
    Intention,
}

impl EpistemicType {
    /// Initial energy by type.
    pub fn initial_energy(&self) -> f32 {
        match self {
            Self::Belief => 0.6,
            Self::Experience => 0.5,
            Self::Pattern => 0.8,
            Self::Signal => 0.3,
            Self::Intention => 0.7,
        }
    }

    /// Energy boost on access.
    pub fn access_energy_boost(&self) -> f32 {
        match self {
            Self::Belief => 0.05,
            Self::Experience => 0.03,
            Self::Pattern => 0.02,
            Self::Signal => 0.10,
            Self::Intention => 0.04,
        }
    }

    /// Natural decay rate.
    pub fn decay_rate(&self) -> f64 {
        match self {
            Self::Belief => 0.001,
            Self::Experience => 0.005,
            Self::Pattern => 0.0005,
            Self::Signal => 0.05,
            Self::Intention => 0.01,
        }
    }

    /// Map to NietzscheDB NodeType string.
    pub fn to_nietzsche_node_type(&self) -> &str {
        match self {
            Self::Belief => "Semantic",
            Self::Experience => "Episodic",
            Self::Pattern => "Semantic",
            Self::Signal => "Semantic",
            Self::Intention => "Concept",
        }
    }

    /// Default valence (emotional positivity) per type.
    pub fn default_valence(&self) -> f32 {
        match self {
            Self::Belief => 0.0,
            Self::Experience => 0.0,
            Self::Pattern => 0.1,
            Self::Signal => 0.0,
            Self::Intention => 0.3,
        }
    }

    /// Default arousal (activation level) per type.
    pub fn default_arousal(&self) -> f32 {
        match self {
            Self::Belief => 0.3,
            Self::Experience => 0.5,
            Self::Pattern => 0.4,
            Self::Signal => 0.8,
            Self::Intention => 0.6,
        }
    }
}

impl std::fmt::Display for EpistemicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Belief => write!(f, "Belief"),
            Self::Experience => write!(f, "Experience"),
            Self::Pattern => write!(f, "Pattern"),
            Self::Signal => write!(f, "Signal"),
            Self::Intention => write!(f, "Intention"),
        }
    }
}

/// Temporal recency degree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecencyDegree {
    Fresh,   // < 5 minutes
    Recent,  // < 1 hour
    Distant, // < 24 hours
    Ancient, // no limit
}

impl RecencyDegree {
    pub fn to_time_window_secs(&self) -> Option<i64> {
        match self {
            Self::Fresh => Some(300),
            Self::Recent => Some(3_600),
            Self::Distant => Some(86_400),
            Self::Ancient => None,
        }
    }

    pub fn to_energy_floor(&self) -> f32 {
        match self {
            Self::Fresh => 0.70,
            Self::Recent => 0.40,
            Self::Distant => 0.20,
            Self::Ancient => 0.05,
        }
    }
}

/// Novelty degree for search results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NoveltyDegree {
    High,
    Medium,
    Low,
}

/// Context scope for queries.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContextScope {
    Session,
    Collection,
    Graph,
    Named(String),
}

/// Curvature degree (density of hyperbolic region).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurvatureDegree {
    High,   // > 20 neighbors in r < 0.1
    Medium, // 10-20 neighbors
    Low,    // 3-10 neighbors
    Flat,   // < 3 neighbors (isolated)
}

/// Valence specification (emotional polarity).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValenceSpec {
    Positive,
    Negative,
    Neutral,
    Exact(f32), // -1.0 to 1.0
}

/// Arousal specification (activation level).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ArousalSpec {
    High,
    Medium,
    Low,
    Calm,
    Exact(f32), // 0.0 to 1.0
}

/// Mood state — modifies planner behavior globally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MoodState {
    Creative,     // NOVELTY high, DEPTH unlimited, RESONATE amplified
    Analytical,   // CONFIDENCE high, DEPTH limited, TRACE preferred
    Anxious,      // RECENCY fresh, CONFIDENCE high, short chains
    Focused,      // LIMIT small, WITHIN session, suppress noise
    Exploratory,  // NOVELTY high, DEPTH high, FADE suppressed
    Conservative, // CONFIDENCE high, NOVELTY low, IMPRINT restricted
}

/// Comparison operator for WHEN conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompOp {
    Gte,
    Lte,
    Gt,
    Lt,
    Eq,
    Neq,
}

impl CompOp {
    pub fn eval_f64(&self, left: f64, right: f64) -> bool {
        match self {
            Self::Gte => left >= right,
            Self::Lte => left <= right,
            Self::Gt => left > right,
            Self::Lt => left < right,
            Self::Eq => (left - right).abs() < f64::EPSILON,
            Self::Neq => (left - right).abs() >= f64::EPSILON,
        }
    }
}

/// Conflict resolution policy for multi-agent NEGOTIATE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConflictPolicy {
    WeightedAverage,
    KeepHigher,
    ReplaceAlways,
    CreateConflict,
}

/// Watch trigger type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WatchTrigger {
    OnChange,
    OnInsert,
}

/// Geometry type supported by a backend.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Geometry {
    Hyperbolic { curvature: f64 },
    Euclidean,
    Spherical,
    None,
}

/// Evidence weight combines observation count with confidence.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EvidenceWeight {
    pub count: u32,
    pub confidence: f32,
}

impl EvidenceWeight {
    pub fn new(count: u32, confidence: f32) -> Self {
        Self { count, confidence }
    }

    pub fn combined_weight(&self) -> f32 {
        self.confidence * (self.count as f32 + 1.0).log2()
    }
}

/// Magnitude range for hyperbolic depth filtering.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MagnitudeRange {
    pub min: f32,
    pub max: f32,
}
