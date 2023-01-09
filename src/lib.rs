pub mod api;

use reqwest::Url;

/// The mempool struct
pub struct MempoolClient {
    mempool_url: Url,
    reqwest_client: reqwest::Client,
}

#[derive(Debug, thiserror::Error)]
pub enum MempoolError {
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("url error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("serde error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("not found")]
    NotFound,

    #[error("Server error")]
    ServerError,
}

impl MempoolClient {
    /// Create a new MemPoolClient
    /// # Arguments
    /// * `url` - The url of the mempool server
    /// * `tor_socket` - If defined, the tor socket to use
    ///
    /// # Example
    /// ```
    /// use mempool_rust::MempoolClient;
    /// let client = MempoolClient::new(
    ///    "https://mempool.space",
    ///    None,
    /// ).unwrap();
    /// ```
    pub fn new(url: &str, tor_socket: Option<&str>) -> Result<Self, MempoolError> {
        let mempool_url = Url::parse(url)?;

        let client = {
            if let Some(tor_socket) = tor_socket {
                let proxy = reqwest::Proxy::all(tor_socket).expect("tor proxy should be there");
                reqwest::Client::builder().proxy(proxy).build()?
            } else {
                reqwest::Client::builder().build()?
            }
        };

        Ok(Self {
            mempool_url,
            reqwest_client: client,
        })
    }
}
