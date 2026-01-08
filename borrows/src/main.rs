#[derive(Debug, Clone, Copy)]
pub struct Foo {
    bar: u8,
}

fn main() {
    // ok so this 'move' where y now owns x is valid
    // the reason it works is that if T has copy then it will
    // copy x and have its own independent value
    // if it does not copy then x can no longer be called because
    // it no longer owns anything and so is destroyed
    let mut x: Foo = Foo { bar: 1 };
    let y = x;

    x = Foo { bar: x.bar + 1 };

    println!("x:{x:?}");
    println!("y:{y:?}");

    let x = 100;

    let y = &&100;

    println!("y:{y}, y:{y:p} y:{:p} y:{}", *y, **y);
}
