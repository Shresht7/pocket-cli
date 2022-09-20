//  Library
use super::endpoint;
use serde::{Deserialize, Serialize};

pub struct Pocket {
    client: reqwest::Client,
    consumer_key: String,
    request_token: Option<String>,
}

impl Pocket {
    pub fn new(consumer_key: &str) -> Self {
        let builder = reqwest::ClientBuilder::new();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_str("application/json; charset=UTF-8").unwrap(),
        );
        headers.insert(
            "X-Accept",
            reqwest::header::HeaderValue::from_str("application/json").unwrap(),
        );

        let client = builder.default_headers(headers).build().unwrap();

        return Pocket {
            client,
            consumer_key: String::from(consumer_key),
            request_token: None,
        };
    }

    pub async fn get_request_token(&mut self) -> Result<String, reqwest::Error> {
        let res: RequestTokenResponse = self
            .client
            .post(endpoint::REQUEST_TOKEN)
            .json(&RequestTokenDetails {
                consumer_key: self.consumer_key.clone(),
                redirect_uri: String::from("http://localhost:3000"),
            })
            .send()
            .await?
            .json()
            .await?;

        self.request_token = Some(res.code.clone());
        Ok(res.code)
    }

    pub fn set_request_token(&mut self, token: String) -> &Self {
        self.request_token = Some(token);
        return self;
    }
}

#[derive(Serialize, Deserialize)]
struct RequestTokenResponse {
    code: String,
}

#[derive(Serialize, Deserialize)]
struct RequestTokenDetails {
    consumer_key: String,
    redirect_uri: String,
}

#[derive(Serialize, Deserialize)]
struct BasicDetails {
    consumer_key: String,
    access_token: String,
}
