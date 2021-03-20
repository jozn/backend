use crate::{errors::TelegramGenErr, types, utils};
use grammers_tl_types as tl;
use std::io::Write;

pub async fn dl_media_to_disk(
    caller: &types::TgPool,
    m: types::FileMedia,
) -> Result<(), TelegramGenErr> {
    println!("++++ Downloading media {:?}", &m);

    let vec8 = dl_media(caller, m.clone()).await?;

    std::fs::create_dir_all("./out/telegram_new/photo").unwrap();
    let name = format!("./out/telegram_new/photo/{}{}", m.id, m.file_extension);
    let mut f = std::fs::File::create(name).unwrap();
    f.write(&vec8);
    Ok(())
}

pub async fn dl_media(
    caller: &types::TgPool,
    m: types::FileMedia,
) -> Result<Vec<u8>, TelegramGenErr> {
    let mut loc = _FileLocation {
        id: m.id,
        access_hash: m.access_hash,
        file_reference: m.file_reference.clone(),
        thumb_size: "".to_string(),
    };

    // Set thumb for image types
    match &m.file_meta {
        types::FileMetaInfo::ImageResizedFile(p) => {
            loc.thumb_size = p.size_type.clone();
        }
        _ => {}
    }

    use types::FileMetaInfo::*;
    match m.file_meta {
        Unknown => Err(TelegramGenErr::BadParam),
        ImageResizedFile(_) => _dl_tg_shared(caller, _TgFileLocation::Image, loc).await,
        ImageFile(_) | VideoFile(_) | AudioFile(_) | DocumentFile(_) | GifFile(_) => {
            _dl_tg_shared(caller, _TgFileLocation::Document, loc).await
        }
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
async fn _dl_tg_shared(
    caller: &types::TgPool,
    file_type: _TgFileLocation,
    m: _FileLocation,
) -> Result<Vec<u8>, TelegramGenErr> {
    let limit = 524288;
    let mut out_buffer = Vec::with_capacity(limit as usize);
    let mut offset = 0;

    loop {
        let mut request = tl::functions::upload::GetFile {
            precise: false,
            cdn_supported: false,
            location: tl::enums::InputFileLocation::InputDocumentFileLocation(
                // default: Document
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
