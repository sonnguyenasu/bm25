// use bm25::*;

// fn main(){
//     let document = vec![
//         String::from("how are you !"),
//         String::from("hello jack! nice to meet you!"),
//         String::from("i am from china, i like math."),
//     ];
//     let mut bm25 = OkapiBM25::new(1.0,0.75);
//     println!("Total document inserted: {}",bm25.len());
//     for doc in document{
//         bm25.insert(doc.leak());
//     }
//     println!("Total document inserted: {}",bm25.len());
//     println!("{:?}", bm25);
//     println!("{:?}", bm25.search("where are you from", 3));
// }

// use bm25::OkapiBM25;

// use std::collections::HashMap;
// use std::fs::File;
// use std::io::Write;
// use std::process::Command;
// use std::time::Instant;

// const DOC_LENGTH: usize = 256; // Each document is 256 words long
// const NUM_DOCS: [usize; 4] = [100, 1000, 5000, 120000]; // Adjust as needed for testing

// // Helper function to generate a document with exactly 256 words
// fn generate_document(id: usize) -> String {
//     let words: Vec<String> = (0..DOC_LENGTH).map(|i| format!("word{}_{}", id, i)).collect();
//     let document = words.join(" ");
    
//     document
// }

// fn measure_memory() -> Option<u64> {
//     let process_id = std::process::id().to_string();

//     // PowerShell command to get memory usage of the current process
//     let output = Command::new("powershell")
//         .arg("-Command")
//         .arg(format!("Get-Process -Id {} | Select-Object -ExpandProperty WorkingSet", process_id))
//         .output()
//         .ok()?;

//     let memory_usage_str = String::from_utf8_lossy(&output.stdout);
//     memory_usage_str.trim().parse().ok()
// }


// fn main() {
//     let mut file = File::create("scalability_results.csv").expect("Failed to create file");
//     writeln!(file, "NumDocs,IndexTime,SearchTime,MemoryUsage").expect("Failed to write header");

//     for &num_docs in NUM_DOCS.iter() {
//         println!("Running benchmark for {} documents", num_docs);

//         // get initial memory usage
//         let initial_memory_usage = measure_memory().unwrap_or(0);

//         // let mut index = Index::new();
//         // let index = BM25Okapi()
        
//         // Insert documents
//         let insert_start_time = Instant::now();
//         let mut docs = Vec::new();
//         for i in 0..num_docs {
//             let doc = generate_document(i);
//             docs.push(doc);
//             // index.upsert(&doc, i as u32);
//         }
//         let mut index = OkapiBM25::new(1.5, 0.75);
//         for doc in docs{
//             index.insert(doc.leak());
//         }
//         let index_time = insert_start_time.elapsed().as_secs_f64();

//         // Perform a search
//         let search_query = "word0_0 word1_1 word2_2 word0_10 word11_11 word12_12 word20_20 word11_1 word22_2"; // Search query for benchmarking
//         let search_start_time = Instant::now();
//         let result = index.search(search_query, 10);
//         let search_time = search_start_time.elapsed().as_secs_f64();
//         println!("{:?}",result);

//         // Measure memory usage now
//         let memory_usage = (measure_memory().unwrap_or(0) - initial_memory_usage) as f64 / 1024.0;

//         writeln!(file, "{},{},{},{}", num_docs, index_time, search_time, memory_usage).expect("Failed to write data");
        
//         println!("Number of Documents: {}", num_docs);
//         println!("Index Time: {:.5}s", index_time);
//         println!("Search Time: {:.5}s", search_time);
//         println!("Memory Usage: {} KB", memory_usage);
//     }
// }


use bm25::*;
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

    // let corpus: Vec<_> = include_str!("../train.csv").lines().skip(1)
    // .map(|line| line.split(",").collect::<Vec<_>>()[2..].join(",").to_ascii_lowercase()).collect();

    let time = std::time::Instant::now();
    let mut bm25 = OkapiBM25::new(1.2, 0.75);
    for doc in &corpus{
        bm25.insert(doc.clone().leak());
    }
    let took = time.elapsed().as_secs_f64();
    println!("{:?} doc loaded in {} seconds", bm25.len(), took);
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
