use std::fmt;

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
