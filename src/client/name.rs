use crate::client::{Client, ClientError, ScryfallError, wait_for_rate_limit};
use crate::card::Card;

impl Client {
    pub async fn name<S: ToString>(&self, name: S, exact: bool) -> Result<Card, ClientError> {
        let name = name.to_string();
        let exact_or_fuzzy = if exact { "exact" } else { "fuzzy" };
        wait_for_rate_limit().await;
        let result = self.named_client().form(&[(exact_or_fuzzy, name)]).send().await.map_err(|x| ClientError::Request(x))?;
        let text = result.text().await.map_err(|x| ClientError::TextDecode(x))?;
        let card = serde_json::from_str::<Card>(text.as_str()).map_err(|x| {
            match serde_json::from_str::<ScryfallError>(text.as_str()) {
                Ok(scryfall) => ClientError::Scryfall(scryfall),
                _ => ClientError::Deserialize(x)
            }
        })?;
        Ok(card)
    }
}
