use std::fs::File;

use scraper::{Html, Selector};
// use rayon::prelude::*;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let all_words =  File::open("data/output/all_words.json").unwrap();
    let all_words = serde_json::from_reader::<File, Vec<String>>(all_words).unwrap();

    for (i, word) in all_words.iter().enumerate() {

        // if i > 500 {
        //     break;
        // }

        let url = format!("https://www.bing.com/search?q={}", word);

        let body = client.get(url.as_str()).send()
        .await?
        .text()
        .await?;
    
        let parsed = Html::parse_document(body.as_str());
    
        let selector = Selector::parse(".sb_count").unwrap();

        let mut found = false;
    
        for element in parsed.select(&selector) {
            // bing might exclude word if it is too popular
            found = true;

            parse_result_count(element.inner_html());

            println!("{}, {}: {:?}", word, i, element.inner_html());
            println!("parsed: {}", parse_result_count(element.inner_html()));
        }

        if !found {
            println!("{} not found", word);
        }
    };



    Ok(())
}

fn parse_result_count(inner_html: String) -> usize {

    let raw_result_count = inner_html.split_ascii_whitespace().next().unwrap();

    let cleaned_result_count = raw_result_count.replace(",", "");

    cleaned_result_count.parse().unwrap()
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test() {
        let x = "5,000 results";

        let y = x.split_ascii_whitespace().next().unwrap();

        let z = y.replace(",", "");

        let y = z.parse::<i32>().unwrap();
        assert_eq!(5000, y)
    }
}