mod searcher;
mod index;
mod sorting;

fn main() {
    let index = index::create_index("data");
    
    let result = searcher::search(&index, "This amazing are Document");
    
    let sorted_result = sorting::bubble_sort(&result);
    for doc in sorted_result {
        println!("{} {}", doc, result.get(&doc).unwrap());
    }

    // Binary Tree mit getauschtem Value Key? dumm weil alter value nicht eindeutig ist

}



//Index builder, nur std libaries
// text als variable dann dateien einlesen txt, dann json
