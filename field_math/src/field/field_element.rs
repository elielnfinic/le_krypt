use super::field::Field;
use std::ops::{Add, BitXor, Div, Mul, Neg, Sub};

#[derive(Clone, Debug, Copy)]
pub struct FieldElement<'a> {
    pub value: i128,
    pub field: &'a Field,
}

impl<'a> FieldElement<'a> {
    pub fn from(value: i128, field: &'a Field) -> FieldElement {
        FieldElement { value, field }
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0
    }
}

impl<'a> Add for FieldElement<'a> {
    type Output = FieldElement<'a>;

    fn add(self, other: FieldElement<'a>) -> FieldElement<'a> {
        self.field.add(self, other)
    }
}

impl<'a> Sub for FieldElement<'a> {
    type Output = FieldElement<'a>;

    fn sub(self, other: FieldElement<'a>) -> FieldElement<'a> {
        self.field.sub(self, other)
    }
}

impl<'a> Mul for FieldElement<'a> {
    type Output = FieldElement<'a>;

    fn mul(self, other: FieldElement<'a>) -> FieldElement<'a> {
        self.field.mul(self, other)
    }
}

impl<'a> Neg for FieldElement<'a> {
    type Output = FieldElement<'a>;

    fn neg(self) -> FieldElement<'a> {
        self.field.negate(self)
    }
}

impl<'a> Div for FieldElement<'a> {
    type Output = FieldElement<'a>;

    fn div(self, other: FieldElement<'a>) -> FieldElement<'a> {
        self.field.div(self, other)
    }
}

impl<'a> PartialEq for FieldElement<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<'a> BitXor for FieldElement<'a> {
    type Output = FieldElement<'a>;

    fn bitxor(self, other: FieldElement<'a>) -> FieldElement<'a> {
        self.field.pow(self.value, other.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::{field::Field, DEFAULT_PRIME};

    #[test]
    fn test_add() {
        let field = Field::new();
        let a = FieldElement::from(2, &field);
        let b = FieldElement::from(3, &field);
        let c = a + b;
        assert_eq!(c.value, 5);
    }

    #[test]
    fn test_sub() {
        let field = Field::new();
        let a = FieldElement::from(3, &field);
        let b = FieldElement::from(2, &field);
        let c = a - b;
        println!("{}", c.value);
        assert_eq!(c.value, FieldElement::from(1, &field).value);
    }

    #[test]
    fn test_mul() {
        let field = Field::new();
        let a = FieldElement::from(2, &field);
        let b = FieldElement::from(3, &field);
        let c = a * b;
        assert_eq!(c.value, 6);
    }

    #[test]
    fn test_neg() {
        let field = Field::new();
        let a = FieldElement::from(2, &field);
        let b = -a;
        assert_eq!(b.value, DEFAULT_PRIME - 2);
    }

    #[test]
    fn test_div() {
        let field = Field::new();
        let a = FieldElement::from(9, &field);
        let b = FieldElement::from(3, &field);
        let c = a / b;
        println!("Division result: {}", c.value);
        assert_eq!(c.value, 3);
    }
}
