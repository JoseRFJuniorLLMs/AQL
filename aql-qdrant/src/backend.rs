//! QdrantBackend — impl AqlBackend for Qdrant.

use async_trait::async_trait;
use aql_core::capabilities::BackendCapabilities;
use aql_core::error::AqlError;
use aql_core::plans::*;
use aql_core::result::*;
use aql_core::traits::AqlBackend;
use aql_core::types::Geometry;

pub struct QdrantBackend {
    url: String,
    collection: String,
}

impl QdrantBackend {
    pub fn new(url: impl Into<String>, collection: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            collection: collection.into(),
        }
    }
}

#[async_trait]
impl AqlBackend for QdrantBackend {
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            geometry: Geometry::Euclidean,
            has_vector_search: true,
            has_full_text: true,
            has_timestamps: true,
            max_batch_size: 10_000,
            ..BackendCapabilities::minimal()
        }
    }

    fn name(&self) -> &str {
        "Qdrant"
    }

    async fn recall(&self, plan: &RecallPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(query = %plan.query, "RECALL via Qdrant");
        Ok(CognitiveResult::empty())
    }

    async fn imprint(&self, plan: &ImprintPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(content = %plan.content, "IMPRINT via Qdrant");
        Ok(CognitiveResult::empty())
    }

    async fn fade(&self, plan: &FadePlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!("FADE via Qdrant");
        Ok(CognitiveResult::empty())
    }

    async fn associate(&self, plan: &AssociatePlan) -> Result<CognitiveResult, AqlError> {
        // Qdrant has no native edges — use payload links
        tracing::info!("ASSOCIATE via Qdrant (payload links)");
        Ok(CognitiveResult::empty())
    }
}
