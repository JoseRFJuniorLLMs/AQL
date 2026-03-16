//! Cognitive result types returned by AQL operations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A cognitive node returned in results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveNode {
    pub id: String,
    pub content: String,
    pub node_type: String,
    pub energy: f32,
    pub confidence: f32,
    pub valence: f32,
    pub arousal: f32,
    pub magnitude: Option<f32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Result of a cognitive operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveResult {
    pub nodes: Vec<CognitiveNode>,
    pub edges: Vec<CognitiveEdge>,
    pub metadata: ResultMetadata,
}

/// An edge in the cognitive result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveEdge {
    pub source: String,
    pub target: String,
    pub edge_type: String,
    pub weight: f32,
}

/// Metadata about the result.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResultMetadata {
    pub count: u32,
    pub avg_energy: f32,
    pub max_energy: f32,
    pub min_confidence: f32,
    pub avg_confidence: f32,
    pub execution_time_ms: u64,
    pub backend: String,
    pub verb: String,
}

impl CognitiveResult {
    pub fn empty() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            metadata: ResultMetadata::default(),
        }
    }

    pub fn from_nodes(nodes: Vec<CognitiveNode>) -> Self {
        let count = nodes.len() as u32;
        let avg_energy = if nodes.is_empty() {
            0.0
        } else {
            nodes.iter().map(|n| n.energy).sum::<f32>() / nodes.len() as f32
        };
        let max_energy = nodes
            .iter()
            .map(|n| n.energy)
            .fold(f32::NEG_INFINITY, f32::max);
        let avg_confidence = if nodes.is_empty() {
            0.0
        } else {
            nodes.iter().map(|n| n.confidence).sum::<f32>() / nodes.len() as f32
        };
        let min_confidence = nodes
            .iter()
            .map(|n| n.confidence)
            .fold(1.0f32, f32::min);

        Self {
            nodes,
            edges: vec![],
            metadata: ResultMetadata {
                count,
                avg_energy,
                max_energy,
                min_confidence,
                avg_confidence,
                ..Default::default()
            },
        }
    }

    pub fn merge(results: Vec<CognitiveResult>) -> Self {
        let mut all_nodes = Vec::new();
        let mut all_edges = Vec::new();
        for r in results {
            all_nodes.extend(r.nodes);
            all_edges.extend(r.edges);
        }
        let mut result = Self::from_nodes(all_nodes);
        result.edges = all_edges;
        result
    }
}

/// Explanation / provenance of an operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explanation {
    pub supported: bool,
    pub verb: String,
    pub strategy: String,
    pub steps: Vec<ExplanationStep>,
    pub confidence_chain: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationStep {
    pub action: String,
    pub detail: String,
}

impl Explanation {
    pub fn not_supported() -> Self {
        Self {
            supported: false,
            verb: String::new(),
            strategy: String::new(),
            steps: vec![],
            confidence_chain: vec![],
        }
    }
}

/// Handle for WATCH subscriptions.
#[derive(Debug, Clone)]
pub struct WatchHandle {
    pub id: String,
    pub active: bool,
}
