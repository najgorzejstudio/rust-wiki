use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufReader, LineWriter, Write, prelude::*};
use std::mem::swap;
use std::path::Path;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Index {
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
                } // else load files
            }
        }
    }

    fn create_indexes(&mut self) -> io::Result<()> {
        let dataset_path = String::from("Article/");
        let id_name_path = String::from("data/id_name.json");
        let id_list_path = String::from("data/id_list.json");
        let page_rank_path = String::from("data/pagerank_vals.txt");
        let word_index = String::from("data/word_index.json");

        let paths: Vec<std::path::PathBuf> = fs::read_dir(dataset_path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .collect();

        self.create_id_name_index(&paths, id_name_path)?;
        self.create_id_list_index(&paths, id_list_path)?;
        self.pagerank(page_rank_path)?;
        self.create_word_index(&paths, word_index);
        Ok(())
    }

    fn create_id_name_index(&mut self, paths: &[PathBuf], id_name_path: String) -> io::Result<()> {
        println!("Creating id_name index...");

        for path in paths {
            let name: String = fs::read_to_string(path.join("articleLink.txt")).unwrap();
            let id: String = path.file_name().unwrap().to_str().unwrap().to_string();

            self.id_name_index.insert(
                name.to_string().parse().unwrap(),
                id.to_string().parse().unwrap(),
            );
        }
        let json = serde_json::to_string(&self.id_name_index).unwrap();
        let mut f = File::create(id_name_path)?;
        f.write_all(json.as_bytes())?;
        Ok(())
    }

    fn create_id_list_index(&mut self, paths: &[PathBuf], id_list_path: String) -> io::Result<()> {
        println!("Creating id_list index...");

        for path in paths {
            let id: String = path.file_name().unwrap().to_str().unwrap().to_string();
            let file = File::open(path.join("bodyLinks.txt"))?;
            let reader = BufReader::new(file);
            let mut link_list: Vec<i32> = Vec::new();

            for line in reader.lines() {
                if let Some(i) = self.id_name_index.get(&line.unwrap()) {
                    if !link_list.contains(&i.parse().unwrap()) {
                        link_list.push(i.to_string().clone().parse().unwrap());
                    }
                }
            }
            self.id_list_index
                .insert(id.to_string().parse().unwrap(), link_list.clone());
        }
        let json = serde_json::to_string(&self.id_list_index)?;
        let mut f = File::create(id_list_path)?;
        f.write_all(json.as_bytes())?;
        Ok(())
    }

    fn pagerank(&mut self, pagerank_path: String) -> io::Result<()> {
        println!("Creating pagerank index...");

        let d = 0.85;
        let iters = 50;
        let n = self.id_list_index.len();
        let n_f = n as f64;
        self.page_rank_index = vec![1.0 / n_f; n];
        println!("max id = {:?}", self.id_list_index.keys().max());
        println!("n = {}", n);

        for _ in 0..iters {
            let mut dangling_sum = 0.0;
            let mut new_rank = vec![(1.0 - d) / n_f; n];

            for (&j, links) in self.id_list_index.iter() {
                let rank_j = self.page_rank_index[j as usize];

                if links.is_empty() {
                    dangling_sum += rank_j;
                } else {
                    let share = rank_j / links.len() as f64;

                    for &k in links {
                        new_rank[k as usize] += d * share;
                    }
                }
            }
            let dangling_distr = d * dangling_sum / n_f;
            for i in 0..n {
                new_rank[i] += dangling_distr;
            }
            swap(&mut self.page_rank_index, &mut new_rank);
        }
        for (j, &i) in self.page_rank_index.iter().enumerate() {
            self.page_rank_id_index.push(vec![j as f64, i]);
        }
        let mut file = File::create(pagerank_path)?;

        for i in &self.page_rank_id_index {
            writeln!(file, "{} {}", i[0], i[1])?;
        }

        Ok(())
    }

    fn create_word_index(&mut self, paths: &[PathBuf], index_path: String) -> io::Result<()> {
        println!("Creating word index...");

        let stop_words = [
            "a", "an", "the", "and", "or", "but", "if", "then", "else", "when", "at", "by", "for",
            "in", "of", "on", "to", "with", "from", "as", "is", "are", "was", "were", "be", "been",
            "being", "have", "has", "had", "do", "does", "did", "will", "would", "shall", "should",
            "can", "could", "may", "might", "must", "i", "you", "he", "she", "it", "we", "they",
            "me", "him", "her", "us", "them", "my", "your", "his", "its", "our", "their", "mine",
            "yours", "hers", "ours", "theirs", "this", "that", "these", "those", "am", "not", "no",
            "nor", "so", "too", "very", "there", "here", "where", "why", "how", "all", "any",
            "both", "each", "few", "more", "most", "other", "some", "such", "only", "own", "same",
            "than", "again", "further", "then", "once", "about", "above", "below", "under", "over",
            "between", "into", "out", "off", "because", "while", "during", "before", "after",
        ];
        let chars_to_trim: &[char] = &[
            ',', '.', '(', ')', '/', '?', '!', '{', '}', '[', ']', '\\', '"',
        ];
        for path in paths {
            let id: i32 = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                .parse()
                .unwrap();
            let file = File::open(path.join("bodyText.txt"))?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let split = line.unwrap();
                let split_line = split.split_whitespace();
                for word in split_line {
                    let word = word.to_string();
                    let word = word.trim_matches(chars_to_trim).to_lowercase();
                    if !stop_words.contains(&word.as_str()) && word.chars().all(char::is_alphabetic)
                    {
                        let h = self
                            .word_index
                            .entry(word.to_string())
                            .or_insert(HashMap::new());
                        match h.get_mut(&id) {
                            Some(amount) => *amount += 1,
                            None => {
                                h.insert(id, 1);
                            }
                        }
                    }
                }
            }
        }
        let json = serde_json::to_string(&self.word_index)?;
        let mut f = File::create(index_path)?;
        f.write_all(json.as_bytes())?;
        Ok(())
    }
}
