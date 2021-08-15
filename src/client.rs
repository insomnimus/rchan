use crate::{
	board::{
		Board,
		Boards,
	},
	catalog::Catalog,
	thread::Thread,
	Error,
	Result,
};

/// A `Client` is used to make requests to the 4chan web API.
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
	/// Creates a new [`Client`] with reasonable defaults.
	///
	/// For more control, use [`Self::with_client`].
	pub fn new() -> Self {
		Self::default()
	}

	/// Creates a new [`Self`] using the supplied [`reqwest::Client`].
	pub fn with_client(client: reqwest::Client) -> Self {
		Self { client }
	}
}

impl Client {
	pub(crate) async fn json<T>(&self, path: &str) -> Result<T>
	where
		T: serde::de::DeserializeOwned,
	{
		let resp = self.client.get(path).send().await?;

		if !resp.status().is_success() {
			return Err(Error::status_code(resp.status()));
		}

		resp.text()
			.await
			.map_err(Error::from)
			.and_then(|s| serde_json::from_str(&s).map_err(Error::from))
	}

	/// Gets a list of boards available from the 4chan API.
	pub async fn get_boards(&self) -> Result<Vec<Board>> {
		self.json::<Boards>(&format!("{}boards.json", crate::BASE))
			.await
			.map(|x| x.boards)
	}

	/// Returns the [`Catalog`] of a [`Board`].
	///
	/// # Arguments
	/// -  `board_abv`: The abbreviation of a board for example `mu`. Corresponds to the `abv` field of a [`Board`].
	pub async fn get_board_catalog(&self, board_abv: &str) -> Result<Catalog> {
		self.json::<Catalog>(&format!("{}{}/catalog.json", crate::BASE, board_abv))
			.await
	}

	/// Gets a full [`Thread`].
	///
	/// # Arguments
	/// -  `board_abv`: The abbreviation of a board; the `abv` field of a [`Board`].
	/// -  `thread_no`: The no of a thread. Can be obtained by calling [`crate::thread::ThreadInfo::thread_no`]. Corresponds to the `no` field of the OP.
	pub async fn get_full_thread(&self, board_abv: &str, thread_no: u32) -> Result<Thread> {
		self.json::<Thread>(&format!(
			"{}{}/thread/{}",
			crate::BASE,
			board_abv,
			thread_no
		))
		.await
	}
}
