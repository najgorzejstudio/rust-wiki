use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub fn get_results(
    query: &String,
    page_rank: &HashMap<i32, f64>,
    word_index: &HashMap<String, HashMap<i32, i32>>,
    name_index: &HashMap<i32, String>,
) -> (String, String) {
    let line: String = query.clone();
    let title = line.strip_prefix("GET /api/result?q=").unwrap().to_string();
    let mut line_clean = title
        .strip_suffix(" HTTP/1.1")
        .unwrap()
        .to_string()
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
    let max_tfidf = rank_hash.values().cloned().fold(0.0, f64::max);
    let max_pagerank = page_rank.values().cloned().fold(0.0, f64::max);
    let mut scores: Vec<(String, String, f64)> = Vec::new();

    for (key, value) in rank_hash {
        let pagerank_score = page_rank.get(&key).unwrap();
        let score = value as f64 / max_tfidf as f64 * 0.2
            + *pagerank_score as f64 / max_pagerank as f64 * 0.8;
        let link = name_index.get(&key).unwrap().to_string();
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
    let json = serde_json::to_string(&scores).unwrap();
    let mut f = File::create("./src/res.json").unwrap();
    f.write_all(json.as_bytes()).unwrap();

    (
        "HTTP/1.1 200 OK\r\nContent-Type: application/json".to_string(),
        "./src/res.json".to_string(),
    )
}

fn tfidf(value: f64, len: f64, n: f64) -> f64 {
    1.0 + (value as f64).log10() * (n / len).log10()
}
