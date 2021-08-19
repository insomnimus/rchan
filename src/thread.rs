use serde::de::Deserializer;

use crate::post::{post_pre::PostPre, Post};

mod thread_pre;

/// Represents a thread as seen from the catalog.
///
/// To get all the posts in a thread, call `[Client::get_full_thread]`.
#[derive(Clone, Debug)]
pub struct ThreadInfo {
    /// The [`Post`] of the OP.
    pub op_post: Post,
    /// Last replies to this thread.
    pub last_replies: Vec<Post>,
    /// `true` if the thread is pinned on top of the page.
    pub sticky: bool,
    /// `true` if the thread is closed.
    pub closed: bool,
    /// The threads title, if any.
    pub subject: Option<String>,
    /// The number of omitted replies.
    pub posts_omitted: i32,
    /// The number of omitted images.
    pub images_omitted: i32,
    /// The total count of replies.
    pub n_replies: i32,
    /// The total number of images in a thread.
    pub n_images: i32,
    /// `true` if the bump limit is reached.
    pub bump_limit_reached: bool,
    /// `true` if the image limit is reached.
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
