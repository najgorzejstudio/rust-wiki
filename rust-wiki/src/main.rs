use rayon::prelude::*;
use std::{
    collections::HashMap,
    env, fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use crate::autocomplete::AutoCompl;
mod autocomplete;
mod index;

fn main() {
    let args: Vec<String> = env::args().collect();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //handle errors
    let newindex: bool;
    if args[0] == "newindex" {
        newindex = true;
    } else {
        newindex = false;
    }

    let mut indexes = index::Index::new();
    indexes.load(newindex);
    let pagerank_index = indexes.get_pagerank();
    let id_name_index = indexes.get_name_id();
    let mut autocomplete_sys = autocomplete::AutoCompl::new();
    autocomplete_sys.load(newindex, pagerank_index);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut autocomplete_sys, &id_name_index);
    }
}

fn handle_connection(
    mut stream: TcpStream,
    autocomplete: &mut AutoCompl,
    name_index: &HashMap<i32, String>,
) {
    let auto_path = "auto.json";
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match request_line.as_str() {
        s if s.starts_with("GET / ") => ("HTTP/1.1 200 OK".to_string(), "hello.html".to_string()),
        s if s.starts_with("GET /script.js") => {
            ("HTTP/1.1 200 OK".to_string(), "script.js".to_string())
        }
        s if s.starts_with("GET /api/search?q=") => {
            autocomplete.complete(&request_line, auto_path, name_index)
        }
        _ => ("HTTP/1.1 404 NOT FOUND".to_string(), "404.html".to_string()),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    println!("Request: {request_line:#?}");
}
