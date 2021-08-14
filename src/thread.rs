use serde_derive::Deserialize;

use crate::post::Post;

#[derive(Clone, Debug, Deserialize)]
pub struct Thread {
	/// The `Post` of the OP.
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
	#[serde(rename = "omitted_posts")]
	pub posts_omitted: i32,
	/// The number of omitted images.
	#[serde(rename = "omitted_images")]
	pub images_omitted: i32,
	/// The total count of replies.
	#[serde(rename = "replies")]
	pub n_replies: i32,
	/// The total number of images in a thread.
	#[serde(rename = "images")]
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
