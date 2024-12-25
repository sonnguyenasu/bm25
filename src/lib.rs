use std::collections::{BinaryHeap};
use rustc_hash::FxHashMap;
use std::cmp::Reverse;
use ordered_float::OrderedFloat;


pub struct BM25Okapi{
    pub corpus: Vec<Vec<String>>,
    pub k1: f64,
    pub b: f64,
    pub avgdl: f64,
    pub idf: FxHashMap<String, f64>,
    pub doc_freq: Vec<FxHashMap<String, u32>>,
}

impl BM25Okapi{
    pub fn new(corpus: &Vec<String>, k1: f64, b: f64) -> Self{
        let corpus: Vec<_> = corpus.iter()
        .map(|doc| doc.split_whitespace().map(|s| s.to_string()).collect())
        .collect();
        let avgdl = corpus.iter().map(|doc: &Vec<String>| doc.len()).sum::<usize>() as f64 / corpus.len() as f64;
        let mut doc_freq = Vec::with_capacity(corpus.len());
        let mut nd = FxHashMap::default();
        for doc in corpus.iter(){
            let mut doc_freq_map = FxHashMap::default();
            for term in doc.iter(){
                *nd.entry(term.clone()).or_insert(0) += 1;
                *doc_freq_map.entry(term.clone()).or_insert(0) += 1;
            }
            doc_freq.push(doc_freq_map);
        }
        let idf = BM25Okapi::calc_idf(nd, corpus.len() as f64);//HashMap::new();
        Self{corpus, k1, b, avgdl, idf, doc_freq}
    }

    fn calc_idf(corpus: FxHashMap<String, usize>, doc_len: f64) -> FxHashMap<String, f64>{
        let mut idf = FxHashMap::default();
        // let n = corpus.len() as f64;
        for (term, freq) in corpus.iter(){
            idf.insert(term.clone(), ((doc_len - *freq as f64 + 0.5) / (*freq as f64 + 0.5) + 1.0).ln());
        }
        idf
    }

    pub fn search(&self, query: &str, top_k: usize)-> Vec<(usize,OrderedFloat<f64>)>{
        let query_words: Vec<_> = query.split_whitespace().collect();

        let idfs: Vec<_> = query_words.iter()
                            .map(|&w| {
                                self.idf.get(w).unwrap_or(&0.0)
                            }).collect();

        let mut top_k_docs = BinaryHeap::new();
        
        for (idx,(doc, doc_freq)) in self.corpus.iter().zip(self.doc_freq.iter()).enumerate(){
            let scale = doc.len() as f64 / self.avgdl;
            let mut score = 0.0;
            for (i, &word) in query_words.iter().enumerate(){
                if let Some(&freq) = doc_freq.get(word){
                    let freq = freq as f64;
                    let idf = idfs[i];//.get(i).unwrap();
                    score += idf*(self.k1+1.0) / (1.0 + self.k1*(1.0-self.b+self.b*scale)/freq);
                }
            }
            if top_k_docs.len() < top_k as usize {
                top_k_docs.push(Reverse((idx, OrderedFloat(score), )));
            } else if let Some(&Reverse((_, lowest_score))) = top_k_docs.peek() {
                if OrderedFloat(score) > lowest_score {
                    top_k_docs.pop();
                    top_k_docs.push(Reverse((idx,OrderedFloat(score))));
                }
            }
            // scores
        }
        // top_k_docs.into_iter().map(|Reverse((doc_id, score))| (score, doc_id as u32)).collect()
        let mut results = Vec::new();
        while let Some(Reverse((doc_id, score))) = top_k_docs.pop() {
            results.push((doc_id, score));
        }

        results.reverse();
        results
    }

}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_bm25(){
//         let corpus = vec![
//             "hello world".to_string(),
//             "world world world".to_string(),
//             "hello hello world".to_string(),
//         ];
//         let bm25 = BM25Okapi::new(&corpus, 1.2, 0.75);
//         eprintln!("{:?}", bm25.idf);
//         assert_eq!(bm25.corpus.len(), 3);
//         assert_eq!(bm25.avgdl, 2.6666666666666665);
//         // assert_eq!(bm25.idf["hello"], 0.6);
//     }

//     #[test]
//     fn test_load_bm25(){
//         let corpus: Vec<_> = include_str!("../train.csv").lines().skip(1)
//         .map(|line| line.split(",").nth(2).unwrap().to_string()).collect();
//         let time = std::time::Instant::now();
//         let bm25 = BM25Okapi::new(&corpus, 1.2, 0.75);
//         let took = time.elapsed().as_secs();
//         println!("{:?} doc loaded in {} seconds", bm25.corpus.len(), took);
//         println!("{}", bm25.get_scores("united")[0]);
//         assert!(true);
//     }
// }
