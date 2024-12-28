# Okapi BM25 implementation

## Method:

This repository implements efficient BM-25 by:
- consider dynamic insertion: insert new document to the database dynamically
- preprocessing most of the information: information that are not related to the query are precomputed so that the search time is minimal
- use binary heap to find top k scores efficiently

## Usage:

Here is an example of how to use this repo:

```rust
use bm25::OkapiBM25;

fn main(){
    let documents = vec![
        String::from("dogs are pets"),
        String::from("cats are pets"),
        String::from("i have two dogs and three cats"),
    ];
    let mut bm25 = OkapiBM25::new(1.5, 0.75);
    for doc in &documents{
        bm25.insert(doc.clone().leak());
    }

    let query = "how many cats do you have";
    let result = bm25.search(query, 1);
    
    assert_eq!(documents[result[0].0], "i have two dogs and three cats");
}
```