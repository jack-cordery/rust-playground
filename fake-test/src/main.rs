use fake::{Fake, faker::lorem::en::Paragraph};

fn main() {
    // the input is the length we want to bound the generator by
    // i.e between 1 and 1 or betwen 5 and 9
    let p: String = Paragraph(1..2).fake();
    println!("with 1..2:{p}");
    let p: String = Paragraph(5..6).fake();
    println!("with 5..6:{p}");
    let p: String = Paragraph(5..10).fake();
    println!("with 5..10:{p}");
}
