use super::{_xgcd, field_element::FieldElement};

const DEFAULT_PRIME : u128 = 1 + 407 * ( 1 << 119 );

pub struct Field{
    p : u128
}

impl Field{
    pub fn new() -> Field{
        Field{
            p : DEFAULT_PRIME
        }
    }

    pub fn from(p : u128) -> Field{
        Field{
            p
        }
    }

    pub fn zero( &self ) -> FieldElement{
        FieldElement::from(0, &self)
    }

    pub fn one( &self ) -> FieldElement{
        FieldElement::from(1, &self)
    }

    pub fn add( &self, a : FieldElement, b : FieldElement ) -> FieldElement{
        FieldElement::from( (a.value + b.value) % self.p, &self )
    }

    pub fn sub( &self, a : FieldElement, b : FieldElement ) -> FieldElement{
        FieldElement::from( (a.value + self.p - b.value) % self.p, &self )
    }

    pub fn mul( &self, a : FieldElement, b : FieldElement ) -> FieldElement{
        FieldElement::from( (a.value * b.value) % self.p, &self )
    }

    pub fn negate( &self, a : FieldElement ) -> FieldElement{
        FieldElement::from( (self.p - a.value) % self.p, &self )
    }

    pub fn inverse( &self, a : FieldElement ) -> FieldElement{
        let (g, x, _) = _xgcd(a.value, self.p);
        if g != 1{
            panic!("{} is not invertible", a.value);
        }
        FieldElement::from( (x + self.p) % self.p, &self )
    }

    pub fn div( &self, a : FieldElement, b : FieldElement ) -> FieldElement{
        self.mul(a, self.inverse(b))
    }
}