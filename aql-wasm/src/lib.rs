//! AQL WASM — Browser/Edge compilation.
//! Exposes parse + plan to JavaScript via wasm-bindgen.

use wasm_bindgen::prelude::*;
use aql_core::{parser, planner::{CognitivePlanner, PlannerConfig}};

/// Parse AQL source and return JSON AST.
#[wasm_bindgen]
pub fn parse_aql(input: &str) -> Result<String, JsValue> {
    let program = parser::parse(input)
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    serde_json::to_string_pretty(&program)
        .map_err(|e| JsValue::from_str(&format!("{e}")))
}

/// Parse and plan AQL, return JSON execution plans.
#[wasm_bindgen]
pub fn plan_aql(input: &str) -> Result<String, JsValue> {
    let program = parser::parse(input)
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    let mut planner = CognitivePlanner::new(PlannerConfig::default());
    let plans = planner.plan_program(&program)
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    serde_json::to_string_pretty(&plans)
        .map_err(|e| JsValue::from_str(&format!("{e}")))
}

/// Get AQL version.
#[wasm_bindgen]
pub fn aql_version() -> String {
    aql_core::VERSION.to_string()
}
