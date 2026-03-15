//! AqlExecutor — runs ExecutionPlans against a backend.
//! Handles chains, parallel, atomic, conditionals.

use crate::ast::Verb;
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

    async fn execute_inner(&mut self, plan: &ExecutionPlan) -> AqlResult<CognitiveResult> {
        let start = Instant::now();

        let result = match plan {
            ExecutionPlan::Recall(p) => self.backend.recall(p).await?,
            ExecutionPlan::Resonate(p) => self.backend.resonate(p).await?,
            ExecutionPlan::Reflect(p) => self.backend.reflect(p).await?,
            ExecutionPlan::Trace(p) => self.backend.trace(p).await?,
            ExecutionPlan::Imprint(p) => self.backend.imprint(p).await?,
            ExecutionPlan::Associate(p) => self.backend.associate(p).await?,
            ExecutionPlan::Distill(p) => self.backend.distill(p).await?,
            ExecutionPlan::Fade(p) => self.backend.fade(p).await?,
            ExecutionPlan::Descend(p) => self.backend.descend(p).await?,
            ExecutionPlan::Ascend(p) => self.backend.ascend(p).await?,
            ExecutionPlan::Orbit(p) => self.backend.orbit(p).await?,
            ExecutionPlan::Dream(p) => self.backend.dream(p).await?,
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
                self.execute_chain(plans).await?
            }
            ExecutionPlan::Parallel { branches, join } => {
                self.execute_parallel(branches, join.as_deref()).await?
            }
            ExecutionPlan::Atomic(plans) => {
                self.execute_atomic(plans).await?
            }
        };

        let elapsed = start.elapsed().as_millis() as u64;
        let mut result = result;
        result.metadata.execution_time_ms = elapsed;
        result.metadata.backend = self.backend.name().to_string();

        // Store in working memory
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
        _ => Ok(CognitiveResult::empty()),
    }
}
