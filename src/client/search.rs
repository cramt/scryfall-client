use crate::client::{Client, ClientError, wait_for_rate_limit};
use crate::card::Card;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SearchResult {
    pub object: String,
    pub total_cards: u64,
    pub has_more: bool,
    pub data: Vec<Card>,
}

impl Client {
    pub async fn search<S: ToString>(&self, query: S) -> Result<Vec<Card>, ClientError> {
        let query = query.to_string();
        let mut results = vec![];
        let mut page = 0u32;
        loop {
            wait_for_rate_limit().await;
            page += 1;
            let result = self.search_client().form(&[("q", &query), ("page", &page.to_string())]).send().await.map_err(|x| ClientError::Request(x))?;
            let text = result.text().await.map_err(|x| ClientError::TextDecode(x))?;
            let card = serde_json::from_str::<SearchResult>(text.as_str()).map_err(|x| ClientError::Deserialize(x))?;
            results.extend(card.data.into_iter());
            if !card.has_more {
                break;
            }
        };
        Ok(results)
    }
}
