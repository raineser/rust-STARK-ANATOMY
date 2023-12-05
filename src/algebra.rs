use std::ops;
use rand::Rng;

const P:u128 = 1 + 407 * ( 1 << 119 );


#[derive(Debug, Copy, Clone)]
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
    
    pub fn primitive_nth_root(&self, n:u128) -> FieldElement {
        
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
    
    pub fn is_zero(&self) -> bool {
        return self.value == 0;
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
    fn mul(self, rhs: FieldElement) -> FieldElement {
        
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

impl ops::Div for FieldElement {
    type Output = FieldElement;
    
    fn div(self, rhs: FieldElement) -> FieldElement {
        
        let a  = FieldElement::new( inv(rhs.value) );
        return self * a;
    } 
}

impl PartialEq for FieldElement  {
    
    fn eq(&self, other: &Self) -> bool {
        
        return self.value == other.value;
    }
    
}

//https://github.com/facebook/winterfell/blob/main/math/src/field/f128/mod.rs
fn sub_192x192(a0: u64, a1: u64, a2: u64, b0: u64, b1: u64, b2: u64) -> (u64, u64, u64) {
    let z0 = (a0 as u128).wrapping_sub(b0 as u128);
    let z1 = (a1 as u128).wrapping_sub((b1 as u128) + (z0 >> 127));
    let z2 = (a2 as u128).wrapping_sub((b2 as u128) + (z1 >> 127));
    (z0 as u64, z1 as u64, z2 as u64)
}

fn add_192x192(a0: u64, a1: u64, a2: u64, b0: u64, b1: u64, b2: u64) -> (u64, u64, u64) {
    let z0 = (a0 as u128) + (b0 as u128);
    let z1 = (a1 as u128) + (b1 as u128) + (z0 >> 64);
    let z2 = (a2 as u128) + (b2 as u128) + (z1 >> 64);
    (z0 as u64, z1 as u64, z2 as u64)
}

pub fn inv(x: u128) -> u128 {
    if x == 0 {
        return 0;
    };

    // initialize v, a, u, and d variables
    let mut v = P;
    let (mut a0, mut a1, mut a2) = (0, 0, 0);
    let (mut u0, mut u1, mut u2) = if x & 1 == 1 {
        // u = x
        (x as u64, (x >> 64) as u64, 0)
    } else {
        // u = x + m
        add_192x192(x as u64, (x >> 64) as u64, 0, P as u64, (P >> 64) as u64, 0)
    };
    // d = m - 1
    let (mut d0, mut d1, mut d2) = ((P as u64) - 1, (P >> 64) as u64, 0);

    // compute the inverse
    while v != 1 {
        while u2 > 0 || ((u0 as u128) + ((u1 as u128) << 64)) > v {
            // u > v
            // u = u - v
            let (t0, t1, t2) = sub_192x192(u0, u1, u2, v as u64, (v >> 64) as u64, 0);
            u0 = t0;
            u1 = t1;
            u2 = t2;

            // d = d + a
            let (t0, t1, t2) = add_192x192(d0, d1, d2, a0, a1, a2);
            d0 = t0;
            d1 = t1;
            d2 = t2;

            while u0 & 1 == 0 {
                if d0 & 1 == 1 {
                    // d = d + m
                    let (t0, t1, t2) = add_192x192(d0, d1, d2, P as u64, (P >> 64) as u64, 0);
                    d0 = t0;
                    d1 = t1;
                    d2 = t2;
                }

                // u = u >> 1
                u0 = (u0 >> 1) | ((u1 & 1) << 63);
                u1 = (u1 >> 1) | ((u2 & 1) << 63);
                u2 >>= 1;

                // d = d >> 1
                d0 = (d0 >> 1) | ((d1 & 1) << 63);
                d1 = (d1 >> 1) | ((d2 & 1) << 63);
                d2 >>= 1;
            }
        }

        // v = v - u (u is less than v at this point)
        v -= (u0 as u128) + ((u1 as u128) << 64);

        // a = a + d
        let (t0, t1, t2) = add_192x192(a0, a1, a2, d0, d1, d2);
        a0 = t0;
        a1 = t1;
        a2 = t2;

        while v & 1 == 0 {
            if a0 & 1 == 1 {
                // a = a + m
                let (t0, t1, t2) = add_192x192(a0, a1, a2, P as u64, (P >> 64) as u64, 0);
                a0 = t0;
                a1 = t1;
                a2 = t2;
            }

            v >>= 1;

            // a = a >> 1
            a0 = (a0 >> 1) | ((a1 & 1) << 63);
            a1 = (a1 >> 1) | ((a2 & 1) << 63);
            a2 >>= 1;
        }
    }

    // a = a mod m
    let mut a = (a0 as u128) + ((a1 as u128) << 64);
    while a2 > 0 || a >= P {
        let (t0, t1, t2) = sub_192x192(a0, a1, a2, P as u64, (P >> 64) as u64, 0);
        a0 = t0;
        a1 = t1;
        a2 = t2;
        a = (a0 as u128) + ((a1 as u128) << 64);
    }

    a
}
