use async_std::task;
use grammers_client::{Client, Config};
use grammers_mtsender::InvocationError;
use grammers_session as session;
use grammers_tl_types as tl;
use grammers_tl_types::enums::messages::Messages;
use grammers_tl_types::enums::{ChatPhoto, FileLocation, Message, MessageEntity};
use grammers_tl_types::RemoteCall;
use std::io::Write;

use crate::tg::converter;
use crate::types::{Avatar, MediaOld, MediaThumb, MsgHolder, TgPool};
use crate::{errors::TelegramGenErr, types, utils};

use log::kv::Source;

#[derive(Clone, Debug)]
pub struct ReqGetMessages {
    pub channel_id: i32,
    pub access_hash: i64,
    pub offset_id: i32,
    pub offset_date: i32,
    pub add_offset: i32,
    pub limit: i32,
    pub max_id: i32,
    pub min_id: i32,
    pub hash: i32,
}

pub async fn get_config(caller: &TgPool) -> Result<tl::enums::Config, TelegramGenErr> {
    let request = tl::functions::help::GetConfig {};
    let res = caller.invoke(&request).await?;
    // println!("config {:#?}", res);
    Ok(res)
}

pub async fn get_channel_info(
    caller: &TgPool,
    channel_id: i32,
    access_hash: i64,
) -> Result<types::ChannelInfo, TelegramGenErr> {
    let request = tl::functions::channels::GetFullChannel {
        channel: tl::enums::InputChannel::Channel(tl::types::InputChannel {
            channel_id: channel_id,
            access_hash: access_hash,
        }),
    };
    let res = caller.invoke(&request).await?;

    let mut ci = types::ChannelInfo::default();
    // println!("++++++ {:#?}",&res);

    use tl::enums::messages::ChatFull;
    match res {
        ChatFull::Full(full) => {
            use tl::enums::ChatFull;
            match full.full_chat {
                ChatFull::ChannelFull(c) => {
                    ci.id = c.id;
                    ci.about = c.about;
                    ci.pts = c.pts;
                    ci.read_inbox_max_id = c.read_inbox_max_id;
                    ci.members_count = c.participants_count.unwrap_or(0);
                    // Extract Avatar form channel photo
                    ci.avatar = converter::conv_photo_to_media(c.chat_photo);
                }
                _ => {}
            }

            // Note if "Discussion Group" is being set chat.len() will be 2
            if full.chats.len() >= 1 {
                // index: 0 channel info - 1 discussion group
                let chat = full.chats.get(0).unwrap();

                use tl::enums::Chat;
                match chat {
                    Chat::Channel(ch) => {
                        // Note: Avatar exist here too and it's a more compact one `ChatPhoto`

                        ci.id = ch.id;
                        ci.title = ch.title.clone();
                        ci.username = ch.username.clone().unwrap_or("".to_string());
                        ci.access_hash = ch.access_hash.unwrap_or(0);
                        ci.date = ch.date;
                        ci.version = ch.version;
                        // ci.members_count = ch.participants_count.unwrap_or(0); // Note: it is None in here! use 'full_chat'
                        ci.restricted = ch.restricted;
                        ci.megagroup = ch.megagroup;
                        ci.full_data = true;
                    }
                    _ => {}
                };
                // println!("channel info {:#?}", ci);
                return Ok(ci);
            }
        }
    }

    Err(TelegramGenErr::TgConverter)
}

pub async fn get_channel_by_username(
    caller: &TgPool,
    username: &str,
) -> Result<types::ChannelInfoCompact, TelegramGenErr> {
    let request = tl::functions::contacts::ResolveUsername {
        username: username.to_string(),
    };
    let res = caller.invoke(&request).await?;
    // println!("+++> get_channel_by_username :  {:#?}", res);

    use tl::enums::contacts::ResolvedPeer;
    match res {
        ResolvedPeer::Peer(peer) => {
            use tl::enums::Chat;
            for chat in peer.chats {
                match chat {
                    Chat::Channel(channel) => {
                        let tg_chan = channel;
                        let res_channel = types::ChannelInfoCompact {
                            id: tg_chan.id,
                            title: tg_chan.title,
                            username: tg_chan.username.unwrap_or("".to_string()),
                            members_count: tg_chan.participants_count.unwrap_or(0),
                            access_hash: tg_chan.access_hash.unwrap_or(0),
                            date: tg_chan.date,
                            photo: 0,
                            version: tg_chan.version,
                            restricted: tg_chan.restricted,
                            megagroup: tg_chan.megagroup,
                        };
                        return (Ok(res_channel));
                        // println!(">>> channel: #{:#?} ", res);
                    }
                    _ => {}
                }
            }
        }
    }
    Err(TelegramGenErr::TgConverter)
}

// Note: this also gets Megagroups messages as well
pub async fn get_messages(
    caller: &TgPool,
    req: ReqGetMessages,
) -> Result<MsgHolder, TelegramGenErr> {
    let request = tl::functions::messages::GetHistory {
        peer: tl::enums::InputPeer::Channel(tl::types::InputPeerChannel {
            channel_id: req.channel_id,
            access_hash: req.access_hash,
        }),
        offset_id: req.offset_id,
        offset_date: req.offset_date,
        add_offset: req.add_offset,
        limit: req.limit, //100
        max_id: req.max_id,
        min_id: req.min_id,
        hash: req.hash,
    };

    // let mt: tl::enums::messages::Messages = send_req(g, &request).await?;
    let mt: tl::enums::messages::Messages = caller.invoke(&request).await?;
    // println!("messages #{:#?}", mt);
    process_channel_msgs(caller, mt).await
}

async fn process_channel_msgs(
    caller: &TgPool,
    mt: tl::enums::messages::Messages,
) -> Result<MsgHolder, TelegramGenErr> {
    let mut msg_holder = MsgHolder {
        msgs: vec![],
        channels: vec![],
        urls: vec![],
        users: vec![],
    };

    // let mut msgs = vec![];
    // let mut urls: Vec<String> = vec![];
    match mt {
        Messages::ChannelMessages(cm) => {
            // println!("messages #{:#?}", cm);
            msg_holder.channels = converter::process_inline_channel_chats(cm.chats.clone());
            converter::process_inline_channel_users(&cm.users);

            let res = converter::process_inline_channel_messages(cm.messages.clone());
            msg_holder.msgs = res.0;
            msg_holder.urls = res.1;
        }
        _ => println!("Not ChannelMessages, other form of messages!"),
    };
    Ok(msg_holder)
    // println!("msgs {:#?} ", msgs);
    // println!("urls {:#?} ", urls);
}
