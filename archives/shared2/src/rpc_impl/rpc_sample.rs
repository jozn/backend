use crate::{com, com::*, mock, pb, utils};
use image;
use image::imageops::crop_imm;
use image::GenericImageView;
use once_cell::sync::OnceCell;
use rand::{Rng, RngCore};
use std::borrow::Borrow;
use std::cell::Cell;
use std::collections::HashMap;
use std::ops::Add;
use std::path::PathBuf;

static FACT: OnceCell<MemDb> = OnceCell::new();

fn _get_fact() -> &'static MemDb {
    let vec = FACT.get_or_init(|| {
        let mut res = MemDb::default();
        res.build();
        res
    });
    vec
}

pub async fn GetUsers1(
    up: &UserParam,
    param: pb::GetUsers1Param,
) -> Result<pb::GetUsers1Response, GenErr> {
    let db = _get_fact();
    Ok(pb::GetUsers1Response::default())
}
pub async fn GetProfiles(
    up: &UserParam,
    param: pb::GetProfilesParam,
) -> Result<pb::GetProfilesResponse, GenErr> {
    let db = _get_fact();
    let mut pros = vec![];

    for p in &db.users {
        pros.push(p.def_profile.as_ref().unwrap().clone());
    }

    let res = pb::GetProfilesResponse { profiles: pros };
    println!("in proile {:}", 1);
    Ok(res)
}
pub async fn GetChannels(
    up: &UserParam,
    param: pb::GetChannelsParam,
) -> Result<pb::GetChannelsResponse, GenErr> {
    let db = _get_fact();
    let mut vec = vec![];

    for p in &db.users {
        let pro = p.def_profile.as_ref().unwrap();
        vec.push(
            p.def_profile
                .as_ref()
                .unwrap()
                .primary_channel
                .as_ref()
                .unwrap()
                .clone(),
        );
        let chs = pro.channels.clone();
        for c in chs {
            // vec.push(c);
        }
        // vec.push_all(p.def_profile.unwrap().channels.clone());
    }

    Ok(pb::GetChannelsResponse { channels: vec })
}
pub async fn GetDirects(
    up: &UserParam,
    param: pb::GetDirectsParam,
) -> Result<pb::GetDirectsResponse, GenErr> {
    let db = _get_fact();
    let mut vec = db.direct_groups.clone();
    println!("sending GetDirect len = {}", vec.len());
    Ok(pb::GetDirectsResponse { directs: vec })
    // Ok(pb::GetDirectsResponse::default())
}
pub async fn GetMessages(
    up: &UserParam,
    param: pb::GetMessagesParam,
) -> Result<pb::GetMessagesResponse, GenErr> {
    let db = _get_fact();
    let mut msgs = vec![];

    for i in 1..10 {
        let ms = (db.messages.get(&i));
        match ms {
            Some(ms) => {
                ms.iter().for_each(|m| msgs.push(m.clone()));
            }
            None => {}
        }
    }
    Ok(pb::GetMessagesResponse { directs: msgs })
}
const MAX_USER: u32 = 50;
#[derive(Debug)]
pub struct MemDb {
    users: Vec<pb::User>,
    messages: HashMap<u32, Vec<pb::Message>>,
    contacts: HashMap<u32, Vec<pb::Contact>>,
    groups: Vec<pb::Group>,
    direct_chats: Vec<pb::Direct>,
    direct_channels: Vec<pb::Direct>,
    direct_groups: Vec<pb::Direct>,
    gid_gen: utils::id_gen::SeqTimeIdGen,
    //  cid3: Cell<u32>,
    cid: u32,
}

impl Default for MemDb {
    fn default() -> Self {
        MemDb {
            users: vec![],
            gid_gen: Default::default(),
            cid: 1,
            messages: HashMap::new(),
            contacts: HashMap::new(),
            groups: vec![],
            direct_chats: vec![],
            direct_channels: vec![],
            direct_groups: vec![],
            // ..Default::default() > will stackoverflow
        }
    }
}

impl MemDb {
    pub fn build(&mut self) {
        self.gen_users();
        self.gen_extra_channels();
        self.gen_messages(); // very slow for images
        self.gen_contacts();
        self.gen_groups();
        self.gen_directs();
    }
    pub fn get_next_cid(&mut self) -> u32 {
        let res = self.cid;
        self.cid += 1;
        res
    }

