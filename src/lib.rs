// #![warn(missing_docs)]

use std::fmt;

use serde::de::{
	self,
	Deserializer,
	Visitor,
};

pub mod board;
pub mod catalog;
pub mod client;
pub mod post;
pub mod prelude;
pub mod thread;

pub type Result<T> = ::std::result::Result<T, self::Error>;

pub(crate) const BASE: &str = "https://a.4cdn.org/";

pub(crate) fn int_to_bool<'de, D>(data: D) -> ::std::result::Result<bool, D::Error>
where
	D: Deserializer<'de>,
{
	struct Visit;
	impl<'de> Visitor<'de> for Visit {
		type Value = bool;
		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			write!(formatter, "an integer")
		}

		fn visit_i64<E: de::Error>(self, n: i64) -> ::std::result::Result<Self::Value, E> {
			Ok(n != 0)
		}
	}

	data.deserialize_i64(Visit {})
}

#[derive(Debug)]
pub enum Error {
	Web(reqwest::Error),
	Json(serde_json::Error),
	StatusCode(reqwest::StatusCode),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match &self {
			Self::Json(e) => write!(f, "{}", e),
			Self::Web(e) => write!(f, "{}", e),
			Self::StatusCode(c) => write!(f, "status code is not success: server returned {}", c),
		}
	}
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
	fn from(e: reqwest::Error) -> Self {
		Self::Web(e)
	}
}

impl From<serde_json::Error> for Error {
	fn from(e: serde_json::Error) -> Self {
		Self::Json(e)
	}
}

impl Error {
	pub(crate) fn status_code(c: reqwest::StatusCode) -> Self {
		Self::StatusCode(c)
	}
}
