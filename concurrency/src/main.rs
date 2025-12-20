// ok so here we want to be able to test a concurrent process
// we will want to have N threads
// that read and write to some state
// we will need Arc Mutex to do this
//
// We have a state x and y which will be repr by a hashmap
// We then need to have N threads which
// concurrently read and write from x and y
// thread::spawn will open a thread
// then we will just need to do some gymanstics with Arc mutexs around x and y

use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread,
};

const N: u8 = 4;

fn main() {
    let x: Arc<Mutex<HashMap<u8, u8>>> = Arc::new(Mutex::new(HashMap::new()));
    let y: Arc<Mutex<HashMap<u8, u8>>> = Arc::new(Mutex::new(HashMap::new()));

    let (tx, rx): (Sender<u8>, Receiver<u8>) = mpsc::channel();

    for i in 0..N {
        let x_clone = Arc::clone(&x);
        let y_clone = Arc::clone(&y);
        let thread_tx = tx.clone();
        thread::spawn(move || {
            let prev = (i + N - 1) % N;
            let mut local_x = x_clone.lock().unwrap();
            local_x.insert(i, 1);
            let mut local_y = y_clone.lock().unwrap();
            let prev_x = local_x.get(&prev).cloned().unwrap_or(0);
            local_y.insert(i, prev_x);
            thread_tx.send(prev_x).unwrap();
        });
    }

    drop(tx);

    for msg in rx {
        println!("{msg:?}");
    }

    println!("{y:?}");
    println!("{x:?}");
}
