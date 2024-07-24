use std::{collections::HashMap, ops::{Add, BitXor, Div, Mul, Rem, Sub}};

use crate::field::field_element::FieldElement;

use super::multi::{Exponents, MPolynomial};

/// This is the Univariate polynomial struct 
/// 

#[derive(Debug, Clone)]
pub struct Uni<'a>{
    pub coefficients : Vec<FieldElement<'a>>
}

impl<'a> Uni<'a>{
    fn from(coefficients : Vec<FieldElement<'a>>) -> Uni<'a>{
        Uni{
            coefficients
        }
    }

    fn degree(self) -> i128 {
        if self.coefficients.len() == 0 {
            return -1;
        } 
        let mut zero = self.coefficients[0].field.zero();
        let _next_zero = zero.clone();
        for coefficient in &self.coefficients {
            zero = zero + *coefficient;
        }

        if zero.value == 0_i128 {
            return - 1;
        }

        let mut max_index = 0_i128;
        let mut counter = 0;
        for i in self.coefficients {
            if i != _next_zero {
                max_index = counter;
            }
            counter = counter + 1;
        }
        return max_index;
    }

    fn neg(&self) -> Uni<'a>{
        let neg_coefficients: Vec<FieldElement<'a>> = self.coefficients.iter()
            .map(|coeff| -coeff.clone())
            .collect();
        Uni::from(neg_coefficients)
    }

    fn is_zero(&self) -> bool{
        // use degree
        self.clone().degree() == -1
    }

