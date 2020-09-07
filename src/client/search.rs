use crate::card::Card;
use crate::client::{Client, ClientError};
use serde::{Deserialize, Serialize};

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
            let query = self
                .search_client()
                .form(&[("q", &query), ("page", &page.to_string())]);
            let result = self.send_request::<SearchResult>(query).await;
            match &result {
                Err(err) => match err {
                    ClientError::Scryfall(scryfall) => {
                        if scryfall.status == 422
                            && scryfall.code == "validation_error"
                            && scryfall
                                .details
                                .starts_with("You have paginated beyond the end of these results")
                        {
                            break;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
            let card = result?;
            results.extend(card.data.into_iter());
            if !card.has_more {
                break;
            }
        }
        Ok(results)
    }
}
