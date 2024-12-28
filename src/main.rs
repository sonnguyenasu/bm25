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