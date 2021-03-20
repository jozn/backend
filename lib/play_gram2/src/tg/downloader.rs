use async_std::task;
use grammers_client::{Client, Config};
use grammers_mtsender::InvocationError;
use grammers_session as session;
use grammers_tl_types as tl;
use grammers_tl_types::enums::messages::Messages;
use grammers_tl_types::enums::{Message, MessageEntity};
use grammers_tl_types::RemoteCall;
use std::io::Write;

// use crate::types::{MediaOld, MediaThumb, FileMetaInfo};
use crate::{errors::TelegramGenErr, types, types::Caller, utils};
use types::{FileMetaInfo};
use super::connection;
/*
pub async fn dl_thumb_to_disk_old(
    caller: &mut Caller,
    t: &types::MediaThumb,
) -> Result<(), TelegramGenErr> {
    // hack: use Media for dl
    let mut m = types::MediaOld::default();
    m.dep_volume_id = t.dep_volume_id;
    m.dep_local_id = t.dep_local_id;
    m.image_width = t.w;
    m.image_height = t.h;
    m.size = t.size;
    m.media_type = types::MediaTypeOld::Image;
    let res = _dl_image(caller, m.clone()).await?;
    std::fs::create_dir_all("./_dl_thumb/").unwrap();
    let name = format!("./_dl_thumb/{}{}", m.id, m.file_extension);
    let mut f = std::fs::File::create(name).unwrap();
    f.write(&res);
    Ok(())
}
*/

/*
// todo: replace video_thumbs_rec > video_thumbs ??
pub async fn dl_media_thumb_to_disk(
    caller: &mut Caller,
    m: types::MediaOld,
) -> Result<(), TelegramGenErr> {
    // let o = *m.video_thumbs_rec;
    if let Some(t) = *m.video_thumbs_rec {
        // println!("++++ Downloading video thumb {}{}", o. );
        let res = _dl_file(caller, t.clone()).await?;
        std::fs::create_dir_all("./_dl_thumb/").unwrap();
        let name = format!("./_dl_thumb/{}.{}", t.id, t.file_extension);
        let mut f = std::fs::File::create(name).unwrap();
        f.write(&res);
    };
    Ok(())
}
*/


pub async fn dl_media_to_disk(caller: &types::TgPool, m: types::FileMedia) -> Result<(), TelegramGenErr> {

    println!("++++ Downloading media {:?}", &m);

    let vec8 = dl_media(caller, m.clone()).await?;

    std::fs::create_dir_all("./out/telegram_new/photo").unwrap();
    let name = format!("./out/telegram_new/photo/{}{}", m.id, m.file_extension);
    let mut f = std::fs::File::create(name).unwrap();
    f.write(&vec8);
    Ok(())
}


pub async fn dl_media(caller: &types::TgPool, m: types::FileMedia) -> Result<Vec<u8>, TelegramGenErr> {
    let mut  loc = _FileLocation {
        id: m.id,
        access_hash: m.access_hash,
        file_reference: m.file_reference.clone(),
        thumb_size: "".to_string()
    };

    // Set thumb for image types
    match &m.file_meta {
        FileMetaInfo::ImageResizedFile(p) => {
            loc.thumb_size = p.size_type.clone();
        }
        _ => {}
    }

    // use types::MediaTypeOld::*;
    use types::FileMetaInfo::*;
    match m.file_meta {
/*        Image => {
            // _dl_image(caller, m.clone()).await
            // let res = _dl_image(caller,m.clone()).await.unwrap();
            // std::fs::create_dir_all("./_dl/").unwrap();
            // let name = format!("./_dl/{}{}", m.id,m.file_extention);
            // let mut f = std::fs::File::create(name).unwrap();
            // f.write(&res);
        }
        Video | Audio | File | ImageFile => {
            // _dl_file(caller, m.clone()).await
            // let res = _dl_file(caller,m.clone()).await.unwrap();
            // std::fs::create_dir_all("./_dl/").unwrap();
            // let name = format!("./_dl/{}{}", m.id,m.file_extention);
            // let mut f = std::fs::File::create(name).unwrap();
            // f.write(&res);
        }
        Unknown => Err(TelegramGenErr::Download),*/
        Unknown => {
            Err(TelegramGenErr::BadParam)
        }
        ImageResizedFile(_) => {
            _dl_tg_shared(caller,_TgFileLocation::Image, loc).await
        }
        ImageFile(_) | VideoFile(_) | AudioFile(_) | DocumentFile(_) | GifFile(_) => {
            _dl_tg_shared(caller,_TgFileLocation::Document, loc).await
        }
        // VideoFile(_) => {}
        // AudioFile(_) => {}
        // DocumentFile(_) => {}
        // GifFile(_) => {}
    }
}

