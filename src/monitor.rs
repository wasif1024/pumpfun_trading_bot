use crate::sniper;
use reqwest;
use tokio::time::{sleep, Duration};
use std::collections::HashSet;
use serde_json::Value;

pub async fn start_monitor() {
    let mut seen_tokens = HashSet::new();

    loop {
        match reqwest::get("https://pump.fun/api/tokens").await {
            Ok(resp) => match resp.json::<Value>().await {
                Ok(json) => {
                    if let Some(tokens) = json["tokens"].as_array() {
                        for token in tokens {
                            if let Some(mint) = token["mint"].as_str() {
                                if seen_tokens.insert(mint.to_string()) {
                                    println!("New token detected: {}", mint);
                                    sniper::snipe(mint).await;
                                }
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Error parsing JSON: {}", e),
            },
            Err(e) => eprintln!("Request failed: {}", e),
        }

        sleep(Duration::from_secs(5)).await;
    }
}
