use std::fmt::Display;
fn something<Something>(x: Something) -> impl Sync + Send + Display
where
    Something: Sync + Send + Display,
{
    x
}

fn main() {
    let x = 100;
    let y = &x;

    let s = something(x);

    let s_string = something("hello world");

    println!("{y:p}");

    let x = Box::new(100);
    let y = &x;

    println!("y is {y:p}");
    println!("y deref is {:p}", *y);
    println!("x is {x:p}");
    println!("s is {s}");
    println!("s_string is {s_string}");
}
