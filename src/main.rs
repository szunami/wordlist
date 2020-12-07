use reqwest::Client;
use regex::Regex;
use reqwest::header::HeaderMap;
use std::{collections::{HashMap, HashSet}, env};
use std::fs::File;
use serde::{Deserialize};

#[tokio::main]
async fn main() {
    
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
    
    println!("deduped words: {}", result.len());
    
    let regex = Regex::new(r"^[A-Z]+$").unwrap();
    
    let mut result: Vec<String> = result.drain().filter(
        |word| {
           regex.is_match(word)
        })
    .collect();
    result.sort();
    
    println!("filtered words: {}", result.len());
    
    let j = serde_json::to_string(&result).unwrap();
    std::fs::write("data/output/all_words.json", &j).expect("Failed to write :(");

    score_words(result).await;

}


pub async fn score_words(words: Vec<String>) {
    let mut headers = HeaderMap::new();
    let bing_key  = env::var("BING_KEY").expect("Failed to get BING_KEY");
    headers.insert("Ocp-Apim-Subscription-Key", bing_key.parse().unwrap());
    // headers.append(key, value)
    
    let client = reqwest::ClientBuilder::new().default_headers(headers).build().expect("Failed to build");    
    
    // let count = fetch_puzzle(&client, String::from("Charlotte")).await.expect("Failed");
    
    let mut result = HashMap::new();
    
    for word in words {
        let score = score_word(&client, word.clone())
            .await.expect(format!("Failed to score {}", word).as_str());
        result.insert(word, score);
    }
    
    write_to_file(result);
    
}

async fn score_word(client: &Client, word: String) -> Result<usize, Box<dyn std::error::Error>> {

    let url = format!("https://api.bing.microsoft.com/v7.0/search?q={}", word);

    let response = client.get(&url).send()
        .await?;
        
    println!("{:?}", response);
        
    let bing_response = response
        .json::<BingResponse>()
        .await?;

    Ok(bing_response.webPages.totalEstimatedMatches)
}

fn write_to_file(scored_words: HashMap<String, usize>) -> serde_json::Result<()> {
    let j = serde_json::to_string(&scored_words)?;
    std::fs::write("data/bing_scores/all_words.json", &j);
    
    Ok(())
}

#[derive(Deserialize, Debug)]
struct BingResponse {
    webPages: WebPages,
}

#[derive(Deserialize, Debug)]
struct WebPages {
    totalEstimatedMatches: usize,
}



#[cfg(test)]
mod tests {
    use crate::Regex;
    
    #[test]

fn test () {
        let regex = Regex::new(r"^[A-Z]+$").unwrap();

        assert!(regex.is_match("ASDF"));
        assert!(!regex.is_match(" SDF"));
        assert!(!regex.is_match("aSDF"));
        assert!(!regex.is_match("2SDF"));
        assert!(!regex.is_match("-SDF"));
    }
}