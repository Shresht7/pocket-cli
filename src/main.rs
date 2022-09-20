//  Library
pub mod lib;

use lib::pocket::{add::AddOptions, Pocket};

use dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let consumer_key = std::env::var("CONSUMER_KEY")?;
    let request_token = std::env::var("REQUEST_TOKEN")?;
    let access_token = std::env::var("ACCESS_TOKEN")?;

    let mut client = Pocket::new(&consumer_key);
    client.set_request_token(request_token);
    client.set_access_token(access_token);

    let response = client.add(AddOptions::new("https://example.com")).await?;

    println!("{:?}", response);

    Ok(())
}
