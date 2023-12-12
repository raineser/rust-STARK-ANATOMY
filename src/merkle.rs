#[derive(Debug)]
pub struct Merkle {}


impl Merkle {
    
    fn commit_(leafs: Vec<[u8;32]>) -> Vec<[u8;32]> {
        
        assert!(leafs.len() & (leafs.len() -1) == 0 );
        
        if leafs.len() == 1 {
            return [leafs[0]].to_vec()
        } else {
            
            let first = leafs[leafs.len()/2..].to_vec();
            let second = leafs[..leafs.len()/2].to_vec();
            
            //gross
            let hash = keccak256(&[Merkle::commit_(second), Merkle::commit_(first)].concat()[..].concat()[..]).to_vec();
            let result: Vec<[u8;32]> = hash.chunks_exact(32).map(|chunk| {
                let mut array: [u8;32] = [0;32];
                array.copy_from_slice(chunk);
                array
            }).collect();
            result
        }
    }
    
    pub fn commit(leafs: &Vec<FieldElement>) -> [u8;32] {
        
        let mut formatted_leafs: Vec<[u8;32]> = vec![];
        
        for i in 0..leafs.len() {
            formatted_leafs[i] = keccak256( &leafs[i].clone().value.to_be_bytes()[..] );
        }
        
        let root = Merkle::commit_(formatted_leafs); 
        
        return root[0].clone();
        
    }
    
     
    fn open_(index: usize, leafs: Vec<[u8;32]> ) -> Vec<[u8;32]> {
        
        assert!(leafs.len() & (leafs.len()-1) == 0);
        assert!(0 <= index && index < leafs.len());
        
        if leafs.len() == 2 {
            return [leafs[1-index]].to_vec();
        }
        
        if index < leafs.len()/2 {
            
            let first = leafs[leafs.len()/2..].to_vec();
            let second = leafs[..leafs.len()/2].to_vec();
            
            return [Merkle::open_(index,second) , Merkle::commit_(first)].concat();
            
        }else {
            
            let first = leafs[leafs.len()/2..].to_vec();
            let second = leafs[..leafs.len()/2].to_vec();
            
            return [Merkle::open_(index - leafs.len()/ 2 ,first) , Merkle::commit_(second)].concat();
        }
    }
    
    fn verify_(root: [u8;32], index: usize, path: Vec<[u8;32]>, leaf: [u8;32]) -> bool {
        
        assert!(0 <= index && index < (1 << path.len()));
        
        if path.len() == 1 {
            if index == 0{

                return root == keccak256( &[leaf, path[0]].concat()[..])
            
            } else {
            
                return root == keccak256( &[path[0], leaf].concat()[..])
            }
            
        } else {
            
            if index % 2 == 0 {
                return Merkle::verify_(root, index >>1, path[1..].to_vec(), keccak256(&[leaf, path[0]].concat()[..]))
            } else {
                return Merkle::verify_(root, index >>1, path[1..].to_vec(), keccak256(&[path[0], leaf].concat()[..]))
            }
        }
    }   
}



// Tests
use rand::RngCore;
use rand::rngs::OsRng;


fn random_leaf() -> FieldElement {
    let mut value = [0u8; 16];
    OsRng.fill_bytes(&mut value);
    
    return FieldElement::new(u128::from_be_bytes(value));
}

fn merkle() {
    
    let n:usize = 64;
    
    let mut leafs: Vec<FieldElement> = vec![];
    
    for i in 0..n {
        leafs.push(random_leaf());
    }
    
    let root = Merkle::commit(&leafs);
    
    // opening any leaf should work
    for i in 0..n {
    
        let path = Merkle::open(i,&leafs);
        assert!(Merkle::verify(root, i, path, leafs[i]));
    }
    
    // opening non-leafs should not work
    
    for i in 0..n {
        let path = Merkle::open(i, &leafs);
        assert!(false == Merkle::verify(root,i,path,random_leaf()));
    }
    
    // opening wrong leafs should not work
    
    for i in 0..n {
        let path = Merkle::open(i, &leafs);
        assert!(false == Merkle::verify(root, i, path, leafs[(i+1) % 64]));
    }
    
    // opening leafs with the wrong index should not work
     for i in 0..n {
        let path = Merkle::open(i, &leafs);
        assert!(false == Merkle::verify(root, (i+1) % 64, path, leafs[i]));
    }

    //opening leafs to a false root should not work
    for i in 0..n {
        let path = Merkle::open(i, &leafs);
        let mut fake_root = [0u8;32];
        OsRng.fill_bytes(&mut fake_root);
        assert!(false == Merkle::verify(fake_root, i, path, leafs[i]));
    }
    
    // opening leafs with even one falsehood in the path should not work
    for i in 0..n {
        let path = Merkle::open(i, &leafs);
        for j in 0..path.len() {
            let mut fake_index = [0u8;32];
            OsRng.fill_bytes(&mut fake_index);
            
            let mut fake_path = path.clone();
            fake_path[j] = fake_index;
            assert!(false == Merkle::verify(root, i, fake_path, leafs[i]));
        }
    }
}
