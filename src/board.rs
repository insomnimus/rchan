use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Board {
	pub title: String,
	#[serde(rename = "board")]
	pub abv: String,
	pub meta_description: String,

	#[serde(default, deserialize_with = "crate::int_to_bool")]
	pub is_archived: bool,
	#[serde(default, deserialize_with = "crate::int_to_bool", rename = "ws_board")]
	pub is_sfw: bool,

	pub per_page: i32,
	pub pages: i32,
	pub max_filesize: usize,
	pub max_webm_filesize: usize,
	pub max_comment_chars: usize,
	pub max_webm_duration: usize,
	pub bump_limit: usize,
	pub image_limit: usize,
	pub cooldowns: Cooldowns,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Cooldowns {
	pub threads: usize,
	pub replies: usize,
	pub images: usize,
}

#[derive(Deserialize)]
pub(crate) struct Boards {
	pub(crate) boards: Vec<Board>,
}
