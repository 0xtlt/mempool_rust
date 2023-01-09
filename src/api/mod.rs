mod blocks;

impl crate::MempoolClient {
    pub async fn make_get(&self, endpoint: &str) -> Result<String, crate::MempoolError> {
        let url = self.mempool_url.join(endpoint)?;
        let response = self.reqwest_client.get(url).send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(crate::MempoolError::NotFound);
        }

        if response.status() != reqwest::StatusCode::OK {
            return Err(crate::MempoolError::ServerError);
        }

        let body = response.text().await?;

        Ok(body)
    }
}