    fn gen_users(&mut self) {
        for u in 0..MAX_USER {
            let user_cid = self.get_next_cid();
            let pro_cid = self.get_next_cid();
            let chanel_cid = self.get_next_cid();

            let ch = pb::Channel {
                cid: chanel_cid,
                user_name: format!("profile{}", u),
                channel_name: format!("channel def u#{}", u),
                creator_profile_cid: pro_cid,
                is_profile_channel: true,
                about: format!("my_dep about user def ch {}", u),
                avatar: Some(_get_sample_avatar()),
                last_message: Some(_make_last_msg(5)),
                ..Default::default()
            };

            let pro = pb::Profile {
                cid: pro_cid,
                user_cid: user_cid,
                primary_channel: Some(ch),
                saved_channel: None,
                created_time: 1,
                setting: None,
                channels: vec![],
                directs: vec![],
                groups: vec![],
                // contacts: //make_contacts(pro_cid, &self),
                ..Default::default()
            };

            let user = pb::User {
                cid: user_cid,
                phone: format!("+9890151323{}", u),
                email: format!("exampl{}@gmail.com", u),
                password_hash: "".to_string(),
                password_salt: "".to_string(),
                created_time: 1,
                version_time: 2,
                is_deleted: false,
                is_banned: false,
                def_profile: Some(pro),
                profiles: vec![],
                stores: vec![],
                sessions: vec![],
                shopping_profile: None,
                first_name: format!("first"),
                last_name: format!("last #{}", u),
                user_counts: None,
            };

            self.users.push(user);
        }
    }

    fn gen_extra_channels(&mut self) {
        let mut id = 0;
        for u in self.users.clone() {
            let user_cid = self.get_next_cid();
            let pro_cid = self.get_next_cid();
            let chanel_cid = self.get_next_cid();

            let mut profile = u.def_profile.borrow().as_ref().unwrap();
            let ch = pb::Channel {
                cid: chanel_cid,
                user_name: format!("Chan#{}", chanel_cid),
                channel_name: format!("channel #{} u{}", chanel_cid, u.cid),
                creator_profile_cid: profile.cid,
                is_profile_channel: false,
                about: format!("my_dep about extra chan {}", u.first_name),
                avatar: Some(_get_sample_avatar()),
                last_message: Some(_make_last_msg(5)),
                ..Default::default()
            };

            self.users
                .get_mut(id)
                .unwrap()
                .def_profile
                .as_mut()
                .unwrap()
                .channels
                .push(ch);
            id += 1;
            // profile.channels.push(ch);
        }
    }

    fn gen_messages(&mut self) {
        for u in &self.users.clone() {
            // profile msgs
            let profile = u.def_profile.as_ref().unwrap();
            let def_ch = (profile.primary_channel.as_ref()).unwrap();
            self.messages
                .insert(def_ch.cid, make_channel_msgs(def_ch.clone(), profile.cid));

            // channels msgs
            for ch in &profile.channels {
                self.messages
                    .insert(ch.cid, make_channel_msgs(ch.clone(), profile.cid));
            }
        }
    }

    fn gen_contacts(&mut self) {
        for u in &self.users.clone() {
            let profile = u.def_profile.as_ref().unwrap();
            self.contacts
                .insert(profile.cid, make_contacts(profile.cid, &self));
        }
    }

    fn gen_groups(&mut self) {
        let u = &self.users.get(0).unwrap();
        for i in 1..5 {
            let cid = self.get_next_cid();
            let g = pb::Group {
                cid: cid,
                group_title: format!("Group #{}", i),
                user_name: "".to_string(),
                creator_profile_cid: 0,
                history_viewable: false,
                is_open_group: false,
                seq: 0,
                avatar_count: 0,
                about: format!("About Group #{}", i),
                invite_link_hash: "abcd".to_string(),
                members_count: 8,
                admins_count: 2,
                moderator_counts: 1,
                sort_time: i,
                sync_time: i,
                created_time: i as u32,
                is_deleted: false,
                is_banned: false,
                last_message: Some(_make_last_msg(5)),
                pinned_message: None,
                avatar: Some(_get_sample_avatar()),
            };
            self.groups.push(g);
        }
    }

