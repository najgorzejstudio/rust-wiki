use std::{
    collections::HashMap,
    env, fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

use crate::autocomplete::AutoCompl;
mod autocomplete;
mod index;
mod results;

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

    let mut autocomplete_sys = autocomplete::AutoCompl::new();
    autocomplete_sys.load(newindex, indexes.get_pagerank());

    let mut conn = Connections::new(
        &mut autocomplete_sys,
        indexes.get_name_id(),
        indexes.get_pagerank_hash(),
        indexes.get_word_index(),
    );
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        conn.handle_connection(stream);
    }
}

struct Connections<'a> {
    autocomplete: &'a mut AutoCompl,
    name_index: &'a HashMap<i32, String>,
    page_rank: HashMap<i32, f64>,
    word_index: &'a HashMap<String, HashMap<i32, i32>>,
}

impl<'a> Connections<'a> {
    pub fn new(
        autocomplete: &'a mut AutoCompl,
        name_index: &'a HashMap<i32, String>,
        page_rank: HashMap<i32, f64>,
        word_index: &'a HashMap<String, HashMap<i32, i32>>,
    ) -> Self {
        Self {
            autocomplete,
            name_index,
            page_rank,
            word_index,
        }
    }

    pub fn handle_connection(&mut self, mut stream: TcpStream) -> Result<(), std::io::Error> {
        let auto_path = "./src/auto.json";
        let buf_reader = BufReader::new(&stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        let (status_line, filename) = match request_line.as_str() {
            s if s.starts_with("GET / ") => (
                "HTTP/1.1 200 OK".to_string(),
                "./src/hello.html".to_string(),
            ),
            s if s.starts_with("GET /script.js") => {
                ("HTTP/1.1 200 OK".to_string(), "./src/script.js".to_string())
            }
            s if s.starts_with("GET /api/search?q=") => {
                self.autocomplete
                    .complete(&request_line, auto_path, self.name_index)
            }
            s if s.starts_with("GET /api/result?q=") => results::get_results(
                &request_line,
                &self.page_rank,
                self.word_index,
                self.name_index,
            ),
            _ => (
                "HTTP/1.1 404 NOT FOUND".to_string(),
                "./src/404.html".to_string(),
            ),
        };

        let contents = fs::read_to_string(filename)?;
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
        println!("Request: {request_line:#?}");
        Ok(())
    }
}
