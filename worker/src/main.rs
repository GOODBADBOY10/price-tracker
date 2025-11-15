use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;
use chrono::Utc;

#[derive(Debug, Deserialize)]
struct DexScreenerResponse {
    #[serde(rename = "schemaVersion")]
    _schema_version: String,
    pairs: Vec<Pair>,
}

#[derive(Debug, Deserialize)]
struct Pair {
    #[serde(rename = "chainId")]
    chain_id: String,
    #[serde(rename = "priceNative")]
    price_native: Option<String>,
    #[serde(rename = "priceUsd")]
    price_usd: Option<String>,
    #[serde(rename = "marketCap")]
    market_cap: Option<f64>,
    fdv: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SavedPriceData {
    chain_type: String,
    price_usd: f64,
    price_native: f64,
    market_cap: f64,
    fdv: f64,
    last_updated: String,
}

const API_URL: &str = "https://api.dexscreener.com/latest/dex/tokens/0x84604526d71bbe7738c3c02d3c8a48778955718289c03d814d8468b58ae9a898::skelsui::SKELSUI";
const SAVE_FILE: &str = "prices.json";

#[tokio::main]
async fn main() {
    println!("🚀 Worker starting...");
    println!("📊 Will fetch data every 30 seconds");
    println!("💾 Saving to: {}", SAVE_FILE);
    println!();

    let mut interval = tokio::time::interval(Duration::from_secs(30));

    loop {
        interval.tick().await;

        println!("⏰ Fetching data at {}...", Utc::now().format("%H:%M:%S"));

        match fetch_and_save().await {
            Ok(_) => println!("✅ Data fetched and saved successfully!\n"),
            Err(e) => eprintln!("❌ Error: {}\n", e),
        }
    }
}

async fn fetch_and_save() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(API_URL)
        .await?
        .json::<DexScreenerResponse>()
        .await?;

    let first_pair = response
        .pairs
        .first()
        .ok_or("No pairs found in response")?;

    let price_usd = first_pair.price_usd
        .as_ref()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    
    let price_native = first_pair.price_native
        .as_ref()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);

    let saved_data = SavedPriceData {
        chain_type: first_pair.chain_id.clone(),
        price_usd,
        price_native,
        market_cap: first_pair.market_cap.unwrap_or(0.0),
        fdv: first_pair.fdv.unwrap_or(0.0),
        last_updated: Utc::now().to_rfc3339(),
    };

    let json_string = serde_json::to_string_pretty(&saved_data)?;

    fs::write(SAVE_FILE, json_string)?;

    println!("   💰 Price USD: ${}", saved_data.price_usd);
    println!("   🔗 Chain: {}", saved_data.chain_type);

    Ok(())
}

// use serde::{Deserialize, Serialize};
// use serde_json;
// use std::fs;
// use std::time::Duration;
// use chrono::Utc;
// use tokio::time;

// #[derive(Debug, Deserialize, Serialize)]
// struct DexscrenerData {
//     // Define the fields for DexscrenerData here
//     #[serde(rename = "schemaVersion")]
//     schema_version: String,
//     pairs: Vec<Pairs>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct Pairs {
//     // Define the fields for Pair here
//     #[serde(rename = "chainId")]
//     chain_id: String,
//     #[serde(rename = "priceNative")]
//     price_native: String,
//     #[serde(rename = "priceUsd")]
//     price_usd: String,
//     #[serde(rename = "marketCap")]
//     market_cap: Option<f64>,
//     fdv: Option<f64>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct SavedData {
//     // Define the fields for saved data here
//     chain_type: String,
//     price_usd: f64,
//     price_native: f64,
//     market_cap: f64,
//     fdv: f64,
//     last_updated: String,
// }

// const API_URL: &str = "https://api.dexscreener.com/latest/dex/tokens/0x84604526d71bbe7738c3c02d3c8a48778955718289c03d814d8468b58ae9a898::skelsui::SKELSUI";
// const SAVE_FILE: &str = "prices.json";

// #[tokio::main]
// async fn main() {
//     println!("Worker starting...");
//     println!("Will fetch data every 30 seconds");
//     println!("Saving to: {}", SAVE_FILE);
//     println!();

//     // Create interval that ticks every 30 seconds
//     let mut interval = time::interval(Duration::from_secs(30));

//     loop {
//         interval.tick().await;

//         println!("⏰ Fetching data at {}...", Utc::now().format("%H:%M:%S"));

//         match fetch_and_save().await {
//             Ok(_) => println!("✅ Data fetched and saved successfully!\n"),
//             Err(e) => eprintln!("❌ Error: {}\n", e),
//         }
//     }
// }

// async fn fetch_and_save() -> Result<(), Box<dyn std::error::Error>> {
//     // Step 1: Fetch data from DexScreener
//     let response = reqwest::get(API_URL)
//         .await?
//         .json::<DexscrenerData>()
//         .await?;

//     // Step 2: Get first pair (or error if empty)
//     let first_pair = response.pairs.first().ok_or("No pairs found in response")?;

//     // Step 3: Parse string prices to f64
//     let price_usd = first_pair.price_usd.parse::<f64>().unwrap_or(0.0);
//     let price_native = first_pair.price_native.parse::<f64>().unwrap_or(0.0);

//     // Step 4: Create our saved data structure
//     let saved_data = SavedData {
//         chain_type: first_pair.chain_id.clone(),
//         price_usd,
//         price_native,
//         market_cap: first_pair.market_cap.unwrap_or(0.0),
//         fdv: first_pair.fdv.unwrap_or(0.0),
//         last_updated: Utc::now().to_rfc3339(),
//     };

//     // Step 5: Convert to JSON string
//     let json_string = serde_json::to_string_pretty(&saved_data)?;

//     // Step 6: Write to file
//     fs::write(SAVE_FILE, json_string)?;

//     println!("   💰 Price USD: ${}", saved_data.price_usd);
//     println!("   🔗 Chain: {}", saved_data.chain_type);

//     Ok(())
// }
