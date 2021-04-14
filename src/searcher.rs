use::std::collections::HashMap;



pub fn search(table: &HashMap<String,HashMap<String, i32>>, query: &str) -> HashMap<String,f64>{
    println!("Searching...");
    let query_vector: Vec<&str> = query.split(" ").collect();
    let mut doc_and_score: HashMap<String,f64> = HashMap::new();
    for key in table.keys(){
        let mut score_vec: Vec<f64> = Vec::new();
        for word in &query_vector {
            if table.get(key).unwrap().contains_key(word.to_owned()){
                //let _x = table.get(key).unwrap().get(word.to_owned()).unwrap();
                
                let word_score = calc_tf_idf(&word, &table.get(key).unwrap(), &table);
                score_vec.push(word_score);

                //println!("Doc: {} word: {} freq: {}  score: {}", key, word, x, word_score);
            }
        }
        let sum: f64 = score_vec.iter().sum();
        if sum != 0.0 {
            doc_and_score.insert(key.to_owned(), sum);
        }
        
    }
    return doc_and_score;
}

fn calc_tf(word: &str, document: &HashMap<String, i32>) -> f64{
    let value = document.get(word).unwrap().clone() as f64;
    let max_value = document.values().max().unwrap().clone() as f64;
    
    return value/max_value;
}

fn calc_idf(word: &str, table: &HashMap<String,HashMap<String, i32>>) -> f64{
    let doc_count = table.keys().len() as f64;
    let mut t_in_docs = 0 as f64;
    for key in table.keys() {
        if table.get(key).unwrap().contains_key(word){
            t_in_docs += 1.0;
        }
    }
    
    return f64::ln(doc_count/t_in_docs);
}

fn calc_tf_idf(word: &str, document: &HashMap<String, i32>, table: &HashMap<String,HashMap<String, i32>>) -> f64{
    let calc_idf = calc_idf(word, table);
    let calc_tf = calc_tf(word, document);
    return calc_tf * calc_idf;
}

