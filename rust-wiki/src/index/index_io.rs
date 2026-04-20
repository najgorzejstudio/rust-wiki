use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Result, prelude::*};

pub fn load_id_name_index(id_name_path: &String) -> Result<HashMap<String, i32>> {
    let file = File::open(id_name_path)?;
    let reader = BufReader::new(file);

    let map: HashMap<String, i32> = serde_json::from_reader(reader)?;

    Ok(map)
}

pub fn load_name_id_index(name_id_path: &String) -> Result<HashMap<i32, String>> {
    let file = File::open(name_id_path)?;
    let reader = BufReader::new(file);

    let map: HashMap<i32, String> = serde_json::from_reader(reader)?;

    Ok(map)
}

pub fn load_id_list_index(id_list_path: &String) -> Result<HashMap<i32, Vec<i32>>> {
    let file = File::open(id_list_path)?;
    let reader = BufReader::new(file);

    let map: HashMap<i32, Vec<i32>> = serde_json::from_reader(reader)?;
    Ok(map)
}

pub fn load_page_rank_index(page_rank_path: &String) -> Result<Vec<(f64, f64)>> {
    let file = File::open(page_rank_path)?;
    let reader = BufReader::new(file);
    let mut pagerank_vec = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut it = line.split_whitespace();
        let id: f64 = it
            .next()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Missing id"))?
            .to_string()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        let value: f64 = it
            .next()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Missing value"))?
            .to_string()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        pagerank_vec.push((id, value));
    }

    Ok(pagerank_vec)
}

pub fn load_word_index(word_index_path: &String) -> Result<HashMap<String, HashMap<i32, i32>>> {
    let file = File::open(word_index_path)?;
    let reader = BufReader::new(file);

    let map: HashMap<String, HashMap<i32, i32>> = serde_json::from_reader(reader)?;
    Ok(map)
}
