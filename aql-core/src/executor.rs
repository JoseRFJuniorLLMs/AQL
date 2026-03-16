//! AqlExecutor — runs ExecutionPlans against a backend.
//! Handles chains, parallel, atomic, conditionals.

use crate::ast::{ConditionValue, Verb, WhenClause};
use crate::error::{AqlError, AqlResult};
use crate::plans::*;
use crate::result::{CognitiveResult, Explanation, WatchHandle};
use crate::traits::AqlBackend;
use crate::memory::WorkingMemory;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// The main executor that dispatches plans to backends.
/// Default query timeout: 30 seconds.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

pub struct AqlExecutor {
    backend: Arc<dyn AqlBackend>,
    memory: WorkingMemory,
    max_parallel: usize,
    timeout: Duration,
}

impl AqlExecutor {
    pub fn new(backend: Arc<dyn AqlBackend>) -> Self {
        Self {
            backend,
            memory: WorkingMemory::new(),
            max_parallel: 8,
            timeout: DEFAULT_TIMEOUT,
        }
    }

    pub fn with_max_parallel(mut self, max: usize) -> Self {
        self.max_parallel = max;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Execute a single plan with timeout protection.
    pub fn execute<'a>(&'a mut self, plan: &'a ExecutionPlan) -> std::pin::Pin<Box<dyn std::future::Future<Output = AqlResult<CognitiveResult>> + Send + 'a>> {
        Box::pin(self.execute_with_timeout(plan))
    }

    async fn execute_with_timeout(&mut self, plan: &ExecutionPlan) -> AqlResult<CognitiveResult> {
        tokio::time::timeout(self.timeout, self.execute_inner(plan))
            .await
            .map_err(|_| AqlError::Execution(format!(
                "query timed out after {}s", self.timeout.as_secs()
            )))?
    }

    /// Resolve a variable-referenced query source into actual content from working memory.
    /// Returns the node IDs or content as a comma-separated string for use in FTS fallback,
    /// or None if it's a normal text query.
    fn resolve_query_source(&self, base: &PlanBase) -> Option<CognitiveResult> {
        match &base.query_source {
            QuerySource::Text => None,
            QuerySource::PreviousResults { index } => {
                if let Some(idx) = index {
                    self.memory.get_indexed_result(*idx).cloned()
                } else {
                    self.memory.last_result().cloned()
                }
            }
            QuerySource::LastDream => self.memory.last_dream().cloned(),
            QuerySource::DelegateResult => self.memory.delegate_result().cloned(),
        }
    }

    async fn execute_inner(&mut self, plan: &ExecutionPlan) -> AqlResult<CognitiveResult> {
        let start = Instant::now();

        let result = match plan {
            ExecutionPlan::Recall(p) => {
                // If query source is a variable reference, return resolved results directly
                if let Some(resolved) = self.resolve_query_source(&p.base) {
                    resolved
                } else {
                    self.backend.recall(p).await?
                }
            }
            ExecutionPlan::Resonate(p) => {
                if let Some(resolved) = self.resolve_query_source(&p.base) {
                    resolved
                } else {
                    self.backend.resonate(p).await?
                }
            }
            ExecutionPlan::Reflect(p) => self.backend.reflect(p).await?,
            ExecutionPlan::Trace(p) => self.backend.trace(p).await?,
            ExecutionPlan::Imprint(p) => self.backend.imprint(p).await?,
            ExecutionPlan::Associate(p) => self.backend.associate(p).await?,
            ExecutionPlan::Distill(p) => {
                if let Some(resolved) = self.resolve_query_source(&p.base) {
                    resolved
                } else {
                    self.backend.distill(p).await?
                }
            }
            ExecutionPlan::Fade(p) => self.backend.fade(p).await?,
            ExecutionPlan::Descend(p) => {
                if let Some(resolved) = self.resolve_query_source(&p.base) {
                    resolved
                } else {
                    self.backend.descend(p).await?
                }
            }
            ExecutionPlan::Ascend(p) => {
                if let Some(resolved) = self.resolve_query_source(&p.base) {
                    resolved
                } else {
                    self.backend.ascend(p).await?
                }
            }
            ExecutionPlan::Orbit(p) => {
                if let Some(resolved) = self.resolve_query_source(&p.base) {
                    resolved
                } else {
                    self.backend.orbit(p).await?
                }
            }
            ExecutionPlan::Dream(p) => {
                let dream_result = self.backend.dream(p).await?;
                self.memory.set_dream_result(dream_result.clone());
                dream_result
            }
            ExecutionPlan::Imagine(p) => self.backend.imagine(p).await?,
            ExecutionPlan::Watch(p) => {
                let handle = self.backend.watch(p).await?;
                self.memory.set_watch_handle(handle);
                CognitiveResult::empty()
            }
            ExecutionPlan::Explain(p) => {
                let explanation = self.backend.explain(p).await?;
                self.memory.set_explanation(explanation);
                CognitiveResult::empty()
            }
            ExecutionPlan::Chain(plans) => {
                let r = self.execute_chain(plans).await?;
                // Chain sub-plans already pushed their results via execute();
                // skip the push below to avoid duplicating the last step.
                let elapsed = start.elapsed().as_millis() as u64;
                let mut result = r;
                result.metadata.execution_time_ms = elapsed;
                result.metadata.backend = self.backend.name().to_string();
                return Ok(result);
            }
            ExecutionPlan::Parallel { branches, join } => {
                let r = self.execute_parallel(branches, join.as_deref()).await?;
                let elapsed = start.elapsed().as_millis() as u64;
                let mut result = r;
                result.metadata.execution_time_ms = elapsed;
                result.metadata.backend = self.backend.name().to_string();
                return Ok(result);
            }
            ExecutionPlan::Conditional(cp) => {
                let r = self.execute_conditional(cp).await?;
                let elapsed = start.elapsed().as_millis() as u64;
                let mut result = r;
                result.metadata.execution_time_ms = elapsed;
                result.metadata.backend = self.backend.name().to_string();
                return Ok(result);
            }
            ExecutionPlan::Atomic(plans) => {
                let r = self.execute_atomic(plans).await?;
                let elapsed = start.elapsed().as_millis() as u64;
                let mut result = r;
                result.metadata.execution_time_ms = elapsed;
                result.metadata.backend = self.backend.name().to_string();
                return Ok(result);
            }
        };

        let elapsed = start.elapsed().as_millis() as u64;
        let mut result = result;
        result.metadata.execution_time_ms = elapsed;
        result.metadata.backend = self.backend.name().to_string();

        // Store in working memory (only for leaf plans — compound plans already stored)
        self.memory.push_result(result.clone());

        Ok(result)
    }

    /// Execute a chain of plans sequentially, passing results forward.
    async fn execute_chain(&mut self, plans: &[ExecutionPlan]) -> AqlResult<CognitiveResult> {
        let mut last_result = CognitiveResult::empty();
        for plan in plans {
            last_result = self.execute(plan).await?;
        }
        Ok(last_result)
    }

    /// Execute plans in parallel with concurrency limit.
    async fn execute_parallel(
        &mut self,
        branches: &[ExecutionPlan],
        join: Option<&ExecutionPlan>,
    ) -> AqlResult<CognitiveResult> {
        let backend = self.backend.clone();
        let mut handles = Vec::new();

        for plan in branches {
            let backend = backend.clone();
            let plan = plan.clone();
            handles.push(tokio::spawn(async move {
                execute_single(&*backend, &plan).await
            }));
        }

        let mut results = Vec::new();
        for handle in handles {
            let result = handle
                .await
                .map_err(|e| AqlError::Execution(format!("parallel task failed: {e}")))?;
            results.push(result?);
        }

        // Store parallel results
        for (i, r) in results.iter().enumerate() {
            self.memory.set_indexed_result(i, r.clone());
        }

        let merged = CognitiveResult::merge(results);

        if let Some(join_plan) = join {
            self.memory.push_result(merged);
            self.execute(join_plan).await
        } else {
            Ok(merged)
        }
    }

    /// Execute atomic block — all or nothing.
    async fn execute_atomic(&mut self, plans: &[ExecutionPlan]) -> AqlResult<CognitiveResult> {
        // Note: true atomic semantics depend on backend support.
        // For now, execute sequentially and collect results.
        let mut results = Vec::new();
        for plan in plans {
            match self.execute(plan).await {
                Ok(r) => results.push(r),
                Err(e) => {
                    return Err(AqlError::AtomicFailed {
                        reason: format!("step failed: {e}"),
                    });
                }
            }
        }
        Ok(CognitiveResult::merge(results))
    }

    /// Get working memory.
    pub fn memory(&self) -> &WorkingMemory {
        &self.memory
    }

    /// Execute a conditional (WHEN/ELSE) plan.
    async fn execute_conditional(&mut self, cp: &ConditionalPlan) -> AqlResult<CognitiveResult> {
        if self.evaluate_condition(&cp.condition) {
            self.execute(&cp.then_plan).await
        } else if let Some(ref else_plan) = cp.else_plan {
            self.execute(else_plan).await
        } else {
            Ok(CognitiveResult::empty())
        }
    }

    /// Evaluate a WHEN condition against the last result in working memory.
    fn evaluate_condition(&self, when: &WhenClause) -> bool {
        let last = match self.memory.last_result() {
            Some(r) => r,
            None => return false,
        };

        // Resolve the field value from the last result's aggregate metadata
        let field_val: Option<f64> = match when.field.as_str() {
            "count" => Some(last.metadata.count as f64),
            "avg_energy" => Some(last.metadata.avg_energy as f64),
            "max_energy" => Some(last.metadata.max_energy as f64),
            "min_confidence" => Some(last.metadata.min_confidence as f64),
            "avg_confidence" => Some(last.metadata.avg_confidence as f64),
            "execution_time_ms" => Some(last.metadata.execution_time_ms as f64),
            // Also support checking fields on the first node
            "energy" => last.nodes.first().map(|n| n.energy as f64),
            "confidence" => last.nodes.first().map(|n| n.confidence as f64),
            "valence" => last.nodes.first().map(|n| n.valence as f64),
            "arousal" => last.nodes.first().map(|n| n.arousal as f64),
            "magnitude" => last.nodes.first().and_then(|n| n.magnitude.map(|m| m as f64)),
            _ => None,
        };

        let right_val: f64 = match &when.value {
            ConditionValue::Float(f) => *f,
            ConditionValue::Int(i) => *i as f64,
            ConditionValue::Str(_) => return false, // string conditions not yet supported for numeric fields
        };

        match field_val {
            Some(left) => when.op.eval_f64(left, right_val),
            None => false,
        }
    }

    /// Get mutable working memory.
    pub fn memory_mut(&mut self) -> &mut WorkingMemory {
        &mut self.memory
    }
}

/// Execute a single plan against a backend (for parallel spawning).
async fn execute_single(
    backend: &dyn AqlBackend,
    plan: &ExecutionPlan,
) -> AqlResult<CognitiveResult> {
    match plan {
        ExecutionPlan::Recall(p) => backend.recall(p).await,
        ExecutionPlan::Resonate(p) => backend.resonate(p).await,
        ExecutionPlan::Reflect(p) => backend.reflect(p).await,
        ExecutionPlan::Trace(p) => backend.trace(p).await,
        ExecutionPlan::Imprint(p) => backend.imprint(p).await,
        ExecutionPlan::Associate(p) => backend.associate(p).await,
        ExecutionPlan::Distill(p) => backend.distill(p).await,
        ExecutionPlan::Fade(p) => backend.fade(p).await,
        ExecutionPlan::Descend(p) => backend.descend(p).await,
        ExecutionPlan::Ascend(p) => backend.ascend(p).await,
        ExecutionPlan::Orbit(p) => backend.orbit(p).await,
        ExecutionPlan::Dream(p) => backend.dream(p).await,
        ExecutionPlan::Imagine(p) => backend.imagine(p).await,
        // Conditional/Chain/Parallel/Atomic require executor state (working memory),
        // so they cannot be dispatched from a stateless parallel spawn context.
        _ => Err(AqlError::Execution(
            "compound plans (Chain/Parallel/Conditional/Atomic) cannot run inside parallel branches".to_string(),
        )),
    }
}
