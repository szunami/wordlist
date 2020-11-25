use std::collections::HashSet;
use std::fs::File;

fn main() {
    
    let nytimes = File::open("data/nytimes/all_words.json").unwrap();
    let nytimes_words: Vec<String> = serde_json::from_reader::<File, Vec<String>>(nytimes).unwrap();
    println!("nytimes words: {}", nytimes_words.len());
    
    let ukacd = File::open("data/UKACD17/all_words.json").unwrap();
    let ukacd_words: Vec<String> = serde_json::from_reader::<File, Vec<String>>(ukacd).unwrap();
    println!("ukacd words: {}", ukacd_words.len());

    let websters = File::open("data/websters/all_words.json").unwrap();
    let websters_words: Vec<String> = serde_json::from_reader::<File, Vec<String>>(websters).unwrap();
    println!("websters words: {}", websters_words.len());

    let mut result = HashSet::new();
    
    for word in nytimes_words {
        result.insert(word);
    }
    
    for word in ukacd_words {
        result.insert(word);
    }
    
    for word in websters_words {
        result.insert(word);
    }
    
    let mut result: Vec<String> = result.drain().collect();
    result.sort();
    
    println!("output words: {}", result.len());
    
    let j = serde_json::to_string(&result).unwrap();
    std::fs::write("data/output/all_words.json", &j);
}