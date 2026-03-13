//! Cognitive Energy Model — hooks for energy management.
//! Energy represents the "activation level" of knowledge nodes.

use crate::types::EpistemicType;

/// Energy configuration.
#[derive(Debug, Clone)]
pub struct EnergyConfig {
    pub min_energy: f32,
    pub max_energy: f32,
    pub delete_threshold: f32,
    pub global_decay_rate: f64,
}

impl Default for EnergyConfig {
    fn default() -> Self {
        Self {
            min_energy: 0.0,
            max_energy: 1.0,
            delete_threshold: 0.05,
            global_decay_rate: 1.0,
        }
    }
}

/// Energy hooks applied before/after operations.
pub struct EnergyHooks {
    config: EnergyConfig,
}

impl EnergyHooks {
    pub fn new(config: EnergyConfig) -> Self {
        Self { config }
    }

    /// Calculate energy boost for an access event.
    pub fn access_boost(&self, etype: EpistemicType) -> f32 {
        etype.access_energy_boost()
    }

    /// Calculate energy after natural decay over time.
    pub fn decay(&self, current: f32, etype: EpistemicType, elapsed_secs: f64) -> f32 {
        let rate = etype.decay_rate() * self.config.global_decay_rate;
        let decayed = current * (-rate * elapsed_secs).exp() as f32;
        decayed.max(self.config.min_energy)
    }

    /// Check if a node should be deleted (energy below threshold).
    pub fn should_delete(&self, energy: f32) -> bool {
        energy < self.config.delete_threshold
    }

    /// Calculate initial energy for a new node.
    pub fn initial_energy(&self, etype: EpistemicType, confidence: Option<f32>) -> f32 {
        confidence.unwrap_or_else(|| etype.initial_energy())
            .min(self.config.max_energy)
    }

    /// Apply FADE energy decrement.
    pub fn fade_energy(&self, current: f32, decrement: f32) -> f32 {
        (current - decrement).max(self.config.min_energy)
    }
}
