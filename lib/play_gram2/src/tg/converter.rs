use async_std::task;
use grammers_client::{Client, Config};
use grammers_mtsender::InvocationError;
use grammers_session as session;
use grammers_tl_types as tl;
use grammers_tl_types::enums::messages::Messages;
use grammers_tl_types::enums::{
    ChatPhoto, FileLocation, Message, MessageEntity, MessageReplyHeader, Peer,
};
use grammers_tl_types::RemoteCall;
use std::io::Write;

use crate::{errors::TelegramGenErr, types, utils};
use types::*;
// use std::default::default;

// Notes:
// Telegram mime_type: "application/x-tgsticker" is telegram own stikers with ~7KB size. "thumb" is also is set.
// thumb in Document is for inline images: being set in gifs, stickers,...
// gifs: if document is animated it's gif (could mp4 gif > hence it set video attributes)
// in documents ttl_seconds is not being set even if auto delete message being set.
// have whitelist filtering not a black list: mpt, ogg, mp3, pdf, docs,...
// if audio is set to voice:true it will not have save to my music
// is audio is set to voice:true it will have waveform otherwise not.
// audio: performer and title is being set just for music not voice,
// filename is being set for music (but not voice), animated (gif) mostly "mp4.mp4",
// image doc: thumb is ~250-320px and ~10-12px, filename as it's being set - thumb is "i" and "m" (inline+ medium")
// filename is not being set for Photo > good for privacy (actualay photo has not the filename attribute anyway)
// .forwards is the number of forwards of a message > if Some(num) it shows it a user message post if None it's a some systems posts.
// .grouped_id in message means the messages are bundles (multi photo, files,...). The order is reversed (recent message is shown last in the bundle view)

// Convert Telegram Message to types::Msg + list of urls
pub(super) fn process_inline_channel_messages(
    messages: Vec<tl::enums::Message>,
) -> (Vec<types::Msg>, Vec<String>) {
    let mut msgs = vec![];
    let mut urls: Vec<String> = vec![];

    for msg_enum in messages {
        match msg_enum {
            Message::Empty(em) => {}
            Message::Service(service_msg) => {}
            Message::Message(m) => {
                // Hack: Some system messages is being sent as normal real messages (ex: "Messages were set to
                // automatically delete ..." in this case Telegram sets views and forwards to null.
                // Not suer about how this act about those old telegram messages who do not have a
                // view fields (did those messages eventually got a views filed?)
                if m.views.is_none() && m.forwards.is_none() {
                    continue;
                }

                let mut msg = conv_message_to_msg(m.clone());
                let mut u = extract_urls_from_message_entity(m.entities.clone());

                msg.text_meta = extract_text_meta_from_message_entity(m.entities);
                // Extract Photo, Video, File ...
                if let Some(mm) = m.media.clone() {
                    // File Media
                    let fm_opt = process_inline_file_media(mm.clone());
                    if fm_opt.is_some() {
                        msg.medias.push(types::Media::File(fm_opt.unwrap()));
                    }

                    // Webpage
                    let wm_opt = process_inline_webpage(mm.clone());
                    if wm_opt.is_some() {
                        msg.medias.push(types::Media::WebPage(wm_opt.unwrap()));
                    }
                }

                // Extract glassy button urls
                if let Some(rm) = m.reply_markup {
                    msg.glassy_urls = process_inline_glassy_urls(rm);
                }

                u.into_iter().for_each(|v| urls.push(v));
                // urls.append(&mut u); // todo fix?
                msgs.push(msg);
            }
        }
    }

    (msgs, urls)
}

