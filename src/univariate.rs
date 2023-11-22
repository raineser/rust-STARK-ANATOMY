use ethnum::U256;
use crate::algebra;


pub struct Polynomial {

    pub coefs: Vec<algebra::FieldElement>

}

impl Polynomial {

    pub fn new(coefs: &Vec<U256> ) -> Polynomial{

        let mut c: Vec<algebra::FieldElement> = vec![];
        for i in coefs {

            c.push( algebra::FieldElement::new(i.clone() ));

        }

        return Polynomial{coefs: c};

    }

    pub fn degree(self) -> i128 {

        if self.coefs.len() == 0 {
            return -1;
        } else if self.coefs[0].value  == 0 {
            return -1;
        } else  {

            let mut maxindex = 0;
            for i in 0..self.coefs.len() {
                if self.coefs[i].value != 0 {
                    maxindex = i as i128;
                }
            }
            return maxindex;
        }

    }

    pub fn evaluate(self, point: i128) -> 
}