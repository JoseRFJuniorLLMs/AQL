//! Multi-agent support: SHARE, DELEGATE, NEGOTIATE.
//! Enables cognitive cooperation between multiple agents.

use crate::types::ConflictPolicy;
use crate::result::CognitiveResult;
use serde::{Deserialize, Serialize};

/// Agent identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentId {
    pub name: String,
    pub collection: Option<String>,
}

/// A shared knowledge item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedKnowledge {
    pub from_agent: String,
    pub to_agent: String,
    pub content: String,
    pub confidence: f32,
    pub policy: ConflictPolicy,
}

/// Delegation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationRequest {
    pub from_agent: String,
    pub to_agent: String,
    pub task: String,
    pub timeout_ms: u64,
}

/// Negotiation between agents when knowledge conflicts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiationResult {
    pub agents: Vec<String>,
    pub topic: String,
    pub resolution: ConflictResolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    Merged {
        confidence: f32,
        content: String,
    },
    HigherWins {
        winner: String,
        confidence: f32,
    },
    Replaced {
        by: String,
        content: String,
    },
    ConflictRecorded {
        positions: Vec<(String, String, f32)>, // (agent, content, confidence)
    },
}

/// Multi-agent coordinator.
pub struct MultiAgentCoordinator {
    pub self_agent: AgentId,
    pub known_agents: Vec<AgentId>,
}

impl MultiAgentCoordinator {
    pub fn new(self_agent: AgentId) -> Self {
        Self {
            self_agent,
            known_agents: vec![],
        }
    }

    pub fn register_agent(&mut self, agent: AgentId) {
        self.known_agents.push(agent);
    }

    pub fn resolve_conflict(
        &self,
        policy: ConflictPolicy,
        existing: (&str, f32),  // (content, confidence)
        incoming: (&str, f32),
    ) -> ConflictResolution {
        match policy {
            ConflictPolicy::WeightedAverage => {
                let total = existing.1 + incoming.1;
                let blended_confidence = if total > 0.0 { total / 2.0 } else { 0.0 };
                ConflictResolution::Merged {
                    confidence: blended_confidence,
                    content: format!("{} | {}", existing.0, incoming.0),
                }
            }
            ConflictPolicy::KeepHigher => {
                if existing.1 >= incoming.1 {
                    ConflictResolution::HigherWins {
                        winner: self.self_agent.name.clone(),
                        confidence: existing.1,
                    }
                } else {
                    ConflictResolution::HigherWins {
                        winner: "incoming".into(),
                        confidence: incoming.1,
                    }
                }
            }
            ConflictPolicy::ReplaceAlways => ConflictResolution::Replaced {
                by: "incoming".into(),
                content: incoming.0.to_string(),
            },
            ConflictPolicy::CreateConflict => ConflictResolution::ConflictRecorded {
                positions: vec![
                    (self.self_agent.name.clone(), existing.0.to_string(), existing.1),
                    ("incoming".to_string(), incoming.0.to_string(), incoming.1),
                ],
            },
        }
    }
}
