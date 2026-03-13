//! Error types for AQL — Agent Cognition Language

use crate::ast::Verb;
use thiserror::Error;

/// All possible AQL errors.
#[derive(Debug, Error)]
pub enum AqlError {
    #[error("parse error at line {line}, col {col}: {message}")]
    Parse {
        line: usize,
        col: usize,
        message: String,
    },

    #[error("unsupported verb {verb}: {reason}")]
    UnsupportedVerb { verb: String, reason: String },

    #[error("backend error: {0}")]
    Backend(String),

    #[error("planning error for {verb}: {reason}")]
    Planning { verb: Verb, reason: String },

    #[error("execution error: {0}")]
    Execution(String),

    #[error("energy insufficient: need {needed}, have {available}")]
    InsufficientEnergy { needed: f32, available: f32 },

    #[error("timeout after {elapsed_ms}ms")]
    Timeout { elapsed_ms: u64 },

    #[error("atomic transaction failed: {reason}")]
    AtomicFailed { reason: String },

    #[error("condition evaluation error: {0}")]
    ConditionEval(String),

    #[error("agent {agent} not found")]
    AgentNotFound { agent: String },

    #[error("watch registration failed: {0}")]
    WatchFailed(String),

    #[error("self reference unresolved: no active collection context")]
    SelfUnresolved,

    #[error("invalid qualifier: {0}")]
    InvalidQualifier(String),

    #[error("feature not available: {feature} (requires {requires})")]
    FeatureUnavailable { feature: String, requires: String },
}

/// Error codes for protocol responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AqlErrorCode {
    ParseError = 1000,
    UnsupportedVerb = 2000,
    BackendError = 3000,
    PlanningError = 4000,
    ExecutionError = 5000,
    InsufficientEnergy = 6000,
    Timeout = 7000,
    AtomicFailed = 8000,
    ConditionError = 9000,
    AgentNotFound = 10000,
    WatchFailed = 11000,
    SelfUnresolved = 12000,
    InvalidQualifier = 13000,
    FeatureUnavailable = 14000,
}

impl AqlError {
    pub fn code(&self) -> AqlErrorCode {
        match self {
            Self::Parse { .. } => AqlErrorCode::ParseError,
            Self::UnsupportedVerb { .. } => AqlErrorCode::UnsupportedVerb,
            Self::Backend(_) => AqlErrorCode::BackendError,
            Self::Planning { .. } => AqlErrorCode::PlanningError,
            Self::Execution(_) => AqlErrorCode::ExecutionError,
            Self::InsufficientEnergy { .. } => AqlErrorCode::InsufficientEnergy,
            Self::Timeout { .. } => AqlErrorCode::Timeout,
            Self::AtomicFailed { .. } => AqlErrorCode::AtomicFailed,
            Self::ConditionEval(_) => AqlErrorCode::ConditionError,
            Self::AgentNotFound { .. } => AqlErrorCode::AgentNotFound,
            Self::WatchFailed(_) => AqlErrorCode::WatchFailed,
            Self::SelfUnresolved => AqlErrorCode::SelfUnresolved,
            Self::InvalidQualifier(_) => AqlErrorCode::InvalidQualifier,
            Self::FeatureUnavailable { .. } => AqlErrorCode::FeatureUnavailable,
        }
    }
}

pub type AqlResult<T> = Result<T, AqlError>;
