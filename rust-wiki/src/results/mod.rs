use std::collections::HashMap;
use std::fs::File;
use std::io::{Result, Write};

pub fn get_results(
    query: &String,
    page_rank: &HashMap<i32, f64>,
    word_index: &HashMap<String, HashMap<i32, i32>>,
    name_index: &HashMap<i32, String>,
) -> Result<(String, String)> {
    let mut line_clean = query
        .strip_prefix("GET /api/result?q=")
        .and_then(|s| s.strip_suffix(" HTTP/1.1"))
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid request"))?
        .to_lowercase();

    line_clean = line_clean.replace("%20", " ").trim().to_string();
    let title_vec: Vec<&str> = line_clean.split(" ").collect();
    let mut rank_hash: HashMap<i32, f64> = HashMap::new();
    let n = page_rank.len();

    for word in title_vec {
        if let Some(map) = word_index.get(word) {
            for (doc, val) in map {
                rank_hash
                    .entry(*doc)
                    .and_modify(|x| *x += tfidf(*val as f64, map.len() as f64, n as f64))
                    .or_insert(tfidf(*val as f64, map.len() as f64, n as f64));
            }
        }
    }
    if rank_hash.len() == 0 {
        let mut em_vec: Vec<(String, String, f64)> = Vec::new();
        em_vec.push(("No results".to_string(), "".to_string(), 0.0));
        let json = serde_json::to_string(&em_vec)?;
        let mut f = File::create("./src/res.json")?;
        f.write_all(json.as_bytes())?;

        return Ok((
            "HTTP/1.1 200 OK\r\nContent-Type: application/json".to_string(),
            "./src/res.json".to_string(),
        ));
    }

    let max_tfidf = rank_hash.values().cloned().fold(0.0, f64::max);
    let max_pagerank = page_rank.values().cloned().fold(0.0, f64::max);
    let mut scores: Vec<(String, String, f64)> = Vec::new();

    for (key, value) in rank_hash {
        let pagerank_score = page_rank.get(&key).copied().unwrap_or(0.0);
        let score = value as f64 / max_tfidf as f64 * 0.2
            + pagerank_score as f64 / max_pagerank as f64 * 0.8;
        let link = match name_index.get(&key) {
            Some(l) => l.to_string(),
            None => continue,
        };
        let title = link
            .clone()
            .strip_prefix("https://en.wikipedia.org//wiki/")
            .unwrap()
            .replace("_", " ")
            .replace("%", " ")
            .trim()
            .to_string();
        scores.push((title, link, score as f64));
    }

    scores.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    println!("{}", scores[0].1);
    let json = serde_json::to_string(&scores)?;
    let mut f = File::create("./src/res.json")?;
    f.write_all(json.as_bytes())?;

    Ok((
        "HTTP/1.1 200 OK\r\nContent-Type: application/json".to_string(),
        "./src/res.json".to_string(),
    ))
}

fn tfidf(value: f64, len: f64, n: f64) -> f64 {
    1.0 + (value as f64).log10() * (n / len).log10()
}
