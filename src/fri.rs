#[derive(Debug)]
pub struct Fri {
    offset: FieldElement,
    omega: FieldElement, 
    domain_length : u128,
    //field: FieldElement,
    expansion_factor: u128,
    num_colinearity_tests: u128
}

impl Fri {
    
    pub fn new(offset: FieldElement, omega: FieldElement, initial_domain_length: u128, 
            expansion_factor: u128,num_colinearity_test: u128 ) -> Self {
        
        return Fri{offset: offset, omega: omega, domain_length: initial_domain_length,
        expansion_factor: expansion_factor, num_colinearity_tests: num_colinearity_test};
    }
    
    pub fn num_rounds(&self) -> u128 {
        
        let mut codeword_length = self.domain_length;
        let mut num_rounds = 0;
        
        while codeword_length > self.expansion_factor && 4*self.num_colinearity_tests < codeword_length {
            codeword_length /= 2;
            num_rounds += 1;
        }
        num_rounds
    }
    
    pub fn eval_domain(&self) -> Vec<FieldElement> {
        let mut domain: Vec<FieldElement> = vec![];
        
        for i in 0..self.domain_length {
            
            domain.push(self.offset * (self.omega ^ i));
        }
        domain
    }
    
    pub fn commit(&self, codeword: &mut Vec<FieldElement>, proof_stream: &mut ProofStream, roundIndex: u128) -> Vec<FieldElement> {
        
        let one = FieldElement::one();
        let two = FieldElement::new(2);
        let mut omega = self.omega.clone();
        let mut offset = self.offset.clone();
        let mut codewords :Vec<FieldElement> = vec![];
        
        
        for r in 0..self.num_rounds() {
            
            // compute and send Merkle root
            let root = Merkle::commit(codeword);
            proof_stream.push(&root);
            
            if r == self.num_rounds() - 1 {
                break
            }
            
            let alpha = FieldElement::sample(&proof_stream.prover_fiat_shamir(32));
            
            codewords.push(alpha.clone());
            
            let mut new_codeword:Vec<FieldElement> = vec![];
            
            for i in 0..codeword.len()/2 {
                
                new_codeword[i] = two.inverse().clone() * ((one.clone() + alpha.clone() / (offset.clone() * (omega.clone() ^ i as u128))) * codeword[i].clone() + (one.clone() - alpha.clone() / (offset.clone() * (omega.clone() ^ i as u128))) * codeword[codeword.len()/2 + i].clone());
                
            }
            omega = omega ^ 2;
            offset = offset ^ 2;
            *codeword = new_codeword.clone();
        }
        
        //proof_stream.push(codeword.clone());
        
        //codewords = [codewords, codeword].concat();
        
        
        codewords
    
    }

}
