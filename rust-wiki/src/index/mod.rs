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
    page_rank_id_index: Vec<(f64, f64)>,
    word_index: HashMap<String, HashMap<i32, i32>>,
    id_name_path: String,
    id_list_path: String,
    page_rank_path: String,
    word_index_path: String,
    data_path: String,
    dataset_path: String,
}

impl Index {
    pub fn new() -> Self {
        Self {
            id_name_index: HashMap::new(),
            id_list_index: HashMap::new(),
            page_rank_index: Vec::new(),
            page_rank_id_index: Vec::new(),
            word_index: HashMap::new(),
            id_name_path: String::from("./data/id_name.json"),
            id_list_path: String::from("./data/id_list.json"),
            page_rank_path: String::from("./data/pagerank_vals.txt"),
            word_index_path: String::from("./data/word_index.json"),
            data_path: String::from("./data/"),
            dataset_path: String::from("./Article/"),
        }
    }

    pub fn load(&mut self, new_idx: bool) {
        match fs::read_dir(&self.data_path) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(paths) => {
                let i = paths.count();
                println!("{}", i);
                if i < 4 || new_idx {
                    println!("loading indexes");
                    self.create_indexes().unwrap();
                } else {
                    self.load_indexes().unwrap();
                }
            }
        }
    }

    fn create_indexes(&mut self) -> io::Result<()> {
        let paths: Vec<std::path::PathBuf> = fs::read_dir(&self.dataset_path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .collect();

        builder::create_id_name_index(&paths, &self.id_name_path, &mut self.id_name_index)?;
        builder::create_id_list_index(
            &paths,
            &self.id_list_path,
            &mut self.id_list_index,
            &mut self.id_name_index,
        )?;
        builder::pagerank(
            &self.page_rank_path,
            &mut self.id_list_index,
            &mut self.page_rank_index,
            &mut self.page_rank_id_index,
        )?;
        builder::create_word_index(&paths, &self.word_index_path, &mut self.word_index)?;
        Ok(())
    }

    fn load_indexes(&mut self) -> io::Result<()> {
        self.id_name_index = index_io::load_id_name_index(&self.id_name_path);
        self.id_list_index = index_io::load_id_list_index(&self.id_list_path);
        self.page_rank_id_index = index_io::load_page_rank_index(&self.page_rank_path);
        self.word_index = index_io::load_word_index(&self.word_index_path);
        Ok(())
    }
}
