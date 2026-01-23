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

fn main() {
    let now = Instant::now();

    let counter = Arc::new(Counter::new());
    let mut handles = vec![];
    let thread_counter = Arc::clone(&counter);
    let join_handle = thread::spawn(move || {
        for _ in 0..5_000_000 {
            thread_counter.increment();
        }
        //lock release on drop
    });
    handles.push(join_handle);
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

    let now = Instant::now();
    let counter = Arc::new(Counter::new());
    let mut handles = vec![];
    for _ in 0..5 {
        let thread_counter = Arc::clone(&counter);
        let join_handle = thread::spawn(move || {
            for _ in 0..1_000_000 {
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
        "Counter for 5 threads is {} and it took {}",
        final_count,
        now.elapsed().as_millis()
    );
}
