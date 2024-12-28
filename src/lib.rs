use std::{cmp::Reverse, collections::BinaryHeap};

use ordered_float::OrderedFloat;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug)]
pub struct DocumentIndex{
    // score is just the sum of score of each word in the query
    // pub scores: HashMap<&'static str, f64>,
    pub index: usize,
    pub term_freq: FxHashMap<&'static str, usize>,
    pub doc_len: usize,
}


#[derive(Debug)]
pub struct OkapiBM25{
    // for each document, there is a Document Index that is inserted each time it is inserted 
    pub indices: Vec<DocumentIndex>,
    // we store the idf score of words to this idf hashmap so the query time is reduced
    pub idf: FxHashMap<&'static str, f64>,
    // to compute idf, we are interested in how many document contain a word
    doc_freq: FxHashMap<&'static str, usize>,
    // k1, b, avgdl
    k1: f64,
    b: f64,
    avgdl: f64,
}

impl OkapiBM25{
    pub fn new(k1: f64, b: f64) -> Self {
        OkapiBM25{
            indices: Vec::new(),
            idf: FxHashMap::default(),
            doc_freq: FxHashMap::default(), 
            avgdl: 0.0,
            k1, b
        }
    }

    pub fn len(&self) -> usize{
        self.indices.len()
    }

    pub fn insert(&mut self, new_doc: &'static str){
        // get index of the new document and size of the current indices database
        let index = self.len();
        let updated_num_doc = index + 1;
        // get the words
        let word_set = new_doc.split_whitespace().collect::<FxHashSet<_>>();
        let word_list = new_doc.split_whitespace().collect::<Vec<_>>();
        // update doc freq
        word_set.iter().for_each(|w| {
            let freq = self.doc_freq.entry(w).or_insert(0);
            *freq += 1;
            let new_idf = (self.k1+1.0)*(((updated_num_doc as f64) +1.0) / (*freq as f64 + 0.5)).ln();
            self.idf.insert(w, new_idf);
        });
        // update avg length
        self.avgdl = self.avgdl - (self.avgdl - word_list.len() as f64) / updated_num_doc as f64;
       
        let mut new_doc = DocumentIndex{
            index,
            term_freq: FxHashMap::default(),
            doc_len: word_list.len(),
        };
        word_list.iter().for_each(|word|{
            *new_doc.term_freq.entry(word).or_insert(0) += 1;
        });
        self.indices.push(new_doc);
    }

    pub fn search(&self, query: &str, top_k: usize)-> Vec<(usize,OrderedFloat<f64>)>{
        let query_words: Vec<_> = query.split_whitespace().collect();
        let query_idfs: Vec<_> = query_words.iter().map(|w| {
            *self.idf.get(w).unwrap_or(&0.0)
        }).collect();

        let mut top_k_docs = BinaryHeap::new();
        
        for (idx, doc) in self.indices.iter().enumerate(){
            let scaled_value = self.k1*(1.0-self.b+self.b*doc.doc_len as f64 / self.avgdl);
            let mut score = 0.0;
            for (i,&word) in query_words.iter().enumerate(){
                if let Some(&freq) = doc.term_freq.get(word){
                    score += query_idfs[i] / (1.0 + scaled_value/freq as f64);
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
        }
        let mut results = Vec::new();
        while let Some(Reverse((doc_id, score))) = top_k_docs.pop() {
            results.push((doc_id, score));
        }

        results.reverse();
        results
    }

}

