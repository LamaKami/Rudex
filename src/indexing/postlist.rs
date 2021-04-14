use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;
//Postlisten : t1 -> [document,[Gewichte],[positionen], list length, [skip pointer]]
//list length:Added to the first entry of the posting list of a term t, Stores the length of the posting list, Equals the number of documents containing t (document frequency df(t, D))
//skip pointer gibt die id des n-ten Dokuments an 
//weight haeufigkeit im dokument

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Posting{
    pub document_id: Uuid,
    pub weight: u32,
    pub positions: Vec<u32>,
    pub list_length: u64,
    pub skip_pointer: Vec<Uuid>
}

impl Posting{
    pub fn increase_weight(&mut self){
        self.weight += 1;
    }

    pub fn add_position(&mut self, pos: u32){
        self.positions.push(pos);
    }
}

pub fn create_postings_for_document(document: &str) -> HashMap<String, Posting>{
    let mut postlist_for_document:HashMap<String, Posting> = HashMap::new();
    //TODO Add better Tokenization Algo
    let tokens: Vec<&str> = document.split(' ').collect();
    
    for (i, token) in tokens.iter().enumerate(){
        let mut post:Posting = Default::default();

        if postlist_for_document.contains_key(token.to_owned()){
            post = postlist_for_document.get(token.to_owned()).unwrap().clone();
            post.increase_weight();
            post.add_position(i as u32);
        }
        else {
            post = Posting{
                document_id: Uuid::new_v5(&Uuid::NAMESPACE_OID,document.as_bytes()),
                weight: 1,
                positions: vec![i as u32],
                list_length: 1,
                skip_pointer: vec![]
            };
        }
        postlist_for_document.insert(token.parse().unwrap(), post);
    }

    return postlist_for_document;
}

pub fn sort_postlist (index: &mut HashMap<String,Vec<Posting>>){
    for (_, mut value) in index{
        value.sort_by_key(|d| d.document_id);
    }
}
