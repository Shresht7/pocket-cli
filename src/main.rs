//  Library
pub mod lib;

use lib::pocket::{add, retrieve, Pocket};

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

    // let options = add::AddOptions::new("https://example.com");
    // let response = client.add(options).await?;
    // println!("{:#?}", response);

    let mut options = retrieve::Options::new();
    options.count(2);
    let response = client.retrieve(options).await?;
    println!("{:#?}", response);

    Ok(())
}
