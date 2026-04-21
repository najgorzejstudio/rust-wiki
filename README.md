# Rust Wikipedia Search Engine

A lightweight search engine written in Rust that supports autocomplete and ranked search over a Wikipedia dataset.

---

## Features

* Autocomplete using a Trie (prefix tree)
* Search ranking using TF-IDF and PageRank
* Simple HTTP server built with the Rust standard library
* Docker support for easy setup

---

## Project Structure

```
.
├── src/
│   ├── autocomplete/
│   ├── index/
│   ├── results/
│   ├── Article/        # Dataset (not included)
│   └── data/           # Generated indexes
├── Dockerfile
├── .dockerignore
├── Cargo.toml
```

---

## Dataset

This project uses a Wikipedia dataset from Kaggle.

Download a dataset containing Wikipedia articles and place it in:

```
./src/Article/
```
##Dataset Structure

Each article is stored as a separate folder inside:

./src/Article/

Each folder contains multiple text files describing different aspects of the article:

Article/<id>/
├── articleLink.txt     # Wikipedia URL of the article
├── bodyText.txt        # Main article content
├── bodyLinks.txt       # Internal links within the article
├── externalLinks.txt   # External references
├── headingsText.txt    # Section headings

###Usage in This Project

articleLink.txt is used to extract and normalize article titles for the Trie (autocomplete)

bodyText.txt is used for TF-IDF computation

bodyLinks.txt is used to build the PageRank graph

Other files can be used for extended ranking or features

###Notes

Folder names act as unique article IDs

The dataset is not included in this repository due to size

Make sure the structure matches exactly, otherwise indexing will fail


---

## Running the Project

### Using Docker

Build the image:

```bash
docker build -t rust-wiki .
```

Run the container:

```bash
docker run -p 7878:7878 rust-wiki
```

Then open:

```
http://localhost:7878
```

---

### Running Locally

Standard run:

```bash
cargo run
```

Force new indexes:

```bash
cargo run -- newindex
```

<LeftMouse>
Each article folder should contain a file such as:

```
articleLink.txt
```

which stores the article URL.

Then open:

```
0.0.0.0:7878
```


---

## How It Works

### Autocomplete

Article titles are inserted into a Trie.
Each node stores a list of top-ranked article IDs, allowing efficient prefix search.

---

### Search Ranking

Results are ranked using a combination of:

* TF-IDF for term relevance
* PageRank for global importance

Each article is scored using a weighted combination of TF-IDF and PageRank:

score = 0.8 * (tfidf / max_tfidf) + 0.2 * (pagerank / max_pagerank)

Where:

* tfidf is the term relevance score for the query within the article

* pagerank represents the global importance of the article

* max_tfidf and max_pagerank are normalization constants (maximum values across all articles)

---

### Web Server

A simple HTTP server built using `TcpListener` handles:

* `/` → serves the frontend
* `/api/search?q=...` → returns search results as JSON

---

## Technologies

* Rust
* Standard library networking (`std::net`)
* Docker

---

## Notes

* Indexes are generated on first run
* Generated data is stored in `./src/data/`
* Startup may take longer if indexes need to be built

---

## Future Improvements

* Fuzzy search and typo tolerance
* Improved ranking strategies
* Parallel indexing
* Persistent storage backend
* Frontend improvements

---

## License

MIT (or your choice)

