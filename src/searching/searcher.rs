use::std::collections::HashMap;
use crate::utils::preprocess_text::tokenize_filter_special_characters;
use crate::indexing::postlist::Posting;

/*
1. Preprocess Query wie Index
2. Waehle alle Postlisten aus dem Index fuer jeden Tokens aus der Query
3. Merke alle Dokumente mit dem jeweiligen Gewicht
4. Berechne den jeweiligen Score des Tokens
 */

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct retrieved_document{
    pub score: f64,
    pub document_name: String
}



pub fn search(index:HashMap<String,Vec<Posting>> , document_count: usize, query: &str, first_n_documents: usize)-> Vec<retrieved_document>{
    println!("Searching");
    let query_tokens = tokenize_filter_special_characters(query);
    let mut retrieved_document_index: HashMap<String, f64> = HashMap::new();

    for token in query_tokens{
        let postlist = index.get(&*token).unwrap();
        let document_count_with_term = postlist.len();
        for posting in postlist.to_owned(){
            let term_score = calc_tf_idf(document_count, document_count_with_term, posting.weight);
            *retrieved_document_index.entry(posting.document_name).or_insert(term_score) += term_score;
        }
    }

    let mut retrieved_document_list: Vec<retrieved_document> = Vec::new();

    for (k,v) in retrieved_document_index{
        retrieved_document_list.push(retrieved_document{document_name: k, score: v});
    }
    retrieved_document_list.sort_by(|a, b| b.partial_cmp(a).unwrap());

    return retrieved_document_list[0..first_n_documents].to_vec();
    println!("{:?}", retrieved_document_list);
    /*
    let rd1 = retrieved_document{document_name: "1".parse().unwrap(), score: 2.0};
    let rd2 = retrieved_document{document_name: "1".parse().unwrap(), score: 1.0};
    let rd3 = retrieved_document{document_name: "1".parse().unwrap(), score: 3.0};
    let mut rdList: Vec<retrieved_document> = Vec::new();
    rdList.push(rd1);
    rdList.push(rd2);
    rdList.push(rd3);
    rdList.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", rdList);
     */

}


fn calc_tf_idf(document_count: usize, document_count_with_term: usize, term_frequency: u32) -> f64{
    return term_frequency as f64 * f64::ln((document_count/document_count_with_term) as f64);
}