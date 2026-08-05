//! Using std::env

use std::{env, net::TcpListener, thread};

const DEFAULT_ADDR: &str = "127.0.0.1:8086";

fn main() -> std::io::Result<()> {
    println!("\n");

    // let resp: [u8; 10] = [48, 49, 50, 51, 52, 53, 54, 55, 56, 57];

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_ADDR.to_string());

    let listener = TcpListener::bind(&addr)?;
    eprintln!("listning on http://{}", addr);

    for stream in listener.incoming() {
        thread::spawn(move || {
            println!("a request received ...");
            println!("current stream is: {:?}", stream);
        });
    }

    println!("\nThe End ...\n");
    Ok(())
}
