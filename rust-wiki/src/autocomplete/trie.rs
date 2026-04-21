use super::TrieNode;
use std::fs::{self, File};
use std::io::{self, BufReader, Error, Result};
use std::path::PathBuf;

pub fn create_prefix_tree(
    paths: &[PathBuf],
    trie: &mut TrieNode,
    trie_path: &String,
    pagerank: &Vec<(f64, f64)>,
) -> io::Result<()> {
    println!("Creating trie");
    for path in paths {
        let id: i32 = path
            .file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing filename"))?
            .to_str()
            .unwrap()
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let mut name: String = fs::read_to_string(path.join("articleLink.txt"))?.to_string();
        name = name.replace("_", " ").trim().to_lowercase().to_string();
        let name = match name.strip_prefix("https://en.wikipedia.org//wiki/") {
            Some(n) => n,
            None => {
                eprintln!("bad prefix in: {} {}", id, name);
                continue;
            }
        };
        let mut value: f64 = 0.0;
        let pagerank_cp = pagerank.clone();
        for val in pagerank_cp.iter() {
            if val.0 == id as f64 {
                value = val.1;
            }
        }

        let letters: Vec<char> = name.chars().collect();
        let mut trie_index = &mut *trie;
        for letter in letters {
            let new_node = TrieNode::new();
            trie_index = trie_index.children.entry(letter).or_insert(new_node);
            trie_index.top_articles.push((id, value));
        }
        trie_index.is_end = true;
    }
    let f = File::create(trie_path)?;
    bincode::serialize_into(f, &trie).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}

pub fn load_prefix_tree(path: &String) -> Result<TrieNode> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let trie: TrieNode =
        bincode::deserialize_from(reader).map_err(|e| Error::new(io::ErrorKind::Other, e))?;
    Ok(trie)
}

pub fn search_trie(letters: Vec<char>, trie: &TrieNode) -> Vec<(i32, f64)> {
    let mut trie_index = &*trie;
    let mut list: Vec<(i32, f64)> = vec![];
    for letter in letters {
        if let Some(next) = trie_index.children.get(&letter) {
            trie_index = next;
            list = trie_index.top_articles.clone();
        } else {
            list.clear();
            break;
        }
    }
    list.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    if list.len() >= 5 {
        let result = &list[0..5];
        result.to_vec()
    } else {
        list
    }
}
