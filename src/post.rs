pub(crate) mod capcode;
pub(crate) mod post_pre;

use core::convert::TryFrom;

use serde::de::Deserializer;
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

/// `Attachment` holds the metadata for a post attachment.
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
    /// The number of the post.
    pub no: u32,
    /// The reply-to number for this post. `0` for the OP.
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
    /// Attachment metadata for this post, if any.
    pub attachment: Option<Attachment>,
    /// `true` if the post had an attachment but was deleted.
    pub file_deleted: bool,

    /// The year 4chan pass was bought.
    pub since_4pass: Option<i32>,
}

impl<'de> serde::Deserialize<'de> for Post {
    fn deserialize<D: Deserializer<'de>>(des: D) -> Result<Self, D::Error> {
        let pre = post_pre::PostPre::deserialize(des)?;
        Ok(pre.into())
    }
}

impl Post {
    /// Returns a URL where the media of this attachment
    /// can be retreived from.
    ///
    /// # Arguments
    /// -  `board`: The abbreviation of the board name this post was posted in. E.g. `"mu"`.
    ///
    /// # Notes
    /// There is no clean way of storing the board name in a [`Post`]
    /// therefore this is currently an argument.
    ///
    /// Calling this method with an invalid board name results in an invalid URL, not `None`.
    pub fn attachment_url(&self, board: &str) -> Option<String> {
        self.attachment.as_ref().map(|a| {
            format!(
                "https://i.4cdn.org/{board}/{post_no}.{ext}",
                board = board,
                ext = &a.ext,
                post_no = &self.no
            )
        })
    }
}
