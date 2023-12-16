
/* 
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct MPolynomial {

    dictionary: HashMap<Vec<usize>,FieldElement>
    
}


impl MPolynomial {
    
    
    pub fn new(d: HashMap<Vec<usize>, FieldElement>) -> Self {
        return MPolynomial{dictionary: d}
    }
    
    
    pub fn zero() -> Self {
        return MPolynomial{dictionary: HashMap::new()}
    }
    
    pub fn constant(element: FieldElement) -> Self {
        let mut d: HashMap<Vec<usize>,FieldElement> = HashMap::new();    
        d.insert(vec![0], element);
        
        return MPolynomial::new(d);
    }
    
    pub fn is_zero(&self) -> bool {
        
        if self.dictionary.is_empty() {
            return true;
        } else {
            for val in self.dictionary.values() {
                if val.is_zero() == false {
                    return false;
                }
            }
            return true;
        }
    }
    
    pub fn variables (num_variables: usize) -> Vec<Self> {
        let mut vars:Vec<Self> = vec![];
        
        for i in 0..num_variables {
            let mut exponent: Vec<usize> = vec![0;num_variables];
            exponent[i] = 1;
            let mut d: HashMap<Vec<usize>, FieldElement> = HashMap::new();
            d.insert(exponent, FieldElement::one());
            vars.push(MPolynomial::new(d));
        }
        
        vars
    }
    
    pub fn lift(polynomial: Polynomial, variable_index: usize) -> MPolynomial {
        if polynomial.is_zero() {
            return MPolynomial::zero();
        }
        
        let variables = MPolynomial::variables(variable_index +1);
        
        let x = variables[variables.len()-1].clone();
        
        let mut acc = MPolynomial::zero();
        
        for i in 0..polynomial.coefs.len() {
            acc = acc.clone() + MPolynomial::constant(polynomial.coefs[i]) * (x.clone() ^ i as u128);
        }
        
        acc
    }
    
    pub fn evaluate(self, point:Vec<FieldElement>) -> FieldElement {
        
        let mut acc = FieldElement::zero();
        for (k, v) in self.dictionary.iter() {
            
            let mut prod = v.clone();
            
            for i in 0..k.len() {
                prod = prod.clone() * (point[i].clone() ^ k[i] as u128);
            }
            acc = acc + prod;
        }
        acc
    }
    
    pub fn evaluate_symbolic(self, point:Vec<Polynomial>) -> Polynomial {
        let mut acc = Polynomial::new(vec![]);
        
        for (k,v) in self.dictionary.iter() {
            let mut prod = Polynomial::new(vec![v.clone()]);
            
            for i in 0..k.len() {
                prod = prod.clone() * (point[i].clone() ^ k[i] as u128)
            }
            
            acc = acc + prod;
        }
        
        acc
    }
}

impl ops::Add for MPolynomial {
    type Output = MPolynomial;
    
    fn add(self, other: MPolynomial) -> MPolynomial {
    
        let mut d:HashMap<Vec<usize>, FieldElement> = HashMap::new();
        
        let mut num_variables:usize = 0;
        
        for k in self.dictionary.keys() {
            if k.len() > num_variables{
                num_variables = k.len();
            }
        }
        for k in other.dictionary.keys() {
            if k.len() > num_variables {
                num_variables = k.len();
            }
        }
        
        for (k, v) in self.dictionary.iter() {
            let mut pad:Vec<usize> = [k.clone(), vec![0; num_variables- k.len()]].concat();
            d.insert(pad,v.clone());
        }
        for (k, v) in other.dictionary.iter() {
            let mut pad:Vec<usize> = [k.clone(), vec![0; num_variables- k.len()]].concat();
            
            if d.contains_key(&pad) {
                
                let f = d.get(&pad).unwrap().clone() + v.clone();
                d.insert(pad,  f);
            } else {
                d.insert(pad,v.clone());
            }
        }
    
        MPolynomial::new(d)
    }
}

impl ops::Mul for MPolynomial {
    type Output = MPolynomial; 
    
    fn mul(self, other:MPolynomial) -> MPolynomial {
        
        let mut d:HashMap<Vec<usize>, FieldElement> = HashMap::new();
        
        let mut num_variables:usize = 0;
        
        for k in self.dictionary.keys() {
            if k.len() > num_variables{
                num_variables = k.len();
            }
        }
        for k in other.dictionary.keys() {
            if k.len() > num_variables {
                num_variables = k.len();
            }
        }
        
        for (k0, v0) in self.dictionary.iter() {
            for (k1, v1) in other.dictionary.iter() {
                
                let mut exponent = vec![0;num_variables];
                
                for k in 0..k0.len() {
                    exponent[k] += k0[k];
                }
                for k in 0..k1.len() {
                    exponent[k] += k1[k];
                }
                
                if d.contains_key(&exponent) {
                    let f = d.get(&exponent).unwrap().clone() + v0.clone() * v1.clone();
                    
                } else {
                    d.insert(exponent, v0.clone() * v1.clone());
                }
            }
        } 
         
        MPolynomial::new(d)
    }
}


impl ops::Sub for MPolynomial {
    type Output = MPolynomial;
    
    fn sub(self, other: MPolynomial) -> MPolynomial {
        
        return self + (-other);
    }
}

impl ops::Neg for MPolynomial {
    type Output = MPolynomial;
    
    fn neg (self) -> Self {
        let mut d:HashMap<Vec<usize>, FieldElement> = HashMap::new();
        
        for (k, v) in self.dictionary.iter() {
            
            d.insert(k.clone(), -v.clone());
           
        }
        MPolynomial::new(d)
    }
}

impl ops::BitXor<u128> for MPolynomial {
    type Output = MPolynomial;
    
    fn bitxor (self, exponent:u128) -> Self {
        if self.is_zero() {
            return MPolynomial::zero();
        }
        
        let mut num_variables:usize = 0; 
        for key in self.dictionary.keys() {
            num_variables = key.len();
            break;
        }
        
        let exp = vec![0;num_variables];
        
        let mut d: HashMap<Vec<usize>, FieldElement> = HashMap::new();
        d.insert(exp, FieldElement::one());
        let mut acc = MPolynomial::new(d);
        
        for b in format!("{exponent:b}").to_string().chars() {
            acc = acc.clone() * acc.clone();
            if b == '1' {
                acc = acc.clone() * self.clone();
            }
        }
        
        acc
    }
}

*/