
use crate::utils::load_and_save;
use crate::postlist;

extern crate serde_derive;
extern crate flexbuffers;
extern crate serde;

use uuid::Uuid;
use std::fs;
use::std::collections::HashMap;
use postlist::Posting;
use crate::indexing::postlist::sort_postlist;



//Todo Anzahl der Dokumente sowie deren UUID speichern


pub fn create_update_index(documents_folder_path: &str, optional_index_path: Option<Box<&str>>, optional_document_ids_path: Option<Box<&str>>){
    let mut index: HashMap<String,Vec<Posting>>;
    let mut document_ids:HashMap<String, bool>;

    match optional_index_path {
        Some(index_path) => { index = load_and_save::load_hash_map(&index_path); },
        None => { index = HashMap::new(); },
    }

    match optional_document_ids_path {
        Some(document_ids_path) => { document_ids = load_and_save::load_hash_map(&document_ids_path); },
        None => { document_ids = HashMap::new(); },
    }

    let doc_list: Vec<String> = create_doc_list(documents_folder_path);

    add_documents_too_index(doc_list, documents_folder_path, &mut index, &mut document_ids);

    sort_postlist(&mut index);

    load_and_save::save_hash_map("data/index/indexlarge.txt", index);
    load_and_save::save_hash_map("data/index/documentIds.txt", document_ids);
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

pub fn add_documents_too_index(doc_list: Vec<String>,documents_folder_path: &str, index: &mut HashMap<String,Vec<Posting>>, document_ids: &mut HashMap<String, bool>){
    for (i, doc) in doc_list.iter().enumerate() {
        println!("Document {} of {}\nPercentage: {:.2}%",i+1, doc_list.len(), ((i as f64 + 1.0) / (doc_list.len() as f64))*100.0);
        let doc_text: String = fs::read_to_string(format!("{}{}", documents_folder_path, doc)).expect("Error while reading file").replace("\n", " ");

        let document_id = Uuid::new_v5(&Uuid::NAMESPACE_OID,doc_text.as_bytes());

        if document_ids.contains_key(&document_id.to_string()){
            continue;
        }
        document_ids.insert(document_id.to_string(),true);

        let postlist_for_doc = postlist::create_postings_for_document(&doc_text, document_id, doc.to_owned());

        for (key, value) in postlist_for_doc{
            index.entry(key).or_insert_with(Vec::new).push(value);
        }
    }
}