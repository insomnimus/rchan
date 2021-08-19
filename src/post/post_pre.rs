use super::*;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct PostPre {
    pub no: u32,
    resto: u32,
    now: String,
    time: u64,
    #[serde(rename = "name", default)]
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
    #[serde(
        rename = "filedeleted",
        default,
        deserialize_with = "crate::int_to_bool"
    )]
    file_deleted: bool,
    #[serde(rename = "since4pass")]
    since_4pass: Option<i32>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct AttachmentPre {
    #[serde(rename = "tim")]
    id: Option<u64>,
    #[serde(default)]
    filename: String,
    ext: Option<String>,
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
    #[serde(rename = "m_img", deserialize_with = "crate::int_to_bool", default)]
    mobile_optimized: bool,
}

impl TryFrom<AttachmentPre> for Attachment {
    type Error = &'static str;
    fn try_from(pre: AttachmentPre) -> Result<Self, Self::Error> {
        let AttachmentPre {
            filename,
            id,
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

        match (id, ext) {
            (None, _) | (_, None) => Err("the ext and the id field must not be None"),
            (Some(i), Some(x)) => Ok(Self {
                id: i,
                filename,
                ext: x,
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

impl From<PostPre> for Post {
    fn from(pre: PostPre) -> Self {
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
        } = pre;

        let attachment = Attachment::try_from(attachment).ok();

        Self {
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
        }
    }
}
