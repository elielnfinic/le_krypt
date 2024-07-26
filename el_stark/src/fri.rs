use field_math::field::{field::Field, field_element::FieldElement};


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
}