//! Reactive system: WATCH and SUBSCRIBE.
//! Enables event-driven cognitive operations.

use crate::types::WatchTrigger;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;

/// A registered subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub subject_query: String,
    pub trigger: WatchTrigger,
    pub active: bool,
}

/// Event emitted when a watched subject changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchEvent {
    pub subscription_id: String,
    pub trigger: WatchTrigger,
    pub subject: String,
    pub details: HashMap<String, serde_json::Value>,
}

/// Reactive event bus for WATCH/SUBSCRIBE.
pub struct ReactiveHub {
    subscriptions: HashMap<String, Subscription>,
    sender: broadcast::Sender<WatchEvent>,
}

impl ReactiveHub {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self {
            subscriptions: HashMap::new(),
            sender,
        }
    }

    pub fn subscribe(
        &mut self,
        id: String,
        subject_query: String,
        trigger: WatchTrigger,
    ) -> broadcast::Receiver<WatchEvent> {
        self.subscriptions.insert(
            id.clone(),
            Subscription {
                id,
                subject_query,
                trigger,
                active: true,
            },
        );
        self.sender.subscribe()
    }

    pub fn unsubscribe(&mut self, id: &str) {
        if let Some(sub) = self.subscriptions.get_mut(id) {
            sub.active = false;
        }
    }

    pub fn emit(&self, event: WatchEvent) -> usize {
        self.sender.send(event).unwrap_or(0)
    }

    pub fn active_subscriptions(&self) -> Vec<&Subscription> {
        self.subscriptions.values().filter(|s| s.active).collect()
    }
}

impl Default for ReactiveHub {
    fn default() -> Self {
        Self::new(256)
    }
}
