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

    pub fn load(&mut self, new_idx: bool) {
        if !Path::new(&self.trie_path).exists() || new_idx {
            self.create_prie().unwrap();
        } else {
            self.load_prie().unwrap();
        }
    }

    fn create_prie(&mut self) -> io::Result<()> {
        let paths: Vec<std::path::PathBuf> = fs::read_dir(&self.dataset_path)
            .expect("dataset_path")
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .collect();

        trie::create_prefix_tree(&paths, &mut self.trie, &self.trie_path)?;
        Ok(())
    }

    fn load_prie(&mut self) -> io::Result<()> {
        self.trie = trie::load_prefix_tree(&self.trie_path);
        Ok(())
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
