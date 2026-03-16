//! # AQL — Agent Query Language
//!
//! A cognitive intent language for agents interacting with knowledge graphs
//! and vector databases. AQL models how an agent *uses* memory, not how to query it.
//!
//! ## The Three Axioms
//!
//! 1. **Intention as Primitive** — The atomic unit is a cognitive act, not a DB instruction.
//! 2. **Uncertainty as Data Type** — `CONFIDENCE 0.7` is epistemic, not a filter.
//! 3. **Effects as Automatic Consequences** — The agent declares intent; the server applies side-effects.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use aql_core::{parser, planner::{CognitivePlanner, PlannerConfig}};
//!
//! let program = parser::parse(r#"RECALL "quantum physics" CONFIDENCE 0.8"#).unwrap();
//! let mut planner = CognitivePlanner::new(PlannerConfig::default());
//! let plans = planner.plan_program(&program).unwrap();
//! ```
//!
//! ## Architecture
//!
//! ```text
//! AQL source → [Parser] → AST → [CognitivePlanner] → ExecutionPlan → [Executor] → Backend → CognitiveResult
//! ```

pub const VERSION: &str = "2.0.0";
pub const PROTOCOL_VERSION: u8 = 2;

pub mod types;
pub mod ast;
pub mod error;
pub mod capabilities;
pub mod parser;
pub mod plans;
pub mod planner;
pub mod traits;
pub mod result;
pub mod executor;
pub mod memory;

// Re-exports for convenience
pub use ast::{Program, Statement, Verb, Subject, Qualifier};
pub use error::{AqlError, AqlResult};
pub use types::EpistemicType;
pub use traits::AqlBackend;
pub use capabilities::BackendCapabilities;
pub use result::CognitiveResult;
pub use executor::AqlExecutor;
pub use planner::CognitivePlanner;
pub use plans::ExecutionPlan;
