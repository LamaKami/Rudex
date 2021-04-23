mod indexing;
mod searching;
mod utils;

use crate::indexing::index;
use crate::indexing::postlist;
use crate::utils::load_and_save;
use crate::searching::searcher;

use std::collections::HashMap;
use postlist::Posting;

use uuid::Uuid;
use crate::searching::searcher::retrieved_document;

//TODO SkipPointer, Multiprocessing, Stoppwords, NGramme, CLI, OnTheFile


fn main() {
    //index::create_update_index("data/pos/", None, None);

    //loading
    let index: HashMap<String,Vec<Posting>> = load_and_save::load_hash_map("data/index/indexlarge.txt");
    let document_ids: HashMap<String,bool>  = load_and_save::load_hash_map("data/index/documentIds.txt");
    let query = "amazing romantic Document";
    let retrieved_top_documents: Vec<retrieved_document> = searcher::search(index, document_ids.keys().len(), query, 10);
    println!("{:?}", retrieved_top_documents);

}