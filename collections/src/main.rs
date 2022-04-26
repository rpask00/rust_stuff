use std::collections::HashMap;


fn main() {
    let mut map = HashMap::new();
    let txt = String::from("i wanna have wonderful life");

    for word in txt.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }

    println!("{:#?}", map);         

}



