use reqwest::{self, Client};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct Data {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct PostResp {
    json: Data,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = client
        .post("https://httpbin.org/post")
        .json(&json!({
            "username": "example",
            "password": "secret"
        }))
        .send()
        .await?
        .json::<PostResp>()
        .await?;

    println!("Response: {:#?}", resp);
    Ok(())
}
