use crate::field::field_element::FieldElement;

/// This is the Univariate polynomial struct 
/// 

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
}