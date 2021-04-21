mod indexing;

use crate::indexing::index;
use crate::indexing::postlist;


use std::collections::HashMap;
use postlist::Posting;

//TODO SkipPointer, Multiprocessing, Stoppwords, NGramme, CLI


fn main() {
    index::create_update_index("data/pos/", None);

    //loading
    //let index: HashMap<String,Vec<Posting>> = index::load_index("data/index/indexlarge.txt");


}