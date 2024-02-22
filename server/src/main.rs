mod utils;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use serde::{Deserialize, Serialize};
use mysql::*;
use mysql::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
struct ShoppingList {
    title: String,
    items: Vec<Vec<String>>
}

#[derive(Debug, Serialize)]
struct GetRequest {
    shopping_lists: Vec<ShoppingList>
}

fn handle_client(mut stream: TcpStream) {
    // read stream
    let mut buffer = [0; 1024];
    if let Ok(n) = stream.read(&mut buffer) {
        if n == 0 {
            println!("none recieved");
            return;
        }

        let request_str = String::from_utf8_lossy(&buffer[..n]);
        println!("{}", request_str);

        // parse
        let method = if let Some(index) = request_str.find(" ") {
            &request_str[..index]
        } else {
            println!("invvalid");
            return;
        };

        match method {
            "GET" => handle_get_request(stream, &request_str),
            "POST" => handle_post_request(stream, &request_str),
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

fn handle_post_request(mut stream: TcpStream, request_str: &str) {
    let mut conn = utils::connect_db();

    let json_start_index = utils::find_start_index(request_str);
    let json_str = &request_str[json_start_index..];
    match serde_json::from_str::<ShoppingList>(&json_str) {
        Ok(request) => {
            println!("request: {:?}", request);
            // check if input is sql friendly
            // index 1 is the items while 0 is title
            // todo: add for title and quant
            for item in request.items[0].iter() {
                if !utils::is_sql_friendly(item) {
                    println!("not sql friendly inputs");
                    let response = "HTTP/1.1 400 Unfriendly Input\r\nContent-Length: 0\r\n\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                    return;
                }
            }

            // insert into ShoppingList 
            let query = r#"
                INSERT INTO ShoppingList (list_name)
                VALUES (:list_name)
            "#;
            let result = conn.exec_drop(query, params! {
                "list_name" => request.title,
            });
            if let Err(err) = result {
                println!("failed insert: {}", err);
                let response = "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n";
                stream.write_all(response.as_bytes()).unwrap();
                return;
            }

            let list_id = conn.last_insert_id();

            // insert items with same id as last list inserted
            for item in &request.items {
                let query = r#" 
                      INSERT INTO Product (list_id, product_name, quantity)
                      VALUES (:list_id, :product_name, :quantity)
                    "#;
                let result = conn.exec_drop(query, params! {
                    "list_id" => list_id,
                    "product_name" => item[0].trim(),
                    "quantity" => item[1].trim().parse::<i32>().unwrap(),
                });
                if let Err(err) = result {
                    println!("Failed to insert product: {}", err);
                    let response = "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                    return;
                }
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


fn handle_get_request(mut stream: TcpStream, request_str: &str) {
    let mut conn = utils::connect_db();

    let shopping_list_query = "SELECT * FROM ShoppingList";
    let shopping_lists: Vec<ShoppingList> = conn
        .query_map(shopping_list_query, |(list_id, title, _time_stamp): (i32, String, String)| {
            let mut inner_conn = utils::connect_db();
            let item_query = format!("SELECT product_name, quantity FROM Product WHERE list_id = {}", list_id);
            let items: Vec<Vec<String>> = inner_conn
                .query_map(&item_query, |(product_name, quantity): (String, i32)| { 
                    vec![product_name, quantity.to_string()]
                })
                .unwrap();

            ShoppingList {
                title,
                items,
            }
        })
        .unwrap();

    let get_request = GetRequest {
        shopping_lists,
    };

    let json_response = serde_json::to_string(&get_request).unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        json_response.len(),
        json_response
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_put_request(mut stream: TcpStream, request_str: &str) {
    let mut conn = utils::connect_db();
    // todo: handle put request to update db
}

fn handle_delete_request(mut stream: TcpStream, request_str: &str) {
    let mut conn = utils::connect_db();

    let json_start_index = utils::find_start_index(request_str);
    let json_str = &request_str[json_start_index..];
    match serde_json::from_str::<ShoppingList>(&json_str) {
        Ok(request) => {
            let list_id_query = format!("SELECT list_id FROM ShoppingList WHERE list_name = '{}'", request.title);
            let list_id: Option<i32> = conn.query_first(list_id_query).unwrap();

            if let Some(id) = list_id {

                // delete products/items
                let delete_products_query = format!("DELETE FROM Product WHERE list_id = {}", id);
                conn.query_drop(delete_products_query).unwrap();

                // delete lists
                let delete_list_query = format!("DELETE FROM ShoppingList WHERE list_id = {}", id);
                conn.query_drop(delete_list_query).unwrap();

                let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
        Err(err) => {
            println!("Failed to parse JSON: {}", err);
            let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
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

