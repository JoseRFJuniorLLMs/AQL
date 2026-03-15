//! AQL NietzscheDB Backend — full cognitive experience.
//! Implements all 13 verbs with native hyperbolic geometry,
//! dream cycles, energy model, and L-System integration.
//!
//! Uses gRPC `ExecuteAql` RPC to send AQL statements to NietzscheDB server.

pub mod backend;
pub mod hyperbolic;
pub mod dream;
pub mod lowering;

/// Generated gRPC client stubs for NietzscheDB proto.
pub mod proto {
    tonic::include_proto!("nietzsche");
}

pub use backend::NietzscheBackend;
