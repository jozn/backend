use grammers_client::{Client, ClientHandle, Config};
use grammers_tl_types::enums::contacts::ResolvedPeer;
use grammers_tl_types::Serializable;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::Write;
// use futures::AsyncWriteExt;
use grammers_tl_types as tl;
use std::cell::Cell;
use std::fmt;
use std::sync::{Arc, Mutex};

// pub type G = Arc<Mutex<App>>;
// pub type G = Arc<App>;
pub type Binary = Vec<u8>;

// use crate::client_pool;
use grammers_mtsender::InvocationError;
use grammers_session::FileSession;
use serde::{Deserialize, Serialize};

/*pub struct App {
    pub login: Vec<LoginPhone>,
    pub channels: HashMap<i64, ChannelSpace>,
    pub sessions: Vec<Session>,
    pub dcs: Vec<DC>,
    // pub client: Client,
    pub clients: Mutex<Cell<client_pool::ClientPool>>,
}*/

// In Tg module MsgHolder is used for processing a channels messages list with all of it's associated
// channels (from forwarded messages) and other metadata
#[derive(Clone, Debug)]
pub struct MsgHolder {
    pub msgs: Vec<Msg>,
    pub channels: Vec<ChannelInfoCompact>,
    pub urls: Vec<String>,
    pub users: Vec<String>,
}

