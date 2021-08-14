use crate::{
    board::{Board, Boards},
    catalog::Catalog,
    error::Error,
    thread::Thread,
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

    pub async fn get_boards(&self) -> Result<Vec<Board>> {
        self.json::<Boards>(&format!("{}boards.json", crate::BASE))
            .await
            .map(|x| x.boards)
    }

    pub async fn get_board_catalog(&self, board_abv: &str) -> Result<Catalog> {
        self.json::<Catalog>(&format!("{}{}/catalog.json", crate::BASE, board_abv))
            .await
    }

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
