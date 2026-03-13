//! Dream cycle integration with NietzscheDB.
//! DREAM activates creative non-deterministic exploration.

/// Dream cycle parameters.
pub struct DreamParams {
    pub topic: String,
    pub depth: u8,
    pub novelty_bias: f32,
    pub max_new_patterns: usize,
}

impl Default for DreamParams {
    fn default() -> Self {
        Self {
            topic: String::new(),
            depth: 3,
            novelty_bias: 0.7,
            max_new_patterns: 5,
        }
    }
}

/// Dream cycle result.
pub struct DreamOutput {
    pub new_patterns: Vec<String>,
    pub reinforced_edges: usize,
    pub activation_spread: usize,
}
