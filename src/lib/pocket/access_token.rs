//  Library
use super::Pocket;
use crate::lib::endpoint;
use serde::{Deserialize, Serialize};

impl Pocket {
    /// Request an access_token using the user's request_token
    pub async fn get_access_token(&mut self) -> Result<String, reqwest::Error> {
        let consumer_key = self.consumer_key.to_owned();
        let code = self
            .request_token
            .as_ref()
            .expect("A request_token is required to obtain an access_token")
            .to_owned();

        //  Send the request and await the response
        let res: AccessTokenResponse = self
            .client
            .post(endpoint::OAUTH)
            .json(&AccessTokenRequest { consumer_key, code })
            .send()
            .await?
            .json()
            .await?;

        //  Update the access_token field and return the result
        self.access_token = Some(res.access_token.clone());
        Ok(res.access_token)
    }

    /// Set the access_token
    pub fn set_access_token(&mut self, token: String) -> &Self {
        self.access_token = Some(token);
        return self;
    }
}

#[derive(Serialize, Deserialize)]
struct AccessTokenRequest {
    consumer_key: String,
    code: String,
}

#[derive(Serialize, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    username: String,
}
