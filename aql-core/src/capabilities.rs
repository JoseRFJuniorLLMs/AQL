//! Backend capabilities declaration.
//! Each backend declares what it supports so the planner can degrade gracefully.

use crate::types::Geometry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendCapabilities {
    // Geometry
    pub geometry: Geometry,
    pub has_magnitude: bool,
    pub has_curvature: bool,

    // Search
    pub has_vector_search: bool,
    pub has_full_text: bool,
    pub has_diffusion: bool,
    pub has_graph_algos: bool,

    // Graph
    pub has_edges: bool,
    pub has_typed_edges: bool,
    pub has_edge_weight: bool,
    pub has_traversal: bool,

    // Energy / Cognition
    pub has_energy: bool,
    pub has_decay: bool,
    pub has_valence: bool,
    pub has_arousal: bool,
    pub has_sleep: bool,
    pub has_dream: bool,

    // Time
    pub has_timestamps: bool,
    pub has_ttl: bool,

    // Operational
    pub max_batch_size: usize,
    pub supports_atomic: bool,
    pub supports_watch: bool,
}

impl BackendCapabilities {
    /// NietzscheDB: full capabilities.
    pub fn nietzschedb() -> Self {
        Self {
            geometry: Geometry::Hyperbolic { curvature: -1.0 },
            has_magnitude: true,
            has_curvature: true,
            has_vector_search: true,
            has_full_text: true,
            has_diffusion: true,
            has_graph_algos: true,
            has_edges: true,
            has_typed_edges: true,
            has_edge_weight: true,
            has_traversal: true,
            has_energy: true,
            has_decay: true,
            has_valence: true,
            has_arousal: true,
            has_sleep: true,
            has_dream: true,
            has_timestamps: true,
            has_ttl: true,
            max_batch_size: 10_000,
            supports_atomic: true,
            supports_watch: true,
        }
    }

    /// Minimal backend (all false).
    pub fn minimal() -> Self {
        Self {
            geometry: Geometry::None,
            has_magnitude: false,
            has_curvature: false,
            has_vector_search: false,
            has_full_text: false,
            has_diffusion: false,
            has_graph_algos: false,
            has_edges: false,
            has_typed_edges: false,
            has_edge_weight: false,
            has_traversal: false,
            has_energy: false,
            has_decay: false,
            has_valence: false,
            has_arousal: false,
            has_sleep: false,
            has_dream: false,
            has_timestamps: false,
            has_ttl: false,
            max_batch_size: 100,
            supports_atomic: false,
            supports_watch: false,
        }
    }

    /// Check if backend supports hyperbolic geometry.
    pub fn is_hyperbolic(&self) -> bool {
        matches!(self.geometry, Geometry::Hyperbolic { .. })
    }
}
