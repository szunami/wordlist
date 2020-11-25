use reqwest::Client;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use serde::{Deserialize};
use serde_json::{to_string};
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
struct PuzzleResponse {
    answers: Answers,
}

#[derive(Deserialize, Debug)]
struct Answers {
    down: Vec<String>,
    across: Vec<String>,
}

#[tokio::main]
async fn main() {

    let websters = load_websters().await.unwrap();

    println!("{}", websters.len());

    let j = serde_json::to_string(&websters).unwrap();
    std::fs::write("data/websters/all_words.json", &j);
}

async fn load_websters() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get("https://raw.githubusercontent.com/adambom/dictionary/master/dictionary.json").send()
    .await?
    .json::<HashMap<String, String>>()
    .await?;

    Ok(response.keys().cloned().collect())
}







