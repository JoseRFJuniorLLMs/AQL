//! Lowering AQL plans → MySQL SQL queries.

use aql_core::plans::*;

/// MySQL FULLTEXT search for RECALL.
pub fn lower_recall(plan: &RecallPlan) -> String {
    let limit = plan.base.limit.unwrap_or(10);
    let type_filter = plan
        .type_filter
        .map(|t| format!("AND node_type = '{}'", t.to_nietzsche_node_type()))
        .unwrap_or_default();

    format!(
        "SELECT *, MATCH(content) AGAINST($1 IN NATURAL LANGUAGE MODE) AS relevance \
         FROM nodes WHERE MATCH(content) AGAINST($1 IN NATURAL LANGUAGE MODE) {} \
         ORDER BY relevance DESC LIMIT {}",
        type_filter, limit
    )
}

/// MySQL INSERT for IMPRINT.
pub fn lower_imprint(plan: &ImprintPlan) -> String {
    let node_type = plan
        .epistemic_type
        .map(|t| t.to_nietzsche_node_type().to_string())
        .unwrap_or("Semantic".into());
    format!(
        "INSERT INTO nodes (id, content, node_type, energy, created_at) \
         VALUES (UUID(), $1, '{}', $2, NOW())",
        node_type
    )
}

/// MySQL recursive CTE for TRACE.
pub fn lower_trace(plan: &TracePlan) -> String {
    let depth = plan.depth.unwrap_or(5);
    format!(
        "WITH RECURSIVE path AS (\
           SELECT source_id, target_id, 1 AS depth, CAST(source_id AS CHAR(1000)) AS visited \
           FROM edges WHERE source_id = $1 \
           UNION ALL \
           SELECT e.source_id, e.target_id, p.depth + 1, CONCAT(p.visited, ',', e.source_id) \
           FROM edges e JOIN path p ON e.source_id = p.target_id \
           WHERE p.depth < {} AND NOT FIND_IN_SET(e.source_id, p.visited)\
         ) SELECT * FROM path WHERE target_id = $2",
        depth
    )
}

/// MySQL INSERT/UPDATE for ASSOCIATE.
pub fn lower_associate(plan: &AssociatePlan) -> String {
    "INSERT INTO edges (source_id, target_id, edge_type, weight, created_at) \
     VALUES ($1, $2, 'ASSOCIATED', 1.0, NOW()) \
     ON DUPLICATE KEY UPDATE weight = weight + 1"
        .to_string()
}

/// MySQL FADE (reduce energy or delete).
pub fn lower_fade(plan: &FadePlan) -> Vec<String> {
    vec![
        format!(
            "UPDATE nodes SET energy = energy - {} WHERE energy >= {}",
            plan.energy_decrement, plan.delete_threshold
        ),
        format!(
            "DELETE FROM nodes WHERE energy < {}",
            plan.delete_threshold
        ),
    ]
}
