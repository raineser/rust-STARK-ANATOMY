use std::ops;
use ethnum::U256;

const P: u128 = 407 * (1 << 119) + 1;

pub fn xgcd(x: i64, y: i64) -> (i64,i64,i64) {

    let mut old_r = x;
    let mut r = y;

    let mut old_s = 1;
    let mut s = 0;

    let mut old_t = 0;
    let mut t = 1;

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    return (old_s, old_t, old_r);

}

#[derive(Debug)]
pub struct FieldElement {

    pub value: U256
}

impl FieldElement {

    pub fn new(val: U256) -> FieldElement {
        return FieldElement{value: val};
    }

    pub fn zero(&self) -> FieldElement {
        return FieldElement{value: U256::from(0 as u32)};
    }

    pub fn one(&self) -> FieldElement {
        return FieldElement{value: U256::from(1 as u32)};
    }

    pub fn generator() -> FieldElement {
        return  FieldElement::new(U256::from(85408008396924667383611388730472331217 as u128));
    }


}


impl ops::Add<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: FieldElement) -> Self::Output {
        
        FieldElement {value: (self.value + rhs.value) % P}
   
    }
}

impl ops::Mul<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: FieldElement) -> Self::Output {
        
        FieldElement {value: (self.value * rhs.value) % P}
    }
}

impl ops::Sub<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: FieldElement) -> Self::Output {
        
        FieldElement {value: (self.value - rhs.value) % P}
    }
}

