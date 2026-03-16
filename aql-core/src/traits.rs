//! AqlBackend trait — the universal interface for cognitive operations.
//! Any database that implements this can run AQL.

use async_trait::async_trait;
use crate::error::AqlError;
use crate::capabilities::BackendCapabilities;
use crate::result::{CognitiveResult, Explanation, WatchHandle};
use crate::plans::*;

/// Universal backend trait for AQL.
/// Fundamental verbs must be implemented.
/// Advanced verbs have default fallbacks.
#[async_trait]
pub trait AqlBackend: Send + Sync {
    /// Declare what this backend supports.
    // TODO: planner/executor should consult capabilities() for graceful degradation
    fn capabilities(&self) -> BackendCapabilities;

    /// Backend name (for logs and EXPLAIN).
    fn name(&self) -> &str;

    // ── Fundamental verbs (MUST implement) ────────────────────

    async fn recall(&self, plan: &RecallPlan) -> Result<CognitiveResult, AqlError>;
    async fn imprint(&self, plan: &ImprintPlan) -> Result<CognitiveResult, AqlError>;
    async fn fade(&self, plan: &FadePlan) -> Result<CognitiveResult, AqlError>;
    async fn associate(&self, plan: &AssociatePlan) -> Result<CognitiveResult, AqlError>;

    // ── Advanced verbs (default with fallback) ────────────────

    async fn resonate(&self, plan: &ResonatePlan) -> Result<CognitiveResult, AqlError> {
        self.recall(&plan.as_recall()).await
    }

    async fn trace(&self, plan: &TracePlan) -> Result<CognitiveResult, AqlError> {
        // Default: simple BFS — backend can override
        Err(AqlError::UnsupportedVerb {
            verb: "TRACE".into(),
            reason: "default BFS not yet implemented; backend should override".into(),
        })
    }

    async fn distill(&self, plan: &DistillPlan) -> Result<CognitiveResult, AqlError> {
        Err(AqlError::UnsupportedVerb {
            verb: "DISTILL".into(),
            reason: "client-side clustering not yet implemented; backend should override".into(),
        })
    }

    async fn reflect(&self, plan: &ReflectPlan) -> Result<CognitiveResult, AqlError> {
        Ok(CognitiveResult::empty())
    }

    // ── Geometric verbs (hyperbolic only) ─────────────────────

    async fn descend(&self, _plan: &DescendPlan) -> Result<CognitiveResult, AqlError> {
        Err(AqlError::UnsupportedVerb {
            verb: "DESCEND".into(),
            reason: "requires hyperbolic geometry backend".into(),
        })
    }

    async fn ascend(&self, _plan: &AscendPlan) -> Result<CognitiveResult, AqlError> {
        Err(AqlError::UnsupportedVerb {
            verb: "ASCEND".into(),
            reason: "requires hyperbolic geometry backend".into(),
        })
    }

    async fn orbit(&self, _plan: &OrbitPlan) -> Result<CognitiveResult, AqlError> {
        Err(AqlError::UnsupportedVerb {
            verb: "ORBIT".into(),
            reason: "requires hyperbolic geometry backend".into(),
        })
    }

    // ── Altered states (NietzscheDB only) ─────────────────────

    async fn dream(&self, _plan: &DreamPlan) -> Result<CognitiveResult, AqlError> {
        Err(AqlError::UnsupportedVerb {
            verb: "DREAM".into(),
            reason: "requires dream cycle support (NietzscheDB)".into(),
        })
    }

    async fn imagine(&self, _plan: &ImaginePlan) -> Result<CognitiveResult, AqlError> {
        Err(AqlError::UnsupportedVerb {
            verb: "IMAGINE".into(),
            reason: "requires counterfactual reasoning support".into(),
        })
    }

    // ── Reactivity (optional) ─────────────────────────────────

    async fn watch(&self, _plan: &WatchPlan) -> Result<WatchHandle, AqlError> {
        Err(AqlError::WatchFailed(
            "backend does not support reactive subscriptions".into(),
        ))
    }

    // ── EXPLAIN (optional) ────────────────────────────────────

    async fn explain(&self, _plan: &ExplainPlan) -> Result<Explanation, AqlError> {
        Ok(Explanation::not_supported())
    }
}
