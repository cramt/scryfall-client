use crate::client::{Client, ClientError, ScryfallError, wait_for_rate_limit};
use crate::card::Card;

impl Client {
    pub async fn name<S: ToString>(&self, name: S, exact: bool) -> Result<Card, ClientError> {
        let name = name.to_string();
        let exact_or_fuzzy = if exact { "exact" } else { "fuzzy" };
        let query = self.named_client().form(&[(exact_or_fuzzy, name)]);
        let result = self.send_request::<Card>(query).await?;
        Ok(result)
    }
}
