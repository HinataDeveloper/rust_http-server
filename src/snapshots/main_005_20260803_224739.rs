use std::{thread, time::Duration};

fn main() {
    println!();

    for batch_start in (0..100).step_by(5) {
        let mut handles = Vec::with_capacity(5);

        for item in batch_start..(batch_start + 5).min(100) {
            let handle = thread::spawn(move || {
                thread::sleep(Duration::from_secs(5));
                println!("I am number {item}");
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("a worker thread panicked");
        }
    }

    println!("\nThe End ...\n");
}
