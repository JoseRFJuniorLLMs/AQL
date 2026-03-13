//! Lowering AQL plans → Redis commands.

use aql_core::plans::*;

/// Redis command for RECALL via RediSearch.
pub fn lower_recall(plan: &RecallPlan) -> Vec<String> {
    let limit = plan.base.limit.unwrap_or(10);
    vec![format!(
        "FT.SEARCH idx:nodes \"{}\" LIMIT 0 {}",
        plan.query, limit
    )]
}

/// Redis commands for IMPRINT.
pub fn lower_imprint(plan: &ImprintPlan) -> Vec<String> {
    let key = format!("node:{}", uuid::Uuid::new_v4());
    let energy = plan.initial_energy.unwrap_or(0.6);
    vec![
        format!(
            "HSET {} content \"{}\" energy {} type \"{}\"",
            key,
            plan.content,
            energy,
            plan.epistemic_type
                .map(|t| t.to_nietzsche_node_type().to_string())
                .unwrap_or("Semantic".into())
        ),
    ]
}

/// Redis commands for FADE (set TTL).
pub fn lower_fade(plan: &FadePlan) -> Vec<String> {
    // Fade in Redis = reduce TTL
    vec!["# Fade: scan and reduce TTL on matching keys".to_string()]
}
