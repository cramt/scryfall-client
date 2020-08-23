use crate::card::Card;
use serde::{Deserialize, Serialize};

pub enum Error {
    request(reqwest::Error),
    text_decode(reqwest::Error),
    deserialize(serde_json::Error),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SearchResult {
    pub object: String,
    pub total_cards: u64,
    pub has_more: bool,
    pub data: Vec<Card>,
}

pub struct Client;

impl Client {
    fn search_client(&self) -> reqwest::RequestBuilder {
        reqwest::Client::new().get("https://api.scryfall.com/cards/search")
    }
    pub async fn search<S: ToString>(&self, query: S) -> Result<Vec<Card>, Error> {
        let query = query.to_string();
        let mut results = vec![];
        loop {
            let result = self.search_client().form(&[("q", &query)]).send().await.map_err(|x| Error::request(x))?;
            let text = result.text().await.map_err(|x| Error::text_decode(x))?;
            let card = serde_json::from_str::<SearchResult>(text.as_str()).map_err(|x| Error::deserialize(x))?;
            results.extend(card.data.into_iter());
            if !card.has_more {
                break;
            }
        };
        Ok(results)
    }
}
