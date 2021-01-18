use shared::{pb, xc};

use crate::{db, session};

pub fn handle_channel_events(command: pb::channel_command::SubCommand) {
    let my_session = session::get_session();

    let m = xc::ChannelMsg {
        channel_id: 1,
        msg_id: 1,
        pb_data: b"msg 1".to_vec(),
    };

    use pb::channel_command::SubCommand::*;

    match command {
        CreateChannel(p) => {
            // let ch = db::get_channel(p.channel_cid as u64);
            // println!("{}", ch.unwrap().about);
            let ch = pb::Channel {
                cid: p.channel_cid,
                user_name: p.user_name,
                channel_name: p.channel_title,
                creator_profile_cid: p.creator_profile_cid,
                is_verified: false,
                is_profile_channel: false,
                avatar_count: 0,
                about: p.about,
                invite_link_hash: "".to_string(),
                message_seq: 0,
                sync_time_ms: 0,
                created_time: 0,
                is_deleted: 0,
                is_banned: 0,
                notification_setting: None,
                privacy: 0,
                counts: None,
                last_message: None,
                pinned_message: None,
                avatar: None,
            };
            db::save_channel(&ch);
            true
        }
        EditChannel(p) => {
            let mut ch = db::get_channel(p.channel_cid as u64).unwrap();
            if p.set_new_about {
                ch.about = p.new_about;
            }

            db::save_channel(&ch);
            /* let mut c = xc::Channel_Selector::new()
                .channel_id_eq(p.channel_cid.into())
                .get_row(&my_session)
                .unwrap();

            c.save(&my_session);*/
            false
        }
        DeleteChannel(_) => false,
        AddAuthor(_) => false,
        ChangeAuthorPermission(_) => false,
        RemoveAuthor(_) => false,
        FollowChannel(_) => false,
        UnFollowChannel(_) => false,
        RemoveFollowers(_) => false,
        Subscribe(_) => false,
        UnSubscribe(_) => false,
        RemoveSubscribers(_) => false,
        ChangePrivacy(_) => false,
        ChangeDefaultPermission(_) => false,
        RevokeLink(_) => false,
        ChangeUsername(_) => false,
        BlockChannel(_) => false,
        SendMessage(_) => false,
        EditMessage(_) => false,
        PinMessage(_) => false,
        UnPinMessage(_) => false,
        DeleteMessage(_) => false,
        DeleteMessages(_) => false,
        ClearHistory(_) => false,
        AvatarAdd(p) => true,
        AvatarChange(_) => false,
        AvatarDelete(_) => false,
        SendDoingAction(_) => false,
        ReportChannel(_) => false,
        ReportMessage(_) => false,
    };
}
