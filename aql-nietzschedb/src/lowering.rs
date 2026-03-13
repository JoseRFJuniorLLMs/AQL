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
pub fn lower_recall(plan: &RecallPlan) -> Vec<NaqInstruction> {
    let collection = plan.base.collection.clone().unwrap_or("default".into());
    let limit = plan.base.limit.unwrap_or(10);

    let mut instructions = vec![];

    // Primary: KNN search
    instructions.push(NaqInstruction::KnnSearch {
        collection: collection.clone(),
        query_text: plan.query.clone(),
        k: limit,
    });

    // If full-text available, also search text
    instructions.push(NaqInstruction::FullTextSearch {
        collection,
        query: plan.query.clone(),
        limit,
    });

    instructions
}

/// Lower an ImprintPlan to NAQ instructions.
pub fn lower_imprint(plan: &ImprintPlan) -> Vec<NaqInstruction> {
    let collection = plan.base.collection.clone().unwrap_or("default".into());
    let node_type = plan
        .epistemic_type
        .map(|t| t.to_nietzsche_node_type().to_string())
        .unwrap_or("Semantic".into());
    let energy = plan.initial_energy.unwrap_or(0.6);

    let mut instructions = vec![NaqInstruction::InsertNode {
        collection: collection.clone(),
        content: plan.content.clone(),
        node_type,
        energy,
    }];

    if let Some(ref link) = plan.link_to {
        instructions.push(NaqInstruction::InsertEdge {
            collection,
            source: plan.content.clone(),
            target: link.clone(),
            edge_type: "ASSOCIATED".into(),
            weight: 1.0,
        });
    }

    instructions
}

/// Lower a TracePlan to NAQ instructions.
pub fn lower_trace(plan: &TracePlan) -> Vec<NaqInstruction> {
    let collection = plan.base.collection.clone().unwrap_or("default".into());
    vec![NaqInstruction::Dijkstra {
        collection,
        start: plan.from.clone(),
        end: plan.to.clone(),
    }]
}
