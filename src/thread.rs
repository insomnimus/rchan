use serde::de::Deserializer;
use serde_derive::Deserialize;

use crate::post::{post_pre::PostPre, Post};

/// Represents a thread as seen from the catalog.
///
/// To get all the posts in a thread, call `[Client::get_full_thread]`.
#[derive(Clone, Debug, Deserialize)]
pub struct ThreadInfo {
    /// The [`Post`] of the OP.
    #[serde(flatten)]
    pub op_post: Post,
    /// `true` if the thread is pinned on top of the page.
    #[serde(default, deserialize_with = "crate::int_to_bool")]
    pub sticky: bool,
    /// `true` if the thread is closed.
    #[serde(default, deserialize_with = "crate::int_to_bool")]
    pub closed: bool,
    /// The threads title, if any.
    #[serde(rename = "sub")]
    pub subject: Option<String>,
    /// The number of omitted replies.
    #[serde(rename = "omitted_posts", default)]
    pub posts_omitted: i32,
    /// The number of omitted images.
    #[serde(rename = "omitted_images", default)]
    pub images_omitted: i32,
    /// The total count of replies.
    #[serde(rename = "replies")]
    pub n_replies: i32,
    /// The total number of images in a thread.
    #[serde(rename = "images", default)]
    pub n_images: i32,
    /// `true` if the bump limit is reached.
    #[serde(
        default,
        rename = "bump_limit",
        deserialize_with = "crate::int_to_bool"
    )]
    pub bump_limit_reached: bool,
    /// `true` if the image limit is reached.
    #[serde(
        default,
        rename = "image_limit",
        deserialize_with = "crate::int_to_bool"
    )]
    pub image_limit_reached: bool,
    /// The UNIX timestamp of the time this thread was last modified.
    pub last_modified: u64,
    /// SEO URL slug for thread.
    pub semantic_url: String,
}

impl ThreadInfo {
    /// Returns the ID/no of the thread.
    ///
    /// Pass this along with the boards abbreviation to
    /// [`crate::client::Client::get_full_thread`] to get a full [`Thread`] object.
    pub fn thread_no(&self) -> u32 {
        self.op_post.no
    }
}

/// Represents a thread, with every post included.
///
/// Obtained by calling [`crate::client::Client::get_full_thread`] with
/// the boards abbreviation(the `abv` field) and the no of a [`ThreadInfo`].
#[derive(Debug, Clone)]
pub struct Thread {
    /// The no of the thread.
    pub no: u32,
    /// `true` if the thread is pinned on top of the page.
    pub sticky: bool,
    /// `true` if the thread is closed.
    pub closed: bool,
    /// The threads title, if any.
    pub subject: Option<String>,
    /// Number of unique posters in this thread.
    pub unique_posters: i32,
    /// Every post in this thread.
    pub posts: Vec<Post>,
}

#[derive(Deserialize)]
struct ThreadPre {
    posts: Vec<ThreadPost>,
}

#[derive(Deserialize)]
struct ThreadPost {
    #[serde(default, deserialize_with = "crate::int_to_bool")]
    sticky: bool,
    #[serde(default, deserialize_with = "crate::int_to_bool")]
    closed: bool,
    #[serde(rename = "sub")]
    subject: Option<String>,
    unique_ips: Option<i32>,

    #[serde(flatten)]
    post: PostPre,
}

impl<'de> serde::Deserialize<'de> for Thread {
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let posts = ThreadPre::deserialize(des)?.posts;
        let op = &posts[0];
        let unique_posters = op.unique_ips.unwrap_or_default();
        let sticky = op.sticky;
        let closed = op.closed;
        let subject = op.subject.clone();
        let no = op.post.no;

        let posts = posts
            .into_iter()
            .map(|p| Post::from(p.post))
            .collect::<Vec<_>>();

        Ok(Self {
            no,
            closed,
            subject,
            sticky,
            unique_posters,
            posts,
        })
    }
}
