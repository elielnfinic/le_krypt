//! # El Stark - STARK Zero-Knowledge Proof Library
//!
//! This library provides a complete implementation of STARKs (Scalable Transparent Arguments of Knowledge),
//! a powerful zero-knowledge proof system for computational integrity.
//!
//! ## Features
//!
//! - **FRI (Fast Reed-Solomon Interactive Oracle Proof)**: Low-degree testing protocol
//! - **STARK Prover/Verifier**: Complete zero-knowledge proof system
//! - **AIR (Algebraic Intermediate Representation)**: Constraint system for computations
//! - **Merkle Trees**: Cryptographic commitments for proof authentication
//! - **Field Arithmetic**: Finite field operations over large prime fields
//!
//! ## Example Usage
//!
//! ```rust
//! use el_stark::{Stark, ComputationTrace, Air};
//! use field_math::field::field::Field;
//!
//! // Create a STARK instance
//! let field = Field::new();
//! let stark = Stark::new(&field, 4, 16, 128);
//!
//! // Create a computation trace (e.g., Fibonacci sequence)
//! let trace = ComputationTrace::fibonacci_trace(&field, 8);
//! let air = Air::fibonacci_air(&field);
//!
//! // Generate a proof
//! let proof = stark.prove(&trace, &air).expect("Proof generation failed");
//!
//! // Verify the proof
//! let public_inputs = vec![/* initial values */];
//! let is_valid = stark.verify(&proof, &air, &public_inputs)
//!     .expect("Verification failed");
//! ```

mod proof_stream;
mod merkle;
mod fri;
mod stark;

// Re-export the main types for public API
pub use stark::{Stark, ComputationTrace, Air};
pub use fri::Fri;
pub use proof_stream::ProofStream;
pub use merkle::Merkle;

// Re-export field math types for convenience
pub use field_math::field::{field::Field, field_element::FieldElement};
pub use field_math::poly::uni::Uni;