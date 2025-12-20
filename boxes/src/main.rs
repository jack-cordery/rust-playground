use std::{
    fmt::Display,
    mem,
    ops::{Deref, DerefMut},
    time::Instant,
};

struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("dropping CustomSmartPointer {}", self.0);
    }
}

fn hello(name: &str) {
    println!("{name}");
}

fn main() {
    let x = 10;
    let y = 20;
    let m = mem::size_of_val(&x);
    println!("{m}");

    let now = Instant::now();
    let score = x * y;
    println!(
        "score on stack is {} and it took {}ns",
        score,
        now.elapsed().as_nanos(),
    );

    let b = Box::new(x);
    println!("b is {b}");
    let m = mem::size_of_val(&b);
    println!("{m}");

    let v = Box::new(y);
    println!("b is {v}");
    let m = mem::size_of_val(&b);
    println!("{m}");
    let now = Instant::now();
    let score = *b * *v;
    println!(
        "score on heap is {} and it took {}ns",
        score,
        now.elapsed().as_nanos(),
    );
}
