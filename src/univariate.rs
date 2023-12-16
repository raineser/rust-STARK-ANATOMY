use std::cmp;
use std::ops;
use crate::algebra::FieldElement;

#[derive(Debug, Clone)]
pub struct Polynomial {

    pub coefs: Vec<FieldElement>

}

impl Polynomial {

    fn new (c: Vec<FieldElement>) -> Polynomial {
        return Polynomial{coefs: c};
    }
    
    fn degree(&self) -> i128 {
        if self.coefs.len() == 0 {
            return -1 as i128;
        }
        
        let zero = FieldElement::zero();
        
        if self.coefs == vec![zero; self.coefs.len()]{
            return -1 as i128
        }
        
        let mut max = 0;
        for i in 0..self.coefs.len() {
            if self.coefs[i] != zero {
                max = i;
            }
        }
        
        return max as i128;
    }
    
    fn is_zero(&self) -> bool {
        if self.degree() == -1 {
            return true;
        }
        return false;
    }
    
    fn leading_coefficient(&self) -> FieldElement {
        
        if self.degree() == -1 {
            return FieldElement::zero()
        }
    
        return self.coefs[self.degree() as usize].clone();
    }
    
    
    
    fn divide(&self, d: &Polynomial) -> (Polynomial, Polynomial) {
        
        if d.degree() == -1 {
            return (Polynomial::new(vec![]), Polynomial::new(vec![]));
        }
        
        if self.degree() < d.degree(){
            return (Polynomial::new(vec![]),d.clone());
        }
        
        let mut remainder = Polynomial::new(self.coefs.clone());
        
        let mut quotient_coefficients = vec![FieldElement::zero(); (self.degree() - d.degree() + 1) as usize];
        
        for i in 0..(self.degree() - d.degree() + 1) as usize {
            if remainder.degree() < d.degree() {
                break;
            }
            
            let coefficient = remainder.leading_coefficient() / d.leading_coefficient();
            let shift = remainder.degree() - d.degree();
            
            let mut s  = vec![FieldElement::zero(); shift as usize];
            s.push(coefficient);
            let subtractee = Polynomial::new(s) * d;
            
            quotient_coefficients[shift as usize] = coefficient;
            remainder = remainder - subtractee;
        }
        
        return (Polynomial::new(quotient_coefficients.clone()), remainder);
    }
    
    pub fn evaluate(&self, point: FieldElement) ->  FieldElement {
        let mut xi = FieldElement::one();
        let mut value = FieldElement::zero(); 
        
        for i in 0..self.coefs.len() {
            value = value + self.coefs[i] * xi;
            xi  = xi * point
        }
        value
    }
    
    pub fn evaluate_domain(&self, domain: &Vec<FieldElement> ) -> Vec<FieldElement> {
        let mut values: Vec<FieldElement> = vec![];
        
        for i in 0..domain.len() {
            values.push(self.evaluate(domain[i]));
        }
        values
    }
    
    pub fn interpolate_domain(domain: &Vec<FieldElement>, values: &Vec<FieldElement> ) -> Polynomial {
        assert!(domain.len() == values.len());
        assert!(domain.len() > 0);
        
        let mut acc = Polynomial::new(vec![FieldElement::new(0)]);
        
        for i in 0..domain.len() {
            let mut prod = Polynomial::new(vec![values[i]]);
            
            for j in 0..domain.len() {
                let x  = Polynomial::new(vec![FieldElement::zero(), FieldElement::one()]);
                
                if j == i {
                    continue;
                }
                
                prod = prod * ( (x - Polynomial::new(vec![domain[j]])) * Polynomial::new(vec![ (domain[i]-domain[j]).inverse()]) );
            }
            acc = acc + prod; 
        }
        acc
    }
    
    pub fn scale(&self, factor: FieldElement) -> Self{
        
        assert!(self.coefs.len() > 0);
        
        let mut scaled: Vec<FieldElement> = vec![];
        
        for i in 0..self.coefs.len() {
            
            scaled.push ((factor ^ i as u128) * self.coefs[i]);
        }
        
        return Polynomial::new(scaled);
    }
    
    pub fn test_colinearity(domain: Vec<FieldElement>, values: Vec<FieldElement> ) -> bool {
        
        let polynomial = Polynomial::interpolate_domain(&domain, &values);
        
        return polynomial.degree() <= 1;
        
    }

