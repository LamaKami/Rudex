use::std::collections::HashMap;
use::std::fs;

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