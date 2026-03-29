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
}

impl Index {
    pub fn new() -> Self {
        Self {
            id_name_index: HashMap::new(),
            id_list_index: HashMap::new(),
            page_rank_index: Vec::new(),
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
        let page_rank_path = String::from("data/page_rank_index.json");

        let paths: Vec<std::path::PathBuf> = fs::read_dir(dataset_path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .collect();

        self.create_id_name_index(&paths, id_name_path)?;
        self.create_id_list_index(&paths, id_list_path)?;
        self.pagerank()?;
        Ok(())
    }

    fn create_id_name_index(&mut self, paths: &[PathBuf], id_name_path: String) -> io::Result<()> {
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

    fn pagerank(&mut self) -> io::Result<()> {
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
        let mut file = File::create("data/pagerank_vals.txt")?;

        for &i in &self.page_rank_index {
            writeln!(file, "{}", i)?;
        }
        Ok(())
    }
}
