use serde_derive::Deserialize;

use crate::thread::ThreadInfo;

#[derive(Debug, Clone, Deserialize)]
pub struct Catalog(pub Vec<Page>);

#[derive(Debug, Clone, Deserialize)]
pub struct Page {
    /// The number of the page.
    pub page: i32,
    /// The list of threads in this page.
    pub threads: Vec<ThreadInfo>,
}
