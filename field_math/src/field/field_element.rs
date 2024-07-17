use super::field::Field;
use std::{ops::Add, process::Output, ops::Mul, ops::Sub, ops::Neg};

pub struct FieldElement<'a>{
    pub value : u64,
    pub field : &'a Field
}

impl<'a> FieldElement<'a>{
    pub fn new(value : u64, field : &'a Field) -> FieldElement{
        FieldElement{
            value,
            field
        }
    }
}

impl<'a> Add for FieldElement<'a>{
    type Output = FieldElement<'a>;

    fn add(self, other : FieldElement<'a>) -> FieldElement<'a>{
        FieldElement{
            field : self.field,
            value : self.value + other.value
        }
    }
}

impl<'a> Sub for FieldElement<'a>{
    type Output = FieldElement<'a>;

    fn sub(self, other : FieldElement<'a>) -> FieldElement<'a>{
        FieldElement{
            field : self.field,
            value : self.value - other.value
        }
    }
}

impl<'a> Mul for FieldElement<'a>{
    type Output = FieldElement<'a>;

    fn mul(self, other : FieldElement<'a>) -> FieldElement<'a>{
        FieldElement{
            field : self.field,
            value : self.value * other.value
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::field::field::Field;

    #[test]
    fn test_add(){
        let field = Field::new(5);
        let a = FieldElement::new(3, &field);
        let b = FieldElement::new(4, &field);
        let c = a + b;
        assert_eq!(c.value, 7);
    }

    #[test]
    fn test_sub(){
        let field = Field::new(5);
        let a = FieldElement::new(10, &field);
        let b = FieldElement::new(4, &field);
        let c = a - b;
        assert_eq!(c.value, 6);
    }

    #[test]
    fn test_mul(){
        let field = Field::new(5);
        let a = FieldElement::new(3, &field);
        let b = FieldElement::new(4, &field);
        let c = a * b;
        assert_eq!(c.value, 12);
    }
}