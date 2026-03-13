//! Execution plans for each AQL verb.
//! The CognitivePlanner converts AST → Plans, then the executor sends Plans → Backend.

use crate::types::*;
use serde::{Deserialize, Serialize};

/// Common fields shared by all plans.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlanBase {
    pub collection: Option<String>,
    pub limit: Option<u32>,
    pub confidence_floor: Option<f32>,
    pub recency: Option<RecencyDegree>,
    pub scope: Option<ContextScope>,
    pub valence: Option<ValenceSpec>,
    pub arousal: Option<ArousalSpec>,
    pub mood: Option<MoodState>,
    pub evidence: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallPlan {
    pub base: PlanBase,
    pub query: String,
    pub type_filter: Option<EpistemicType>,
    pub novelty: Option<NoveltyDegree>,
    pub magnitude_range: Option<MagnitudeRange>,
    pub curvature: Option<CurvatureDegree>,
}

impl RecallPlan {
    pub fn new(query: String) -> Self {
        Self {
            base: PlanBase::default(),
            query,
            type_filter: None,
            novelty: None,
            magnitude_range: None,
            curvature: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonatePlan {
    pub base: PlanBase,
    pub query: String,
    pub depth: Option<u8>,
    pub novelty: Option<NoveltyDegree>,
}

impl ResonatePlan {
    pub fn as_recall(&self) -> RecallPlan {
        RecallPlan {
            base: self.base.clone(),
            query: self.query.clone(),
            type_filter: None,
            novelty: self.novelty,
            magnitude_range: None,
            curvature: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectPlan {
    pub base: PlanBase,
    pub target: ReflectTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReflectTarget {
    SelfAgent,
    TypeFilter(EpistemicType),
    Collection(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracePlan {
    pub base: PlanBase,
    pub from: String,
    pub to: String,
    pub depth: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprintPlan {
    pub base: PlanBase,
    pub content: String,
    pub epistemic_type: Option<EpistemicType>,
    pub link_to: Option<String>,
    pub initial_energy: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociatePlan {
    pub base: PlanBase,
    pub source: String,
    pub source_type: Option<EpistemicType>,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistillPlan {
    pub base: PlanBase,
    pub type_filter: Option<EpistemicType>,
    pub depth: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FadePlan {
    pub base: PlanBase,
    pub type_filter: Option<EpistemicType>,
    pub energy_decrement: f32,
    pub delete_threshold: f32,
}

impl Default for FadePlan {
    fn default() -> Self {
        Self {
            base: PlanBase::default(),
            type_filter: None,
            energy_decrement: 0.2,
            delete_threshold: 0.05,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescendPlan {
    pub base: PlanBase,
    pub content: String,
    pub depth: u8,
    pub magnitude_range: Option<MagnitudeRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AscendPlan {
    pub base: PlanBase,
    pub content: String,
    pub depth: u8,
    pub curvature: Option<CurvatureDegree>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitPlan {
    pub base: PlanBase,
    pub content: String,
    pub radius: f32,
    pub novelty: Option<NoveltyDegree>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamPlan {
    pub base: PlanBase,
    pub topic: String,
    pub depth: Option<u8>,
    pub novelty: Option<NoveltyDegree>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImaginePlan {
    pub base: PlanBase,
    pub premise: String,
    pub depth: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchPlan {
    pub subject_query: String,
    pub trigger: WatchTrigger,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainPlan {
    pub inner_verb: String,
    pub inner_query: String,
}

/// The complete execution plan for a statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionPlan {
    Recall(RecallPlan),
    Resonate(ResonatePlan),
    Reflect(ReflectPlan),
    Trace(TracePlan),
    Imprint(ImprintPlan),
    Associate(AssociatePlan),
    Distill(DistillPlan),
    Fade(FadePlan),
    Descend(DescendPlan),
    Ascend(AscendPlan),
    Orbit(OrbitPlan),
    Dream(DreamPlan),
    Imagine(ImaginePlan),
    Watch(WatchPlan),
    Explain(ExplainPlan),
    Chain(Vec<ExecutionPlan>),
    Parallel {
        branches: Vec<ExecutionPlan>,
        join: Option<Box<ExecutionPlan>>,
    },
    Atomic(Vec<ExecutionPlan>),
}
