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
            page += 1;
            let query = self.search_client().form(&[("q", &query), ("page", &page.to_string())]);
            let card = self.send_request::<SearchResult>(query).await?;
            results.extend(card.data.into_iter());
            if !card.has_more {
                break;
            }
        };
        Ok(results)
    }
}
