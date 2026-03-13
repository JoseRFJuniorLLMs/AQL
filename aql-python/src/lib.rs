//! AQL Python Bindings via PyO3.
//! Exposes parse, plan, and type system to Python.

use pyo3::prelude::*;
use aql_core::{parser, planner::{CognitivePlanner, PlannerConfig}};

/// Parse AQL source text and return JSON AST.
#[pyfunction]
fn parse_aql(input: &str) -> PyResult<String> {
    let program = parser::parse(input)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
    serde_json::to_string_pretty(&program)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e}")))
}

/// Parse and plan AQL, return JSON execution plans.
#[pyfunction]
fn plan_aql(input: &str) -> PyResult<String> {
    let program = parser::parse(input)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
    let mut planner = CognitivePlanner::new(PlannerConfig::default());
    let plans = planner.plan_program(&program)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e}")))?;
    serde_json::to_string_pretty(&plans)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e}")))
}

/// Get AQL version.
#[pyfunction]
fn version() -> String {
    aql_core::VERSION.to_string()
}

/// List all cognitive verbs.
#[pyfunction]
fn verbs() -> Vec<String> {
    vec![
        "RECALL", "RESONATE", "REFLECT", "TRACE",
        "IMPRINT", "ASSOCIATE", "DISTILL", "FADE",
        "DESCEND", "ASCEND", "ORBIT",
        "DREAM", "IMAGINE",
    ].into_iter().map(String::from).collect()
}

/// List all epistemic types.
#[pyfunction]
fn epistemic_types() -> Vec<String> {
    vec![
        "Belief", "Experience", "Pattern", "Signal", "Intention",
    ].into_iter().map(String::from).collect()
}

#[pymodule]
fn aql(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_aql, m)?)?;
    m.add_function(wrap_pyfunction!(plan_aql, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(verbs, m)?)?;
    m.add_function(wrap_pyfunction!(epistemic_types, m)?)?;
    Ok(())
}
