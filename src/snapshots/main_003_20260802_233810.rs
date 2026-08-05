//use core::panic;
use std::{env, io::Write, net::TcpListener};

const DEFAULT_ADDR: &str = "127.0.0.1:8086";

fn main() {
    println!("\n");

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(move || DEFAULT_ADDR.to_string());

    let http_response = "HTTP/1.1 200 OK\r\n\
        Content-Type: text/plain; charset=utf-8\r\n\
        Content-Length: 29\r\n\
        Connection: close\r\n\
        \r\n\
        What a good time to see you!!";

    let listener_result = TcpListener::bind(&addr);
    let listener = match listener_result {
        Ok(lst) => lst,
        Err(err) => panic!("Error: {}", err),
    };

    eprintln!("Server bootstrapping is over on {}", addr);

    for stream in listener.incoming() {
        let mut my_stream = match stream {
            Ok(sr) => sr,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };
        println!("I received: {:?}", my_stream);
        match my_stream.write_all(http_response.as_bytes()) {
            Ok(_) => match my_stream.flush() {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Error: {}", err);
                    continue;
                }
            },
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        }
    }

    println!("\nThe End ...");
}