#[derive(Debug)]
struct _FileLocation {
    id: i64,
    access_hash: i64,
    file_reference: Vec<u8>,
    thumb_size: String,
}

#[derive(PartialEq, Debug)]
enum _TgFileLocation {
    Image,
    Document,
}

// This is shared methods for downloading both Image (typical resized size[incding Avatar], and Document),
// as telegram api needs to different construction for calling Image and Document this function
// builds appropriate request. The reason those two api is shared is to handle buffering and other
// things in just one place (DRY).
async fn _dl_tg_shared(caller: &types::TgPool,file_type: _TgFileLocation, m: _FileLocation) -> Result<Vec<u8>, TelegramGenErr> {
    let limit = 524288;
    let mut out_buffer = Vec::with_capacity(limit as usize);
    let mut offset = 0;

    loop {
        let mut request = tl::functions::upload::GetFile {
            precise: false,
            cdn_supported: false,
            location: tl::enums::InputFileLocation::InputDocumentFileLocation( // default: Document
                tl::types::InputDocumentFileLocation {
                    id: m.id,
                    access_hash: m.access_hash,
                    file_reference: m.file_reference.clone(),
                    thumb_size: "".to_string(), // For doc files there is no resized images
                },
            ),
            offset: offset,
            limit: limit,
        };

        // if it's Image set proper .location property
        if file_type == _TgFileLocation::Image {
            request.location = tl::enums::InputFileLocation::InputPhotoFileLocation(
                tl::types::InputPhotoFileLocation {
                    id: m.id,
                    access_hash: m.access_hash,
                    file_reference: m.file_reference.clone(),
                    thumb_size: m.thumb_size.clone(),
                },
            )
        }


        let res = caller.invoke(&request).await;

        match res {
            Ok(res) => {
                use tl::enums::upload::File;
                match res {
                    File::File(tfile) => {
                        let len = tfile.bytes.len() as i32;
                        out_buffer.write(&tfile.bytes);
                        if len == limit {
                            offset = offset + limit;
                        } else {
                            break;
                        }
                    }
                    File::CdnRedirect(red) => {
                        // todo
                        break;
                    }
                };
            }
            Err(err) => {
                // todo dl is incomplete
                break;
            }
        }
    }

    if out_buffer.len() == 0 {
        return Err(TelegramGenErr::Download);
    }

    Ok(out_buffer)
}

async fn _dl_file(caller: &types::TgPool, m: types::FileMedia) -> Result<Vec<u8>, TelegramGenErr> {
    let limit = 524288;
    let mut out_buffer = Vec::with_capacity(limit as usize);
    let mut offset = 0;

    loop {
        let request = tl::functions::upload::GetFile {
            precise: false, // todo
            cdn_supported: false,
            location: tl::enums::InputFileLocation::InputDocumentFileLocation(
                tl::types::InputDocumentFileLocation {
                    id: m.id,
                    access_hash: m.access_hash,
                    file_reference: m.file_reference.clone(),
                    thumb_size: "".to_string(), // For doc files there is no resized images
                },
            ),
            offset: offset,
            limit: limit,
        };
        let res = caller.invoke(&request).await;

        match res {
            Ok(res) => {
                use tl::enums::upload::File;
                match res {
                    File::File(tfile) => {
                        let len = tfile.bytes.len() as i32;
                        out_buffer.write(&tfile.bytes);
                        if len == limit {
                            offset = offset + limit;
                        } else {
                            break;
                        }
                    }
                    File::CdnRedirect(red) => {
                        // todo
                        break;
                    }
                };
            }
            Err(err) => {
                // todo
                break;
            }
        }
    }

    if out_buffer.len() == 0 {
        return Err(TelegramGenErr::Download);
    }

    Ok(out_buffer)
}

async fn _dl_image_old(caller: &types::TgPool, m: types::FileMedia) -> Result<Vec<u8>, TelegramGenErr> {
    let mut photo_size_type = "m".to_string();
    match m.file_meta {
        FileMetaInfo::ImageResizedFile(p) => {
            photo_size_type = p.size_type;
        }
        _ => {
            return Err(TelegramGenErr::BadParam)
        }
    }

    let request = tl::functions::upload::GetFile {
        precise: false,
        cdn_supported: false,
        location: tl::enums::InputFileLocation::InputPhotoFileLocation(
            tl::types::InputPhotoFileLocation {
                id: m.id,
                access_hash: m.access_hash,
                file_reference: m.file_reference,
                thumb_size: photo_size_type,
            },
        ),
        offset: 0,
        limit: 524288,
    };
    // let res = send_req(caller, &request).await?;
    let res = caller.invoke(&request).await?;

    let mut out = vec![];
    use tl::enums::upload::File;
    match res {
        File::File(tfile) => {
            // f.write(&tfile.bytes);
            out.write(&tfile.bytes);
        }
        File::CdnRedirect(red) => {
            println!("cdn redirect");
        }
    };
    Ok(out)
}

