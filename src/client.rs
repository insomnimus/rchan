use crate::{
	board::{
		Board,
		Boards,
	},
	error::Error,
	Result,
};

pub struct Client {
	client: reqwest::Client,
}

impl Default for Client {
	fn default() -> Self {
		Self {
			client: reqwest::Client::new(),
		}
	}
}

impl Client {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_client(client: reqwest::Client) -> Self {
		Self { client }
	}
}

impl Client {
	pub async fn get_boards(&self) -> Result<Vec<Board>> {
		let resp = self
			.client
			.get(&format!("{}boards.json", crate::BASE))
			.send()
			.await?;

		if !resp.status().is_success() {
			return Err(Error::status_code(resp.status()));
		}

		resp.text().await.map_err(Error::from).and_then(|s| {
			serde_json::from_str(&s)
				.map_err(Error::from)
				.map(|x: Boards| x.boards)
		})
	}
}
