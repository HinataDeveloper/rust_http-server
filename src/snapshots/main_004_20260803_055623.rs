use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

fn main() {
    println!("\n");

    for _item in (0..100).step_by(5) {
        let mut thread_pool: Vec<JoinHandle<()>> = Vec::with_capacity(5);

        for item in 0..5 {
            let handle = thread::spawn(move || {
                thread::sleep(Duration::from_secs(5));
                println!("I am number {}", item);
            });

            thread_pool.push(handle);
        }

        for trd in thread_pool {
            trd.join().expect("Invalid thread join ...");
        }
    }

    println!("\nThe End ...\n");
}
