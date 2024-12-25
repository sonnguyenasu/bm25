use bm25::BM25Okapi;
// use std::collections::{HashMap, HashSet};

fn main(){
    let corpus: Vec<_> = include_str!("../train.csv").lines().skip(1)
            .map(|line| line.split(",").collect::<Vec<_>>()[2..].join(",").to_ascii_lowercase()).collect();
            
    let time = std::time::Instant::now();
    let bm25 = BM25Okapi::new(&corpus, 1.2, 0.75);
    let took = time.elapsed().as_secs_f64();
    println!("{:?} doc loaded in {} seconds", bm25.corpus.len(), took);
    let word = "lebron james";
    let time = std::time::Instant::now();
    let score = bm25.search(word, 10);
    let took = time.elapsed().as_secs_f64();
    println!("Top score: {}, took: {}", score[0].1, took);
    println!("Top sentences:");
    for i in 0..5{
        println!("- {}", corpus[score[i].0]);
    }
    let word = "what is the role of climate change in current society";
    let time = std::time::Instant::now();
    let score = bm25.search(word, 10);
    let took = time.elapsed().as_secs_f64();
    println!("Top score: {}, took: {}", score[0].1, took);
    println!("Top sentences:");
    for i in 0..5{
        println!("- {}", corpus[score[i].0]);
    }
}