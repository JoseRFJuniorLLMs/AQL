//! WorkingMemory — state between THEN steps and parallel branches.

use crate::result::{CognitiveResult, Explanation, WatchHandle};
use std::collections::HashMap;

/// Working memory holds results between chain steps.
#[derive(Debug, Default)]
pub struct WorkingMemory {
    /// Stack of results (most recent first).
    results: Vec<CognitiveResult>,
    /// Indexed results from parallel branches.
    indexed: HashMap<usize, CognitiveResult>,
    /// Last DREAM result.
    last_dream: Option<CognitiveResult>,
    /// Last DELEGATE result.
    delegate_result: Option<CognitiveResult>,
    /// Last EXPLAIN.
    explanation: Option<Explanation>,
    /// Active WATCH handles.
    watch_handles: Vec<WatchHandle>,
    /// Session-level metadata.
    session: HashMap<String, serde_json::Value>,
}

impl WorkingMemory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_result(&mut self, result: CognitiveResult) {
        self.results.push(result);
    }

    pub fn last_result(&self) -> Option<&CognitiveResult> {
        self.results.last()
    }

    pub fn all_results(&self) -> &[CognitiveResult] {
        &self.results
    }

    pub fn set_indexed_result(&mut self, index: usize, result: CognitiveResult) {
        self.indexed.insert(index, result);
    }

    pub fn get_indexed_result(&self, index: usize) -> Option<&CognitiveResult> {
        self.indexed.get(&index)
    }

    pub fn set_dream_result(&mut self, result: CognitiveResult) {
        self.last_dream = Some(result);
    }

    pub fn last_dream(&self) -> Option<&CognitiveResult> {
        self.last_dream.as_ref()
    }

    pub fn set_delegate_result(&mut self, result: CognitiveResult) {
        self.delegate_result = Some(result);
    }

    pub fn delegate_result(&self) -> Option<&CognitiveResult> {
        self.delegate_result.as_ref()
    }

    pub fn set_explanation(&mut self, explanation: Explanation) {
        self.explanation = Some(explanation);
    }

    pub fn explanation(&self) -> Option<&Explanation> {
        self.explanation.as_ref()
    }

    pub fn set_watch_handle(&mut self, handle: WatchHandle) {
        self.watch_handles.push(handle);
    }

    pub fn watch_handles(&self) -> &[WatchHandle] {
        &self.watch_handles
    }

    pub fn set_session(&mut self, key: String, value: serde_json::Value) {
        self.session.insert(key, value);
    }

    pub fn get_session(&self, key: &str) -> Option<&serde_json::Value> {
        self.session.get(key)
    }

    pub fn clear(&mut self) {
        self.results.clear();
        self.indexed.clear();
        self.last_dream = None;
        self.delegate_result = None;
        self.explanation = None;
    }
}
