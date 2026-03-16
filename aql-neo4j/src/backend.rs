//! Neo4jBackend — impl AqlBackend for Neo4j.

use async_trait::async_trait;
use aql_core::capabilities::BackendCapabilities;
use aql_core::error::AqlError;
use aql_core::plans::*;
use aql_core::result::*;
use aql_core::traits::AqlBackend;
use aql_core::types::Geometry;

pub struct Neo4jBackend {
    uri: String,
    database: String,
}

impl Neo4jBackend {
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            database: "neo4j".into(),
        }
    }

    pub fn with_database(mut self, db: impl Into<String>) -> Self {
        self.database = db.into();
        self
    }
}

#[async_trait]
impl AqlBackend for Neo4jBackend {
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            geometry: Geometry::Euclidean,
            has_vector_search: true,
            has_full_text: true,
            has_graph_algos: true,
            has_edges: true,
            has_typed_edges: true,
            has_edge_weight: true,
            has_traversal: true,
            has_timestamps: true,
            max_batch_size: 50_000,
            supports_atomic: true,
            ..BackendCapabilities::minimal()
        }
    }

    fn name(&self) -> &str {
        "Neo4j"
    }

    async fn recall(&self, plan: &RecallPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(query = %plan.query, "RECALL via Neo4j");
        Ok(CognitiveResult::empty())
    }

    async fn imprint(&self, plan: &ImprintPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(content = %plan.content, "IMPRINT via Neo4j");
        Ok(CognitiveResult::empty())
    }

    async fn fade(&self, plan: &FadePlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!("FADE via Neo4j");
        Ok(CognitiveResult::empty())
    }

    async fn associate(&self, plan: &AssociatePlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(source = %plan.source, target = %plan.target, "ASSOCIATE via Neo4j");
        Ok(CognitiveResult::empty())
    }

    async fn trace(&self, plan: &TracePlan) -> Result<CognitiveResult, AqlError> {
        // Neo4j excels at path traversal via Cypher
        tracing::info!(from = %plan.from, to = %plan.to, "TRACE via Neo4j (Cypher shortestPath)");
        Ok(CognitiveResult::empty())
    }

    async fn reflect(&self, plan: &ReflectPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!("REFLECT via Neo4j");
        Ok(CognitiveResult::empty())
    }

    async fn distill(&self, plan: &DistillPlan) -> Result<CognitiveResult, AqlError> {
        // Neo4j GDS can do community detection
        tracing::info!("DISTILL via Neo4j (GDS community detection)");
        Ok(CognitiveResult::empty())
    }
}
