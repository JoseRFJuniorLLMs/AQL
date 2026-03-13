//! AQL MySQL/MariaDB Backend — relational with FULLTEXT search.
//! Supports RECALL (FULLTEXT), IMPRINT (INSERT), ASSOCIATE (JOIN table),
//! TRACE (recursive CTE - MySQL 8+), FADE (DELETE/UPDATE).
pub mod backend;
pub mod lowering;
pub use backend::MysqlBackend;
