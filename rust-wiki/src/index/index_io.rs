use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufReader, LineWriter, Write, prelude::*};
use std::mem::swap;
use std::path::Path;
use std::path::PathBuf;

pub fn load_id_name_index(id_name_path: &String) -> HashMap<String, i32> {
    let file = File::open(id_name_path).unwrap();
    let reader = BufReader::new(file);

    let map: HashMap<String, i32> = serde_json::from_reader(reader).unwrap();

    map
}

pub fn load_name_id_index(name_id_path: &String) -> HashMap<i32, String> {
    let file = File::open(name_id_path).unwrap();
    let reader = BufReader::new(file);

    let map: HashMap<i32, String> = serde_json::from_reader(reader).unwrap();

    map
}

pub fn load_id_list_index(id_list_path: &String) -> HashMap<i32, Vec<i32>> {
    let file = File::open(id_list_path).unwrap();
    let reader = BufReader::new(file);

    let map: HashMap<i32, Vec<i32>> = serde_json::from_reader(reader).unwrap();
    map
}

pub fn load_page_rank_index(page_rank_path: &String) -> Vec<(f64, f64)> {
    let file = File::open(page_rank_path).unwrap();
    let reader = BufReader::new(file);
    let mut pagerank_vec = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut it = line.split_whitespace();
        let id: f64 = it.next().unwrap().to_string().parse().unwrap();
        let value: f64 = it.next().unwrap().to_string().parse().unwrap();

        pagerank_vec.push((id, value));
    }

    pagerank_vec
}

pub fn load_word_index(word_index_path: &String) -> HashMap<String, HashMap<i32, i32>> {
    let file = File::open(word_index_path).unwrap();
    let reader = BufReader::new(file);

    let map: HashMap<String, HashMap<i32, i32>> = serde_json::from_reader(reader).unwrap();
    map
}
