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

use std::{collections::HashMap, sync::Arc, sync::Mutex};

use tokio::sync::mpsc::{self, Receiver, Sender};

const N: u32 = 100;

#[tokio::main]
async fn main() {
    let x: Arc<Mutex<HashMap<u32, u32>>> = Arc::new(Mutex::new(HashMap::new()));
    let y: Arc<Mutex<HashMap<u32, u32>>> = Arc::new(Mutex::new(HashMap::new()));

    let (tx, mut rx): (Sender<u32>, Receiver<u32>) = mpsc::channel(1);

    for i in 0..N {
        let x_clone = Arc::clone(&x);
        let y_clone = Arc::clone(&y);
        let thread_tx = tx.clone();
        tokio::spawn(async move {
            let prev = (i + N - 1) % N;
            let x: u32;
            {
                // you can use sync::mutex if youre not sending
                // something in the await
                let mut local_x = x_clone.lock().unwrap();
                local_x.insert(i, 1);
                let mut local_y = y_clone.lock().unwrap();
                let prev_x = local_x.get(&prev).cloned().unwrap_or(0);
                x = prev_x;
                local_y.insert(i, prev_x);
            }
            thread_tx.send(x).await.unwrap();
        });
    }

    drop(tx);

    while let Some(msg) = rx.recv().await {
        println!("{msg:?}");
    }

    println!("{y:?}");
    println!("{x:?}");
}
