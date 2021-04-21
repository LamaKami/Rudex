use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::indexing::preprocess_text::tokenize_filter_special_characters;


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Posting{
    pub document_id: Uuid,
    pub document_name: String,
    pub weight: u32,
    pub positions: Vec<u32>,
    pub list_length: u32,
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

pub fn create_postings_for_document(document: &str, uuid: Uuid, document_name: String) -> HashMap<String, Posting>{
    let mut postlist_for_document:HashMap<String, Posting> = HashMap::new();

    let tokens: Vec<String> = tokenize_filter_special_characters(document);

    for (i, token) in tokens.iter().enumerate(){
        let mut post:Posting = Default::default();

        if postlist_for_document.contains_key(token){
            post = postlist_for_document.get(token).unwrap().clone();
            post.increase_weight();
            post.add_position(i as u32);
        }
        else {
            post = Posting{
                document_id: uuid,
                document_name: document_name.clone(),
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
    for (_, value) in index{
        value.sort_by_key(|d| d.document_id);
    }
}
