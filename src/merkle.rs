#[derive(Debug)]
pub struct data {
    
    name: String,
    value: u128
}

impl data {
    pub fn new(n: String, v: u128) -> data {
        return data{name: n, value:v};
    }
    
    
    pub fn SetName(&mut self, newName: String) {
        self.name = newName;
    }
    
    pub fn SetValue(&mut self, newValue: u128) {
        self.value = newValue;
    }
    
    pub fn serial(&self) -> Vec<u8> {
        let mut name_string = "name:".to_string();
        name_string.push_str(&self.name);
        
        let mut value_string = ",value:".to_string();
        value_string.push_str(&self.value.to_string());
        
        name_string.push_str(&value_string);
      
        let bytes = name_string.as_bytes().to_vec();

        bytes
        
    }
    
    pub fn serialHash(&self) -> [u8;32] {
        keccak256(&self.serial())
    }
}

#[derive(Debug)]
pub struct Merkle {}


impl Merkle {

    pub fn commit(leafs: Vec<[u8;32]> ) -> ([u8;32], Vec<[u8;32]>) {
        
        assert!(leafs.len() & (leafs.len() -1) == 0 );
        
        let mut ileafs = leafs.clone();
        
        for branch in 0..leafs.len().ilog2() {
            
            let mut branch: Vec<[u8;32]> = vec![];
            
            for i in (0..ileafs.len()).step_by(2) {
                
                let mut a = [ileafs[i], ileafs[i+1]].concat();
                
                branch.push(keccak256(&a));
                
            }
            
            println!("{}", branch.len());
            
            ileafs = branch.clone();
        }
        
        let root:[u8;32]  = ileafs[0];
        return (root, leafs);
    }
}
