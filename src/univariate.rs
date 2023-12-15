use std::cmp;

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


