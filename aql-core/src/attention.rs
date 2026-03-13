//! Attention system — Focus, Salience, and Attention budget.
//! Controls what the agent pays attention to during cognitive operations.

use crate::types::*;

/// Attention budget — limits cognitive resources per operation.
#[derive(Debug, Clone)]
pub struct AttentionBudget {
    pub max_nodes: usize,
    pub max_depth: u8,
    pub max_time_ms: u64,
    pub focus_weight: f32,
}

impl Default for AttentionBudget {
    fn default() -> Self {
        Self {
            max_nodes: 100,
            max_depth: 5,
            max_time_ms: 5000,
            focus_weight: 1.0,
        }
    }
}

impl AttentionBudget {
    /// Apply mood to adjust attention budget.
    pub fn apply_mood(&mut self, mood: MoodState) {
        match mood {
            MoodState::Creative => {
                self.max_nodes *= 2;
                self.max_depth += 3;
                self.focus_weight = 0.5;
            }
            MoodState::Analytical => {
                self.max_nodes = 50;
                self.max_depth = 3;
                self.focus_weight = 1.5;
            }
            MoodState::Anxious => {
                self.max_nodes = 30;
                self.max_depth = 2;
                self.max_time_ms = 2000;
            }
            MoodState::Focused => {
                self.max_nodes = 20;
                self.max_depth = 2;
                self.focus_weight = 2.0;
            }
            MoodState::Exploratory => {
                self.max_nodes *= 3;
                self.max_depth = 10;
                self.focus_weight = 0.3;
            }
            MoodState::Conservative => {
                self.max_nodes = 30;
                self.max_depth = 3;
                self.focus_weight = 1.5;
            }
        }
    }
}

/// Salience score — how relevant/important a node is in context.
#[derive(Debug, Clone)]
pub struct SalienceScore {
    pub relevance: f32,
    pub recency: f32,
    pub energy: f32,
    pub emotional: f32,
}

impl SalienceScore {
    pub fn compute(
        relevance: f32,
        recency_factor: f32,
        energy: f32,
        valence: f32,
        arousal: f32,
        focus_weight: f32,
    ) -> Self {
        let emotional = (valence.abs() + arousal) / 2.0;
        Self {
            relevance: relevance * focus_weight,
            recency: recency_factor,
            energy,
            emotional,
        }
    }

    /// Combined salience (weighted sum).
    pub fn total(&self) -> f32 {
        self.relevance * 0.4 + self.recency * 0.2 + self.energy * 0.2 + self.emotional * 0.2
    }
}
