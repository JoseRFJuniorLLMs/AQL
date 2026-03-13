//! Lowering AQL plans → Qdrant API calls.

use aql_core::plans::*;
use serde_json::{json, Value};

/// Build Qdrant search request for RECALL.
pub fn lower_recall(plan: &RecallPlan) -> Value {
    let limit = plan.base.limit.unwrap_or(10);
    let mut filter = json!({});

    if let Some(ref etype) = plan.type_filter {
        filter = json!({
            "must": [{
                "key": "epistemic_type",
                "match": { "value": etype.to_nietzsche_node_type() }
            }]
        });
    }

    json!({
        "query": plan.query,
        "limit": limit,
        "filter": filter,
        "with_payload": true,
    })
}

/// Build Qdrant upsert for IMPRINT.
pub fn lower_imprint(plan: &ImprintPlan) -> Value {
    json!({
        "points": [{
            "id": uuid::Uuid::new_v4().to_string(),
            "payload": {
                "content": plan.content,
                "epistemic_type": plan.epistemic_type.map(|t| t.to_string()),
                "energy": plan.initial_energy.unwrap_or(0.6),
            }
        }]
    })
}
