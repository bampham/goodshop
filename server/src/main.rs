use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use serde::{Deserialize};
use mysql::*;
use mysql::prelude::*;

#[derive(Debug, Deserialize)]
struct JsonRequest {
    items: Vec<Vec<String>>,
}

fn handle_client(mut stream: TcpStream) {
    // make mysql connection
    let url = "mysql://mathias:password@localhost:3306/Handleliste";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let mut buffer = [0; 1024];
    if let Ok(n) = stream.read(&mut buffer) {
        if n == 0 {
            println!("none");
            return;
        }

        let request_str = String::from_utf8_lossy(&buffer[..n]);

        // parse
        let method = if let Some(index) = request_str.find(" ") {
            &request_str[..index]
        } else {
            println!("invvalid");
            return;
        };


        match method {
            "GET" => handle_get_request(stream),
            "POST" => handle_post_request(stream, &request_str, conn),
            "PUT" => handle_put_request(stream, &request_str),
            "DELETE" => handle_delete_request(stream, &request_str),
            "OPTIONS" => handle_options_request(stream),
            _ => {
                println!("none of the above: {}", method);
                let response = "HTTP/1.1 405 method not found\r\nContent-Length: 0\r\n\r\n";
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    } else {
        println!("failed to read stream");
    }
}

fn handle_get_request(mut stream: TcpStream) {
}

fn handle_post_request(mut stream: TcpStream, request_str: &str, mut conn: PooledConn) {
    let json_start_index = match request_str.find("{") {
        Some(index) => index,
        None => {
            println!("no json data in request");
            return;
        }
    };

    let json_str = &request_str[json_start_index..];
    match serde_json::from_str::<JsonRequest>(&json_str) {
        Ok(request) => {
            // println!("request: {:?}", request);
            
            // insert to product
            for item in &request.items {
                let query = r#" 
                      INSERT INTO Product (list_id, product_name, quantity)
                      VALUES (1, :product_name, :quantity)
                    "#;
                conn.exec_drop(query, params! {
                    "product_name" => item[0].trim(),
                    "quantity" => item[1].trim().parse::<i32>().unwrap(),
                }).unwrap();
            }

            // response: OK
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(err) => {
            println!("Failed to parse JSON: {}", err);
            let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

fn handle_put_request(mut stream: TcpStream, request_str: &str) {
}

fn handle_delete_request(mut stream: TcpStream, request_str: &str) {
}

fn handle_options_request(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: 0\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4040").unwrap();
    println!("Listening on 127.0.0.1:4040...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