// this extract ChannelInfo from an array of chats
pub(super) fn process_inline_channel_chats(
    chats: Vec<tl::enums::Chat>,
) -> Vec<types::ChannelInfoCompact> {
    let mut out = vec![];

    for chat in chats {
        // let mut ci = types::ChannelInfo::default(); // todo embed this

        use tl::enums::Chat;
        match chat {
            Chat::Channel(ch) => {
                let ci = types::ChannelInfoCompact {
                    id: ch.id,
                    title: ch.title.clone(),
                    username: ch.username.clone().unwrap_or("".to_string()),
                    members_count: ch.participants_count.unwrap_or(0),
                    access_hash: ch.access_hash.unwrap_or(0),
                    date: ch.date,
                    photo: 0,
                    version: ch.version,
                    restricted: ch.restricted,
                    megagroup: ch.megagroup,
                };
                out.push(ci);
            }
            _ => {}
        };
    }
    out
}

pub(super) fn process_inline_channel_users(bots: &Vec<tl::enums::User>) {}

fn process_inline_file_media(mm: tl::enums::MessageMedia) -> Option<types::FileMedia> {
    // let mut m = types::Media::default();

    use tl::enums::MessageMedia;
    match mm {
        MessageMedia::Photo(photo) => {
            if let Some(pic) = photo.photo {
                let mp = conv_photo_to_file_media(pic);
                if let Some(mut mp) = mp {
                    // mp.media_type = MediaType::Image;
                    // mp.ttl_seconds = photo.ttl_seconds.unwrap_or(0);
                    return Some(mp);
                }
            }
        }

        MessageMedia::Document(doc1) => {
            // println!("============== document {:#?}", doc);
            // m.ttl_seconds = doc1.ttl_seconds.unwrap_or(0);
            if let Some(document) = doc1.document {
                use tl::enums::Document;
                match document {
                    Document::Document(doc) => {
                        // Note: we didnt saw "doc.video_thumbs" being set. > legacy?
                        let p = doc.clone();

                        let mut j = types::FileMedia {
                            file_meta: FileMetaInfo::DocumentFile(DocumentFile::default()), // It should be to docs as pdf, zip,.. do not set any especial attribute later
                            id: p.id,
                            access_hash: p.access_hash,
                            file_reference: p.file_reference,
                            date: p.date,
                            mime_type: p.mime_type.clone(),
                            size: p.size,
                            dc_id: p.dc_id,
                            file_name: "".to_string(), // set later
                            file_extension: utils::get_file_extension_from_mime_type(&p.mime_type),
                        };

                        let mut is_animated = false; //todo gif

                        for atr in p.attributes {
                            use tl::enums::DocumentAttribute;
                            match atr {
                                DocumentAttribute::ImageSize(s) => {
                                    let mut k = types::ImageFile {
                                        w: s.w,
                                        h: s.h,
                                        cover: None,
                                    };
                                    if p.thumbs.is_some() {
                                        k.cover = conv_thumb_cover(p.thumbs.clone().unwrap())
                                    }
                                    j.file_meta = types::FileMetaInfo::ImageFile(k);
                                    // m.media_type = MediaTypeOld::ImageFile;
                                    // m.image_width = s.w;
                                    // m.image_height = s.h;
                                }
                                DocumentAttribute::Animated => {
                                    is_animated = true; // What is this?
                                }
                                DocumentAttribute::Sticker(s) => {
                                    // We do not support Sticker
                                }
                                DocumentAttribute::Video(s) => {
                                    let mut k = types::VideoFile {
                                        round_message: s.round_message,
                                        supports_streaming: s.supports_streaming,
                                        duration: s.duration,
                                        w: s.w,
                                        h: s.h,
                                        cover: None,
                                    };
                                    if p.thumbs.is_some() {
                                        k.cover = conv_thumb_cover(p.thumbs.clone().unwrap())
                                    }
                                    j.file_meta = types::FileMetaInfo::VideoFile(k);
                                    // m.media_type = MediaTypeOld::Video;
                                    // m.video_round_message = s.round_message;
                                    // m.video_supports_streaming = s.supports_streaming;
                                    // m.video_duration = s.duration;
                                    // m.image_width = s.w;
                                    // m.image_height = s.h;
                                }
                                DocumentAttribute::Audio(s) => {
                                    let mut k = types::AudioFile {
                                        voice: s.voice,
                                        duration: s.duration,
                                        title: s.title.unwrap_or("".to_string()),
                                        performer: s.performer.unwrap_or("".to_string()),
                                        waveform: s.waveform.unwrap_or(vec![]),
                                        cover: None, // set laer
                                    };
                                    if p.thumbs.is_some()  {
                                        k.cover = conv_thumb_cover(p.thumbs.clone().unwrap())
                                    }
                                    j.file_meta = types::FileMetaInfo::AudioFile(k);
                                    // m.media_type = MediaTypeOld::Audio;
                                    // m.audio_voice = s.voice;
                                    // m.audio_duration = s.duration;
                                    // m.audio_title = s.title.unwrap_or("".to_string());
                                    // m.audio_performer = s.performer.unwrap_or("".to_string());
                                    // m.audio_waveform = s.waveform.unwrap_or(vec![]);
                                }
                                DocumentAttribute::Filename(s) => {
                                    j.file_name = s.file_name;
                                }
                                DocumentAttribute::HasStickers => {
                                    //m.has_stickers = true;
                                }
                            }
                        }
                        // todo fix cover for docs

                        //tod move to just video + remove rec
                        /*                        if p.thumbs.is_some() {
                            // m.video_thumbs_rec =
                            //     Box::new(conv_video_thumbs_rec(&m, p.thumbs.clone().unwrap()));
                            m.video_thumbs = conv_thumb_cover(p.thumbs.clone().unwrap());
                            m.image_thumbs = conv_thumb_cover(p.thumbs.unwrap());
                            // println!("+++ vidoe: {:#?} ", doc)
                        }*/

                        return Some(j);
                    }
                    Document::Empty(e) => {}
                }
                /* match document {
                    Document::Document(doc) => {
                        // Note: we didnt saw "doc.video_thumbs" being set. > legacy?
                        let p = doc.clone();

                        // todo
                        let mut m = types::MediaOld {
                            media_type: MediaTypeOld::File,
                            // has_stickers: false,
                            id: p.id,
                            access_hash: p.access_hash,
                            file_reference: p.file_reference,
                            date: p.date,
                            dc_id: p.dc_id,
                            photo_size_type: "".to_string(),
                            dep_volume_id: 0,
                            dep_local_id: 0,
                            image_width: 0,
                            image_height: 0,
                            size: p.size,
                            mime_type: p.mime_type.clone(),
                            animated: false,
                            video_round_message: false,
                            video_supports_streaming: false,
                            video_duration: 0,
                            video_thumbs_rec: Box::new(None),
                            video_thumbs: None,
                            audio_voice: false,
                            audio_title: "".to_string(),
                            audio_performer: "".to_string(),
                            audio_waveform: vec![],
                            file_name: "".to_string(),
                            has_sticker: false,
                            ttl_seconds: doc1.ttl_seconds.unwrap_or(0),
                            file_extension: utils::get_file_extension_from_mime_type(&p.mime_type),
                            ..Default::default()
                        };


                        //todo move to just video + remove rec
                        if p.thumbs.is_some() {
                            // m.video_thumbs_rec =
                            //     Box::new(conv_video_thumbs_rec(&m, p.thumbs.clone().unwrap()));
                            m.video_thumbs = conv_video_thumbs(p.thumbs.clone().unwrap());
                            m.image_thumbs = conv_video_thumbs(p.thumbs.unwrap());
                            // println!("+++ vidoe: {:#?} ", doc)
                        }

                        for atr in p.attributes {
                            use tl::enums::DocumentAttribute;
                            match atr {
                                DocumentAttribute::ImageSize(s) => {
                                    m.media_type = MediaTypeOld::ImageFile;
                                    m.image_width = s.w;
                                    m.image_height = s.h;
                                }
                                DocumentAttribute::Animated => {
                                    m.animated = true; // What is this?
                                }
                                DocumentAttribute::Sticker(s) => {
                                    // We do not support Sticker
                                }
                                DocumentAttribute::Video(s) => {
                                    m.media_type = MediaTypeOld::Video;
                                    m.video_round_message = s.round_message;
                                    m.video_supports_streaming = s.supports_streaming;
                                    m.video_duration = s.duration;
                                    m.image_width = s.w;
                                    m.image_height = s.h;
                                }
                                DocumentAttribute::Audio(s) => {
                                    m.media_type = MediaTypeOld::Audio;
                                    m.audio_voice = s.voice;
                                    m.audio_duration = s.duration;
                                    m.audio_title = s.title.unwrap_or("".to_string());
                                    m.audio_performer = s.performer.unwrap_or("".to_string());
                                    m.audio_waveform = s.waveform.unwrap_or(vec![]);
                                }
                                DocumentAttribute::Filename(s) => {
                                    m.file_name = s.file_name;
                                }
                                DocumentAttribute::HasStickers => {
                                    m.has_stickers = true;
                                }
                            }
                        }
                        return Some(m);
                    }
                    Document::Empty(e) => {}
                }*/
            };
        }
        MessageMedia::Empty => {}
        MessageMedia::Geo(t) => {}
        MessageMedia::Contact(t) => {}
        MessageMedia::Unsupported => {}
        MessageMedia::WebPage(t) => {
            // We process webpage in: process_inline_webpage()
        }
        MessageMedia::Venue(t) => {}
        MessageMedia::Game(t) => {}
        MessageMedia::Invoice(t) => {}
        MessageMedia::GeoLive(t) => {}
        MessageMedia::Poll(t) => {}
        MessageMedia::Dice(t) => {}
    };
    None
}

