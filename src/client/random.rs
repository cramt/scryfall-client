use crate::card::Card;
use crate::client::{Client, ClientError};
use crate::search_builder::{Builder, SearchBuilderTrait};

impl Client {
    pub async fn random(&self) -> Result<Card, ClientError> {
        self.random_with_query(Builder::all_cards().stringify())
            .await
    }

    pub async fn random_with_query<S: ToString>(&self, query: S) -> Result<Card, ClientError> {
        let query = query.to_string();
        let query = self.random_client().form(&[("q", query)]);
        let result = self.send_request::<Card>(query).await?;
        Ok(result)
    }
}
