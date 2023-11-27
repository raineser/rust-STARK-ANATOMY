use std::ops;
use rand::Rng;

const P:u128 = 1 + 407 * ( 1 << 119 );

#[derive(Debug)]
struct FieldElement {
    pub value: u128,
    prime: u128,
}

impl FieldElement {

    pub fn new(n:u128) -> FieldElement {
        return FieldElement{value: n % P, prime: P}
    }
    
    pub fn zero() -> FieldElement {
        return FieldElement{value: 0, prime: P}
    }
    
    pub fn one() -> FieldElement {
        return FieldElement{value: 1, prime: P}
    }
    
    pub fn generator() -> FieldElement {
        return FieldElement::new(85408008396924667383611388730472331217);
    }
    
    pub fn primitive_nth_root(self, n:u128) -> FieldElement {
        
        assert!(n <= 1 << 119 && (n & (n-1)) == 0);
        let mut root = FieldElement::generator();
        let mut order: u128 = 1 << 119;
        
        while order != n {
            root = root ^ 2;
            order = order / 2;
        }
        root
    }
    
    pub fn sample() -> FieldElement {
        
        let mut rng = rand::thread_rng();
        return FieldElement::new(rng.gen_range(1..P));
        
    }
}

impl ops::Add<FieldElement> for FieldElement {
    type Output = FieldElement;
    
    fn add(self, rhs: FieldElement) -> FieldElement {
        
        let a = P - self.value;
        
        if a <= rhs.value {
            return FieldElement::new(rhs.value - a);
        } else {
            return FieldElement::new(self.value + rhs.value);
        }
    }
}

impl ops::Mul<FieldElement> for FieldElement {
    type Output = FieldElement;
    
    //https://stackoverflow.com/questions/12168348/ways-to-do-modulo-multiplication-with-primitive-types
    fn mul(self, rhs:FieldElement) -> FieldElement {
        
        let mut a = self.value;
        let mut b = rhs.value;
        let mut res: u128 = 0;
        let mut temp: u128 = 0;
        
        while a != 0 {
            if ( (a & 1) != 0) {
                
                if (b >= P.wrapping_sub(res) ) {
                    res = res.wrapping_sub(P);
                }
                res = res.wrapping_add(b);
            }
            a >>= 1;
            
            temp = b;
            if ( b >= P.wrapping_sub(b)) {
                temp = temp.wrapping_sub(P);
            }
            b = b.wrapping_add(temp);
        }
        return FieldElement::new(res);
    }
}

impl ops::Sub<FieldElement> for FieldElement {
    type Output = FieldElement;
    
    fn sub(self, rhs: FieldElement) -> FieldElement {
        
        if rhs.value > self.value {
        
            let a = rhs.value - self.value;
            
            return FieldElement::new(P-a);
            
        } else {
            
            return FieldElement::new(self.value - rhs.value);
        }
    }
}

impl ops::Neg for FieldElement {
    type Output = FieldElement;
    
    fn neg(self) -> FieldElement {
        return FieldElement::new(P-self.value);
    }
}

impl ops::BitXor<u128> for FieldElement {
    type Output = FieldElement;
    
    fn bitxor(self, rhs: u128) -> FieldElement {
        
        let mut acc = FieldElement::one();
        
        for i in (0.. format!("{rhs:b}").chars().count()).rev() {
            let mut val = FieldElement::new(self.value);
            let acc2 = FieldElement::new(acc.value);
            acc = acc * acc2;
            if (1 << i != 0) && rhs != 0 {
                acc = acc * val;
            }
        }
        
        return acc;
    }
}