     pub fn zerofier_domain(domain: &Vec<FieldElement>) -> Polynomial {
        
        let x = Polynomial::new(vec![FieldElement::zero(), FieldElement::one()]);
        let mut acc = Polynomial::new(vec![FieldElement::one()]);
        
        for d in 0..domain.len() {
            
            acc = acc.clone() * (x.clone() - Polynomial::new(vec![domain[d]]));
        }
        acc
    }
}
    
impl ops::Neg for Polynomial {
    
    type Output = Polynomial;
    
    fn neg(self) -> Polynomial {
    
        let mut n = self.coefs.clone();
        
        for i in 0..n.len() {
            n[i] = -n[i];
        }
        
        return Polynomial::new(n);
    }
}

impl ops::Add for Polynomial {
    type Output = Polynomial;
    
    fn add(self, rhs: Polynomial) -> Polynomial {
        
        if self.degree() == -1 {
            return rhs;
        }
        
        if rhs.degree() == -1 {
            return self;
        }
        
        let mut coeffs = vec![FieldElement::zero(); cmp::max(self.coefs.len(), rhs.coefs.len())];
        
        for i in 0..self.coefs.len() {
            coeffs[i] = coeffs[i] + self.coefs[i];
        }
        
        for i in 0..rhs.coefs.len() {
            coeffs[i] = coeffs[i] + rhs.coefs[i];
        }
        
        return Polynomial::new(coeffs);
        
    }
}

impl ops::Sub for Polynomial {
    type Output = Polynomial;
    
    fn sub(self, rhs: Polynomial) -> Polynomial {
        
        return self + -rhs;
    }
}

impl ops::Mul for Polynomial {
    type Output = Polynomial;
    
    fn mul(self, rhs: Polynomial) -> Polynomial {
        
        if self.coefs.len() == 0 || rhs.coefs.len() == 0 {
            return Polynomial::new(vec![]);
        }
        let mut buf = vec![FieldElement::zero(); (self.coefs.len() + rhs.coefs.len() -1)];
        
        for i in 0..self.coefs.len() {
            if self.coefs[i].is_zero() {
                continue;
            }
            
            for j in 0..rhs.coefs.len() {
                buf[i+j] = buf[i+j] + self.coefs[i] * rhs.coefs[j];
            }
        }
        
        return Polynomial::new(buf);
    }
}

impl ops::Mul<&Polynomial> for Polynomial {
    type Output = Polynomial; 
    
     
    fn mul(self, rhs: &Polynomial) -> Polynomial {
        
        if self.coefs.len() == 0 || rhs.coefs.len() == 0 {
            return Polynomial::new(vec![]);
        }
        let mut buf = vec![FieldElement::zero(); (self.coefs.len() + rhs.coefs.len() -1)];
        
        for i in 0..self.coefs.len() {
            if self.coefs[i].is_zero() {
                continue;
            }
            
            for j in 0..rhs.coefs.len() {
                buf[i+j] = buf[i+j] + self.coefs[i] * rhs.coefs[j];
            }
        }
        
        return Polynomial::new(buf);
    }
    
}

impl ops::Div for Polynomial {
    type Output = Polynomial;
    
    fn div (self, rhs: Polynomial) -> Polynomial {
        let (quo, rem) = self.divide(&rhs);
        assert!(!rem.is_zero());
        quo 
    }
}

impl ops::Rem for Polynomial {
    type Output = Polynomial;
    
    fn rem(self, rhs: Polynomial) -> Polynomial {
        let (_, rem) = self.divide(&rhs);
        rem
    }
}



impl PartialEq for Polynomial  {
    
    fn eq(&self, other: &Self) -> bool {
        if self.degree() != other.degree() {
            return false; 
        }
        if self.degree() == -1 {
            return false;
        }
        
        for i in 0..self.coefs.len() {
            if self.coefs[i] != other.coefs[i] {
                return false;
            }
        }
        
        return true;
    }
    
}

impl ops::BitXor<u128> for Polynomial {
    type Output = Polynomial;
    
