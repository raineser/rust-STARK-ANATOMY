use std::cmp;

#[derive(Debug)]
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
            return Self;
        }
        
        let mut coeffs = vec![FieldElement::zero(); cmp::max(self.coefs.len(), rhs.coefs.len())];
        
        
        for i in 0..self.coefs.len() {
            coeffs[i] = coeff[i] + self.coefs[i];
        }
        
        
    }
}
