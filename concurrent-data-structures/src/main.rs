use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

/// so here we want to try and implement a basic concurrent counter both
/// from a "precise" and approximate perspective
pub struct Counter {
    count: Mutex<u32>,
}

impl Counter {
    pub fn new() -> Self {
        let count = Mutex::new(0);
        Self { count }
    }

    pub fn increment(&self) {
        let mut locked_count = self.count.lock().unwrap();
        *locked_count += 1;
        // unlocked when locked_count is dropped
    }
}

pub enum CounterType {
    Precise,
    Approximate,
}

fn work(increments: usize, n_threads: usize, c: CounterType) {
    let now = Instant::now();

    let counter = match c {
        CounterType::Precise => Counter::new(),
        CounterType::Approximate => todo!(),
    };

    let counter = Arc::new(counter);
    let mut handles = vec![];
    let n_increments = increments / n_threads;
    assert_eq!(0, increments % n_threads);
    for _ in 0..n_threads {
        let thread_n = n_increments;
        let thread_counter = Arc::clone(&counter);
        let join_handle = thread::spawn(move || {
            for _ in 0..thread_n {
                thread_counter.increment();
            }
            //lock release on drop
        });
        handles.push(join_handle);
    }
    //Wait
    for handle in handles {
        handle.join().unwrap();
    }
    let final_count = counter.count.lock().unwrap();
    println!(
        "Counter for one thread is {} and it took {}",
        final_count,
        now.elapsed().as_millis()
    );
}

fn main() {
    work(5_000_000, 1, CounterType::Precise);
    work(5_000_000, 5, CounterType::Precise);
}
