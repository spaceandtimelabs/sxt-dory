//! # Dory
//!
//! Dory is a polynomial commitment scheme with excellent asymptotic performance as well as
//! practical efficiency. It is based on the work of Jonathan Lee (<https://eprint.iacr.org/2020/1274.pdf>).
//!
//! This crate provides a Rust implementation of the commitment scheme, intended to be usable as a
//! building block for other zk/SNARK protocols.

/// Module containing the [`ProofBuilder`] trait.
mod builder;
/// Module containing the [`inner_product_prove`] function.
mod inner_product;
/// Module containing structs of the messages exchanged between the prover and verifier.
///
/// This consists of two messages each during the Dory-Reduce phase,
/// the verifier challenge during the Fold-Scalars phase,
/// and the final prover message during the Scalar-Product phase.
pub mod messages;
/// Module containing the [`ProverState`] trait.
mod state;

pub use builder::ProofBuilder;
pub use inner_product::inner_product_prove;
pub use state::ProverState;
