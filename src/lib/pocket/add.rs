//  Library
use super::Pocket;
use crate::lib::endpoint;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct AddOptions {
    pub url: String,
    pub title: Option<String>,
    pub tags: Option<String>,
    pub tweet_id: Option<String>,
}

impl AddOptions {
    pub fn new(url: &str) -> Self {
        Self {
            url: String::from(url),
            ..Default::default()
        }
    }
    pub fn set_title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        return self;
    }
    pub fn set_tags(&mut self, tags: String) -> &mut Self {
        self.tags = Some(tags);
        return self;
    }
    pub fn set_tweet_id(&mut self, tweet_id: String) -> &mut Self {
        self.tweet_id = Some(tweet_id);
        return self;
    }
}

impl Pocket {
    /// Save an item to the user's Pocket list
    pub async fn add(&self, options: AddOptions) -> Result<AddResponse, reqwest::Error> {
        let consumer_key = self.consumer_key.clone();
        let access_token = self
            .access_token
            .as_ref()
            .expect("A valid access_token is required for this operation")
            .to_owned();

        let payload = &AddPayload {
            url: String::from(options.url),
            title: options.title,
            tags: options.tags,
            tweet_id: options.tweet_id,
            consumer_key,
            access_token,
        };

        Ok(self
            .client
            .post(endpoint::ADD)
            .json(payload)
            .send()
            .await?
            .json::<AddResponse>()
            .await?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddPayload {
    url: String,
    title: Option<String>,
    tags: Option<String>,
    tweet_id: Option<String>,
    consumer_key: String,
    access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddResponse {
    status: u8,
    item: AddItem,
}

#[derive(Debug, Serialize, Deserialize)]
struct AddItem {
    /// A unique identifier for the added item
    item_id: String,
    /// The original url for the added item
    normal_url: String,
    /// A unique identifier for the resolved item
    resolved_id: String,
    /// The resolved url for the added item. The easiest way to think about the resolved_url - if you add a bit.ly link, the resolved_url will be the url of the page the bit.ly link points to
    resolved_url: String,
    /// A unique identifier for the domain of the resolved_url
    domain_id: String,
    /// A unique identifier for the domain of the normal_url
    origin_domain_id: String,
    /// The response code received by the Pocket parser when it tried to access the item
    response_code: String,
    /// The MIME type returned by the item
    mime_type: String,
    /// The content length of the item
    content_length: String,
    /// The encoding of the item
    encoding: String,
    /// The date the item was resolved
    date_resolved: String,
    /// The date the item was published (if the parser was able to find one)
    date_published: String,
    /// The title of the resolved_url
    title: String,
    /// The excerpt of the resolved_url
    excerpt: String,
    /// For an article, the number of words
    word_count: String,
    /// 0: no image; 1: has an image in the body of the article; 2: is an image
    has_image: String,
    /// 0: no video; 1: has a video in the body of the article; 2: is a video
    has_video: String,
    /// 0 or 1; If the parser thinks this item is an index page it will be set to 1
    is_index: String,
    /// 0 or 1; If the parser thinks this item is an article it will be set to 1
    is_article: String,
    /// Array of author data (if author(s) were found)
    authors: Vec<String>,
    /// Array of image data (if image(s) were found)
    images: Vec<ImageData>,
    /// Array of video data (if video(s) were found)
    videos: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ImageData {
    item_id: String,
    image_id: String,
    src: String,
    width: u16,  //  String
    height: u16, // String
    caption: String,
    credit: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VideoData {
    item_id: String,
    video_id: String,
    src: String,
    width: u16,            //  String
    height: u16,           //  String
    length: Option<usize>, //  String
    vid: String,
    vtype: u16,
}
