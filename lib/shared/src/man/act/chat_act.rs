use crate::{
    act,
    my,
    db_helper,
    errors::GenErr,
    pb,
    utils::{rand, time},
};

pub struct ChatAct {
    db: db_helper::DBMySql,
}

#[rustfmt::skip]
impl ChatAct {

    pub async fn delete_chat(&self, profile_id: u32, chat_gid: u64) -> Result<pb::Chat,GenErr> {
        // let d = my::get_chat(profile_cid as u32, chat_gid,).await?;
        // let d = my::get_ch
        let _ = pb::ChatDeleteChatParam::default();
        unimplemented!("");
    }

    pub async fn send_message(&self, p: param::SendMessage) -> Result<pb::Chat,GenErr> {
        let _ = pb::ChatSendMessageParam::default();
        unimplemented!("");
    }

    pub async fn edit_message(&self, p: param::EditMessage) -> Result<pb::Chat,GenErr> {
        let _ = pb::ChatEditMessageParam::default();
        unimplemented!("");
    }

    pub async fn delete_message(&self, p: param::DeleteMessage) -> Result<pb::Chat,GenErr> {
        let _ = pb::ChatDeleteMessagesParam::default();
        unimplemented!("");
    }

    pub async fn delete_history(&self, p: param::DeleteHistory) -> Result<pb::Chat,GenErr> {
        let _ = pb::ChatDeleteHistoryParam::default();
        unimplemented!("");
    }

}

pub mod param {
    use crate::{pb};

    #[derive(Clone, Default, Debug)]
    pub struct SendMessage {
        pub profile_id: u64,
        pub chat_gid: u64,
        pub message_input: pb::NewMessageInput,
    }

    #[derive(Clone, Default, Debug)]
    pub struct EditMessage {
        pub chat_gid: u64,
        pub message_gid: u64,
        pub by_profile_id: u32,
        pub new_text: String,
    }

    #[derive(Clone, Default, Debug)]
    pub struct DeleteMessage {
        pub profile_id: u64,
        pub chat_gid: u64,
        pub message_gids: Vec<u64>,
    }

    #[derive(Clone, Default, Debug)]
    pub struct DeleteHistory {
        pub profile_id: u64,
        pub chat_gid: u64,
    }

}

// #[cfg(test)]
pub mod tests {
    use super::*;

    // #[test]
    pub async fn play1() {

    }
}
