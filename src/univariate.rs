
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
            return -2 as i128
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
    
    /*
    
    fn divide(&self, d: &Polynomial) -> (Polynomial, Polynomial) {
        
        if d.degree() == -1 {
            return (Polynomial::new(vec![]), Polynomial::new(vec![]));
        }
        
        if self.degree() < d.degree(){
            return (Polynomial::new(vec![]),d.clone());
        }
        
        let mut remainder = Polynomial::new(self.coefs.clone());
        
        let mut quotient_coefficients = Polynomial::new(vec![FieldElement::zero(); (self.degree() - d.degree() + 1) as usize]);
        
        for i in 0..(self.degree() - d.degree() + 1) as usize {
            if remainder.degree() < denominator.degree() {
                break;
            }
            
            let coefficient = remainder.leading_coefficient() / denominator.leading_coefficient();
            let shift = remainder.degree() - denominator.degree();
            
            
            let mut s  = vec![FieldElement::zero(); shift as usize];
            s.push(coefficient)
            let subtractee = Polynomial::new()
            
        }
        
        return (Polynomial::new(vec![]), Polynomial::new(vec![]));
    }
    */
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
        
        println!("{:?}", self.coefs);
        
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

//https://github.com/facebook/winterfell/bl
