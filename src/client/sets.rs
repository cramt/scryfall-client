use crate::card::magic_set::MagicSet;
use crate::client::{Client, ClientError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SetsResult {
    object: String,
    has_more: bool,
    data: Vec<MagicSet>,
}

impl Client {
    pub async fn sets(&self) -> Result<Vec<MagicSet>, ClientError> {
        let result = self.send_request::<SetsResult>(self.sets_client()).await?;
        Ok(result.data)
    }
}
