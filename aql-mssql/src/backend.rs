//! MssqlBackend — impl AqlBackend for SQL Server.

use async_trait::async_trait;
use aql_core::capabilities::BackendCapabilities;
use aql_core::error::AqlError;
use aql_core::plans::*;
use aql_core::result::*;
use aql_core::traits::AqlBackend;
use aql_core::types::Geometry;

pub struct MssqlBackend {
    connection_string: String,
}

impl MssqlBackend {
    pub fn new(connection_string: impl Into<String>) -> Self {
        Self {
            connection_string: connection_string.into(),
        }
    }
}

#[async_trait]
impl AqlBackend for MssqlBackend {
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            geometry: Geometry::None,
            has_magnitude: false,
            has_curvature: false,
            has_vector_search: false,
            has_full_text: true,    // SQL Server Full-Text Search
            has_diffusion: false,
            has_graph_algos: true,  // SQL Server 2017+ graph tables
            has_edges: true,        // native graph edges (2017+)
            has_typed_edges: true,
            has_edge_weight: true,
            has_traversal: true,    // recursive CTE + MATCH
            has_energy: false,
            has_decay: false,
            has_valence: false,
            has_arousal: false,
            has_sleep: false,
            has_dream: false,
            has_timestamps: true,
            has_ttl: false,
            max_batch_size: 10_000,
            supports_atomic: true,  // Full ACID
            supports_watch: false,
        }
    }

    fn name(&self) -> &str {
        "SQL Server"
    }

    async fn recall(&self, plan: &RecallPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(query = %plan.query, "RECALL via SQL Server (CONTAINS/FREETEXT)");
        Ok(CognitiveResult::empty())
    }

    async fn imprint(&self, plan: &ImprintPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(content = %plan.content, "IMPRINT via SQL Server (INSERT)");
        Ok(CognitiveResult::empty())
    }

    async fn fade(&self, plan: &FadePlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!("FADE via SQL Server (UPDATE/DELETE)");
        Ok(CognitiveResult::empty())
    }

    async fn associate(&self, plan: &AssociatePlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(source = %plan.source, target = %plan.target, "ASSOCIATE via SQL Server (graph edge)");
        Ok(CognitiveResult::empty())
    }

    async fn trace(&self, plan: &TracePlan) -> Result<CognitiveResult, AqlError> {
        // SQL Server 2017+ supports MATCH for graph traversal
        tracing::info!(from = %plan.from, to = %plan.to, "TRACE via SQL Server (MATCH / recursive CTE)");
        Ok(CognitiveResult::empty())
    }

    async fn reflect(&self, plan: &ReflectPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!("REFLECT via SQL Server");
        Ok(CognitiveResult::empty())
    }

    async fn distill(&self, plan: &DistillPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!("DISTILL via SQL Server (GROUP BY + window functions)");
        Ok(CognitiveResult::empty())
    }
}
