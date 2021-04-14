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

pub fn create_index(documents_folder_path: &str){
    let mut index: HashMap<String,Vec<Posting>> = HashMap::new();

    let mut doc_list: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(documents_folder_path) {
        for entry in entries {
            if let Ok(entry) = &entry {
                doc_list.push(entry.file_name().into_string().unwrap());
            }
        }
    }

    for (i,doc) in doc_list.iter().enumerate() {
        println!("{}",i);
        let doc_text: String = fs::read_to_string(format!("{}{}", documents_folder_path, doc)).expect("Error while reading file").replace("\n", " ");
        let postlist_for_doc = postlist::create_postings_for_document(&doc_text);

        for (key, value) in postlist_for_doc{
            if index.contains_key(&key){
                let mut key_postlist = index.get(&key).unwrap().to_owned();
                key_postlist.push(value);
                //key_postlist.sort_by_key(|d| d.document_id);
                index.insert(key, key_postlist);

            }
            else {
                index.insert(key, vec![value]);
            }
        }
    }
    sort_postlist(&mut index);


    save_index("data/index/indexlarge.txt",index);
}

/*
2. Loop ueber alle Dokumente drueber
    2.1 Falls Dokuement schon vorhanden (teste ueber UUID) skip
        2.1.2 Eigene Date mit allen Doks ist glaube ich sinnvoll -> DB
    2.2 Dokument nicht vorhanden -> Loop ueber jedes Wort
        2.2.1 Lemmatisiere oder Stemme Wort etc. (Spaeter)
        2.2.2 Schau ob Wort im Index ist
            Ja: Erzeuge ein Posting und fuege es an der richtigen Stelle in der Postlist hinzu
                Spaeter noch update Skippointer
            Nein: Erzeuge ein Posting, Erstelle neue Postlist und fuege es dann hinzu
pub fn create_index(path: &str) -> HashMap<String,HashMap<String, i32>>{
    println!("Indexing...");
    //let doc_iterator = fs::read_dir("/Users/meruem/Documents/Coding/Rust/indexing/data").unwrap();
    let mut doc_list: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = &entry {
                //let my_str: String = entry.file_name().into_string().unwrap();
                doc_list.push(entry.file_name().into_string().unwrap());
                //println!("{:?}", entry.file_name());
            }
        }
    }
    let mut table: HashMap<String,HashMap<String, i32>> = HashMap::new();
    
    for doc in doc_list{
        let mut index: HashMap<String, i32> = HashMap::new();
        let text: String = fs::read_to_string(format!("{}{}", path, doc)).expect("Error while reading file").replace("\n", " ");
        //let text = String::from("This is a Document Text, amazing Text in ourer Document. Can this Document be any better. This is Document.");
        let text_vector: Vec<&str> = text.split(' ').collect();
        for element in text_vector{
            let mut word = element.to_owned();
            while word.ends_with('\n') || word.ends_with('\r') {
                word.pop();
            }
            *index.entry(word).or_insert(0) += 1;
        }
        table.insert(doc, index);
    }
    return table;
}

pub fn print_index(table: &HashMap<String,HashMap<String, i32>>){
    for (doc, index) in table {
        for (key, value) in index{
            println!("{}-{}: {}",doc, key, value);
        }
    }
}
 */