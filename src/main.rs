use std::io::{self, Write};
use reqwest::header::USER_AGENT;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("What Item Are We Searching For?: ");
    io::stdout().flush().unwrap();

    let mut item = String::new();
    io::stdin().read_line(&mut item)?;
    let item = item.trim().to_lowercase().replace(" ", "_");

    let url = format!("https://api.warframe.market/v1/items/{}/orders", item);

    let resp = reqwest::Client::new()
        .get(&url)
        .header(USER_AGENT, "simple-script")
        .send()
        .await?;

    let resp_json: Value = resp.json().await?;

    if resp_json.get("payload").is_none() {
        println!("Error: Item not found or API returned unexpected response.");
        println!("{}", serde_json::to_string_pretty(&resp_json)?);
        return Ok(());
    }

    let orders = resp_json["payload"]["orders"].as_array().unwrap();

    println!("Fetched {} orders", orders.len());

    let lowest_sell = orders
        .iter()
        .filter(|o| o["order_type"] == "sell" && o["visible"] == true)
        .filter_map(|o| o["platinum"].as_u64())
        .min();

    let highest_buy = orders
        .iter()
        .filter(|o| o["order_type"] == "buy" && o["visible"] == true)
        .filter_map(|o| o["platinum"].as_u64())
        .max();

    match highest_buy {
        Some(value) => println!("The Highest Buy is: {}", value),
        None => println!("No highest_buy"),
    }

    match lowest_sell {
        Some(value) => println!("The Lowest Sell is: {}", value),
        None => println!("No lowest_sell"),
    }

    Ok(())
}

