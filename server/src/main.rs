use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct JsonRequest {
    items: Vec<String>,
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Ok(n) = stream.read(&mut buffer) {
        if n == 0 {
            println!("Empty request received");
            return;
        }
        let request_str = String::from_utf8_lossy(&buffer[..n]);
        if request_str.starts_with("OPTIONS") {
            let response = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: POST\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
            return;
        }
        if let Ok(request) = serde_json::from_str::<JsonRequest>(&request_str) {
            println!("Received JSON: {:?}", request);
        } else {
            println!("Failed to parse JSON: {}", request_str);
        }
    } else {
        println!("Failed to read from stream");
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4040").unwrap();
    println!("listening...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    println!("success!");
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

