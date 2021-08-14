use core::convert::TryFrom;
use std::fmt;

use serde::de::{self, Deserializer, Visitor};
use serde_derive::Deserialize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Capcode {
    Mod,
    Admin,
    AdminHighlight,
    Manager,
    Developer,
    Founder,
}

struct CapcodeVisitor;

impl<'de> Visitor<'de> for CapcodeVisitor {
    type Value = Capcode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "one of [mod, admin, admin_highlight, manager, developer,, founder]"
        )
    }

    fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
        Ok(match s {
            "mod" => Capcode::Mod,
            "admin" => Capcode::Admin,
            "admin_highlight" => Capcode::AdminHighlight,
            "manager" => Capcode::Manager,
            "developer" => Capcode::Developer,
            "founder" => Capcode::Founder,
            _ => return Err(E::custom(format!("{} is not a valid capcode", s))),
        })
    }
}

impl<'de> serde::Deserialize<'de> for Capcode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CapcodeVisitor)
    }
}

#[derive(Debug, Clone)]
pub struct Attachment {
    /// A UNIX timestamp + micro time of when the image was uploaded.
    pub uploaded: u64,
    /// The name of the attachment file.
    pub filename: String,
    /// The extension of the attachment.
    pub ext: String,
    /// The size of the uploaded file in bytes.
    pub size: u64,
    /// 24 character, packed base64 MD5 hash of file.
    pub md5: String,
    /// The width of the media attachment.
    pub width: i32,
    /// The height of the media attachment.
    pub height: i32,
    /// The width of the thumbnail.
    pub thumbnail_width: i32,
    /// The height of the thumbnail.
    pub thumbnail_height: i32,
    /// If the attachment is spoilered.
    pub spoiler: bool,
    /// `true` if the attachment is mobile optimized.
    pub mobile_optimized: bool,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub no: u32,
    pub resto: u32,
    pub now: String,
    pub time: u64,
    pub author: String,
    pub trip: Option<String>,
    pub author_id: Option<String>,
    pub capcode: Option<Capcode>,
    pub country: Option<String>,
    pub country_name: Option<String>,
    pub board_flag: Option<String>,
    pub flag_name: Option<String>,

    /// The body of the post, if any. The comment is HTML escaped.
    pub comment: Option<String>,
    pub attachment: Option<Attachment>,
    /// `true` if the post had an attachment but was deleted.
    pub file_deleted: bool,

    /// The year 4chan pass was bought.
    pub since_4pass: i32,
}

#[derive(Debug, Deserialize, Clone)]
struct PostPre {
    no: u32,
    resto: u32,
    now: String,
    time: u64,
    #[serde(rename = "name")]
    author: String,
    trip: Option<String>,
    #[serde(rename = "id")]
    author_id: Option<String>,
    capcode: Option<Capcode>,
    country: Option<String>,
    country_name: Option<String>,
    board_flag: Option<String>,
    flag_name: Option<String>,
    #[serde(rename = "com")]
    comment: Option<String>,
    #[serde(flatten)]
    attachment: AttachmentPre,
    #[serde(rename = "filedeleted")]
    file_deleted: bool,
    #[serde(rename = "since4pass")]
    since_4pass: i32,
}

#[derive(Debug, Deserialize, Clone)]
struct AttachmentPre {
    #[serde(default, rename = "tim")]
    uploaded: u64,
    filename: Option<String>,
    #[serde(default)]
    ext: String,
    #[serde(default, rename = "fsize")]
    size: u64,
    #[serde(default)]
    md5: String,
    #[serde(default, rename = "w")]
    width: i32,
    #[serde(default, rename = "h")]
    height: i32,
    #[serde(default, rename = "tn_w")]
    thumbnail_width: i32,
    #[serde(default, rename = "tn_h")]
    thumbnail_height: i32,
    #[serde(default, deserialize_with = "crate::int_to_bool")]
    spoiler: bool,
    #[serde(rename = "m_img", deserialize_with = "crate::int_to_bool")]
    mobile_optimized: bool,
}

impl<'de> serde::Deserialize<'de> for Post {
    fn deserialize<D: Deserializer<'de>>(des: D) -> Result<Self, D::Error> {
        let PostPre {
            no,
            resto,
            now,
            time,
            author,
            trip,
            author_id,
            capcode,
            country,
            country_name,
            board_flag,
            flag_name,
            comment,
            attachment,
            file_deleted,
            since_4pass,
        } = PostPre::deserialize(des)?;

        let attachment = Attachment::try_from(attachment).ok();

        Ok(Self {
            no,
            resto,
            now,
            time,
            author,
            trip,
            author_id,
            capcode,
            country,
            country_name,
            board_flag,
            flag_name,
            comment,
            attachment,
            file_deleted,
            since_4pass,
        })
    }
}

impl TryFrom<AttachmentPre> for Attachment {
    type Error = &'static str;
    fn try_from(pre: AttachmentPre) -> Result<Self, Self::Error> {
        let AttachmentPre {
            filename,
            uploaded,
            ext,
            size,
            md5,
            width,
            height,
            thumbnail_width,
            thumbnail_height,
            spoiler,
            mobile_optimized,
        } = pre;

        match filename {
            None => Err("the attachment has no filename"),
            Some(f) => Ok(Self {
                uploaded,
                filename: f,
                ext,
                size,
                md5,
                width,
                height,
                thumbnail_width,
                thumbnail_height,
                spoiler,
                mobile_optimized,
            }),
        }
    }
}
