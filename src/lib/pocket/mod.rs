//  Library
mod access_token;
mod default_headers;
mod request_token;

pub struct Pocket {
    client: reqwest::Client,
    consumer_key: String,
    request_token: Option<String>,
    access_token: Option<String>,
    redirect_uri: Option<String>,
}

impl Pocket {
    pub fn new(consumer_key: &str) -> Self {
        let headers = default_headers::get();

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        return Pocket {
            client,
            consumer_key: String::from(consumer_key),
            request_token: None,
            access_token: None,
            redirect_uri: None,
        };
    }
}
