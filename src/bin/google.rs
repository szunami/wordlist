use futures::{stream, StreamExt};
use scraper::{Html, Selector};
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::default();
    let all_words = File::open("data/output/all_words.json").unwrap();
    let all_words = serde_json::from_reader::<File, Vec<String>>(all_words).unwrap();

    let all_words_tmp = all_words.clone();

    let urls = all_words_tmp
        .iter()
        .map(|word| format!("https://www.bing.com/search?q={}&form=QBRE", word));

    let texts = stream::iter(urls)
        .map(|url| {
            let client = client.clone();
            tokio::spawn(async move {
                let resp = client.get(url.as_str()).send().await?;
                resp.text().await
            })
        })
        .buffered(16);

    let to_remove = texts
        .zip(stream::iter(all_words))
        .filter_map(|(text, word)| async move {
            match text {
                Ok(Ok(text)) => {
                    let parsed = Html::parse_document(text.as_str());

                    let requery_selector = Selector::parse("#sp_requery").unwrap();

                    if let Some(_requery) = parsed.select(&requery_selector).next() {
                        println!("Requery for {}. Filtering", word);
                        return Some(word);
                    }

                    let result_count_selector = Selector::parse(".sb_count").unwrap();
                    for element in parsed.select(&result_count_selector) {
                        // bing might exclude word if it is too popular

                        match parse_result_count(element.inner_html()) {
                            Some(result_count) => {
                                if result_count > 1_000_000 {
                                    println!("{} is popular, not filtering", word);
                                    return None;
                                }
                                println!("{} is unpopular, filtering", word);
                                return Some(word);
                            }
                            None => {}
                        }
                    }

                    let no_results_selector = Selector::parse(".b_no").unwrap();

                    for _element in parsed.select(&no_results_selector) {
                        println!("No results found for {}, filtering", word);
                        return Some(word);
                    }

                    println!("Neither found for {}. It might be a real thing!", word);
                    None
                }
                Ok(Err(e)) => {
                    eprintln!("{}", e);
                    return None;
                }
                Err(e) => {
                    eprintln!("{}", e);
                    return None;
                }
            }
        })
        .collect::<Vec<String>>()
        .await;

    println!("{:?}", to_remove);

    Ok(())
}

fn parse_result_count(inner_html: String) -> Option<usize> {
    let raw_result_count = inner_html.split_ascii_whitespace().next().unwrap();

    let cleaned_result_count = raw_result_count.replace(",", "");

    match cleaned_result_count.parse() {
        Ok(x) => Some(x),
        Err(_) => {
            println!("Failed to parse {}", inner_html);
            None
        }
    }
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
