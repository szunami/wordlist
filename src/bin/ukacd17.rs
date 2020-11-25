use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    
    let mut result: Vec<String> = vec![];
    if let Ok(lines) = read_lines("./data/UKACD17/raw/UKACD17.TXT") {
        for line in lines {
            if let Ok(line) = line {
                let x: String = line.to_uppercase().split_whitespace().flat_map(|c| c.chars()).collect();
                result.push(x);
            }
        }
    }
    let j = serde_json::to_string(&result).unwrap();
    std::fs::write("data/UKACD17/all_words.json", &j);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}