use std::collections::HashMap;
use std::ops::{Add, Mul, Neg, Sub};
use crate::field::field_element::FieldElement;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Exponents(pub Vec<i32>);

#[derive(Debug, Clone)]
pub struct MPolynomial<'a> {
    pub dictionary: HashMap<Exponents, FieldElement<'a>>,
}

impl<'a> MPolynomial<'a> {
    pub fn new(dictionary: HashMap<Exponents, FieldElement<'a>>) -> Self {
        MPolynomial { dictionary }
    }

    pub fn zero() -> Self {
        MPolynomial {
            dictionary: HashMap::new(),
        }
    }

    fn constant(element: FieldElement<'a>) -> Self {
        let mut dictionary = HashMap::new();
        dictionary.insert(Exponents(vec![0]), element);
        MPolynomial { dictionary }
    }

    fn is_zero(&self) -> bool {
        if self.dictionary.is_empty() {
            return true;
        }
        for v in self.dictionary.values() {
            if !v.is_zero() {
                return false;
            }
        }
        true
    }

    fn variables(num_variables: usize, one: FieldElement<'a>) -> Vec<Self> {
        let mut variables = Vec::new();
        for i in 0..num_variables {
            let mut exponent = vec![0; num_variables];
            exponent[i] = 1;
            let mut dictionary = HashMap::new();
            dictionary.insert(Exponents(exponent), one.clone());
            variables.push(MPolynomial { dictionary });
        }
        variables
    }

    fn pad_exponents(&self, exponents: &Exponents, num_variables: usize) -> Exponents {
        let mut pad = exponents.0.clone();
        pad.resize(num_variables, 0);
        Exponents(pad)
    }

    fn evaluate(&self, values: &[FieldElement<'a>]) -> FieldElement<'a> {
        let field = values[0].field;    
        let mut result = FieldElement::from(0, values[0].field);
        for (exponents, coefficient) in self.dictionary.iter() {
            let mut term = coefficient.clone();
            for (i, &e) in exponents.0.iter().enumerate() {
                term = term * values[i].clone() ^ FieldElement::from(e as i128, field );
            }
            result = result + term;
        }
        result
    }

    fn evaluate_symbolic(&self, values: &[FieldElement<'a>]) -> Self {
        let mut dictionary = HashMap::new();
        for (exponents, coefficient) in self.dictionary.iter() {
            let mut term = coefficient.clone();
            for (i, &e) in exponents.0.iter().enumerate() {
                term = term * (values[i].clone() ^ FieldElement::from(e as i128, values[i].field));
            }
            dictionary.insert(exponents.clone(), term);
        }
        MPolynomial { dictionary }
    }
}

impl<'a> Add for MPolynomial<'a> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut dictionary = HashMap::new();
        let num_variables = self.dictionary.keys().map(|k| k.0.len()).max().unwrap_or(0)
            .max(other.dictionary.keys().map(|k| k.0.len()).max().unwrap_or(0));

        for (k, v) in self.clone().dictionary {
            let pad = self.pad_exponents(&k, num_variables);
            dictionary.insert(pad, v);
        }

        for (k, v) in other.clone().dictionary {
            let pad = other.pad_exponents(&k, num_variables);
            dictionary.entry(pad).and_modify(|e| *e = e.clone() + v.clone()).or_insert(v);
        }

        MPolynomial { dictionary }
    }
}

impl<'a> Mul for MPolynomial<'a> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut dictionary = HashMap::new();
        let num_variables = self.dictionary.keys().map(|k| k.0.len()).max().unwrap_or(0)
            .max(other.dictionary.keys().map(|k| k.0.len()).max().unwrap_or(0));

        for (k0, v0) in self.dictionary.iter() {
            for (k1, v1) in other.dictionary.iter() {
                let mut exponent = vec![0; num_variables];
                for (i, &e) in k0.0.iter().enumerate() {
                    exponent[i] += e;
                }
                for (i, &e) in k1.0.iter().enumerate() {
                    exponent[i] += e;
                }
                let exponent = Exponents(exponent);
                dictionary.entry(exponent).and_modify(|e: &mut FieldElement<'a>| *e = e.clone() + v0.clone() * v1.clone()).or_insert(v0.clone() * v1.clone());
            }
        }

        MPolynomial { dictionary }
    }
}

impl<'a> Neg for MPolynomial<'a> {
    type Output = Self;

    fn neg(self) -> Self {
        let mut dictionary = HashMap::new();
        for (k, v) in self.dictionary {
            dictionary.insert(k, -v);
        }
        MPolynomial { dictionary }
    }
}

impl<'a> Sub for MPolynomial<'a> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl<'a> MPolynomial<'a> {
    fn pow(&self, exponent: u32) -> Self {
        if self.is_zero() {
            return MPolynomial::zero();
        }

        let one = self.dictionary.values().next().cloned().unwrap();
        let num_variables = self.dictionary.keys().next().map_or(0, |k| k.0.len());
        let mut acc = MPolynomial::constant(one);

        for b in format!("{:b}", exponent).chars() {
            acc = acc.clone() * acc.clone();
            if b == '1' {
                acc = acc * self.clone();
            }
        }

        acc
    }
}