    fn gen_directs(&mut self) {
        let u = &self.users.get(0).unwrap();
        // for i in 1..5 {
        let mut i = 1;
        for group in &self.groups {
            i += 1;
            let gc = group.clone();
            let d = pb::Direct {
                gid: 6,
                profile_cid: 4,
                direct_type: pb::DirectTypeEnum::Group as i32,
                custom_title: format!("direct groups {}", i),
                pin_time_ms: 0,
                unseen_count: i * 4,
                seq: 6,
                is_active: true,
                created_time: i,
                sort_time_ms: i as u64,
                sync_time_ms: i as u64,
                my_last_seen_seq: 0,
                my_last_seen_msg_id: 0,
                pined_msgs_count: 0,
                visible_from_msg_gid: 0,
                channel: None,
                contact: None,
                group: Some(gc),
                last_message: None,
                pinned_message: None,
                group_member: None,
                draft: None,
                custom_notification: None,
                ..Default::default()
            };
            self.direct_groups.push(d);
        }
    }
}

fn make_channel_msgs(ch: pb::Channel, pid: u32) -> Vec<pb::Message> {
    let mut v = vec![];
    let mut genid = utils::id_gen::SeqTimeIdGen::new(15);
    for i in 1..20 {
        let m = _make_last_msg(pid);
        /*        let mut m = pb::Message {
            gid: genid.get_next_id().to_u64(),
            by_profile_cid: pid,
            message_type: pb::MessageType::Text as i32,
            text: _get_sample_text(80), //format!("some random text {}", i),
            via_app_id: 1,
            seq: i,
            edited_time: 0,
            created_time: 123,
            verified: false,
            delivery_status: pb::MessageDeliveryStatues::Seen as i32,
            delivery_time: 345,
            deleted: false,
            flags: 0,
            forward: None,
            reply_to: None,
            title: "".to_string(),
            counts: None,
            setting: None,
            product: None,
            files: vec![],
        };
        if i % 3 == 0 {
            m.message_type = pb::MessageType::Image as i32;
            m.files = vec![_get_sample_image()];
        }*/
        v.push(m);
    }
    v
}

fn make_contacts(pid: u32, db: &MemDb) -> Vec<pb::Contact> {
    let mut v = vec![];
    let mut genid = utils::id_gen::SeqTimeIdGen::new(15);
    for u in &db.users {
        let peer_cid = u.def_profile.as_ref().unwrap().cid;
        if peer_cid == pid {
            continue;
        }
        let m = pb::Contact {
            gid: genid.get_next_id().to_u64(),
            profile_cid: pid,
            device_id: 0,
            phone: "+989015132328".to_string(),
            first_name: "dev name".to_string(),
            last_name: "last".to_string(),
            peer_profile_cid: peer_cid,
            created_time: 123412341,
        };

        v.push(m);
    }
    v
}

fn make_sample_file() -> pb::FileMsg {
    let img = _get_sample_image__slow_dep();
    pb::FileMsg {
        gid: 234,
        width: img.1,
        height: img.2,
        full_path: img.0,
        ..Default::default()
    }
}

fn _make_last_msg(pid: u32) -> pb::Message {
    let mut genid = utils::id_gen::SeqTimeIdGen::new(15);
    let mut m = pb::Message {
        gid: genid.get_next_id().to_u64(),
        by_profile_cid: pid,
        message_type: pb::MessageType::Text as i32,
        text: _get_sample_text(90), //format!("some random text for last msgs"),
        via_app_id: 1,
        seq: 5,
        edited_time: 0,
        created_time: 123,
        verified: false,
        delivery_status: pb::MessageDeliveryStatues::Seen as i32,
        delivery_time: 345,
        deleted: false,
        flags: 0,
        forward: None,
        reply_to: None,
        title: "".to_string(),
        counts: None,
        setting: None,
        product: None,
        files: vec![],
    };
    if rand::thread_rng().gen_range(0, 10) < 8 {
        m.message_type = pb::MessageType::Image as i32;
        m.files = vec![_get_sample_image()];
    }
    m
}

static IMAGE_FILES: OnceCell<Vec<std::path::PathBuf>> = OnceCell::new();

fn _get_sample_image__slow_dep() -> (String, u32, u32) {
    let mut rng = rand::thread_rng();

    let vec = IMAGE_FILES.get_or_init(|| {
        let imgs = std::fs::read_dir("/home/hamid/life/__files__/Telegram/images").unwrap();
        println!("init vec of image files");
        let mut vec = vec![];
        for ig in imgs {
            let ig = ig.unwrap();
            vec.push(ig.path());
        }
        vec
    });

    let id = rng.gen_range(0, vec.len());

    let img_path = vec.get(id).unwrap();
    let dim = image::open(img_path).unwrap().dimensions();
    (
        img_path.clone().into_os_string().into_string().unwrap(),
        dim.0,
        dim.1,
    )
}

