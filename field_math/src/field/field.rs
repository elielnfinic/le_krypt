use super::{xgcd, field_element::FieldElement};
use serde::{Serialize, Deserialize};
use super::DEFAULT_PRIME;


#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct Field {
    pub p: i128,
}

impl Field {
    pub fn new() -> Field {
        Field { p: DEFAULT_PRIME }
    }

    pub fn from(p: i128) -> Field {
        Field { p }
    }

    pub fn zero(&self) -> FieldElement {
        FieldElement::from(0, self)
    }

    pub fn one(&self) -> FieldElement {
        FieldElement::from(1, self)
    }

    pub fn add(&self, a: FieldElement, b: FieldElement) -> FieldElement {
        FieldElement::from((a.value + b.value) % self.p, self)
    }

    pub fn sub(&self, a: FieldElement, b: FieldElement) -> FieldElement {
        FieldElement::from((a.value + self.p - b.value) % self.p, self)
    }

    pub fn mul(&self, a: FieldElement, b: FieldElement) -> FieldElement {
        FieldElement::from((a.value * b.value) % self.p, self)
    }

    pub fn negate(&self, a: FieldElement) -> FieldElement {
        FieldElement::from((self.p - a.value) % self.p, self)
    }

    pub fn inverse(&self, a: FieldElement) -> FieldElement {
        let (g, x, _) = xgcd(a.value, self.p);
        if g != 1 {
            panic!("{} is not invertible", a.value);
        }
        FieldElement::from((x + self.p) % self.p, self)
    }

    pub fn div(&self, a: FieldElement, b: FieldElement) -> FieldElement {
        let (g, x, _x) = xgcd(b.value, self.p);
        if g != 1 {
            panic!("{} is not invertible", b.value);
        }
        FieldElement::from((a.value * x) % self.p, self)
    }

    pub fn pow(&self, a: i128, b: i128) -> FieldElement {
        FieldElement::from(a.pow(b as u32) % self.p, self)
    }

    pub fn pow_i32(&self, a : FieldElement, b : i32) -> FieldElement {
        FieldElement::from(a.value.pow(b as u32) % self.p, self)
    }

    pub fn primitive_nth_root(&self, n: i128) -> FieldElement {
        if self.p == DEFAULT_PRIME {
            assert!(n <= 1 << 64 && (n & (n - 1)) == 0, "Field does not have nth root of unity where n > 2^119 or not power of two.");
            let mut root = FieldElement::from(DEFAULT_PRIME, self);
            let mut order = 1 << 64;
            while order != n {
                root = self.pow(root.value, 2);
                order = order / 2;
            }
            return root;
        } else {
            panic!("Unknown field, can't return root of unity.");
        }
    }

    pub fn sample(&self, byte_array: Vec<u8>) -> FieldElement {
        let mut acc = 0;
        for b in byte_array {
            acc = (acc << 8) ^ b as i128;
        }
        FieldElement::from(acc % self.p, self)
    }
}

impl Default for Field {
    fn default() -> Field {
        Field::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::field_element::FieldElement;

    #[test]
    fn test_default_field() {
        let field : Field = Default::default();
        assert_eq!(field.p, DEFAULT_PRIME);
    }
}