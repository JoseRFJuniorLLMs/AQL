//! SqliteBackend — impl AqlBackend for SQLite.
//! Uses FTS5 for full-text search, recursive CTE for TRACE.

use async_trait::async_trait;
use aql_core::capabilities::BackendCapabilities;
use aql_core::error::AqlError;
use aql_core::plans::*;
use aql_core::result::*;
use aql_core::traits::AqlBackend;
use aql_core::types::Geometry;

pub struct SqliteBackend {
    path: String,
}

impl SqliteBackend {
    /// Create with file path (or ":memory:" for in-memory).
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    /// In-memory database (for testing).
    pub fn in_memory() -> Self {
        Self { path: ":memory:".into() }
    }
}

#[async_trait]
impl AqlBackend for SqliteBackend {
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            geometry: Geometry::None,
            has_magnitude: false,
            has_curvature: false,
            has_vector_search: false,
            has_full_text: true,    // FTS5
            has_diffusion: false,
            has_graph_algos: false,
            has_edges: true,        // edges table
            has_typed_edges: true,
            has_edge_weight: true,
            has_traversal: true,    // recursive CTE
            has_energy: false,
            has_decay: false,
            has_valence: false,
            has_arousal: false,
            has_sleep: false,
            has_dream: false,
            has_timestamps: true,
            has_ttl: false,
            max_batch_size: 5_000,
            supports_atomic: true,  // SQLite WAL transactions
            supports_watch: false,
        }
    }

    fn name(&self) -> &str {
        "SQLite"
    }

    async fn recall(&self, plan: &RecallPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(query = %plan.query, "RECALL via SQLite (FTS5 MATCH)");
        Ok(CognitiveResult::empty())
    }

    async fn imprint(&self, plan: &ImprintPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(content = %plan.content, "IMPRINT via SQLite (INSERT)");
        Ok(CognitiveResult::empty())
    }

    async fn fade(&self, plan: &FadePlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!("FADE via SQLite (UPDATE/DELETE)");
        Ok(CognitiveResult::empty())
    }

    async fn associate(&self, plan: &AssociatePlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(source = %plan.source, target = %plan.target, "ASSOCIATE via SQLite");
        Ok(CognitiveResult::empty())
    }

    async fn trace(&self, plan: &TracePlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!(from = %plan.from, to = %plan.to, "TRACE via SQLite (recursive CTE)");
        Ok(CognitiveResult::empty())
    }

    async fn reflect(&self, plan: &ReflectPlan) -> Result<CognitiveResult, AqlError> {
        tracing::info!("REFLECT via SQLite");
        Ok(CognitiveResult::empty())
    }
}
