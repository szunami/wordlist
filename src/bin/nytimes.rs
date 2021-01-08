// use reqwest::Client;
// use std::fs::File;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use serde::{Deserialize};
// use serde_json::{to_string};
// use std::io::prelude::*;

// #[tokio::main]
// async fn main() {
//     let client = reqwest::Client::new();

//     let mut all_words: HashMap<String, usize>  = HashMap::new();

//     for year in 1977..2019 {
//         for month in 1..13 {
//             for day in 1..32 {
//                 println!("Fetching {}-{}-{}", year, month, day);
//                 match fetch_puzzle(&client, year, month, day).await {
//                     Ok(v) => {
//                         for word in v {
//                             let key = all_words.entry(word).or_insert(0);
//                             *key += 1;
//                         }
//                     },
//                     Err(e) =>             println!("Error fetching date {}-{}-{}: {:?}", year, month, day, e)
//                 }
//             }
//         }
//     }

//     write_to_file(all_words);
// }

// fn write_to_file(all_words: HashMap<String, usize>) -> serde_json::Result<()> {
//     println!("{:?}", all_words.len());
//     let words: Vec<String> = all_words.keys().cloned().collect();
//     let j = serde_json::to_string(&words)?;
//     std::fs::write("data/nytimes/all_words.json", &j);
//     Ok(())
// }

// async fn fetch_puzzle(client: &Client, year: usize, month: usize, day: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {

//     let url = format!("https://raw.githubusercontent.com/doshea/nyt_crosswords/master/{}/{:02}/{:02}.json", year, month, day);

//     let response = client.get(&url).send()
//         .await?
//         .json::<PuzzleResponse>()
//         .await?;


//     let mut result = response.answers.down.clone();

//     result.extend(response.answers.across);

//     Ok(result)
// }