use serde::{Deserialize};

// This `derive` requires the `serde` dependency.
#[derive(Deserialize)]
struct Ip {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip = reqwest::get("http://httpbin.org/ip")
        .await?
        .json::<Ip>()
        .await?;

    println!("ip: {}", ip.origin);

    Ok(())
}

