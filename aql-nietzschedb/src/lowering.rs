//! Lowering AQL plans → NietzscheDB operations (NAQ/gRPC).
//! Converts high-level cognitive plans to concrete NietzscheDB calls.

use aql_core::plans::*;
use aql_core::types::*;

/// NAQ instruction for NietzscheDB.
#[derive(Debug, Clone)]
pub enum NaqInstruction {
    QueryNodes {
        collection: String,
        nql: String,
        limit: u32,
    },
    KnnSearch {
        collection: String,
        query_text: String,
        k: u32,
    },
    FullTextSearch {
        collection: String,
        query: String,
        limit: u32,
    },
    InsertNode {
        collection: String,
        content: String,
        node_type: String,
        energy: f32,
    },
    InsertEdge {
        collection: String,
        source: String,
        target: String,
        edge_type: String,
        weight: f32,
    },
    UpdateEnergy {
        collection: String,
        node_id: String,
        new_energy: f32,
    },
    DeleteNode {
        collection: String,
        node_id: String,
    },
    Bfs {
        collection: String,
        start: String,
        max_depth: u32,
    },
    Dijkstra {
        collection: String,
        start: String,
        end: String,
    },
    TriggerDream {
        collection: String,
        topic: String,
    },
    TriggerSleep {
        collection: String,
    },
}

/// Lower a RecallPlan to NAQ instructions.
///
/// Only emits FullTextSearch because KnnSearch currently falls back to FTS
/// in the backend (no real embedding pipeline yet). Emitting both would
/// produce 2x identical FTS queries with duplicate results.
pub fn lower_recall(plan: &RecallPlan) -> Vec<NaqInstruction> {
    let collection = plan.base.collection.clone().unwrap_or("default".into());
    let limit = plan.base.limit.unwrap_or(10);

    // TODO: When embedding pipeline is available, emit KnnSearch here
    // and keep FullTextSearch as a fallback with deduplication.
    vec![NaqInstruction::FullTextSearch {
        collection,
        query: plan.query.clone(),
        limit,
    }]
}

// NOTE: lower_imprint() was removed — it was dead code that incorrectly used
// plan.content as edge source instead of the UUID returned by InsertNode.
// The correct implementation lives in NietzscheBackend::imprint() in backend.rs,
// which captures the actual node UUID from the gRPC InsertNode response.

/// Lower a TracePlan to NAQ instructions.
pub fn lower_trace(plan: &TracePlan) -> Vec<NaqInstruction> {
    let collection = plan.base.collection.clone().unwrap_or("default".into());
    vec![NaqInstruction::Dijkstra {
        collection,
        start: plan.from.clone(),
        end: plan.to.clone(),
    }]
}
