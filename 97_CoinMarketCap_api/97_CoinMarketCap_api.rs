/*
Cargo.toml should be like this

[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0" 

*/

use std::io::{self, Write};
use serde::Deserialize;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, ACCEPT, HeaderName};

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: std::collections::HashMap<String, CryptoData>,
}

#[derive(Debug, Deserialize)]
struct CryptoData {
    quote: std::collections::HashMap<String, Quote>,
    name: String,
    symbol: String,
}

#[derive(Debug, Deserialize)]
struct Quote {
    price: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    print!("Enter token symbol (e.g. BTC, ETH): ");
    io::stdout().flush()?;
    let mut token = String::new();
    io::stdin().read_line(&mut token)?;
    let token = token.trim().to_uppercase();

    let api_key = "d2a37fe2-3152-4915-8f08-070acfbdd5e6";
    let url = format!(
        "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol={}",
        token
    );

    let mut headers = HeaderMap::new();
    headers.insert("X-CMC_PRO_API_KEY", HeaderValue::from_str(api_key)?);
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let res = client.get(&url)
        .headers(headers)
        .send()
        .await?;

    if !res.status().is_success() {
        println!("Failed to fetch data. HTTP Status: {}", res.status());
        return Ok(());
    }

    let json: ApiResponse = res.json().await?;

    if let Some(data) = json.data.get(&token) {
        if let Some(usd_quote) = data.quote.get("USD") {
            println!(
                "Token: {} ({})\nPrice (USD): ${:.2}",
                data.name, data.symbol, usd_quote.price
            );
        } else {
            println!("No USD quote found.");
        }
    } else {
        println!("Token not found in API response.");
    }

    Ok(())
}
