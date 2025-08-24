# le_krypt

This is a messy place where I put the libraries I create as part of my research in zk, zkML, quantum computing, and bioinformatics. If you find any disorder, I apologize, but I try to keep the journal of this workspace up to date. Contact me to discuss at elielmathe (WrapCast, TG, and X).

Feel free to let me know if you’d like any adjustments!  

# Libraries 

## ✅ el_stark - STARK Zero-Knowledge Proof Library

A complete implementation of STARKs (Scalable Transparent Arguments of Knowledge) in Rust, providing a powerful zero-knowledge proof system for computational integrity.

### 🚀 Features

- **Complete STARK Implementation**: Full prover and verifier for zero-knowledge proofs
- **FRI Protocol**: Fast Reed-Solomon Interactive Oracle Proof for low-degree testing
- **AIR Constraints**: Algebraic Intermediate Representation for computation verification
- **Merkle Tree Commitments**: Cryptographic authentication for proof components
- **Field Arithmetic**: Efficient finite field operations over large prime fields
- **Computation Traces**: Structured execution traces for proof generation

### 📖 Quick Start

```rust
use el_stark::{Stark, ComputationTrace, Air};
use field_math::field::field::Field;

// Initialize STARK system
let field = Field::new();
let stark = Stark::new(&field, 4, 16, 128);

// Create computation trace (Fibonacci example)
let trace = ComputationTrace::fibonacci_trace(&field, 8);
let air = Air::fibonacci_air(&field);

// Generate and verify proof
let proof = stark.prove(&trace, &air)?;
let public_inputs = vec![/* initial values */];
let is_valid = stark.verify(&proof, &air, &public_inputs)?;
```

### 🛠 Usage

```bash
# Build the project
cargo build

# Run the STARK demo
cargo run -p el_stark --bin el_stark

# Run tests
cargo test -p el_stark
```

### 🏗 Architecture

- **STARK Prover/Verifier**: Main interface for proof generation and verification
- **FRI Protocol**: Low-degree testing for polynomial verification  
- **Computation Traces**: Structured execution trace representation
- **AIR Constraints**: Algebraic constraint system definitions
- **Merkle Trees**: Cryptographic commitments with Blake2 hashing
- **Proof Streams**: Serialization and Fiat-Shamir challenge generation

### 📊 Performance

- **Proof Generation**: O(n log n) where n is trace length
- **Proof Size**: O(log² n)
- **Verification Time**: O(log² n)
- **Security**: 80-128 bit security levels supported
