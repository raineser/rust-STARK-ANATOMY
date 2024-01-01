use crate::algebra::FieldElement;
use crate::ip::{Object, ProofStream};
use crate::merkle::Merkle;
use crate::keccak::keccak256;

#[derive(Debug)]
pub struct Fri {
    offset: FieldElement,
    omega: FieldElement, 
    domain_length : u128,
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

    pub fn sample_index(byte_array: &[u8], size: usize) -> usize {
        let mut acc = FieldElement::new(0);
        for b in byte_array.iter() {
            acc = acc.clone() * FieldElement::new(2_u128.pow(8))  ^ (*b as u128);
        }

        (acc.value % size as u128) as usize
    }
    

    pub fn sample_indeices(&self, seed: &[u8], size:u128, reduced_size: usize, number: usize) -> Vec<usize> {
        assert!(number <= reduced_size);
        assert!(number <= 2*reduced_size, "not enough entropy in indices wrt last codeword");


        let mut indices: Vec<usize> = vec![];

        let mut reduced_indices: Vec<usize> = vec![];

        let mut counter: i32 = 0;

        while indices.len() < number {
            let bytes: Vec<u8> = [seed.clone(), &counter.to_be_bytes()].concat();
            let index = Fri::sample_index(&keccak256(&bytes[..]), size);
            let mut reduced_index = index % reduced_size;
            counter += 1;

            if !reduced_indices.contains(&reduced_index) {

                indices.push(index);
                reduced_indices.push(reduced_index);
            }
        }

        indices

    }

    pub fn eval_domain(&self) -> Vec<FieldElement> {
        let mut domain: Vec<FieldElement> = vec![];
        
        for i in 0..self.domain_length {
            
            domain.push(self.offset * (self.omega ^ i));
        }
        domain
    }
    
    pub fn commit(&self, codeword: &mut Vec<FieldElement>, proof_stream: &mut ProofStream) -> Vec<Vec<FieldElement>> {
        
        let one = FieldElement::one();
        let two = FieldElement::new(2);
        let mut omega = self.omega.clone();
        let mut offset = self.offset.clone();
        let mut codewords :Vec<Vec<FieldElement>> = vec![];
        
        for r in 0..self.num_rounds(){

            let N = codeword.len() as u128;

            //make sure omega has the right order
            assert!(omega^(N-1) == omega.inverse(),"error in commit: omega does not have the right order!" );

            // compute and send Merkle root
            let root = Merkle::commit(codeword);
            proof_stream.push(&Object::MerkleRoot(root));

            if r == self.num_rounds() -1 {
                break;
            }

            let mut alpha = FieldElement::sample(&proof_stream.prover_fiat_shamir());
            
            codewords.push(codeword.clone());

            
            let mut temp_codeword:Vec<FieldElement> = vec![];

            
            for i in (0..N/2) {

                let word = two.inverse() * ( ( one.clone() + alpha.clone() / (offset.clone() * (omega.clone()^i as u128)) ) * codeword[i as usize] + (one.clone() - alpha.clone() / (offset.clone() * (omega.clone()^i as u128)) ) * codeword[ (N/2 + i) as usize]);
                temp_codeword.push(word);
            }

            *codeword = temp_codeword;

            omega = omega^2;
            offset = offset^2;
        }

        // send last codeword
        proof_stream.push(&Object::CodeWord(codeword.clone()));
        
        codewords.push(codeword.clone());

        codewords
    
    }

    fn query (&self, current_codeword: &Vec<FieldElement>, next_codeword: &Vec<FieldElement>, c_indices: Vec<usize>,  proof_stream: &mut ProofStream) -> Vec<usize>{

        let mut a_indices = c_indices.clone();
        let mut b_indices:Vec<usize> = c_indices.iter().map(|x| x + (current_codeword.len()/2) ).collect();

        // reveal leafs
        for s in (0..self.num_colinearity_tests as usize) {

            proof_stream.push(&Object::ColinearityTest((current_codeword[a_indices[s]], current_codeword[b_indices[s]], next_codeword[c_indices[s]])));

        }

        // reveal authentication paths
        for s in (0..self.num_colinearity_tests as usize) {

            proof_stream.push( &Object::MerklePath(Merkle::open(a_indices[s], current_codeword)));
            proof_stream.push( &Object::MerklePath(Merkle::open(b_indices[s], current_codeword)));
            proof_stream.push( &Object::MerklePath(Merkle::open(c_indices[s], next_codeword)));

        }
        
        vec![a_indices, b_indices].concat()
    }

    fn prove (&self, codeword: &mut Vec<FieldElement>, proof_stream: &mut ProofStream)  -> Vec<usize>{

        assert!(self.domain_length == codeword.len() as u128);

        // commit phase

        let mut codewords = self.commit(codeword, proof_stream);

        // get indices
        let mut top_level_indices = self.sample_indeices(&proof_stream.prover_fiat_shamir()[..], codewords[1].len() as u128, codewords[codewords.len()-1].len(), self.num_colinearity_tests as usize);

        let mut indices = top_level_indices.clone();

        // query phase 
        for i in 0..codewords.len() -1{
            indices = indices.iter().map(|x| x % (codewords[i].len()/2)).collect();
            self.query(&codewords[i], &codewords[i+1], indices, proof_stream);
        }   

        top_level_indices

    }

    fn verify (&self, proof_stream: &mut ProofStream) -> Vec<(usize, FieldElement)> {


        let mut omega = self.omega.clone();
        let mut offset = self.offset.clone();

        let mut roots: Vec<Object> = vec![];
        let mut alphas: Vec<FieldElement> = vec![];

        for r in 0..self.num_rounds() as usize {
            roots.push(proof_stream.pull());
            alphas.push(FieldElement::sample(&proof_stream.verifier_fiat_shamir()));
        }

        let last_codeword = proof_stream.pull();

        assert!(roots[roots.len() - 1 ] != Merkle::commit(last_codeword));
            







        let r: Vec<(usize, FieldElement)> = vec![];
        r
    }


    

 
}
