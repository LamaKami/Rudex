use crate::postlist;

extern crate serde_derive;
extern crate flexbuffers;
extern crate serde;

use std::fs;
use std::path::Path;
use::std::collections::HashMap;
use serde::{Serialize, Deserialize};
use postlist::Posting;
use crate::indexing::postlist::sort_postlist;
use uuid::Uuid;

//Todo Anzahl der Dokumente sowie deren UUID speichern

pub fn save_index(save_path: &str, index: HashMap<String, Vec<postlist::Posting>>){
    let mut buffer = flexbuffers::FlexbufferSerializer::new();
    index.serialize(&mut buffer).unwrap();
    fs::write(save_path, buffer.take_buffer()).unwrap();
}

pub fn load_index(load_path: &str) -> HashMap<String, Vec<postlist::Posting>>{
    if !Path::new(load_path).exists(){
        return HashMap::new();
    }
    let file = fs::read(load_path).unwrap();
    let read_buffer = flexbuffers::Reader::get_root(&file).unwrap();
    return HashMap::deserialize(read_buffer).unwrap();
}

pub fn create_update_index(documents_folder_path: &str, optional_index_path: Option<Box<&str>>){
    let mut index: HashMap<String,Vec<Posting>>;

    match optional_index_path {
        Some(index_path) => { index = load_index(&index_path); },
        None => { index = HashMap::new(); },
    }

    let doc_list: Vec<String> = create_doc_list(documents_folder_path);

    add_documents_too_index(doc_list, documents_folder_path, &mut index);

    sort_postlist(&mut index);
    save_index("data/index/indexlarge.txt",index);
}

pub fn create_doc_list(documents_folder_path: &str) -> Vec<String>{
    let mut doc_list: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(documents_folder_path) {
        for entry in entries {
            if let Ok(entry) = &entry {
                doc_list.push(entry.file_name().into_string().unwrap());
            }
        }
    }
    return doc_list;
}

pub fn add_documents_too_index(doc_list: Vec<String>,documents_folder_path: &str, index: &mut HashMap<String,Vec<Posting>>){
    let mut document_ids:HashMap<Uuid, bool> = HashMap::new();

    for (i, doc) in doc_list.iter().enumerate() {
        println!("Document {} of {}\nPercentage: {:.2}%",i, doc_list.len(), ((i as f64) / (doc_list.len() as f64))*100.0);
        let doc_text: String = fs::read_to_string(format!("{}{}", documents_folder_path, doc)).expect("Error while reading file").replace("\n", " ");

        let document_id = Uuid::new_v5(&Uuid::NAMESPACE_OID,doc_text.as_bytes());

        if document_ids.contains_key(&document_id){
            println!("{}",document_id);
            continue;
        }
        document_ids.insert(document_id,true);

        let postlist_for_doc = postlist::create_postings_for_document(&doc_text, document_id, doc.to_owned());

        for (key, value) in postlist_for_doc{
            index.entry(key).or_insert_with(Vec::new).push(value);
        }
    }
}
