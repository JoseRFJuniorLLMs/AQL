//! Lowering AQL plans → SQLite SQL queries.

use aql_core::plans::*;

/// SQLite FTS5 search for RECALL.
pub fn lower_recall(plan: &RecallPlan) -> String {
    let limit = plan.base.limit.unwrap_or(10);
    let type_filter = plan
        .type_filter
        .map(|t| format!("AND n.node_type = '{}'", t.to_nietzsche_node_type()))
        .unwrap_or_default();

    format!(
        "SELECT n.*, fts.rank FROM nodes n \
         JOIN nodes_fts fts ON n.id = fts.rowid \
         WHERE nodes_fts MATCH $1 {} \
         ORDER BY fts.rank LIMIT {}",
        type_filter, limit
    )
}

/// SQLite INSERT for IMPRINT.
pub fn lower_imprint(plan: &ImprintPlan) -> String {
    let node_type = plan
        .epistemic_type
        .map(|t| t.to_nietzsche_node_type().to_string())
        .unwrap_or("Semantic".into());
    format!(
        "INSERT INTO nodes (id, content, node_type, energy, created_at) \
         VALUES ($1, $2, '{}', $3, datetime('now'))",
        node_type
    )
}

/// SQLite recursive CTE for TRACE.
pub fn lower_trace(plan: &TracePlan) -> String {
    let depth = plan.depth.unwrap_or(5);
    format!(
        "WITH RECURSIVE path(source_id, target_id, depth, visited) AS (\
           SELECT source_id, target_id, 1, source_id \
           FROM edges WHERE source_id = $1 \
           UNION ALL \
           SELECT e.source_id, e.target_id, p.depth + 1, p.visited || ',' || e.source_id \
           FROM edges e JOIN path p ON e.source_id = p.target_id \
           WHERE p.depth < {} AND p.visited NOT LIKE '%%' || e.source_id || '%%'\
         ) SELECT * FROM path WHERE target_id = $2",
        depth
    )
}

/// SQLite UPSERT for ASSOCIATE.
pub fn lower_associate(_plan: &AssociatePlan) -> String {
    "INSERT INTO edges (source_id, target_id, edge_type, weight, created_at) \
     VALUES ($1, $2, 'ASSOCIATED', 1.0, datetime('now')) \
     ON CONFLICT(source_id, target_id, edge_type) \
     DO UPDATE SET weight = weight + 1"
        .to_string()
}

/// Schema creation for SQLite (convenience).
pub fn create_schema() -> Vec<String> {
    vec![
        "CREATE TABLE IF NOT EXISTS nodes (\
           id TEXT PRIMARY KEY, content TEXT NOT NULL, \
           node_type TEXT DEFAULT 'Semantic', energy REAL DEFAULT 0.6, \
           created_at TEXT DEFAULT (datetime('now')), \
           updated_at TEXT DEFAULT (datetime('now')))"
            .to_string(),
        "CREATE VIRTUAL TABLE IF NOT EXISTS nodes_fts USING fts5(\
           content, content='nodes', content_rowid='rowid')"
            .to_string(),
        "CREATE TABLE IF NOT EXISTS edges (\
           source_id TEXT NOT NULL, target_id TEXT NOT NULL, \
           edge_type TEXT DEFAULT 'ASSOCIATED', weight REAL DEFAULT 1.0, \
           created_at TEXT DEFAULT (datetime('now')), \
           UNIQUE(source_id, target_id, edge_type))"
            .to_string(),
    ]
}
