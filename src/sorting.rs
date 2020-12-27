use::std::collections::HashMap;

pub fn bubble_sort(result:&HashMap<std::string::String, f64>) -> Vec<String>{
    let mut docs: Vec<String> = Vec::new();
    for key in result.keys(){
        docs.push(key.to_string());
    }
    let mut done = false;
    while !done{
        done = true;
        for i in 0..docs.len()-1{
            if result.get(&docs[i]).unwrap() < result.get(&docs[i+1]).unwrap() {
                done = false;
                let tmp = docs[i].clone();
                docs[i] = docs[i+1].clone();
                docs[i+1] = tmp;
            }
        }
    }
    return docs;
}