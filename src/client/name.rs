use crate::card::Card;
use crate::client::{Client, ClientError};

impl Client {
    pub async fn name<S: ToString>(&self, name: S, exact: bool) -> Result<Card, ClientError> {
        self.name_in_set(name, exact, String::new()).await
    }

    pub async fn name_in_set<S1: ToString, S2: ToString>(
        &self,
        name: S1,
        exact: bool,
        set: S2,
    ) -> Result<Card, ClientError> {
        let name = name.to_string();
        let set = set.to_string();
        let exact_or_fuzzy = if exact { "exact" } else { "fuzzy" };
        let query = self
            .named_client()
            .form(&[(exact_or_fuzzy, name), ("set", set)]);
        let result = self.send_request::<Card>(query).await?;
        Ok(result)
    }
}
