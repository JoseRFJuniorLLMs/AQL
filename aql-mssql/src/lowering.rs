//! Lowering AQL plans → T-SQL queries for SQL Server.

use aql_core::plans::*;

/// SQL Server FREETEXT search for RECALL.
pub fn lower_recall(plan: &RecallPlan) -> String {
    let limit = plan.base.limit.unwrap_or(10);
    let type_filter = plan
        .type_filter
        .map(|t| format!("AND node_type = '{}'", t.to_nietzsche_node_type()))
        .unwrap_or_default();

    format!(
        "SELECT TOP {} *, KEY_TBL.RANK FROM nodes n \
         INNER JOIN FREETEXTTABLE(nodes, content, @query) AS KEY_TBL \
         ON n.id = KEY_TBL.[KEY] \
         WHERE 1=1 {} \
         ORDER BY KEY_TBL.RANK DESC",
        limit, type_filter
    )
}

/// SQL Server INSERT for IMPRINT.
pub fn lower_imprint(plan: &ImprintPlan) -> String {
    let node_type = plan
        .epistemic_type
        .map(|t| t.to_nietzsche_node_type().to_string())
        .unwrap_or("Semantic".into());
    format!(
        "INSERT INTO nodes (id, content, node_type, energy, created_at) \
         OUTPUT INSERTED.* \
         VALUES (NEWID(), @content, '{}', @energy, GETUTCDATE())",
        node_type
    )
}

/// SQL Server recursive CTE for TRACE.
pub fn lower_trace(plan: &TracePlan) -> String {
    let depth = plan.depth.unwrap_or(5);
    format!(
        "WITH path AS (\
           SELECT source_id, target_id, 1 AS depth, \
                  CAST(source_id AS VARCHAR(MAX)) AS visited \
           FROM edges WHERE source_id = @from \
           UNION ALL \
           SELECT e.source_id, e.target_id, p.depth + 1, \
                  p.visited + ',' + CAST(e.source_id AS VARCHAR(36)) \
           FROM edges e INNER JOIN path p ON e.source_id = p.target_id \
           WHERE p.depth < {} AND CHARINDEX(CAST(e.source_id AS VARCHAR(36)), p.visited) = 0\
         ) SELECT * FROM path WHERE target_id = @to",
        depth
    )
}

/// SQL Server MERGE for ASSOCIATE (upsert edge).
pub fn lower_associate(_plan: &AssociatePlan) -> String {
    "MERGE edges AS target \
     USING (SELECT @source AS source_id, @target AS target_id, 'ASSOCIATED' AS edge_type) AS source \
     ON target.source_id = source.source_id AND target.target_id = source.target_id AND target.edge_type = source.edge_type \
     WHEN MATCHED THEN UPDATE SET weight = target.weight + 1 \
     WHEN NOT MATCHED THEN INSERT (source_id, target_id, edge_type, weight, created_at) \
     VALUES (source.source_id, source.target_id, source.edge_type, 1.0, GETUTCDATE());"
        .to_string()
}

/// SQL Server graph MATCH for TRACE (2017+).
pub fn lower_trace_graph(plan: &TracePlan) -> String {
    format!(
        "SELECT n1.content AS [from], n2.content AS [to] \
         FROM nodes AS n1, edges AS e, nodes AS n2 \
         WHERE MATCH(n1-(e)->n2) \
         AND n1.content = @from"
    )
}
