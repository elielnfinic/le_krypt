mod proof_stream;
mod merkle;
mod fri;
mod stark;

pub use stark::{Stark, ComputationTrace, Air};
pub use fri::Fri;
pub use proof_stream::ProofStream;
pub use merkle::Merkle;

fn main() {
    println!("STARK Library - Zero Knowledge Proof System");
    
    // Demonstrate basic STARK functionality
    use field_math::field::field::Field;
    
    let field = Field::new();
    let stark = Stark::new(&field, 4, 16, 128);
    
    println!("Creating Fibonacci computation trace...");
    let trace = ComputationTrace::fibonacci_trace(&field, 8);
    let air = Air::fibonacci_air(&field);
    
    println!("Generating STARK proof...");
    match stark.prove(&trace, &air) {
        Ok(proof) => {
            println!("✓ Proof generated successfully ({} bytes)", proof.len());
            
            let public_inputs = vec![
                field_math::field::field_element::FieldElement::from(1, &field),
                field_math::field::field_element::FieldElement::from(1, &field),
            ];
            
            println!("Verifying STARK proof...");
            match stark.verify(&proof, &air, &public_inputs) {
                Ok(true) => println!("✓ Proof verified successfully!"),
                Ok(false) => println!("✗ Proof verification failed"),
                Err(e) => println!("✗ Verification error: {}", e),
            }
        }
        Err(e) => println!("✗ Proof generation failed: {}", e),
    }
}
