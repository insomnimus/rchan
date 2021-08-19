use serde_derive::Deserialize;

use super::*;

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

#[derive(Deserialize)]
pub(crate) struct ThreadInfoPre {
    #[serde(flatten)]
    pub op_post: PostPre,
    #[serde(default)]
    pub last_replies: Vec<PostPre>,
    #[serde(default, deserialize_with = "crate::int_to_bool")]
    pub sticky: bool,
    #[serde(default, deserialize_with = "crate::int_to_bool")]
    pub closed: bool,
    #[serde(rename = "sub")]
    pub subject: Option<String>,
    #[serde(rename = "omitted_posts", default)]
    pub posts_omitted: i32,
    #[serde(rename = "omitted_images", default)]
    pub images_omitted: i32,
    #[serde(rename = "replies")]
    pub n_replies: i32,
    #[serde(rename = "images", default)]
    pub n_images: i32,
    #[serde(
        default,
        rename = "bump_limit",
        deserialize_with = "crate::int_to_bool"
    )]
    pub bump_limit_reached: bool,
    #[serde(
        default,
        rename = "image_limit",
        deserialize_with = "crate::int_to_bool"
    )]
    pub image_limit_reached: bool,
    pub last_modified: u64,
    pub semantic_url: String,
}

impl<'de> serde::Deserialize<'de> for ThreadInfo {
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ThreadInfoPre {
            bump_limit_reached,
            closed,
            image_limit_reached,
            images_omitted,
            last_modified,
            n_images,
            n_replies,
            op_post,
            posts_omitted,
            semantic_url,
            sticky,
            subject,
            last_replies,
        } = ThreadInfoPre::deserialize(des)?;
        let last_replies: Vec<_> = last_replies.into_iter().map(Post::from).collect();
        let op_post = Post::from(op_post);

        Ok(Self {
            bump_limit_reached,
            closed,
            image_limit_reached,
            images_omitted,
            last_modified,
            n_images,
            n_replies,
            op_post,
            posts_omitted,
            semantic_url,
            sticky,
            subject,
            last_replies,
        })
    }
}
