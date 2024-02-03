use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, ACCEPT};
use serde_json::json;
use serde::Deserialize;
use std::env;
use anyhow::anyhow;

#[derive(Deserialize)]
struct ValidationResponse {
    valid: bool,
}

pub async fn neynar_message_validation(message_bytes: &str) -> Result<bool, reqwest::Error> {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let api_key = env::var("NEYNAR_API_KEY").expect("NEYNAR_API_KEY must be set");
    headers.insert("api_key", HeaderValue::from_str(&api_key).expect("Invalid header value for api_key"));

    let json = json!({
        "message_bytes_in_hex": message_bytes,
    });

    // The POST request
    let response = client.post("https://api.neynar.com/v2/farcaster/frame/validate")
        .headers(headers)
        .json(&json) // Uncomment if you have a JSON payload
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        let response_text = response.text().await?;
        println!("Response Text: {}", response_text);

        // Deserialize the JSON response into the ValidationResponse struct
        let validation_response: ValidationResponse = serde_json::from_str(&response_text).expect("Failed to deserialize validation response");
        // Return the valid field's value
        Ok(validation_response.valid)
    } else {
        eprintln!("Request failed with status: {}", response.status());
        Ok(false)
    }
}