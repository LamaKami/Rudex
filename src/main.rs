mod searcher;
mod index;


fn main() {
    let index = index::create_index("data");
    
    let result = searcher::search(&index, "This amazing are Document");
    
    for (k, v) in result {
        println!("{} {}", k, v);
    }
    // Binary Tree mit getauschtem Value Key? dumm weil alter value nicht eindeutig ist

}



//Index builder, nur std libaries
// text als variable dann dateien einlesen txt, dann json
