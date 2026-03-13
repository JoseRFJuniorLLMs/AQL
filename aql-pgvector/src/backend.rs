//! PgVectorBackend — impl AqlBackend for PostgreSQL+pgvector.

use async_trait::async_trait;
use aql_core::capabilities::BackendCapabilities;
use aql_core::error::AqlError;
use aql_core::plans::*;
use aql_core::result::*;
use aql_core::traits::AqlBackend;

pub struct PgVectorBackend {
    connection_string: String,
}

impl PgVectorBackend {
    pub fn new(connection_string: impl Into<String>) -> Self {
        Self {
            connection_string: connection_string.into(),
        }
    }
}

#[async_trait]
impl AqlBackend for PgVectorBackend {
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities::pgvector()
    }

    fn name(&self) -> &str {
        "PostgreSQL+pgvector"
    }

    async fn recall(&self, plan: &RecallPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(query = %plan.query, "RECALL via pgvector");
        Ok(CognitiveResult::empty())
    }

    async fn imprint(&self, plan: &ImprintPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(content = %plan.content, "IMPRINT via pgvector");
        Ok(CognitiveResult::empty())
    }

    async fn fade(&self, plan: &FadePlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!("FADE via pgvector");
        Ok(CognitiveResult::empty())
    }

    async fn associate(&self, plan: &AssociatePlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(source = %plan.source, target = %plan.target, "ASSOCIATE via pgvector");
        Ok(CognitiveResult::empty())
    }

    async fn trace(&self, plan: &TracePlan) -> Result<CognitiveResult, AqlError> {
        // pgvector uses recursive CTE for path traversal
        tracing::info!(from = %plan.from, to = %plan.to, "TRACE via pgvector (recursive CTE)");
        Ok(CognitiveResult::empty())
    }
}
