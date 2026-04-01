use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufReader, LineWriter, Write, prelude::*};
use std::mem::swap;
use std::path::Path;
use std::path::PathBuf;

pub mod builder;
pub mod index_io;

#[derive(Serialize, Deserialize)]
pub(super) struct Index {
    id_name_index: HashMap<String, String>,
    id_list_index: HashMap<i32, Vec<i32>>,
    page_rank_index: Vec<f64>,
    page_rank_id_index: Vec<Vec<f64>>,
    word_index: HashMap<String, HashMap<i32, i32>>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            id_name_index: HashMap::new(),
            id_list_index: HashMap::new(),
            page_rank_index: Vec::new(),
            page_rank_id_index: Vec::new(),
            word_index: HashMap::new(),
        }
    }

    pub fn load(&mut self, new_idx: bool) {
        let data_path = String::from("data/");

        match fs::read_dir(data_path) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(paths) => {
                if paths.count() != 3 || new_idx {
                    self.create_indexes().unwrap();
                } else {
                    self.load_indexes();
                }
            }
        }
    }

    fn create_indexes(&mut self) -> io::Result<()> {
        let dataset_path = String::from("Article/");
        let id_name_path = String::from("data/id_name.json");
        let id_list_path = String::from("data/id_list.json");
        let page_rank_path = String::from("data/pagerank_vals.txt");
        let word_index_path = String::from("data/word_index.json");

        let paths: Vec<std::path::PathBuf> = fs::read_dir(dataset_path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .collect();

        builder::create_id_name_index(&paths, id_name_path, &mut self.id_name_index)?;
        builder::create_id_list_index(
            &paths,
            id_list_path,
            &mut self.id_list_index,
            &mut self.id_name_index,
        )?;
        builder::pagerank(
            page_rank_path,
            &mut self.id_list_index,
            &mut self.page_rank_index,
            &mut self.page_rank_id_index,
        )?;
        builder::create_word_index(&paths, word_index_path, &mut self.word_index)?;
        Ok(())
    }

    fn load_indexes(&mut self) -> io::Result<()> {
        Ok(())
    }
}
