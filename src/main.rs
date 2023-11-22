mod algebra;
use ethnum::U256;
mod univariate;
fn main() {


    let f1 = algebra::FieldElement::new(U256::from(15 as u128));
    let f2 = algebra::FieldElement::new(U256::from(15 as u128));

    //let f3 =algebra::FieldElement::new(0);

    let f4 = f1 - f2;

    println!("{}", f4.value);

    let c: Vec<U256> = vec![U256::from(1 as u128),U256::from( 2 as u128),U256::from(1 as u128)];

    let p1 = univariate::Polynomial::new(&c);

    println!("{:?}", p1.coefs);

    println!("{}", p1.degree());



    



}
