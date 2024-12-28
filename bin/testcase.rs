use bm25::BM25Okapi;
use parquet::file::reader::SerializedFileReader;
use std::fs::File;
// use parquet::async_reader::ParquetRecordBatchStreamBuilder;

fn main() {
    let file =
        File::open("/Users/sonnguyen/research/languages/rust/rust_projects/bm25/train.parquet")
            .unwrap();

    let reader = SerializedFileReader::new(file).unwrap();
    let corpus = reader
        .into_iter()
        .map(|row| {
            let row = row.unwrap();
            row.into_columns()
        })
        .map(|row| row[3].1.to_string().to_ascii_lowercase())
        .collect::<Vec<_>>();

    // let corpus: Vec<_> = include_str!("../../train.csv").lines().skip(1)
    // .map(|line| line.split(",").collect::<Vec<_>>()[2..].join(",").to_ascii_lowercase()).collect();

    let time = std::time::Instant::now();
    let bm25 = BM25Okapi::new(&corpus, 1.2, 0.75);
    let took = time.elapsed().as_secs_f64();
    println!("{:?} doc loaded in {} seconds", bm25.corpus.len(), took);
    let word = "cryptography expert";
    let time = std::time::Instant::now();
    let score = bm25.search(word, 10);
    let took = time.elapsed().as_secs_f64();
    println!("Top score: {}, took: {}", score[0].1, took);
    println!("Top sentences:");
    for i in 0..10 {
        println!("- {}", corpus[score[i].0]);
    }
    // let word = "climate change united states";
    // let time = std::time::Instant::now();
    // let score = bm25.search(word, 10);
    // let took = time.elapsed().as_secs_f64();
    // println!("Top score: {}, took: {}", score[0].1, took);
    // println!("Top sentences:");
    // for i in 0..5{
    //     println!("- {}", corpus[score[i].0]);
    // }
    loop {
        println!("enter query:");
        let mut word = String::new();
        std::io::stdin()
            .read_line(&mut word)
            .expect("failed to read line");
        let score = bm25.search(&word, 5);
        println!("Top score: {}, took: {}", score[0].1, took);
        println!("Top sentences:");
        for i in 0..5 {
            println!("- {}", corpus[score[i].0]);
        }
    }
}
