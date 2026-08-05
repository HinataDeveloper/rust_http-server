//! very simple basic HTTP server without dependency.

use std::{env, io::Write, net::TcpListener, thread};

const DEFAULT_ADDR: &str = "127.0.0.1:8086";

fn main() {
    println!("\n");

    let http_response = "HTTP/1.1 200 OK\r\n\
           Content-Type: text/plain; charset=utf-8\r\n\
           Content-Length: 1024\r\n\
           Connection: close\r\n\
           \r\n\
           I am a beginner Rustacean ...";

    let response = http_response.as_bytes();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(move || DEFAULT_ADDR.to_string());

    let listener_result = TcpListener::bind(&addr);
    let listener = match listener_result {
        Ok(lst) => lst,
        Err(err) => panic!("Error: {}", err),
    };

    println!("Server bootstrapping was over in {} address ...", addr);

    for stream in listener.incoming() {
        thread::spawn(move || {
            let mut my_stream = stream.unwrap();
            println!("I received {:?}", my_stream);

            match my_stream.write_all(response) {
                Ok(_) => (),
                Err(err) => eprintln!("Error: {}", err),
            }

            match my_stream.flush() {
                Ok(_) => (),
                Err(err) => eprintln!("Error: {}", err),
            }
        });
    }

    println!("\nThe End ...");
}
