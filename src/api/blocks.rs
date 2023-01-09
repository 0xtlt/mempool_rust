use serde::{Deserialize, Serialize};

use crate::MempoolClient;

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockExtraPool {
    pub id: u16,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockExtra {
    #[serde(rename = "coinbaseRaw")]
    pub coinbase_raw: String,
    #[serde(rename = "medianFee")]
    pub median_fee: u64,
    #[serde(rename = "feeRange")]
    pub fee_range: Vec<u64>,
    pub reward: u64,
    #[serde(rename = "totalFees")]
    pub total_fees: u64,
    #[serde(rename = "avgFee")]
    pub avg_fee: u64,
    #[serde(rename = "avgFeeRate")]
    pub avg_fee_rate: u64,
    pub pool: BlockExtraPool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
    pub id: String,
    pub timestamp: u64,
    pub height: u32,
    pub version: u32,
    pub bits: u64,
    pub nonce: u64,
    pub difficulty: f64,
    pub merkle_root: String,
    pub tx_count: u32,
    pub size: u32,
    pub weight: u32,
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: String,
    pub extras: BlockExtra,
}

impl MempoolClient {
    /// Get blocks
    /// # Arguments
    /// * `height` - Optional height of the block
    ///
    /// # Example
    /// ```
    /// use mempool_rust::MempoolClient;
    ///
    /// async fn test_get_blocks() {
    ///   let client = MempoolClient::new(
    ///      "https://mempool.space",
    ///      None,
    ///   ).unwrap();
    ///
    ///  let blocks = client.get_blocks(None).await.unwrap();
    /// }
    ///
    /// tokio_test::block_on(async {
    ///     test_get_blocks().await;
    /// })
    /// ```
    pub async fn get_blocks(&self, height: Option<u32>) -> Result<Vec<Block>, crate::MempoolError> {
        let api_response = self
            .make_get(&format!("api/v1/blocks{}", {
                if let Some(height) = height {
                    format!("/{}", height)
                } else {
                    "".to_string()
                }
            }))
            .await?;

        let api_response: Vec<Block> = serde_json::from_str(&api_response)?;
        Ok(api_response)
    }
}