/*async fn send_req<R: RemoteCall>(
    caller: &mut Caller,
    request: &R,
) -> Result<R::Return, InvocationError> {
    caller.client.invoke(request).await
}*/

///////////// Old Codes ///////////
/*
async fn get_file(caller: &mut Caller, req: tl::types::InputFileLocation) {
    let request = tl::functions::upload::GetFile {
        precise: false,
        cdn_supported: false,
        location: tl::enums::InputFileLocation::Location(req),
        offset: 0,
        limit: 524288,
    };
    let res = send_req(caller, &request).await.unwrap();
    // println!("get_chat_id:  {:#?}", res);
}

async fn get_file_photo(caller: &mut Caller, req: tl::types::InputPhotoFileLocation) {
    // println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  {:#?}", req);
    let request = tl::functions::upload::GetFile {
        precise: false,
        cdn_supported: false,
        location: tl::enums::InputFileLocation::InputPhotoFileLocation(req.clone()),
        offset: 0,
        limit: 524288,
    };
    let res = send_req(caller, &request).await.unwrap();

    std::fs::create_dir_all("./out/").unwrap();
    let name = format!("./out/{}.jpg", req.id);
    let mut f = std::fs::File::create(name).unwrap();

    use tl::enums::upload::File;

    match res {
        File::File(tfile) => {
            f.write(&tfile.bytes);
        }
        File::CdnRedirect(red) => {}
    };
}

async fn get_file_doc(caller: &mut Caller, req: tl::types::InputDocumentFileLocation) {
    let limit = 524288;
    let mut out_buffer = Vec::with_capacity(limit as usize);
    let mut offset = 0;

    loop {
        let request = tl::functions::upload::GetFile {
            precise: false,
            cdn_supported: false,
            location: tl::enums::InputFileLocation::InputDocumentFileLocation(req.clone()),
            offset: offset,
            limit: limit,
        };
        let res = send_req(caller, &request).await;

        match res {
            Ok(res) => {
                use tl::enums::upload::File;
                match res {
                    File::File(tfile) => {
                        let len = tfile.bytes.len() as i32;
                        out_buffer.write(&tfile.bytes);
                        if len == limit {
                            offset = offset + limit;
                        } else {
                            break;
                        }
                    }
                    File::CdnRedirect(red) => {
                        break;
                    }
                };
            }
            Err(err) => {
                break;
            }
        }
        //println!("%%%%%% get_file_photo :  {:#?}", res);
    }

    if out_buffer.len() == 0 {
        return;
    }

    std::fs::create_dir_all("./out/").unwrap();
    let name = format!("./out/{}.file", req.id);
    let mut f = std::fs::File::create(name).unwrap();
    f.write(&out_buffer);
}
*/



/*async fn _dl_image(caller: &types::TgPool, m: types::FileMedia) -> Result<Vec<u8>, TelegramGenErr> {
    let mut photo_size_type = "m".to_string();
    let limit = 524288;
    let mut out_buffer = Vec::with_capacity(limit as usize);
    let mut offset = 0;

    match m.file_meta {
        FileMetaInfo::ImageResizedFile(p) => {
            photo_size_type = p.size_type;
        }
        _ => {
            return Err(TelegramGenErr::BadParam)
        }
    }

    loop {
        let request = tl::functions::upload::GetFile {
            precise: false,
            cdn_supported: false,
            location: tl::enums::InputFileLocation::InputDocumentFileLocation(
                tl::types::InputDocumentFileLocation {
                    id: m.id,
                    access_hash: m.access_hash,
                    file_reference: m.file_reference.clone(),
                    thumb_size: "".to_string(), // For doc files there is no resized images
                },
            ),
            offset: offset,
            limit: limit,
        };
        let res = caller.invoke(&request).await;

        match res {
            Ok(res) => {
                use tl::enums::upload::File;
                match res {
                    File::File(tfile) => {
                        let len = tfile.bytes.len() as i32;
                        out_buffer.write(&tfile.bytes);
                        if len == limit {
                            offset = offset + limit;
                        } else {
                            break;
                        }
                    }
                    File::CdnRedirect(red) => {
                        // todo
                        break;
                    }
                };
            }
            Err(err) => {
                // todo
                break;
            }
        }
    }

    if out_buffer.len() == 0 {
        return Err(TelegramGenErr::Download);
    }

    Ok(out_buffer)
}*/


