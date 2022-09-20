//  Library
use super::Pocket;
use crate::lib::endpoint;
use serde::{Deserialize, Serialize};

impl Pocket {
    pub async fn get_access_token(&mut self) -> Result<String, reqwest::Error> {
        let consumer_key = self.consumer_key.to_owned();
        let code = self
            .request_token
            .as_ref()
            .expect("request token is needed")
            .to_owned();

        let res: AccessTokenResponse = self
            .client
            .post(endpoint::OAUTH)
            .json(&AccessTokenDetails { consumer_key, code })
            .send()
            .await?
            .json()
            .await?;

        self.access_token = Some(res.access_token.clone());
        Ok(res.access_token)
    }

    pub fn set_access_token(&mut self, token: String) -> &Self {
        self.access_token = Some(token);
        return self;
    }
}

#[derive(Serialize, Deserialize)]
struct AccessTokenDetails {
    consumer_key: String,
    code: String,
}

#[derive(Serialize, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
}
