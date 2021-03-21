// + Note: Datatypes is strip down in order to simplify working with them, just essentioal fileds:
// + tweets(just text and media) + user object
// + If tweet text start with "RT @usernaem:..." it is a retweet and "retweeted_status" will be
//  present
// + Add tweet_mode="extended" to url to get full_text instead of truncated text
// + if in_reply_to_user_id is set it is a replay to use own tweets maybe harvest? it is like
// comments
// + retweets and quote? can be truncated
// + Just use normal tweets, no retweet, quote, replay,
// + Urls are present and can be hidden
// + tweets text contain many things that are not showed to uesr: "RT @usernaem:" and links
// + Links are not shown many times: if it is external link it just shows some layout of that
//    external link

extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Welcome = Vec<Tweet>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub code: i64,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub created_at: String,
    pub id: u64,
    pub id_str: String,
    pub full_text: String,
    pub truncated: bool,
    pub entities: Entities, // Just for Urls if is needed
    pub in_reply_to_status_id: Option<u64>,
    pub in_reply_to_user_id: Option<u64>,
    pub user: User,
    pub is_quote_status: bool,
    pub retweet_count: i64,
    pub favorite_count: i64,
    pub favorited: bool, // me?
    pub retweeted: bool, // me?
    pub lang: String,
    pub extended_entities: Option<ExtendedEntities>, // Use this for any media counsumtion
    pub possibly_sensitive: Option<bool>,
}

// Do not use this for media consumption: 1) Just "photo" even for videos 2) Just one media even
// if included 5 images in tweet
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entities {
    pub urls: Vec<Url>,
    pub media: Option<Vec<Media>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtendedEntities {
    pub media: Vec<Media>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Media {
    pub id: f64,
    pub id_str: String,
    pub indices: Vec<i64>,
    pub media_url: String,
    pub media_url_https: String,
    pub url: String,
    pub display_url: String,
    pub expanded_url: String,
    #[serde(rename = "type")]
    pub media_type: String, // "photo", "video", "
    pub sizes: Sizes,
    pub video_info: Option<VideoInfo>,
    //    pub additional_media_info: Option<AdditionalMediaInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sizes {
    pub medium: Large,
    pub thumb: Large,
    pub small: Large,
    pub large: Large,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Large {
    pub w: i64,
    pub h: i64,
    pub resize: String, // fit , crop
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoInfo {
    pub aspect_ratio: Vec<i64>,
    pub duration_millis: Option<i64>,
    pub variants: Vec<Variant>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Variant {
    pub bitrate: Option<i64>,
    pub content_type: String, // "video/mp4"
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Url {
    pub url: String,
    pub expanded_url: String,
    pub display_url: String,
    pub indices: Vec<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub screen_name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    //    pub entities: UserEntities,
    pub protected: bool,
    pub followers_count: i64,
    pub friends_count: i64,
    pub listed_count: i64,
    pub created_at: String,
    pub favourites_count: i64,
    pub verified: bool,
    pub statuses_count: i64,
    pub profile_image_url: Option<String>,
    pub profile_image_url_https: Option<String>,
    pub profile_banner_url: Option<String>,
    pub default_profile_image: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserEntities {
    pub url: Vec<Url>,
    pub description: Vec<Url>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Followers {
    pub ids: Vec<u64>,
    pub next_cursor: i64,
    pub next_cursor_str: String,
    pub previous_cursor: i64,
    pub previous_cursor_str: String,
    pub total_count: Option<serde_json::Value>,
}

pub type LookupUsers = Vec<User>;

// Rate limiting
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RateLimiting {
    pub rate_limit_context: RateLimitContext,
    pub resources: Resources,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RateLimitContext {
    pub application: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Resources {
    //    pub lists: HashMap<String, AccountActivity>,
//    pub application: Application,
//    pub guest: Guest,
//    pub friendships: Friendships,
//    pub promoted_content: PromotedContent,
//    pub users: HashMap<String, AccountActivity>,
//    pub teams: Teams,
//    pub followers: Followers,
//    pub statuses: HashMap<String, AccountActivity>,
//    pub custom_profiles: CustomProfiles,
//    pub webhooks: Webhooks,
//    pub labs: HashMap<String, AccountActivity>,
//    pub i: HashMap<String, AccountActivity>,
//    pub help: Help,
//    #[serde(rename = "graphql&POST")]
//    pub graphql_post: GraphqlPost,
//    pub friends: Friends,
//    pub sandbox: Sandbox,
//    pub traffic: Traffic,
//    pub account_activity: HashMap<String, AccountActivity>,
//    pub favorites: Favorites,
//    pub device: Device,
//    pub tweets: Tweets,
//    pub search: Search,
//    pub trends: Trends,
//    pub live_pipeline: LivePipeline,
//    pub graphql: Graphql,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    //    #[serde(rename = "/application/rate_limit_status")]
// pub application_rate_limit_status: AccountActivity,
}
