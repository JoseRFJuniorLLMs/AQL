//! MysqlBackend — impl AqlBackend for MySQL/MariaDB.

use async_trait::async_trait;
use aql_core::capabilities::BackendCapabilities;
use aql_core::error::AqlError;
use aql_core::plans::*;
use aql_core::result::*;
use aql_core::traits::AqlBackend;
use aql_core::types::Geometry;

pub struct MysqlBackend {
    connection_string: String,
    database: String,
}

impl MysqlBackend {
    pub fn new(connection_string: impl Into<String>) -> Self {
        Self {
            connection_string: connection_string.into(),
            database: "aql".into(),
        }
    }

    pub fn with_database(mut self, db: impl Into<String>) -> Self {
        self.database = db.into();
        self
    }
}

#[async_trait]
impl AqlBackend for MysqlBackend {
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            geometry: Geometry::None,
            has_magnitude: false,
            has_curvature: false,
            has_vector_search: false,
            has_full_text: true,    // MySQL FULLTEXT index
            has_diffusion: false,
            has_graph_algos: false,
            has_edges: true,        // edges table
            has_typed_edges: true,
            has_edge_weight: true,
            has_traversal: true,    // recursive CTE (MySQL 8+)
            has_energy: false,
            has_decay: false,
            has_valence: false,
            has_arousal: false,
            has_sleep: false,
            has_dream: false,
            has_timestamps: true,
            has_ttl: false,
            max_batch_size: 10_000,
            supports_atomic: true,  // InnoDB ACID
            supports_watch: false,
        }
    }

    fn name(&self) -> &str {
        "MySQL"
    }

    async fn recall(&self, plan: &RecallPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(query = %plan.query, "RECALL via MySQL (FULLTEXT MATCH)");
        Ok(CognitiveResult::empty())
    }

    async fn imprint(&self, plan: &ImprintPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(content = %plan.content, "IMPRINT via MySQL (INSERT)");
        Ok(CognitiveResult::empty())
    }

    async fn fade(&self, plan: &FadePlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!("FADE via MySQL (UPDATE energy / DELETE)");
        Ok(CognitiveResult::empty())
    }

    async fn associate(&self, plan: &AssociatePlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(source = %plan.source, target = %plan.target, "ASSOCIATE via MySQL (edges table)");
        Ok(CognitiveResult::empty())
    }

    async fn trace(&self, plan: &TracePlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(from = %plan.from, to = %plan.to, "TRACE via MySQL (recursive CTE)");
        Ok(CognitiveResult::empty())
    }

    async fn reflect(&self, plan: &ReflectPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!("REFLECT via MySQL (aggregate stats)");
        Ok(CognitiveResult::empty())
    }

    async fn distill(&self, plan: &DistillPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!("DISTILL via MySQL (GROUP BY clustering)");
        Ok(CognitiveResult::empty())
    }
}