fn process_inline_webpage(mm: tl::enums::MessageMedia) -> Option<types::WebPageMedia> {
    use tl::enums::MessageMedia;
    match mm {
        MessageMedia::WebPage(t) => {
            use tl::enums::WebPage;
            match t.webpage {
                WebPage::Empty(v) => {}
                WebPage::Pending(v) => {}
                WebPage::Page(v) => {
                    let mut w = types::WebPageMedia {
                        id: v.id,
                        url: v.url,
                        display_url: v.display_url,
                        hash: v.hash,
                        page_type: v.r#type.unwrap_or("".to_string()),
                        site_name: v.site_name.unwrap_or("".to_string()),
                        title: v.title.unwrap_or("".to_string()),
                        description: v.description.unwrap_or("".to_string()),
                        photo: None,
                    };

                    if v.photo.is_some() {
                        w.photo = conv_photo_to_file_media(v.photo.unwrap())
                    }

                    return Some(w);
                }
                WebPage::NotModified(v) => {}
            }
        }
        _ => {}
    };
    None
}

fn process_inline_glassy_urls(
    reply_markups: tl::enums::ReplyMarkup,
) -> Option<Vec<types::GlassyUrl>> {
    let mut arr = vec![];
    match reply_markups {
        tl::enums::ReplyMarkup::ReplyInlineMarkup(inline_markup) => {
            let mut current_row_num = -1;
            for row in inline_markup.rows {
                current_row_num += 1;

                match row {
                    tl::enums::KeyboardButtonRow::Row(button_row) => {
                        for keyborad_button in button_row.buttons {
                            match keyborad_button {
                                tl::enums::KeyboardButton::Url(button_url) => {
                                    let out_btn = types::GlassyUrl {
                                        row_id: current_row_num,
                                        text: button_url.text,
                                        url: button_url.url,
                                    };
                                    arr.push(out_btn);
                                }

                                tl::enums::KeyboardButton::UrlAuth(u) => {
                                    // This is for things like comments bot
                                }
                                _ => {
                                    // Not supported
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }

    if arr.len() > 0 {
        return Some(arr);
    }
    None
}

fn conv_message_to_msg(m: tl::types::Message) -> types::Msg {
    let mut fwd = None;
    println!("+++++++++> Mesage: {:#?}", &m);
    // m.reply_to
    // Processing forward header
    if let Some(fw) = m.fwd_from {
        use tl::enums::MessageFwdHeader::*;
        match fw {
            Header(f) => {
                let mut channel_id = 0;
                let mut user_id = 0;
                if f.from_id.is_some() {
                    match f.from_id.unwrap() {
                        tl::enums::Peer::Channel(ch) => {
                            channel_id = ch.channel_id;
                        }
                        tl::enums::Peer::User(u) => {
                            user_id = u.user_id;
                        }
                        tl::enums::Peer::Chat(_) => { /*Legacy?*/ }
                    }
                }

                fwd = Some(types::MsgForwarded {
                    date: f.date,
                    user_id: user_id,
                    channel_id: channel_id,
                    channel_post: f.channel_post.unwrap_or(0),
                });
            }
        }
    };

    // Processing replay header
    let mut replay_to_msgs_id = 0;
    if m.reply_to.is_some() {
        match m.reply_to.unwrap() {
            MessageReplyHeader::Header(h) => {
                replay_to_msgs_id = h.reply_to_msg_id;
            }
        }
    }
    // println!("forward {:#?} ", fwd);
    types::Msg {
        silent: m.silent,
        post: m.post,
        id: m.id,
        from_id: m.id,
        via_bot_id: m.via_bot_id.unwrap_or(0),
        reply_to_msg_id: replay_to_msgs_id,
        date: m.date,
        message: m.message,
        text_meta: vec![], // set later
        views: m.views.unwrap_or(0),
        edit_date: m.edit_date.unwrap_or(0),
        restricted: m.restriction_reason.is_some(),
        forward: fwd,
        // media_old: None,
        // webpage_old: None,
        medias: vec![],
        glassy_urls: None,
    }
}

fn extract_urls_from_message_entity(
    entities: Option<Vec<tl::enums::MessageEntity>>,
) -> Vec<String> {
    let mut urls = vec![];
    if let Some(ent) = entities {
        for v in ent {
            use tl::enums::MessageEntity::*;
            match v {
                TextUrl(t) => urls.push(t.url),
                _ => {}
            }
        }
    };
    urls
}

fn extract_text_meta_from_message_entity(
    entities: Option<Vec<tl::enums::MessageEntity>>,
) -> Vec<MsgTextMeta> {
    let mut out = vec![];
    if let Some(ent) = entities {
        for v in ent {
            // use tl::enums::MessageEntity::*;
            use tl::enums::MessageEntity;
            match v {
                // tl::enums::MessageEntity::TextUrl(t) => urls.push(t.url),
                MessageEntity::Unknown(_) => {}
                MessageEntity::Mention(_) => {}
                MessageEntity::Hashtag(p) => {
                    let m = MsgTextMeta {
                        meta_type: MsgTextMetaType::Hashtag,
                        offset: p.offset,
                        length: p.length,
                        ..Default::default()
                    };
                    out.push(m);
                }
                MessageEntity::BotCommand(_) => {}
                MessageEntity::Url(p) => {
                    let m = MsgTextMeta {
                        meta_type: MsgTextMetaType::Url,
                        offset: p.offset,
                        length: p.length,
                        ..Default::default()
                    };
                    out.push(m);
                }
                MessageEntity::Email(p) => {
                    let m = MsgTextMeta {
                        meta_type: MsgTextMetaType::Email,
                        offset: p.offset,
                        length: p.length,
                        ..Default::default()
                    };
                    out.push(m);
                }
                MessageEntity::Bold(p) => {
                    let m = MsgTextMeta {
                        meta_type: MsgTextMetaType::Bold,
                        offset: p.offset,
                        length: p.length,
                        ..Default::default()
                    };
                    out.push(m);
                }
                MessageEntity::Italic(p) => {
                    let m = MsgTextMeta {
                        meta_type: MsgTextMetaType::Italic,
                        offset: p.offset,
                        length: p.length,
                        ..Default::default()
                    };
                    out.push(m);
                }
                MessageEntity::Code(p) => {
                    let m = MsgTextMeta {
                        meta_type: MsgTextMetaType::Code,
                        offset: p.offset,
                        length: p.length,
                        ..Default::default()
                    };
                    out.push(m);
                }
                MessageEntity::Pre(p) => {
                    let m = MsgTextMeta {
                        meta_type: MsgTextMetaType::Pre,
                        offset: p.offset,
                        length: p.length,
                        ..Default::default()
                    };
                    out.push(m);
                }
                MessageEntity::TextUrl(p) => {
                    let m = MsgTextMeta {
                        meta_type: MsgTextMetaType::TextUrl,
                        offset: p.offset,
                        length: p.length,
                        url: p.url,
                    };
                    out.push(m);
                }
                MessageEntity::MentionName(_) => {}
                MessageEntity::InputMessageEntityMentionName(_) => {}
                MessageEntity::Phone(_) => {
                    // Not supported as Telegram output contains error for Phone numbers (many number is parsed as phone number)
                }
                MessageEntity::Cashtag(_) => {}
                MessageEntity::Underline(p) => {
                    let m = MsgTextMeta {
                        meta_type: MsgTextMetaType::Underline,
                        offset: p.offset,
                        length: p.length,
                        ..Default::default()
                    };
                    out.push(m);
                }
                MessageEntity::Strike(p) => {
                    let m = MsgTextMeta {
                        meta_type: MsgTextMetaType::Strike,
                        offset: p.offset,
                        length: p.length,
                        ..Default::default()
                    };
                    out.push(m);
                }
                MessageEntity::Blockquote(p) => {
                    let m = MsgTextMeta {
                        meta_type: MsgTextMetaType::Blockquote,
                        offset: p.offset,
                        length: p.length,
                        ..Default::default()
                    };
                    out.push(m);
                }
                MessageEntity::BankCard(_) => {}
            }
        }
    };
    out
}

fn conv_file_location(fl: tl::enums::FileLocation) -> (i64, i32) {
    match fl {
        tl::enums::FileLocation::ToBeDeprecated(l) => (l.volume_id, l.local_id),
    }
}

// pub is used for avatar extraction
pub fn conv_photo_to_file_media(photo_enum: tl::enums::Photo) -> Option<types::FileMedia> {
    use tl::enums::Photo;
    match photo_enum {
        Photo::Photo(photo) => {
            let p = photo;

            let mut j = types::FileMedia {
                file_meta: Default::default(), // set later
                id: p.id,
                access_hash: p.access_hash,
                file_reference: p.file_reference,
                date: p.date,
                mime_type: "".to_string(), // Photo does not have mime type: seems all Photo are jpeg (todo verify this for png, and webp)
                size: 0,                   // set later
                dc_id: p.dc_id,
                file_name: "".to_string(),      // None for Photo
                file_extension: ".jpg".to_string(), // None for Photo
            };

            let mut k = types::ImageResizedFile {
                has_stickers: p.has_stickers,
                w: 0,
                h: 0,
                size_type: "".to_string(),
                dep_volume_id: 0,
                dep_local_id: 0,
                size: 0,
            };

            for s in p.sizes {
                use tl::enums::PhotoSize;
                match s {
                    PhotoSize::Size(ps) => {
                        if j.size < ps.size {
                            // select the highest resolution
                            k.w = ps.w;
                            k.h = ps.h;
                            k.size_type = ps.r#type;

                            j.size = ps.size;
                            k.size = ps.size;

                            let fl = conv_file_location(ps.location);
                            k.dep_volume_id = fl.0; //todo k?
                            k.dep_local_id = fl.1;

                            j.file_meta = types::FileMetaInfo::ImageResizedFile(k.clone())
                        }
                    }
                    _ => {}
                }
            }
            return Some(j);

            /*
            let mut m = types::MediaOld::default(); // TODO inline one

                        m.media_type = types::MediaTypeOld::Image;
                        m.has_sticker = p.has_stickers;
                        m.id = p.id;
                        m.access_hash = p.access_hash;
                        m.file_reference = p.file_reference;
                        m.date = p.date;
                        m.dc_id = p.dc_id;
                        m.file_extension = ".jpg".to_string();

                        for s in p.sizes {
                            use tl::enums::PhotoSize;
                            match s {
                                PhotoSize::Size(ps) => {
                                    if m.size < ps.size {
                                        // select the maximum
                                        m.image_width = ps.w;
                                        m.image_height = ps.h;
                                        m.size = ps.size;
                                        m.photo_size_type = ps.r#type;

                                        let fl = conv_file_location(ps.location);
                                        m.dep_volume_id = fl.0;
                                        m.dep_local_id = fl.1;
                                    }
                                }
                                _ => {}
                            }
                        }*/
        }
        Photo::Empty(e) => {}
    };
    None
}

fn conv_thumb_cover(vts: Vec<tl::enums::PhotoSize>) -> Option<ImageResizedFile> {
    if vts.len() == 0 {
        return None;
    }

    let mut m = types::ImageResizedFile::default();
    let mut has_set = false;
    for vt in vts {
        use tl::enums::PhotoSize;
        match vt {
            PhotoSize::Size(s) => {
                // select the maximum one
                if m.size < s.size {
                    has_set = true;
                    m.size_type = s.r#type;
                    m.w = s.w;
                    m.h = s.h;
                    m.size = s.size;

                    use tl::enums::FileLocation;
                    match s.location {
                        FileLocation::ToBeDeprecated(l) => {
                            m.dep_volume_id = l.volume_id;
                            m.dep_local_id = l.local_id;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    if has_set {
        Some(m)
    } else {
        None // should never reach
    }
}

////////////////// Archive of old Media ///////////////////
/*

fn conv_video_thumbs(vts: Vec<tl::enums::PhotoSize>) -> Option<MediaThumb> {
    if vts.len() == 0 {
        return None;
    }

    let mut m = types::MediaThumb::default();

    for vt in vts {
        use tl::enums::PhotoSize;
        match vt {
            PhotoSize::Size(s) => {
                // select the maximum one
                if m.size < s.size {
                    m.size_type = s.r#type;
                    m.w = s.w;
                    m.h = s.h;
                    m.size = s.size;

                    use tl::enums::FileLocation;
                    match s.location {
                        FileLocation::ToBeDeprecated(l) => {
                            m.dep_volume_id = l.volume_id;
                            m.dep_local_id = l.local_id;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Some(m)
}

fn conv_video_thumbs_rec(med: &types::MediaOld, sizes: Vec<tl::enums::PhotoSize>) -> Option<MediaOld> {
    let mut media_out = MediaOld {
        id: med.id,
        access_hash: med.access_hash,
        file_reference: med.file_reference.clone(),
        file_extension: "jpg".to_string(),
        ..Default::default()
    };

    for photo_size in sizes {
        use tl::enums::PhotoSize;
        match photo_size {
            PhotoSize::Size(size) => {
                // Select the maximum one
                if media_out.size < size.size {
                    media_out.photo_size_type = size.r#type;
                    media_out.image_width = size.w;
                    media_out.image_height = size.h;
                    media_out.size = size.size;

                    use tl::enums::FileLocation;
                    match size.location {
                        FileLocation::ToBeDeprecated(file_loc) => {
                            media_out.dep_volume_id = file_loc.volume_id;
                            media_out.dep_local_id = file_loc.local_id;
                        }
                    }
                };
                return Some(media_out);
            }
            _ => {}
        }
    }
    None
}
*/
