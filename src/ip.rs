
use crate::algebra::FieldElement;
use crate::keccak::keccak256;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone,Deserialize, Serialize, PartialEq)]
pub enum Object {

    Point(FieldElement),
    MerkleRoot([u8;32]),
    CodeWord(Vec<FieldElement>),
    ColinearityTest((FieldElement, FieldElement, FieldElement)),
    MerklePath(Vec<[u8;32]>)
}


#[derive(Clone, Debug,Deserialize, Serialize)]
pub struct ProofStream {
    
    objects: Vec<Object>,
    read_index: u8,
}

impl ProofStream {
    pub fn new() -> ProofStream {
        return ProofStream{objects: vec![], read_index:0};
    }
    
    pub fn push(&mut self, obj: &Object) {
        self.objects.push(obj.clone())
    }
    
    pub fn pull(&mut self) -> Object {
        assert!((self.read_index as usize) < self.objects.len());
        let obj = self.objects[self.read_index as usize].clone();
        self.read_index += 1;
        obj
    }
    
    
    pub fn serial(&self) -> Vec<u8> {
    
        bincode::serialize(&self.objects).unwrap()
        
    }
    
    pub fn deserial(bb: &Vec<u8>) -> ProofStream {
        
        let new_objects = bincode::deserialize(bb).unwrap();
        
        ProofStream{objects: new_objects, read_index: 0} 
    }
    

    pub fn prover_fiat_shamir(&self) -> [u8;32] {
        keccak256(&self.serial())
    }
    
    pub fn verifier_fiat_shamir(&self) -> [u8;32] {
        let verifier = ProofStream{objects: self.objects[..self.read_index as usize].to_vec(), read_index: 0};
        keccak256(&verifier.serial())
    }
    
}




#[cfg(test)]
mod test {
    use std::iter::Once;

    use super::*;

    #[test]
    fn test_serialize() {

        let mut proof1 = ProofStream::new();
        proof1.push(&Object::Point(FieldElement::new(1)));
        proof1.push(&Object::MerkleRoot([1_u8; 32]));
        proof1.push(&Object::CodeWord(vec![FieldElement::new(1), FieldElement::new(2)]));
        proof1.push(&Object::ColinearityTest((FieldElement::new(3), FieldElement::new(4), FieldElement::new(5))));
        proof1.push(&Object::MerklePath(vec![[1_u8;32], [2_u8; 32], [3_u8; 32]]));
        proof1.push(&Object::Point(FieldElement::new(2)));


        let ser = proof1.serial();
        let mut proof2 = ProofStream::deserial(&ser);

        assert!(proof1.pull() == proof2.pull());
        assert!(proof1.pull() == proof2.pull());
        assert!(proof1.pull() == proof2.pull());
        assert!(proof1.pull() == proof2.pull());
        assert!(proof1.pull() == proof2.pull());

        assert!(proof1.pull() == Object::Point(FieldElement::new(2)));
        assert!(proof2.pull() == Object::Point(FieldElement::new(2)));
    }

}
