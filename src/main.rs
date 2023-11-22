mod algebra;

fn main() {


    let f1 = algebra::FieldElement::new(15);
    let f2 = algebra::FieldElement::new(15);

    //let f3 =algebra::FieldElement::new(0);

    let f4 = f1 - f2;

    println!("{}", f4.value);
}
