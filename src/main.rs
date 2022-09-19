// use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: reqwest::Client = reqwest::Client::new();
    let res = client
        .get("https://api.github.com/zen")
        .header("User-Agent", "Shresht7")
        .send()
        .await?
        .text()
        .await?;
    println!("{:?}", res);
    Ok(())
}
