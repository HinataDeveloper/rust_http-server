use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};

fn main() {
    let worker_count = 5;
    let (sender, receiver) = mpsc::channel::<u32>();

    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec::with_capacity(worker_count);

    for worker_id in 0..worker_count {
        let receiver = Arc::clone(&receiver);

        let handle = thread::spawn(move || {
            loop {
                let item = {
                    let receiver = receiver.lock().expect("receiver mutex was poisoned");
                    receiver.recv()
                };

                match item {
                    Ok(item) => {
                        thread::sleep(Duration::from_secs(5));
                        println!("worker {worker_id}: I am number {item}");
                    }

                    Err(_) => break,
                }
            }
        });

        workers.push(handle);
    }

    for item in 0..100 {
        sender.send(item).expect("all workers disconnected");
    }

    drop(sender);

    for worker in workers {
        worker.join().expect("a worker thread panicked");
    }

    println!("\nThe End ...\n");
}
