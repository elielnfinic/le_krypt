use blake2::Blake2b512;
use field_math::field::{field::Field, field_element::FieldElement};
use sha3::Digest;

use crate::proof_stream::ProofStream;
use crate::merkle::Merkle;


pub struct Fri<'a> {
    offset: FieldElement<'a>,
    omega: FieldElement<'a>,
    field: &'a Field,
    domain_length: usize,
    expansion_factor: usize,
    num_colinearity_tests: usize,
}

impl<'a> Fri<'a> {
    pub fn new(
        offset: FieldElement<'a>,
        omega: FieldElement<'a>,
        initial_domain_length: usize,
        expansion_factor: usize,
        num_colinearity_tests: usize,
    ) -> Self {
        Fri {
            offset,
            omega,
            field: omega.field,
            domain_length: initial_domain_length,
            expansion_factor,
            num_colinearity_tests,
        }
    }

    pub fn num_rounds(&self) -> usize {
        let mut codeword_length = self.domain_length;
        let mut num_rounds = 0;
        while codeword_length > self.expansion_factor && 4 * self.num_colinearity_tests < codeword_length {
            codeword_length /= 2;
            num_rounds += 1;
        }
        num_rounds
    }

    pub fn eval_domain(&self) -> Vec<FieldElement> {
        (0..self.domain_length)
            .map(|i| self.offset.clone() * self.omega.pow(i as u32))
            .collect()
    }

    pub fn prove(&self, codeword: Vec<FieldElement>, proof_stream: &mut ProofStream) -> Vec<usize> {
        assert_eq!(self.domain_length, codeword.len(), "initial codeword length does not match length of initial codeword");

        // commit phase
        let codewords = self.commit(codeword, proof_stream);

        // get indices
        let top_level_indices = self.sample_indices(proof_stream.prover_fiat_shamir(32), codewords[1].len(), codewords.last().unwrap().len(), self.num_colinearity_tests);
        let mut indices = top_level_indices.clone();

        // query phase
        for i in 0..codewords.len()-1 {
            indices = indices.iter().map(|index| index % (codewords[i].len() / 2)).collect(); // fold
            self.query(&codewords[i], &codewords[i+1], &indices, proof_stream);
        }

        top_level_indices
    }

    pub fn commit(&self, mut codeword: Vec<FieldElement<'a>>, proof_stream: &mut ProofStream) -> Vec<Vec<FieldElement>> {
        let one = self.field.one();
        let two = FieldElement::from(2, self.field);
        let mut omega = self.omega.clone();
        let mut offset = self.offset.clone();
        let mut codewords = Vec::new();

        for r in 0..self.num_rounds() {
            let root = Merkle::commit(&codeword.iter().map(|x| x.to_bytes()).collect::<Vec<_>>());
            proof_stream.push(root);

            if r == self.num_rounds() - 1 {
                break;
            }

            let alpha = self.field.sample(proof_stream.prover_fiat_shamir(32));

            codewords.push(codeword.clone());

            codeword = (0..codeword.len() / 2)
                .map(|i| {
                    // let i = i as i128;
                    let i_i128 = i as i128;
                    let i_u32: u32 = i.try_into().unwrap();
                    let omega_i = omega.pow(i_u32);
                    let alpha_omega_i = alpha / (offset.clone() * omega_i.clone());
                    let alpha_omega_i_inv = alpha_omega_i.clone().inverse();
                    let codeword_i = codeword[i].clone();
                    let codeword_i_plus = one.clone() + alpha_omega_i.clone();
                    let codeword_i_minus = one.clone() - alpha_omega_i.clone();
                    let codeword_i_2 = codeword[codeword.len() / 2 + i].clone();
                    let codeword_i_2_plus = codeword_i_plus.clone() * codeword_i.clone();
                    let codeword_i_2_minus = codeword_i_minus.clone() * codeword_i_2.clone();
                    let codeword_i_final = two.clone().inverse() * (codeword_i_2_plus + codeword_i_2_minus);
                    codeword_i_final
                })
                .collect();
            omega = omega.pow(2);
            offset = offset.pow(2);
        }

        proof_stream.push(codeword.clone());
        codewords.push(codeword);

        codewords
    }

    pub fn query(&self, current_codeword: &Vec<FieldElement>, next_codeword: &Vec<FieldElement>, c_indices: &Vec<usize>, proof_stream: &mut ProofStream) -> Vec<usize> {
        let a_indices = c_indices.clone();
        let b_indices: Vec<usize> = c_indices.iter().map(|index| index + current_codeword.len() / 2).collect();

        for s in 0..self.num_colinearity_tests {
            proof_stream.push((current_codeword[a_indices[s]].clone(), current_codeword[b_indices[s]].clone(), next_codeword[c_indices[s]].clone()));
        }

        for s in 0..self.num_colinearity_tests {
            proof_stream.push(Merkle::open(a_indices[s], &current_codeword.iter().map(|x| x.to_bytes()).collect::<Vec<_>>()));
            proof_stream.push(Merkle::open(b_indices[s], &current_codeword.iter().map(|x| x.to_bytes()).collect::<Vec<_>>()));
            proof_stream.push(Merkle::open(c_indices[s], &next_codeword.iter().map(|x| x.to_bytes()).collect::<Vec<_>>()));
        }

        a_indices.iter().chain(b_indices.iter()).cloned().collect()
    }