    fn leading_coefficient(self) -> FieldElement<'a>{
        if self.is_zero(){
            return self.coefficients[0].field.zero();
        }
        self.coefficients[self.clone().degree() as usize]
    }

    fn true_division(self, rhs: Self) -> (Uni<'a>, Uni<'a>){
        let mut quotient_coefficients: Vec<FieldElement<'a>> = vec![];
        let mut remainder = self.clone();
        let mut divisor = rhs.clone();
        while !remainder.is_zero() && remainder.clone().degree() >= divisor.clone().degree(){
            let leading_coefficient = remainder.clone().leading_coefficient() / divisor.clone().leading_coefficient();
            let degree_difference = remainder.clone().degree() - divisor.clone().degree();
            let mut term_coefficients: Vec<FieldElement<'a>> = vec![leading_coefficient];
            for _ in 0..degree_difference{
                term_coefficients.push(remainder.coefficients[0].field.zero());
            }
            let term = Uni::from(term_coefficients);
            quotient_coefficients.push(leading_coefficient);
            remainder = remainder - (divisor.clone() * term);
        }
        (Uni::from(quotient_coefficients), remainder)
    }

    fn evaluate(self, x: FieldElement<'a>) -> FieldElement<'a>{
        let mut result = self.coefficients[0].clone();
        let mut x_power = x.clone();
        for i in 1..self.coefficients.len(){
            result = result + (self.coefficients[i].clone() * x_power.clone());
            x_power = x_power * x.clone();
        }
        result
    }

    fn evaluate_domain(self, domain: Vec<FieldElement<'a>>) -> Vec<FieldElement<'a>>{
        domain.iter().map(|x| self.clone().evaluate(x.clone())).collect()
    }

    fn interpolate_domain(domain: Vec<FieldElement<'a>>, values: Vec<FieldElement<'a>>) -> Uni<'a>{
        let mut result = Uni::from(vec![]);
        for i in 0..domain.len(){
            let mut term_coefficients: Vec<FieldElement<'a>> = vec![values[i].clone()];
            for j in 0..domain.len(){
                if i != j{
                    term_coefficients = vec![
                        term_coefficients[0] * domain[j].clone() * domain[j].clone() / (domain[j].clone() - domain[i].clone())
                    ];
                }
            }
            result = result + Uni::from(term_coefficients);
        }
        result
    }

    fn zerofier_domain(domain: Vec<FieldElement<'a>>) -> Uni<'a>{
        let mut result = Uni::from(vec![]);
        for i in 0..domain.len(){
            let mut term_coefficients: Vec<FieldElement<'a>> = vec![domain[i].field.zero()];
            for j in 0..domain.len(){
                if i != j{
                    term_coefficients = vec![
                        term_coefficients[0] * domain[j].clone() / (domain[j].clone() - domain[i].clone())
                    ];
                }
            }
            result = result + Uni::from(term_coefficients);
        }
        result
    }

    fn scale(self, scalar: FieldElement<'a>) -> Uni<'a>{
        let scaled_coefficients: Vec<FieldElement<'a>> = self.coefficients.iter()
            .map(|coeff| coeff.clone() * scalar.clone())
            .collect();
        Uni::from(scaled_coefficients)
    }

    fn test_colinearity(points : Vec<(FieldElement<'a>, FieldElement<'a>)>) -> bool{
        let mut result = true;
        for i in 0..points.len(){
            for j in 0..points.len(){
                if i != j{
                    let x_diff = points[i].0.clone() - points[j].0.clone();
                    let y_diff = points[i].1.clone() - points[j].1.clone();
                    if x_diff.clone().value == 0 || y_diff.clone().value == 0{
                        result = false;
                    } else {
                        let slope = y_diff.clone() / x_diff.clone();
                        for k in 0..points.len(){
                            if k != i && k != j{
                                let x_diff_2 = points[i].0.clone() - points[k].0.clone();
                                let y_diff_2 = points[i].1.clone() - points[k].1.clone();
                                if x_diff_2.clone().value == 0 || y_diff_2.clone().value == 0{
                                    result = false;
                                } else {
                                    let slope_2 = y_diff_2.clone() / x_diff_2.clone();
                                    if slope != slope_2{
                                        result = false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        result
    }

    fn lift(univariate_poly : Uni<'a>, variable_index : usize) -> MPolynomial<'a>{
        let mut dictionary = HashMap::new();
        for i in 0..univariate_poly.coefficients.len(){
            let mut exponents = vec![0; univariate_poly.coefficients.len()];
            exponents[i] = 1;
            dictionary.insert(Exponents(exponents), univariate_poly.coefficients[i].clone());
        }
        MPolynomial::new(dictionary)
    }

    
}

impl<'a> Add for Uni<'a>{
    type Output = Uni<'a>;

    fn add(self, rhs: Self) -> Uni<'a> {
        let mut sum_coefficients: Vec<FieldElement<'a>> = vec![];
        let mut i = 0;
        let mut j = 0;
        while i < self.coefficients.len() && j < rhs.coefficients.len(){
            sum_coefficients.push(self.coefficients[i] + rhs.coefficients[j]);
            i += 1;
            j += 1;
        }

        while i < self.coefficients.len(){
            sum_coefficients.push(self.coefficients[i]);
            i += 1;
        }

        while j < rhs.coefficients.len(){
            sum_coefficients.push(rhs.coefficients[j]);
            j += 1;
        }

        Uni::from(sum_coefficients)
    }
}
impl<'a> PartialEq for Uni<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.coefficients == other.coefficients
    }
}

impl<'a> Sub for Uni<'a>{
    type Output = Uni<'a>;

    fn sub(self, rhs: Self) -> Uni<'a> {
        let neg_rhs = rhs.neg();
        self + neg_rhs
    }
}

impl<'a> Mul for Uni<'a>{
    type Output = Uni<'a>;

    fn mul(self, rhs: Self) -> Uni<'a> {
        let mut product_coefficients: Vec<FieldElement<'a>> = vec![];
        for i in 0..self.coefficients.len(){
            for j in 0..rhs.coefficients.len(){
                let product = self.coefficients[i] * rhs.coefficients[j];
                let index = i + j;
                if index >= product_coefficients.len(){
                    product_coefficients.push(product);
                } else {
                    product_coefficients[index] = product_coefficients[index] + product;
                }
            }
        }
        Uni::from(product_coefficients)
    }
}



impl<'a> Div for Uni<'a>{
    type Output = Uni<'a>;

    fn div(self, rhs: Self) -> Uni<'a> {
        let mut quotient_coefficients: Vec<FieldElement<'a>> = vec![];
        let mut remainder = self.clone();
        let mut divisor = rhs.clone();
        while !remainder.is_zero() && remainder.clone().degree() >= divisor.clone().degree(){
            let leading_coefficient = remainder.clone().leading_coefficient() / divisor.clone().leading_coefficient();
            let degree_difference = remainder.clone().degree() - divisor.clone().degree();
            let mut term_coefficients: Vec<FieldElement<'a>> = vec![leading_coefficient];
            for _ in 0..degree_difference{
                term_coefficients.push(remainder.coefficients[0].field.zero());
            }
            let term = Uni::from(term_coefficients);
            quotient_coefficients.push(leading_coefficient);
            remainder = remainder - (divisor.clone() * term);
        }
        Uni::from(quotient_coefficients)
    }
}

impl<'a> Rem for Uni<'a>{
    type Output = Uni<'a>;

    fn rem(self, rhs: Self) -> Uni<'a> {
        let mut quotient_coefficients: Vec<FieldElement<'a>> = vec![];
        let mut remainder = self.clone();
        let mut divisor = rhs.clone();
        while !remainder.is_zero() && remainder.clone().degree() >= divisor.clone().degree(){
            let leading_coefficient = remainder.clone().leading_coefficient() / divisor.clone().leading_coefficient();
            let degree_difference = remainder.clone().degree() - divisor.clone().degree();
            let mut term_coefficients: Vec<FieldElement<'a>> = vec![leading_coefficient];
            for _ in 0..degree_difference{
                term_coefficients.push(remainder.coefficients[0].field.zero());
            }
            let term = Uni::from(term_coefficients);
            quotient_coefficients.push(leading_coefficient);
            remainder = remainder - (divisor.clone() * term);
        }
        remainder
    }
}

// implement the xor bit symbol for exponentiation of the polynomial
impl<'a> BitXor for Uni<'a>{
    type Output = Uni<'a>;

    fn bitxor(self, rhs: Self) -> Uni<'a> {
        let mut result = Uni::from(vec![self.coefficients[0].field.one()]);
        let mut base = self.clone();
        let mut exponent = rhs.clone();
        while !exponent.is_zero(){
            if exponent.clone().leading_coefficient() == exponent.coefficients[0].field.one(){
                result = result * base.clone();
            }
            base = base.clone() * base.clone();
            exponent = exponent.clone() - Uni::from(vec![exponent.coefficients[0].field.one()]);
        }
        result
    }
}

#[cfg(test)]
mod test{
    use crate::field::field::Field;

    use super::*;

    #[test]
    fn create_univariate_polynomial(){
        let field = Field::new();
        let coefficients = vec![
            FieldElement::from(2, &field),
            FieldElement::from(3, &field),
        ];

        let uni = Uni::from(coefficients);
        assert_eq!(uni.coefficients.len(), 2);
    }

    #[test]
    fn get_polynomial_degree__1(){
        let field = Field::new();
        let coefficients = vec![];
        let uni = Uni::from(coefficients);
        assert_eq!(uni.degree(), -1);
    }

    #[test]
    fn get_polynomial_degree(){
        let field = Field::new();
        let coefficients = vec![
            FieldElement::from(2, &field),
            FieldElement::from(3, &field),
            FieldElement::from(4, &field),
        ];
        let uni = Uni::from(coefficients);
        assert_eq!(uni.degree(), 2);
    }

    #[test]
    fn polynomial_negation(){
        let field = Field::new();
        let coefficients = vec![
            FieldElement::from(2, &field),
            FieldElement::from(3, &field),
            FieldElement::from(4, &field),
        ];
        let uni = Uni::from(coefficients);
        let uni_neg = uni.neg();
        let uni_coefficients_length = uni.coefficients.len();
        for i in 0..uni_coefficients_length{
            assert_eq!(uni.coefficients[i], -uni_neg.coefficients[i])
        }
    }


    #[test]
    fn polynomial_addition(){
        let field = Field::new();

        let uni_1 = Uni::from(vec![
            FieldElement::from(2, &field),
            FieldElement::from(3, &field),
            FieldElement::from(4, &field),
        ]);

        let uni_2 = Uni::from(vec![
            FieldElement::from(2, &field),
            FieldElement::from(3, &field),
            FieldElement::from(4, &field),
        ]);


        let uni_sum = uni_1 + uni_2;
        
        assert_eq!(uni_sum, Uni::from(vec![
            FieldElement::from(4, &field),
            FieldElement::from(6, &field),
            FieldElement::from(8, &field),
        ]));
    }

    #[test]
    fn interpolate_domain(){
        let field = Field::new();
        let domain = vec![
            FieldElement::from(1, &field),
            FieldElement::from(2, &field),
            FieldElement::from(3, &field),
        ];

        let values = vec![
            FieldElement::from(1, &field),
            FieldElement::from(4, &field),
            FieldElement::from(9, &field),
        ];

        let uni = Uni::interpolate_domain(domain, values);
        dbg!(&uni);
        assert_eq!(uni, Uni::from(vec![
            FieldElement::from(1, &field),
            FieldElement::from(0, &field),
            FieldElement::from(0, &field),
        ]));
    }
}