fn _get_sample_avatar() -> pb::FileMsg {
    let mut rnd = rand::thread_rng();
    let avatars = mock::avatars::get_images();
    let img = avatars.get(rnd.gen_range(0, avatars.len())).unwrap();
    pb::FileMsg {
        gid: rnd.next_u64(),
        file_type: 0,
        width: img.width,
        height: img.height,
        extension: ".jpg".to_string(),
        full_path: img.src.to_string(),
        user_cid: 1,
        ..Default::default()
    }
}

fn _get_sample_image() -> pb::FileMsg {
    let mut rnd = rand::thread_rng();
    let imgs = mock::images::get_images();
    let img = imgs.get(rnd.gen_range(0, imgs.len())).unwrap();
    pb::FileMsg {
        gid: rnd.next_u64(),
        file_type: 0,
        width: img.width,
        height: img.height,
        extension: ".jpg".to_string(),
        full_path: img.src.to_string(),
        user_cid: 1,
        ..Default::default()
    }
}

fn _get_sample_text(size: usize) -> String {
    let mut str = String::new();
    let arr: Vec<&str> = FARSI_TEXT.split(" ").collect();
    let len = arr.len();
    let mut rnd = rand::thread_rng();

    for i in 0..size {
        let index = rnd.gen_range(0, len);
        //out_att.push(arr.get(index).unwrap());
        let s = format!("{} ", arr.get(index).unwrap());
        str.push_str(&s);
    }

    str
}

const FARSI_TEXT: &str = r###"🔸قیمت خودروهای نامتعارف براساس بازار آزاد!/ بیمه ها چقدر خسارت تصادف را می پردازند؟

🔹همزمان با اعلام تغییر سقف پرداخت خسارت خودرو‌های نامتعارف به ۲۲۰ میلیون،۲۰۶ وارد طبقه خودرو‌های نامتعارف شد. از این رو رسانه‌های مختلف این موضوع را دست گرفته و برچسب " خودروی لاکچری " به ۲۰۶ چسباندند!

🔹قانون مصوب در بیمه شخص ثالث با تحولات اقتصادی و افزایش قیمت‌های بازار خودرو همخوانی ندارد؛ از این رو یک ماشین یک میلیاردی به اندازه یک خودروی ۲۲۰ میلیونی خسارت دریافت می‌کند.

🔹اگر شما یک خودروی متعارف که قیمت آن حداکثر ٢٢٠ میلیون است، داشته و  در یک حادثه رانندگی دچار خسارت مالی شوید، میزان خسارت به هر اندازه ای که باشد باید توسط مقصر حادثه جبران شود.

🔹بر این اساس اگر خودروی زیان دیده مثلا ۵۰۰ میلیون تومان ارزش داشته باشد، خودروی شما لوکس محسوب شده، مقصر حادثه فقط تا حد ۲۲۰ میلیون تومان مسئول بوده و پرداخت خسارت بیش از این مبلغ بر عهده مقصر و به تبع آن بر عهده بیمه گر نیز نخواهد بود.  داغدیدگان کرونا را چگونه به آرامش برسانیم؟

🔸بهناز عطاری، روانشناس و مشاور خانواده در گفت‌وگو با باشگاه خبرنگاران جوان:

🔹گفتن عباراتی نسنجیده و بدون درک متقابل تنها وضعیت بهم ریختگی روانی و آشوب احساسی فرد داغدیده را تشدید می‌کند و چنین رفتاری چه بسا تحمل داغ را بر فرد مذکور سخت‌تر خواهد کرد.

🔹در هنگام مواجهه با فردی که با مرگ عزیزان خود روبرو شده است باید بتوانیم با قرار دادن خود در جای چنین فردی ضمن درک حال او، آن فرد را در بیان احساسات و حالاتی که در این دوران دارد یاری کنیم.

🔹در حال حاضر این امکان وجود ندارد که به صورت مستقیم به فرد داغدیده سر بزنیم و او را دلداری بدهیم، اما با برقراری تماس با او و رفع نیاز‌های عادی روزانه او می‌توان دلگرمی بزرگی به چنین فردی بخشید.

🔹برای تخلیه روانی کسی که یکی از عزیزان خود را به دلیل ابتلا به بیماری کرونا از دست داده است، باید به او نشان دهیم که حال نامساعد روحی و روانی‌اش را درک می‌کنیم و با او همدل و همراه هستیم."
"###;
