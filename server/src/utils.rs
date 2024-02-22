use std::io;
use mysql::*;
use mysql::prelude::*;

pub fn connect_db() -> PooledConn {
    let url = "mysql://mathias:password@localhost:3306/Handleliste";
    let pool = Pool::new(url).unwrap();
    pool.get_conn().unwrap()
}

pub fn is_sql_friendly(str: &str) -> bool {
    if str.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        true
    } else {
        false
    }
}

pub fn find_start_index(request_str: &str) -> usize {
    match request_str.find("{") {
        Some(index) => index,
        None => {
            println!("no json data in request");
            0
        }
    }
}

