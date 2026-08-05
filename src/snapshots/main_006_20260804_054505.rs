use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

fn main() {
    println!("\n");

    for batch_start in (0..100).step_by(5) {
        let mut thread_pool: Vec<JoinHandle<()>> = Vec::with_capacity(5);
        for item in batch_start..(batch_start + 5).min(100) {
            let handle = thread::spawn(move || {
                thread::sleep(Duration::from_secs(5));
                println!("thread number is: {}", item + 1);
            });
            thread_pool.push(handle);
        }

        for trd in thread_pool {
            trd.join().expect("There is a bad join ...");
        }
    }

    println!("\nThe End ...\n");
}
