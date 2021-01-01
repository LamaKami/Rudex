mod searcher;
mod index;
mod sorting;

fn main() {
    let index = index::create_index("data/pos/");
    
    let result = searcher::search(&index, "Israel stereotyping");
    
    let sorted_result = sorting::bubble_sort(&result);
    for (i, doc) in sorted_result.iter().enumerate() {
        if i > 2{
            break;
        }
        println!("{} {}", doc, result.get(doc).unwrap());
    }

    // Binary Tree mit getauschtem Value Key? dumm weil alter value nicht eindeutig ist

}



//Index builder, nur std libaries
// text als variable dann dateien einlesen txt, dann json

//Baue Hash Map die ein query wort als Key hat und der value die documente sind?
//Postlisten