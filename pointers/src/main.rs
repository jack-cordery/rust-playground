fn main() {
    let x = 100;
    let y = &x;

    println!("{y:p}");

    let x = Box::new(100);
    let y = &x;

    println!("y is {y:p}");
    println!("y deref is {:p}", *y);
    println!("x is {x:p}");
}
