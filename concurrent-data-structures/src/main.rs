use std::{
    process,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

/// we now want to implement approximate counter
/// the general idea here is we have a local lock and counter
/// and a global aggregate counter but the local locks
/// only update glocal after a threshold S
/// We will need some kind of vec of locks where each
/// ok so when a number is incremented it
/// acquires first a local lock, it can pick any actually
/// it then increments the local lock
/// if the local value is equal to S then we go and acquire the big lock to update it
pub struct ApproximateCounter {
    count: Mutex<u32>,
    local: Vec<Mutex<u32>>,
    s: usize,
    n_locals: usize,
}

impl ApproximateCounter {
    pub fn new(n_locals: usize, s: usize) -> Self {
        let mut local = vec![];
        for _ in 0..n_locals {
            local.push(Mutex::new(0));
        }

        let count = Mutex::new(0);
        Self {
            count,
            local,
            s,
            n_locals,
        }
    }

    pub fn increment(&self, n: usize) {
        let local_n = n % self.n_locals;

        let mut local_n = self.local[local_n].lock().unwrap();
        *local_n += 1;

        if *local_n >= self.s as u32 {
            let mut g = self.count.lock().unwrap();
            *g += *local_n;
            *local_n = 0;
        }
    }

    pub fn total(&self) {
        // here we need to flush out the last locals to globals
        let mut locked_g = self.count.lock().unwrap();
        for l in &self.local {
            let mut locked_l = l.lock().unwrap();
            *locked_g += *locked_l;
            *locked_l = 0;
        }
    }
}

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

fn work_precise(increments: usize, n_threads: usize) {
    let now = Instant::now();

    let counter = Counter::new();

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
fn work_approximate(increments: usize, n_threads: usize, n_locals: usize, s: usize) {
    let now = Instant::now();

    let counter = ApproximateCounter::new(n_locals, s);

    let counter = Arc::new(counter);
    let mut handles = vec![];
    let n_increments = increments / n_threads;
    assert_eq!(0, increments % n_threads);
    for _ in 0..n_threads {
        let thread_n = n_increments;
        let thread_counter = Arc::clone(&counter);
        let join_handle = thread::spawn(move || {
            let pid = process::id();
            for _ in 0..thread_n {
                thread_counter.increment(pid as usize);
            }
            //lock release on drop
        });
        handles.push(join_handle);
    }
    //Wait
    for handle in handles {
        handle.join().unwrap();
    }
    counter.total();
    let final_count = counter.count.lock().unwrap();
    println!(
        "Counter for one thread is {} and it took {}",
        final_count,
        now.elapsed().as_millis()
    );
}

fn main() {
    work_precise(5_000_000, 1);
    work_precise(5_000_000, 100);

    work_approximate(5_000_000, 100, 100, 1);
    work_approximate(5_000_000, 10, 10, 10000000);
}
