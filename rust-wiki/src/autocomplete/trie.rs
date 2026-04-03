use super::TrieNode;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufReader, LineWriter, Write, prelude::*};
use std::mem::swap;
use std::path::Path;
use std::path::PathBuf;

pub fn create_prefix_tree(
    paths: &[PathBuf],
    trie: &mut TrieNode,
    trie_path: &String,
) -> io::Result<()> {
    println!("Creating trie");
    for path in paths {
        let mut id: String = path.file_name().unwrap().to_str().unwrap().to_string();
        id = id.parse().unwrap();
        let mut name: String = fs::read_to_string(path.join("articleLink.txt"))
            .expect("articlelink")
            .to_string();
        name = name.replace("_", " ").trim().to_string();
        name = name
            .strip_prefix("https://en.wikipedia.org//wiki/")
            .unwrap_or_else(|| panic!("bad prefix in: {} {}", id, name))
            .to_string()
            .to_lowercase();
        let letters: Vec<char> = name.chars().collect();
        let mut trie_index = &mut *trie;
        for letter in letters {
            let new_node = TrieNode::new();
            trie_index = trie_index.children.entry(letter).or_insert(new_node);
            trie_index.top_articles.push((id.parse().unwrap(), 0.01));
        }
        trie_index.is_end = true;
    }
    let json = serde_json::to_string(&trie).unwrap();
    let mut f = File::create(trie_path)?;
    f.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_prefix_tree(path: &String) -> TrieNode {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let trie: TrieNode = serde_json::from_reader(reader).unwrap();
    trie
}
