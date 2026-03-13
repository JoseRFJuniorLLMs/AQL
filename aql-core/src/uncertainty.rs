//! Uncertainty and Confidence propagation.
//! Confidence is epistemic, not a filter — it represents the agent's certainty.

use crate::types::EvidenceWeight;

/// Propagate confidence through a chain of operations.
pub fn propagate_confidence(confidences: &[f32]) -> f32 {
    if confidences.is_empty() {
        return 0.0;
    }
    // Product of confidences (each step can only reduce certainty).
    confidences.iter().copied().product()
}

/// Combine confidence from multiple sources (e.g., parallel results).
pub fn combine_confidence(confidences: &[f32]) -> f32 {
    if confidences.is_empty() {
        return 0.0;
    }
    // Noisy-OR: 1 - product(1 - c_i)
    1.0 - confidences.iter().map(|c| 1.0 - c).product::<f32>()
}

/// Weight confidence by evidence count.
pub fn evidence_weighted_confidence(confidence: f32, evidence_count: u32) -> f32 {
    let weight = EvidenceWeight::new(evidence_count, confidence);
    weight.combined_weight().min(1.0)
}

/// Bayesian update: prior + new observation → posterior.
pub fn bayesian_update(prior: f32, likelihood: f32, evidence_strength: f32) -> f32 {
    let numerator = likelihood * prior;
    let denominator = likelihood * prior + (1.0 - likelihood) * (1.0 - prior);
    if denominator < f32::EPSILON {
        return prior;
    }
    let posterior = numerator / denominator;
    // Blend with evidence strength
    prior + (posterior - prior) * evidence_strength
}

/// Confidence floor based on recency.
pub fn recency_confidence_floor(recency_secs: i64) -> f32 {
    if recency_secs < 300 {
        0.70
    } else if recency_secs < 3600 {
        0.40
    } else if recency_secs < 86400 {
        0.20
    } else {
        0.05
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_propagate() {
        let result = propagate_confidence(&[0.9, 0.8, 0.7]);
        assert!((result - 0.504).abs() < 0.01);
    }

    #[test]
    fn test_combine() {
        let result = combine_confidence(&[0.8, 0.7]);
        assert!((result - 0.94).abs() < 0.01);
    }

    #[test]
    fn test_evidence_weight() {
        let w = evidence_weighted_confidence(0.9, 100);
        assert!(w > 0.9); // More evidence = higher combined weight
    }
}
