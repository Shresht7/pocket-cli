//  Library
use super::Pocket;
use crate::lib::endpoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveOptions {
    state: String,
    favorite: Option<u8>,
    count: Option<u32>,
}

impl Default for RetrieveOptions {
    fn default() -> Self {
        Self {
            state: String::from("all"),
            favorite: Some(0),
            count: Some(3),
        }
    }
}

impl RetrieveOptions {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Pocket {
    pub async fn retrieve(
        &self,
        options: Option<RetrieveOptions>,
    ) -> Result<String, reqwest::Error> {
        let consumer_key = self.consumer_key.to_owned();
        let access_token = self
            .access_token
            .as_ref()
            .expect("need access_token")
            .to_owned();

        let payload = RetrieveDetails::new(consumer_key, access_token);

        Ok(self
            .client
            .post(endpoint::RETRIEVE)
            .json(&payload)
            .send()
            .await?
            .json()
            // .json::<RetrieveResponse>()
            .await?)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct RetrieveDetails {
    count: Option<u32>,
    consumer_key: String,
    access_token: String,
}

impl RetrieveDetails {
    pub fn new(consumer_key: String, access_token: String) -> Self {
        return RetrieveDetails {
            consumer_key,
            access_token,
            ..Default::default()
        };
    }

    pub fn build(&self, opts: Option<RetrieveOptions>) {}
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveResponse {
    status: u8,
    complete: u8,
    since: u32,
    error: String,
    search_meta: SearchMeta,
    list: Vec<RetrieveItem>,
}

#[derive(Serialize, Deserialize)]
struct SearchMeta {
    search_type: String,
}

#[derive(Serialize, Deserialize)]
struct RetrieveItem {
    /// A unique identifier matching the saved item. This id must be used to perform any actions through the v3/modify endpoint.
    item_id: String,
    /// A unique identifier similar to the item_id but is unique to the actual url of the saved item. The resolved_id identifies unique urls. For example a direct link to a New York Times article and a link that redirects (ex a shortened bit.ly url) to the same article will share the same resolved_id. If this value is 0, it means that Pocket has not processed the item. Normally this happens within seconds but is possible you may request the item before it has been resolved.
    resolved_id: String,
    /// The actual url that was saved with the item. This url should be used if the user wants to view the item.
    given_url: String,
    /// The final url of the item. For example if the item was a shortened bit.ly link, this will be the actual article the url linked to.
    resolved_url: String,
    /// The title that was saved along with the item.
    given_title: String,
    /// The title that Pocket found for the item when it was parsed
    resolved_title: String,
    /// 0 or 1 - 1 If the item is favorite
    favorite: String,
    /// 0, 1, 2 - 1 if the item is archived - 2 if the item should be deleted
    status: String,
    /// The first few lines of the item (articles only)
    excerpt: String,
    /// 0 or 1 - 1 if the item is an article
    is_article: String,
    /// 0, 1, or 2 - 1 if the item has images in it - 2 if the item is an image
    has_image: String,
    /// 0, 1, or 2 - 1 if the item has videos in it - 2 if the item is a video
    has_video: String,
    /// How many words are in the article
    word_count: String,
    /// A JSON object of the user tags associated with the item
    tags: String,
    /// A JSON object listing all of the authors associated with the item
    authors: String,
    /// A JSON object listing all of the images associated with the item
    images: String,
    /// A JSON object listing all of the videos associated with the item
    videos: String,
    /// Timestamp of when the resource was added
    time_added: String,
    /// Timestamp of when the resource was last updated
    time_updated: String,
    /// Timestamp of when the resource was last read
    time_read: String,
    /// Timestamp of when the resource was favorited
    time_favorited: String,
    /// Sorting ID number
    sort_id: i32,
    is_index: String,
    /// Language
    lang: String,
    /// Estimated listening time in seconds
    listen_duration_estimate: u32,
    /// Estimated reading time in seconds
    time_to_read: u32,
    /// Image URL
    top_image_url: String,
}
