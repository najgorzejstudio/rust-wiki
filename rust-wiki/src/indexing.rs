use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Index {
    id_name_index: HashMap<String, String>,
    id_list_index: HashMap<String, Vec<i32>>,
    page_rank_index: HashMap<i32, i32>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            id_name_index: HashMap::new(),
            id_list_index: HashMap::new(),
            page_rank_index: HashMap::new(),
        }
    }

    pub fn load(&mut self, new_idx: bool) {
        let data_path = String::from("data/");

        match fs::read_dir(data_path) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(paths) => {
                if paths.count() != 3 || new_idx {
                    self.create_id_name_index(); //and others
                } // else load files
            }
        }
    }

    fn create_id_name_index(&mut self) -> io::Result<()> {
        let dataset_path = String::from("Article/");
        let id_name_path = String::from("data/id_name.json");

        let paths = fs::read_dir(dataset_path).unwrap();
        for path in paths {
            let path_u = path.unwrap();
            let path_buf = path_u.path();
            let name: String = fs::read_to_string(path_buf.join("articleLink.txt")).unwrap();
            let id: String = path_buf.file_name().unwrap().to_str().unwrap().to_string();

            self.id_name_index.insert(id.to_string(), name.to_string());
        }
        let json = serde_json::to_string(&self.id_name_index).unwrap();
        let mut f = File::create(id_name_path)?;
        f.write_all(json.as_bytes())
    }
}
