//! Lowering AQL plans → Cypher queries for Neo4j.

use aql_core::plans::*;

/// Generate Cypher query for RECALL.
pub fn lower_recall(plan: &RecallPlan) -> String {
    let limit = plan.base.limit.unwrap_or(10);
    if let Some(ref etype) = plan.type_filter {
        format!(
            "MATCH (n:{}) WHERE n.content CONTAINS $query RETURN n LIMIT {}",
            etype.to_nietzsche_node_type(),
            limit
        )
    } else {
        format!(
            "MATCH (n) WHERE n.content CONTAINS $query RETURN n LIMIT {}",
            limit
        )
    }
}

/// Generate Cypher for TRACE (shortest path).
pub fn lower_trace(plan: &TracePlan) -> String {
    let depth = plan.depth.unwrap_or(5);
    format!(
        "MATCH p=shortestPath((a)-[*..{}]->(b)) WHERE a.content = $from AND b.content = $to RETURN p",
        depth
    )
}

/// Generate Cypher for IMPRINT.
pub fn lower_imprint(plan: &ImprintPlan) -> String {
    let label = plan
        .epistemic_type
        .map(|t| t.to_nietzsche_node_type().to_string())
        .unwrap_or("Node".into());
    format!(
        "CREATE (n:{} {{content: $content, energy: $energy, created_at: datetime()}}) RETURN n",
        label
    )
}

/// Generate Cypher for ASSOCIATE.
pub fn lower_associate(plan: &AssociatePlan) -> String {
    format!(
        "MATCH (a), (b) WHERE a.content = $source AND b.content = $target \
         MERGE (a)-[r:ASSOCIATED]->(b) SET r.weight = coalesce(r.weight, 0) + 1 RETURN r"
    )
}
