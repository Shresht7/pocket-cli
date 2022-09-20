//  Library
pub mod default_headers;

pub mod access_token;
pub mod request_token;

pub mod add;
pub mod retrieve;

pub struct Pocket {
    client: reqwest::Client,
    consumer_key: String,
    request_token: Option<String>,
    access_token: Option<String>,
    redirect_uri: Option<String>,
}

impl Pocket {
    /// Create a new Pocket Client
    pub fn new(consumer_key: &str) -> Self {
        //  Get default headers to use in every request
        let headers = default_headers::get();

        //  Build the reqwest client
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        return Self {
            client,
            consumer_key: String::from(consumer_key),
            request_token: None,
            access_token: None,
            redirect_uri: None,
        };
    }
}
