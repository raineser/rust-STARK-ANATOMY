#[derive(Debug)]
pub struct ProofStream {
    
    objects: Vec<[u8;32]>,
    read_index: u8,
}

impl ProofStream {
    pub fn new() -> ProofStream {
        return ProofStream{objects: vec![], read_index:0};
    }
    
    pub fn push(&mut self, obj: &[u8;32]) {
        self.objects.push(obj.clone())
    }
    
    pub fn pull(&mut self) -> [u8;32] {
        assert!((self.read_index as usize) < self.objects.len());
        let obj = self.objects[self.read_index as usize];
        self.read_index += 1;
        obj
    }
    
    pub fn serial(&self) -> Vec<u8> {
    
        let mut serial_vec: Vec<u8> = vec![]; 
       
        let mut objects_string = "objects:".to_string().into_bytes();
        
        //"objects:"
        serial_vec.extend(objects_string);

        
        for i in 0..self.objects.len() {
            
            for byte in 0..32 {
                
                serial_vec.push( self.objects[i][byte]);

            }
            
        }

        let mut read_index_string = ":read_index:".to_string().into_bytes();
        
        serial_vec.extend(read_index_string);
        
        serial_vec.push(self.read_index);
        
        serial_vec
    }
    
    pub fn deserial(bb: &Vec<u8>) -> ProofStream {
        let mut new_objects: Vec<[u8;32]> = vec![];
        let mut new_read_index: u8 = 0;
    
        let mut index: usize = 8;
    
        loop {
            
            println!("{}", bb[index]);
        
            let mut arr:[u8;32] = [0;32];
            
            for i in 0..32 {
                
               arr[i] =  bb[index+i];
            }
            index += 32;
        
            
            new_objects.push(arr.clone());
            if bb[index] == 58 {
                new_read_index = bb[index+12];
                    break;
            }
        }
        
        return ProofStream{objects: new_objects, read_index: new_read_index} 
    }
    
    pub fn prover_fiat_shamir(&self, num_bytes:u64) -> [u8;32] {
        keccak256(&self.serial())
    }
    
    pub fn verifier_fiat_shamir(&self, num_bytes:u64) -> [u8;32] {
        let verifier = ProofStream{objects: self.objects[..self.read_index as usize].to_vec(), read_index: 0};
        keccak256(&verifier.serial())
    }
    
}
