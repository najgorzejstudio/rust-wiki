use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Result, Write};
use std::path::Path;

pub mod trie;

pub(super) struct AutoCompl {
    trie: TrieNode,
    trie_path: String,
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
            trie_path: String::from("./src/data/prie.json"),
            dataset_path: String::from("./src/Article/"),
        }
    }

    pub fn load(&mut self, new_idx: bool, pagerank: &Vec<(f64, f64)>) -> Result<()> {
        if !Path::new(&self.trie_path).exists() || new_idx {
            self.create_prie(pagerank)?;
        } else {
            self.load_prie()?;
        }
        Ok(())
    }

    fn create_prie(&mut self, pagerank: &Vec<(f64, f64)>) -> Result<()> {
        let paths: Vec<std::path::PathBuf> = fs::read_dir(&self.dataset_path)
            .expect("dataset_path")
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .collect();

        trie::create_prefix_tree(&paths, &mut self.trie, &self.trie_path, pagerank)?;
        Ok(())
    }

    fn load_prie(&mut self) -> Result<()> {
        self.trie = trie::load_prefix_tree(&self.trie_path)?;
        Ok(())
    }

    pub fn complete(
        &mut self,
        text: &String,
        auto_path: &str,
        name_index: &HashMap<i32, String>,
    ) -> Result<(String, String)> {
        let line_clean = text
            .strip_prefix("GET /api/search?q=")
            .and_then(|s| s.strip_suffix(" HTTP/1.1"))
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid request")
            })?
            .to_lowercase();
        let letters: Vec<char> = line_clean.chars().collect();
        let list = trie::search_trie(letters, &self.trie);
        let mut titles: Vec<(String, String)> = vec![];
        for id in list {
            let link = match name_index.get(&id.0) {
                Some(l) => l,
                None => continue,
            };
            let title = link
                .strip_prefix("https://en.wikipedia.org//wiki/")
                .unwrap_or(link)
                .replace("_", " ")
                .replace("%", " ")
                .trim()
                .to_string();
            titles.push((title, link.clone()));
        }

        let json = serde_json::to_string(&titles)?;
        let mut f = File::create(auto_path)?;
        f.write_all(json.as_bytes())?;
        Ok((
            "HTTP/1.1 200 OK\r\nContent-Type: application/json".to_string(),
            auto_path.to_string(),
        ))
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