// Simplified and removed none present fields from tl::types::MessageFwdHeader
// Note: we did not found any Group Chat being set in tl::types::MessageFwdHeader > if forwarded from group user is being set
// Note: for forwarded messages their corssponding channel is being set in
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MsgForwarded {
    pub date: i32,         // Always set
    pub user_id: i32, // Set if forwarded from User's message (including bots) - also if message forwarded form a User group message
    pub channel_id: i32, // Set if forwarded from Channel's message
    pub channel_post: i32, // Shows the id of channel post
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Msg {
    pub silent: bool,
    pub post: bool,
    pub id: i32,
    pub from_id: i32,
    // pub to_id: crate::enums::Peer,
    // pub fwd_from: Option<crate::enums::MessageFwdHeader>,
    pub via_bot_id: i32,
    pub reply_to_msg_id: i32,
    pub date: i32,
    pub message: String,

    pub text_meta: Vec<MsgTextMeta>,
    // pub media: Option<crate::enums::MessageMedia>,
    // pub reply_markup: Option<crate::enums::ReplyMarkup>,
    // pub entities: Option<Vec<crate::enums::MessageEntity>>,//todo
    pub views: i32,
    pub edit_date: i32,
    // pub post_author: Option<String>,
    // pub grouped_id: Option<i64>,
    pub restricted: bool,
    pub forward: Option<MsgForwarded>,

    pub media_old: Option<MediaOld>,
    pub webpage_old: Option<WebPage>,

    pub medias: Vec<Media>,
    pub glassy_urls: Option<Vec<GlassyUrl>>,  // Extracted from telegram ReplyMarkup. used in below of some messages(like: Stock market links)
    // raw: tl::types::Message,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum MsgTextMetaType {
    Unknown,
    Hashtag,
    Url,
    TextUrl, // hyperLink

    Bold,
    Italic,
    Underline,
    Strike,
    Blockquote,
    Code, // In telegram client it's called "mono"
    Pre,

    Phone,
    Email,
    // bot command , mention,...
}

#[derive(Default,Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct MsgTextMeta {
    pub meta_type: MsgTextMetaType,
    pub offset: i32,
    pub length: i32,
    pub url: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Media {
    Empty,
    File(File),
    WebPage(WebPage),

    Poll, // Concept
    Unsupported, // Concept

    // Image, // Resized shared image
    // Video, //
    // Audio, // voice, music
    // File, // pdf, docs, apks,..
    // ImageFile, // Unresized image as file. Has thumb for display
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum FileMeta {
    Unknown, // todo remove
    ImageResizedFile(ImageResizedFile), // Resized shared image
    ImageFile(ImageFile), // Unresized image as file. Has thumb for display
    VideoFile(VideoFile), //
    AudioFile(AudioFile), // voice, music
    DocumentFile(DocumentFile), // pdf, docs, apks,..
    GifFile(GifFile),
    // Concept
    // Sticker,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct File {
    pub file_meta: FileMeta,

    pub id: i64,
    pub access_hash: i64,
    pub file_reference: Vec<u8>,
    pub date: i32,
    pub mime_type: String,
    pub size: i32,
    // pub thumbs: Option<Vec<crate::enums::PhotoSize>>,
    // pub video_thumbs: Option<Vec<crate::enums::VideoSize>>,
    pub dc_id: i32,
    // pub attributes: Vec<crate::enums::DocumentAttribute>,

    // If used just for Image > move to Image
    // FileLocationToBeDeprecated
    pub dep_volume_id: i64, //
    pub dep_local_id: i32,

    pub filename: String,
    // Us
    pub file_extension: String,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct ImageResizedFile {
    pub has_stickers: bool,
    pub w: i32,
    pub h: i32,
    // pub id: i64,
    // pub access_hash: i64,
    // pub file_reference: Vec<u8>,
    // pub date: i32,
    // pub sizes: Vec<crate::enums::PhotoSize>,
    // pub video_sizes: Option<Vec<crate::enums::VideoSize>>,
    // pub dc_id: i32,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct ImageFile {
    pub w: i32,
    pub h: i32,

    pub cover: Box<File>,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct VideoFile {
    pub round_message: bool,
    pub supports_streaming: bool,
    pub duration: i32,
    pub w: i32,
    pub h: i32,

    pub cover: Box<File>,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct AudioFile {
    pub voice: bool,
    pub duration: i32,
    pub title: String,
    pub performer: String,
    pub waveform: Vec<u8>,

    pub cover: Box<File>,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct DocumentFile {
    pub cover:  Box<File>,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct GifFile {
    pub duration: i32,
    pub w: i32,
    pub h: i32,

    pub cover:  Box<File>,
}

////////// Older Media //////////
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum MediaTypeOld {
    Unknown, // todo remove
    Image, // Resized shared image
    Video, //
    Audio, // voice, music
    File, // pdf, docs, apks,..
    ImageFile, // Unresized image as file. Has thumb for display
}

// #[derive(Derivative)]
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
// #[derive(Clone, Debug, Default)]
pub struct MediaOld { // todo: extract simple image form media
    pub media_type: MediaTypeOld,
    pub has_stickers: bool,
    pub id: i64,
    pub access_hash: i64,
    // #[derivative(Debug="ignore")]
    pub file_reference: Vec<u8>,
    pub date: i32,
    // pub sizes: Vec<tl::enums::PhotoSize>,
    pub dc_id: i32,
    pub photo_size_type: String, // for Image (Resized shared photos)

    // FileLocationToBeDeprecated
    pub dep_volume_id: i64, //
    pub dep_local_id: i32,

    // pub location: tl::enums::FileLocation,
    pub image_width: i32, // for Image, ImageFile
    pub image_height: i32, // for Image, ImageFile
    pub image_thumbs: Option<MediaThumb>,

    // shared among all
    pub size: i32,

    // Document
    // pub id: i64,
    // pub access_hash: i64,
    // pub file_reference: Vec<u8>,
    // pub date: i32,
    pub mime_type: String,
    // pub size: i32,
    // pub thumbs: Option<Vec<tl::enums::PhotoSize>>,
    // pub dc_id: i32,
    // pub attributes: Vec<tl::enums::DocumentAttribute>,
    pub animated: bool, // What is this?  gifs?

    // Video
    pub video_round_message: bool,
    pub video_supports_streaming: bool,
    pub video_duration: i32,
    pub video_width: i32,
    pub video_height: i32,
    pub video_thumbs_rec: Box<Option<MediaOld>>, // todo remove?
    pub video_thumbs: Option<MediaThumb>,     // todo with document thumb

    // Audio
    pub audio_voice: bool,
    pub audio_duration: i32, // merge
    pub audio_title: String,
    pub audio_performer: String,
    pub audio_waveform: Vec<u8>,

    pub file_name: String,

    pub has_sticker: bool,
    pub ttl_seconds: i32,

    // Us
    pub file_extension: String,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
// #[derive(Clone, Debug, Default)]
pub struct MediaThumb {
    pub size_type: String,
    pub dep_volume_id: i64,
    pub dep_local_id: i32,
    pub w: i32,
    pub h: i32,
    pub size: i32,
    // pub file_extention: String,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct GlassyUrl {
    pub row_id: i64,
    pub text: String,
    pub url: String,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
// #[derive(Clone, Debug, Default)]
pub struct WebPage { // In Flip site maybe add an lang field for "fa" ,"en"
    pub id: i64,
    pub url: String,         // "https://m.youtube.com/watch?v=fQVhppRP4Wo"
    pub display_url: String, // "youtube.com/watch?v=fQVhppRP4Wo"
    pub hash: i32,           // 0 58695
    pub page_type: String,   // opt - video photo article || video: YouTube - photo: Telegraph - article: Tabnak
    pub site_name: String,   // opt - showed in first line
    pub title: String,       // opt - showed below site name
    pub description: String, // opt - showed below title
    // pub photo: Option<crate::enums::Photo>,
    pub photo: Option<MediaOld>,
    // pub embed_url: Option<String>, // for video (Youtube) is being set - "https://www.youtube.com/embed/TkoYtz3Jm-o" | aparat: "https://hajifirouz1.cdn.asset.aparat.com/aparat-video/963d41be6d46..."
    // pub embed_type: Option<String>, // YT: "iframe" - Aparat: "video/mp4"
    // pub embed_width: Option<i32>, // 1280 - 640
    // pub embed_height: Option<i32>, // 720
    // pub duration: Option<i32>,
    // pub author: Option<String>,
    // pub document: Option<crate::enums::Document>, // For Aparat is being set to video mp4 file
    // pub cached_page: Option<crate::enums::Page>,
    // pub attributes: Option<Vec<crate::enums::WebPageAttribute>>,*/
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChannelSpace {
    pub info: ChannelInfo,
    pub msgs: HashMap<u32, Msg>,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct ChannelInfo {
    pub id: i32,
    pub title: String,
    pub username: String,
    pub about: String,
    pub link: String, //todo: remove? there is no where in telegram api we can get this > other means must link to chan
    pub members_count: i32,
    pub read_inbox_max_id: i32,
    pub access_hash: i64,
    pub date: i32,
    // pub avatar: Option<Avatar>,
    pub avatar: Option<MediaOld>,
    pub photo: u8,
    pub version: i32,
    // https://core.telegram.org/api/updates for pts
    pub pts: i32, // "pts indicates the server state after the new channel message events are generated" > not used now > maybe useful is we subscribe to channel
    pub restricted: bool, // true for porn is accessed with an Iran phone
    pub megagroup: bool,
    pub full_data: bool, // true if fetched directly - false for inline processing in message. Only if full_data is true it must be saved to database.
}

// In order to build ChannelInfo we need to query telegram with multi rpc. This compact used in two places:
// 1. struct is for the result of resolve channel by username (get_channel_by_username)
// 2. used for inline channel info in processing forwarded messages
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct ChannelInfoCompact {
    pub id: i32,
    pub title: String,
    pub username: String,
    pub members_count: i32,
    pub access_hash: i64,
    pub date: i32,
    pub photo: u8,
    pub version: i32,
    pub restricted: bool, // true for porn
    pub megagroup: bool,  // true for telegram groups (public groups) - false for channels
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DC {}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Session {}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LoginPhone {}

/////////// Tg ///////////
pub struct TgPool {
    pub client: ClientHandle,
}

impl TgPool {
    pub async fn invoke<R: tl::RemoteCall>(
        &self,
        request: &R,
    ) -> Result<R::Return, InvocationError> {
        let mut cp = self.client.clone();
        cp.invoke(request).await
    }
}

impl Default for MediaTypeOld {
    fn default() -> Self {
        MediaTypeOld::Unknown
    }
}

impl Default for MsgTextMetaType {
    fn default() -> Self {
        MsgTextMetaType::Unknown
    }
}

impl Default for FileMeta {
    fn default() -> Self {
        FileMeta::Unknown
    }
}
// Storage

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CachedUsernameData {
    pub username: String,
    pub channel_id: i32, // we do not care about others: super groups, users,...
    // pub tg_result: Option<ChannelByUsernameResult>,
    pub channel_info: Option<ChannelInfo>,
    pub taken: bool,
    pub last_checked: u32,
}

///////////////// Deprecated or Maybe /////////////
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Avatar {
    // it's photo_big
    pub is_video: bool,
    pub dep_volume_id: i64,
    pub dep_local_id: i32,
    pub dc_id: i32,
}

pub struct Caller {
    pub client: Client<FileSession>,
}

/////////// Sqlite ///////
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChannelData {
    pub channel_info: ChannelInfo,
    pub last_checked: u32,
}

///////////////////////////
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReqSyncChannel {
    pub channel_id: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResSyncChannel {
    pub channel_info: ChannelInfo,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReqSyncMessages {
    pub channel_id: i32,
    pub access_id: i64,
    pub from_message_id: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResSyncMessages {
    pub req: ReqSyncMessages,
    pub messages: Vec<Msg>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReqResolveUsername {
    pub username: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResResolveUsername {
    pub channel_id: i32,
    pub access_id: i64,
}
