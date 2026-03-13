//! Lowering AQL plans → SQL queries for PostgreSQL+pgvector.

use aql_core::plans::*;

/// Generate SQL for RECALL (vector similarity + text search).
pub fn lower_recall(plan: &RecallPlan) -> String {
    let limit = plan.base.limit.unwrap_or(10);
    let type_filter = plan
        .type_filter
        .map(|t| format!("AND node_type = '{}'", t.to_nietzsche_node_type()))
        .unwrap_or_default();

    format!(
        "SELECT *, embedding <-> $1 AS distance \
         FROM nodes WHERE content ILIKE '%' || $2 || '%' {} \
         ORDER BY distance LIMIT {}",
        type_filter, limit
    )
}

/// Generate SQL for IMPRINT.
pub fn lower_imprint(plan: &ImprintPlan) -> String {
    let node_type = plan
        .epistemic_type
        .map(|t| t.to_nietzsche_node_type().to_string())
        .unwrap_or("Semantic".into());
    format!(
        "INSERT INTO nodes (id, content, node_type, energy, embedding, created_at) \
         VALUES (gen_random_uuid(), $1, '{}', $2, $3, NOW()) RETURNING *",
        node_type
    )
}

/// Generate SQL for TRACE (recursive CTE).
pub fn lower_trace(plan: &TracePlan) -> String {
    let depth = plan.depth.unwrap_or(5);
    format!(
        "WITH RECURSIVE path AS (\
           SELECT source_id, target_id, 1 AS depth, ARRAY[source_id] AS visited \
           FROM edges WHERE source_id = $1 \
           UNION ALL \
           SELECT e.source_id, e.target_id, p.depth + 1, p.visited || e.source_id \
           FROM edges e JOIN path p ON e.source_id = p.target_id \
           WHERE p.depth < {} AND NOT e.source_id = ANY(p.visited)\
         ) SELECT * FROM path WHERE target_id = $2",
        depth
    )
}

/// Generate SQL for ASSOCIATE.
pub fn lower_associate(plan: &AssociatePlan) -> String {
    "INSERT INTO edges (source_id, target_id, edge_type, weight) \
     VALUES ($1, $2, 'ASSOCIATED', 1.0) \
     ON CONFLICT (source_id, target_id, edge_type) \
     DO UPDATE SET weight = edges.weight + 1"
        .to_string()
}
