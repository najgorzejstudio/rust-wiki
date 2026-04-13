use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufReader, LineWriter, Write, prelude::*};
use std::mem::swap;
use std::path::Path;
use std::path::PathBuf;

pub mod trie;

pub(super) struct AutoCompl {
    trie: TrieNode,
    trie_path: String,
    data_path: String,
    dataset_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct TrieNode {
    children: HashMap<char, TrieNode>,
    top_articles: Vec<(i32, f64)>,
    is_end: bool,
}

impl AutoCompl {
    pub fn new() -> Self {
        Self {
            trie: TrieNode::new(),
            trie_path: String::from("./data/prie.json"),
            data_path: String::from("../data/"),
            dataset_path: String::from("./Article/"),
        }
    }

    pub fn load(&mut self, new_idx: bool, pagerank: &Vec<(f64, f64)>) {
        if !Path::new(&self.trie_path).exists() || new_idx {
            self.create_prie(pagerank).unwrap();
        } else {
            self.load_prie().unwrap();
        }
    }

    fn create_prie(&mut self, pagerank: &Vec<(f64, f64)>) -> io::Result<()> {
        let paths: Vec<std::path::PathBuf> = fs::read_dir(&self.dataset_path)
            .expect("dataset_path")
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .collect();

        trie::create_prefix_tree(&paths, &mut self.trie, &self.trie_path, pagerank)?;
        Ok(())
    }

    fn load_prie(&mut self) -> io::Result<()> {
        self.trie = trie::load_prefix_tree(&self.trie_path);
        Ok(())
    }

    pub fn complete(
        &mut self,
        text: &String,
        auto_path: &str,
        name_index: &HashMap<i32, String>,
    ) -> (String, String) {
        let mut line: String = text.clone();
        line = line.strip_prefix("GET /api/search?q=").unwrap().to_string();
        let line_clean = line
            .strip_suffix(" HTTP/1.1")
            .unwrap()
            .to_string()
            .to_lowercase();
        let letters: Vec<char> = line_clean.chars().collect();
        let mut list = trie::search_trie(letters, &self.trie);
        let mut titles: Vec<(String, String)> = vec![];
        for id in list {
            let link = name_index[&id.0].clone();
            let title = name_index[&id.0]
                .clone()
                .strip_prefix("https://en.wikipedia.org//wiki/")
                .unwrap()
                .replace("_", " ")
                .replace("%", " ")
                .trim()
                .to_string();
            titles.push((title, link));
        }

        let json = serde_json::to_string(&titles).unwrap();
        let mut f = File::create(auto_path).unwrap();
        f.write_all(json.as_bytes()).unwrap();
        (
            "HTTP/1.1 200 OK\r\nContent-Type: application/json".to_string(),
            auto_path.to_string(),
        )
    }
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            top_articles: Vec::new(),
            is_end: false,
        }
    }
}
