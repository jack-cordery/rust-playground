use std::time::Instant;

fn main() {
    let now = Instant::now();

    println!("undefined capacity took {}", now.elapsed().as_micros());

    let now = Instant::now();
    let mut v: Vec<u16> = Vec::new();

    for i in 0..(1usize << 16) {
        v.push(i as u16);
        // println!("len: {} cap: {}", v.len(), v.capacity());
    }

    println!("defined capacity took {}", now.elapsed().as_micros());
}
