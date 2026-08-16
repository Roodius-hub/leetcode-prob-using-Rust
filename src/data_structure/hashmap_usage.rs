use std::collections::HashMap;


fn main() {
    let mut map = HashMap::new();

    map.insert("osman", 12);
    map.insert("osmasdvn", 12);
    map.insert("osmanvd", 1223);
    map.insert("osmansdv", 1232);
    map.insert("osmansdv", 1232);
    map.insert("osmasdvn", 1232);
    map.insert("osmansdv", 1232);
    map.insert("osmasdvn", 1232);
    map.insert("osvdsman", 1223);
    map.insert("ossdvman", 1223);

    if let Some(value) = map.get(&"osman") {
        println!("{}", value);
    }

    if map.contains_key(&"osman") {
        println!("Key Exist");
    }

        
    
    println!("{:?}", map);
    
}   