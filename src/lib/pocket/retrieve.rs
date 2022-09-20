//  Library
use super::Pocket;
use crate::lib::endpoint;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Pocket {
    pub async fn retrieve(&self, options: Options) -> Result<RetrieveResponse, reqwest::Error> {
        let consumer_key = self.consumer_key.to_owned();
        let access_token = self
            .access_token
            .as_ref()
            .expect("A valid access_token is required to interact with the API")
            .to_owned();

        let payload = Options {
            consumer_key: Some(consumer_key),
            access_token: Some(access_token),
            ..options
        };

        Ok(self
            .client
            .post(endpoint::RETRIEVE)
            .json(&payload)
            .send()
            .await?
            // .text()
            .json::<RetrieveResponse>()
            .await?)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Options {
    /// Retrieve unread, archive or all items
    state: Option<State>,
    /// Retrieve favorite if 1 else return non-favorite if 0
    favorite: Option<Favorite>,
    /// Returns items with the given tag-name. Return all untagged items if __untagged__
    tag: Option<String>,
    /// Return only article, video or images
    content_type: Option<ContentType>,
    /// Sort the results
    sort: Option<Sort>,
    /// Specify how much detail must be returned about the retrieved item
    detail_type: Option<DetailType>,
    /// Only return items whose title or url contains the search string
    search: Option<String>,
    /// Only return items from a particular domain
    domain: Option<String>,
    /// Only return items modified since the given unix timestamp
    since: Option<u32>,
    /// Only return count number of items
    count: Option<u32>,
    /// Only used with count; start returning from offset position of results
    offset: Option<u32>,
    /// Application consumer_key
    consumer_key: Option<String>,
    /// User access_token
    access_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum State {
    Unread,
    Archive,
    All,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Favorite {
    No = 0,
    Yes = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContentType {
    Article,
    Video,
    Image,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Sort {
    Newest,
    Oldest,
    Title,
    Site,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DetailType {
    Simple,
    Complete,
}

impl Options {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn state(&mut self, state: State) -> &mut Self {
        self.state = Some(state);
        return self;
    }

    pub fn favorite(&mut self, favorite: bool) -> &mut Self {
        let favorite = if favorite {
            Favorite::Yes
        } else {
            Favorite::No
        };
        self.favorite = Some(favorite);
        return self;
    }

    pub fn tag(&mut self, tag: String) -> &mut Self {
        self.tag = Some(tag);
        return self;
    }

    pub fn content_type(&mut self, content_type: ContentType) -> &mut Self {
        self.content_type = Some(content_type);
        return self;
    }

    pub fn sort(&mut self, sort: Sort) -> &mut Self {
        self.sort = Some(sort);
        return self;
    }

    pub fn detail_type(&mut self, detail_type: DetailType) -> &mut Self {
        self.detail_type = Some(detail_type);
        return self;
    }

    pub fn search(&mut self, search: String) -> &mut Self {
        self.search = Some(search);
        return self;
    }

    pub fn domain(&mut self, domain: String) -> &mut Self {
        self.domain = Some(domain);
        return self;
    }

    pub fn since(&mut self, since: u32) -> &mut Self {
        self.since = Some(since);
        return self;
    }

    pub fn count(&mut self, count: u32) -> &mut Self {
        self.count = Some(count);
        return self;
    }

    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.offset = Some(offset);
        return self;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveResponse {
    status: u8,
    complete: u8,
    since: u32,
    error: Option<String>,
    search_meta: SearchMeta,
    list: HashMap<String, RetrieveItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchMeta {
    search_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
    tags: Option<String>,
    /// A JSON object listing all of the authors associated with the item
    authors: Option<String>,
    /// A JSON object listing all of the images associated with the item
    images: Option<String>,
    /// A JSON object listing all of the videos associated with the item
    videos: Option<String>,
    /// Timestamp of when the resource was added
    time_added: String,
    /// Timestamp of when the resource was last updated
    time_updated: String,
    /// Timestamp of when the resource was last read
    time_read: String,
    /// Timestamp of when the resource was favorited
    time_favorited: String,
    /// Sorting ID number
    sort_id: u32,
    is_index: String,
    /// Language
    lang: String,
    /// Estimated listening time in seconds
    listen_duration_estimate: u32,
    /// Estimated reading time in seconds
    time_to_read: Option<u32>,
    /// Image URL
    top_image_url: Option<String>,
}
