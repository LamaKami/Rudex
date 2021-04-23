use std::hash::Hash;
use std::path::Path;
use serde::{Serialize, Deserialize};
use std::fs;
use::std::collections::HashMap;
use serde::de::DeserializeOwned;


pub fn save_hash_map<T: Eq + Hash + Serialize, V: Eq + Hash + Serialize>(path: &str, map: HashMap<T,V>){
    let mut buffer = flexbuffers::FlexbufferSerializer::new();
    map.serialize(&mut buffer).unwrap();
    fs::write(path, buffer.take_buffer()).unwrap();
}

pub fn load_hash_map<T: Eq + Hash + DeserializeOwned, V: Eq + Hash + DeserializeOwned>(hash_map_path: &str) -> HashMap<T,V>{
    if !Path::new(hash_map_path).exists(){
        return HashMap::new();
    }
    let file = fs::read(hash_map_path).unwrap();
    let read_buffer = flexbuffers::Reader::get_root(&file).unwrap();
    return HashMap::deserialize(read_buffer).unwrap();
}