//! Affective dimensions — Valence, Arousal, and Mood.
//! Models the emotional context of cognitive operations.

use crate::types::*;

/// Affective state of the agent.
#[derive(Debug, Clone)]
pub struct AffectiveState {
    pub valence: f32,  // -1.0 (negative) to 1.0 (positive)
    pub arousal: f32,  // 0.0 (calm) to 1.0 (excited)
    pub mood: Option<MoodState>,
}

impl Default for AffectiveState {
    fn default() -> Self {
        Self {
            valence: 0.0,
            arousal: 0.5,
            mood: None,
        }
    }
}

impl AffectiveState {
    /// Create from epistemic type defaults.
    pub fn from_type(etype: EpistemicType) -> Self {
        Self {
            valence: etype.default_valence(),
            arousal: etype.default_arousal(),
            mood: None,
        }
    }

    /// Apply valence qualifier.
    pub fn with_valence(mut self, spec: &ValenceSpec) -> Self {
        self.valence = match spec {
            ValenceSpec::Positive => 0.7,
            ValenceSpec::Negative => -0.7,
            ValenceSpec::Neutral => 0.0,
            ValenceSpec::Exact(v) => v.clamp(-1.0, 1.0),
        };
        self
    }

    /// Apply arousal qualifier.
    pub fn with_arousal(mut self, spec: &ArousalSpec) -> Self {
        self.arousal = match spec {
            ArousalSpec::High => 0.9,
            ArousalSpec::Medium => 0.5,
            ArousalSpec::Low => 0.2,
            ArousalSpec::Calm => 0.1,
            ArousalSpec::Exact(a) => a.clamp(0.0, 1.0),
        };
        self
    }

    /// Set mood.
    pub fn with_mood(mut self, mood: MoodState) -> Self {
        self.mood = Some(mood);
        self
    }

    /// Emotional intensity (magnitude of affect).
    pub fn intensity(&self) -> f32 {
        (self.valence.abs() + self.arousal) / 2.0
    }

    /// Is this a positive emotional state?
    pub fn is_positive(&self) -> bool {
        self.valence > 0.1
    }

    /// Is this a high-arousal state?
    pub fn is_activated(&self) -> bool {
        self.arousal > 0.6
    }
}

/// Filter nodes by affective criteria.
pub fn matches_valence(node_valence: f32, spec: &ValenceSpec) -> bool {
    match spec {
        ValenceSpec::Positive => node_valence > 0.1,
        ValenceSpec::Negative => node_valence < -0.1,
        ValenceSpec::Neutral => node_valence.abs() <= 0.1,
        ValenceSpec::Exact(target) => (node_valence - target).abs() < 0.2,
    }
}

pub fn matches_arousal(node_arousal: f32, spec: &ArousalSpec) -> bool {
    match spec {
        ArousalSpec::High => node_arousal > 0.7,
        ArousalSpec::Medium => node_arousal > 0.3 && node_arousal <= 0.7,
        ArousalSpec::Low => node_arousal > 0.1 && node_arousal <= 0.3,
        ArousalSpec::Calm => node_arousal <= 0.1,
        ArousalSpec::Exact(target) => (node_arousal - target).abs() < 0.15,
    }
}
