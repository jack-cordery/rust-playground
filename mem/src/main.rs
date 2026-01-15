use std::{process, time::Instant};

/// I would like to replicate the mem.c file from OSTEP
/// That program calls malloc for a user input number of Mb
/// in the form of an array
/// it then loops over the array printing out how long it took
/// to iterate the whole array
///
fn main() {
    let mut args = std::env::args();

    let program = args.next().unwrap();

    let mem_size: String = match args.next() {
        Some(mem_size) => mem_size,
        None => {
            eprintln!("Usage {program} <mem_size>Mb");
            process::exit(1)
        }
    };

    let mem_size: usize = match str::parse(&mem_size) {
        Ok(mem_size) => mem_size,
        Err(_) => {
            eprintln!("{mem_size} is not a valid u8");
            process::exit(1)
        }
    };

    let now = Instant::now();
    let mut x: Box<[u8]> = vec![1; (mem_size as usize) * 1024 * 1024].into_boxed_slice();
    let size_x = size_of_val(&*x);
    let elapsed = now.elapsed().as_micros();
    println!("allocation of {size_x} took {elapsed}us");

    let start = Instant::now();
    for i in x.iter_mut() {
        *i += 1;
    }
    let end = start.elapsed().as_millis();

    println!("full loop took {end}us");
}
