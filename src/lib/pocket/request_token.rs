//  Library
use super::Pocket;
use crate::lib::endpoint;
use serde::{Deserialize, Serialize};

impl Pocket {
    /// Request a request_token using the consumer_key.
    /// The client will need to be redirected to the redirect_uri to complete authentication.
    pub async fn get_request_token(&mut self) -> Result<String, reqwest::Error> {
        //  If a valid request_token already exists, return it
        if let Some(code) = &self.request_token {
            return Ok(code.to_owned());
        }

        //  Set consumer_token and redirect_uri for auth flow
        let consumer_key = self.consumer_key.clone();
        let redirect_uri = self
            .redirect_uri
            .as_ref()
            .expect("Please provide a valid redirect URL")
            .clone();

        //  Send the request and await the response
        let res: RequestTokenResponse = self
            .client
            .post(endpoint::REQUEST_TOKEN)
            .json(&RequestTokenRequest {
                consumer_key,
                redirect_uri,
            })
            .send()
            .await?
            .json()
            .await?;

        //  Update the request_token field and return the value
        self.request_token = Some(res.code.clone());
        Ok(res.code)
    }

    /// Set the request_token
    pub fn set_request_token(&mut self, token: String) -> &Self {
        self.request_token = Some(token);
        return self;
    }

    /// Set the redirect_uri
    pub fn set_redirect_uri(&mut self, uri: &str) -> &Self {
        self.redirect_uri = Some(uri.to_owned());
        return self;
    }

    /// Get the URL the user must be sent to for authorization
    pub async fn get_auth_url(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let request_token = match &self.request_token {
            Some(token) => token.to_owned(),
            None => self.get_request_token().await?.to_owned(),
        };

        let redirect_uri = self
            .redirect_uri
            .as_ref()
            .expect("PocketAuthRedirect")
            .to_owned();

        //  Build URL using the params
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
struct RequestTokenRequest {
    consumer_key: String,
    redirect_uri: String,
}
