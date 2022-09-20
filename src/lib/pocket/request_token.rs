//  Library
use super::Pocket;
use crate::lib::endpoint;
use serde::{Deserialize, Serialize};

impl Pocket {
    pub async fn get_request_token(&mut self) -> Result<String, reqwest::Error> {
        if let Some(code) = &self.request_token { return Ok(code.to_owned()); }

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
