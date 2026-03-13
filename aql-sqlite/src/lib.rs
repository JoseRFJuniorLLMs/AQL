//! AQL SQLite Backend — embedded, zero-config, with FTS5 full-text search.
//! Perfect for local agents, edge devices, and testing.
pub mod backend;
pub mod lowering;
pub use backend::SqliteBackend;
