//! RedisBackend — impl AqlBackend for Redis Stack.

use async_trait::async_trait;
use aql_core::capabilities::BackendCapabilities;
use aql_core::error::AqlError;
use aql_core::plans::*;
use aql_core::result::*;
use aql_core::traits::AqlBackend;
use aql_core::types::Geometry;

pub struct RedisBackend {
    url: String,
    prefix: String,
}

impl RedisBackend {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            prefix: "aql:".into(),
        }
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }
}

#[async_trait]
impl AqlBackend for RedisBackend {
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            geometry: Geometry::None,
            has_vector_search: true,
            has_full_text: true,
            has_ttl: true,
            has_timestamps: true,
            max_batch_size: 10_000,
            ..BackendCapabilities::minimal()
        }
    }

    fn name(&self) -> &str {
        "Redis Stack"
    }

    async fn recall(&self, plan: &RecallPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(query = %plan.query, "RECALL via Redis (RediSearch)");
        Ok(CognitiveResult::empty())
    }

    async fn imprint(&self, plan: &ImprintPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(content = %plan.content, "IMPRINT via Redis");
        Ok(CognitiveResult::empty())
    }

    async fn fade(&self, plan: &FadePlan) -> Result<CognitiveResult, AqlError> {
        // Redis: set TTL to expire nodes
        tracing::info!("FADE via Redis (TTL expiration)");
        Ok(CognitiveResult::empty())
    }

    async fn associate(&self, plan: &AssociatePlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!("ASSOCIATE via Redis (hash links)");
        Ok(CognitiveResult::empty())
    }
}