    pub fn sample_index(byte_array: Vec<u8>, size: usize) -> usize {
        let mut acc = 0;
        for b in byte_array {
            acc = (acc << 8) ^ b as usize;
        }
        acc % size
    }

    pub fn sample_indices(&self, seed: Vec<u8>, size: usize, reduced_size: usize, number: usize) -> Vec<usize> {
        assert!(number <= 2 * reduced_size, "not enough entropy in indices wrt last codeword");
        let error_msg = format!("cannot sample more indices than available in last codeword; requested: {}, available: {}", number, reduced_size);
        assert!(number <= reduced_size, &error_msg);

        let mut indices = Vec::new();
        let mut reduced_indices = Vec::new();
        let mut counter = 0;
        while indices.len() < number {
            let index = Fri::sample_index(Blake2b512::digest(&(seed.clone() + &counter.to_be_bytes().to_vec())), size);
            let reduced_index = index % reduced_size;
            counter += 1;
            if !reduced_indices.contains(&reduced_index) {
                indices.push(index);
                reduced_indices.push(reduced_index);
            }
        }

        indices
    }

    pub fn verify(&self, proof_stream: &mut ProofStream, polynomial_values: &mut Vec<(usize, FieldElement<'a>)>) -> bool {
        let mut omega = self.omega.clone();
        let mut offset = self.offset.clone();

        let mut roots = Vec::new();
        let mut alphas = Vec::new();

        for _ in 0..self.num_rounds() {
            roots.push(proof_stream.pull());
            alphas.push(self.field.sample(proof_stream.verifier_fiat_shamir(32)));
        }

        let last_codeword: Vec<FieldElement> = proof_stream.pull();

        if roots.last().unwrap() != &Merkle::commit(&last_codeword.iter().map(|x| x.to_bytes()).collect::<Vec<_>>()) {
            println!("last codeword is not well formed");
            return false;
        }

        let degree = (last_codeword.len() / self.expansion_factor) - 1;
        let mut last_omega = omega.clone();
        let mut last_offset = offset.clone();

        for _ in 0..(self.num_rounds() - 1) {
            last_omega = last_omega.pow(2);
            last_offset = last_offset.pow(2);
        }

        assert!(last_omega.clone().inverse() == last_omega.clone().pow((last_codeword.len() - 1) as u32), "omega does not have right order");

        let last_domain: Vec<FieldElement> = (0..last_codeword.len())
            .map(|i| last_offset.clone() * last_omega.clone().pow(i as u32))
            .collect();

        let poly = Polynomial::interpolate_domain(&last_domain, &last_codeword);

        assert!(poly.evaluate_domain(&last_domain) == last_codeword, "re-evaluated codeword does not match original!");

        if poly.degree() > degree {
            println!("last codeword does not correspond to polynomial of low enough degree");
            println!("observed degree: {}", poly.degree());
            println!("but should be: {}", degree);
            return false;
        }

        let top_level_indices = self.sample_indices(
            proof_stream.verifier_fiat_shamir(),
            self.domain_length >> 1,
            self.domain_length >> (self.num_rounds() - 1),
            self.num_colinearity_tests,
        );

        for r in 0..(self.num_rounds() - 1) {
            let c_indices: Vec<usize> = top_level_indices
                .iter()
                .map(|index| index % (self.domain_length >> (r + 1)))
                .collect();

            let a_indices: Vec<usize> = c_indices.clone();
            let b_indices: Vec<usize> = c_indices
                .iter()
                .map(|index| index + (self.domain_length >> (r + 1)))
                .collect();

            let mut aa = Vec::new();
            let mut bb = Vec::new();
            let mut cc = Vec::new();

            for s in 0..self.num_colinearity_tests {
                let (ay, by, cy) = proof_stream.pull();
                aa.push(ay);
                bb.push(by);
                cc.push(cy);

                if r == 0 {
                    polynomial_values.push((a_indices[s], ay.clone()));
                    polynomial_values.push((b_indices[s], by.clone()));
                }

                let ax = offset.clone() * omega.clone().pow(a_indices[s] as u32);
                let bx = offset.clone() * omega.clone().pow(b_indices[s] as u32);
                let cx = alphas[r].clone();

                if !test_colinearity(&[(ax, ay.clone()), (bx, by.clone()), (cx, cy.clone())]) {
                    println!("colinearity check failure");
                    return false;
                }
            }

            for i in 0..self.num_colinearity_tests {
                let path = proof_stream.pull();
                if !Merkle::verify(&roots[r], a_indices[i], path, aa[i].clone()) {
                    println!("merkle authentication path verification fails for aa");
                    return false;
                }
                let path = proof_stream.pull();
                if !Merkle::verify(&roots[r], b_indices[i], path, bb[i].clone()) {
                    println!("merkle authentication path verification fails for bb");
                    return false;
                }
                let path = proof_stream.pull();
                if !Merkle::verify(&roots[r + 1], c_indices[i], path, cc[i].clone()) {
                    println!("merkle authentication path verification fails for cc");
                    return false;
                }
            }

            omega = omega.pow(2);
            offset = offset.pow(2);
        }

        true
    }
}