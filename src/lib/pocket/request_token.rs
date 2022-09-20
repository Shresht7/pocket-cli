//  Library
use super::Pocket;
use crate::lib::endpoint;
use serde::{Deserialize, Serialize};

impl Pocket {
    pub async fn get_request_token(&mut self) -> Result<String, reqwest::Error> {
        if let Some(code) = &self.request_token {
            return Ok(code.to_owned());
        }

        let consumer_key = self.consumer_key.clone();
        let redirect_uri = if let Some(uri) = &self.redirect_uri {
            uri.clone()
        } else {
            String::from("PocketAuthRedirect")
        };

        let res: RequestTokenResponse = self
            .client
            .post(endpoint::REQUEST_TOKEN)
            .json(&RequestTokenDetails {
                consumer_key,
                redirect_uri,
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

    pub fn set_redirect_uri(&mut self, uri: String) -> &Self {
        self.redirect_uri = Some(uri);
        return self;
    }

    pub async fn get_auth_url(&self) -> Result<String, Box<dyn std::error::Error>> {
        let request_token = self
            .request_token
            .as_ref()
            .expect("need request_token for auth-url");

        let redirect_uri = self.redirect_uri.as_ref().expect("PocketAuthRedirect");

        let url = reqwest::Url::parse_with_params(
            endpoint::AUTHORIZE,
            &[
                ("request_token", request_token),
                ("redirect_uri", redirect_uri),
            ],
        )?
        .as_str()
        .to_owned();

        Ok(url)
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