    fn bitxor (self, exponent:u128) -> Polynomial {
        if self.is_zero() {
            return Polynomial::new(vec![]);
        }
        if exponent == 0 {
            return Polynomial::new(vec![FieldElement::one()]);
        }
        
        let mut acc = Polynomial::new(vec![FieldElement::one()]);
        
        for i in (0..format!("{exponent:b}").to_string().chars().count()).rev() {
            
            acc = acc.clone() * acc.clone();
            if (1 << i) & exponent != 0 {
                acc = acc * self.clone();
            }
            
        }
        
        Polynomial::new(vec![])
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::algebra::FieldElement;
    use rand::RngCore;
    use rand::rngs::OsRng;
    use rand::Rng;

    #[test]
    pub fn test_distributivity() {

        let zero = FieldElement::zero();
        let one = FieldElement::one();
        let two = FieldElement::new(2);
        let five = FieldElement::new(5);

        let a = Polynomial::new(vec![one, zero, five, two]);
        let b = Polynomial::new(vec![two, two, one]);
        let c = Polynomial::new(vec![zero, five, two, five, five, one]);

        let lhs = a.clone() * (b.clone() + c.clone());
        let rhs = a.clone() * b.clone()  + a.clone() * c.clone();

        assert!(lhs == rhs);

        println!("univariate polynomial distributivity success \\o/")
    }

    #[test]
    pub fn test_divsion() {

        let zero = FieldElement::zero();
        let one = FieldElement::one();
        let two = FieldElement::new(2);
        let five = FieldElement::new(5);

        let a = Polynomial::new(vec![one, zero, five, two]);
        let b = Polynomial::new(vec![two, two, one]);
        let c = Polynomial::new(vec![zero, five, two, five, five, one]);

        // a should divide a*b, quotient should be b
        let (quo, rem) = Polynomial::divide( &(a.clone() * b.clone()), &a);
        assert!(rem.is_zero(), "fail division test 1");
        assert!(quo == b, "fail division test 2");

        // b should divide a*b, quotient should be a
        let (quo, rem) = Polynomial::divide( &(a.clone() * b.clone()), &b);
        assert!(rem.is_zero(), "fail division test 3");
        assert!(quo == a, "fail division test 4");

        // c should not divide a*b
        let (quo, rem) = Polynomial::divide( &(a.clone() * b.clone()), &c);
        assert!(!rem.is_zero(), "fail division test 5");


        // ... but quo * c + rem == a*b
        assert!(quo.clone() * c.clone() + rem.clone()    == a.clone() * b.clone(),  "fail division test 6");

        println!("univariate polynomial division success \\o/");

    }

    #[test]
    pub fn test_interpolate() {

        let zero = FieldElement::zero();
        let one = FieldElement::one();
        let two = FieldElement::new(2);
        let five = FieldElement::new(5);

        let values = vec![five, two, two, one, five, zero];
        let mut domain = vec![];
        for i in 1..6 {
            domain.push(FieldElement::new(i as u128));
        }

        let poly = Polynomial::interpolate_domain(&domain, &values);

        for i in 0..domain.len() {
            assert!(poly.evaluate(domain[i]) == values[i], "fail interpolate test 1");
        }

        // evaluation in random point is nonzero with high probability
        assert!(poly.evaluate(FieldElement::new(363)) != zero, "fail interpolate test 2");

        assert!( poly.degree() == (domain.len()-1) as i128, "fail interpolate test 3");

        println!("univariate polynomial interpolate success \\o/");
    }

    #[test]
    pub fn test_zerofier() {

        for trial in 0..100 {
            let mut rng = rand::thread_rng();

            let degree:u8 = rng.gen();


            let mut domain: Vec<FieldElement> = vec![];

            while domain.len() != degree as usize {

                let mut fake_index = [0u8;32];
                OsRng.fill_bytes(&mut fake_index);
                let new = FieldElement::sample(&fake_index);

                domain.push(new.clone());


            }

            let zerofier = Polynomial::zerofier_domain(&domain);

            assert!(zerofier.degree()  == degree as i128, "zerofier has degree unequal to size of domain");

            for d in 0..domain.len() {

                assert!(zerofier.evaluate(domain[d]) == FieldElement::zero(), "zerofier has degree unequal to size of domain");
            }

            let mut fake_index = [0u8;32];
            OsRng.fill_bytes(&mut fake_index);

            // Almost no chance of collision here
            let random = FieldElement::sample(&fake_index);

            assert!(zerofier.evaluate(random.clone()) != FieldElement::zero(), "zerofier evaluates to zero where it should not");


        }
        println!("univariate zerofier test success \\o/");
    }

}