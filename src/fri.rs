struct Fri {
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

}
