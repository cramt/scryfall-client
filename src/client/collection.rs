use crate::client::{Client, ClientError};
use crate::client::card_identifier::CardIdentifier;
use crate::card::Card;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CollectionResult {
    pub object: String,
    pub data: Vec<Card>,
}

impl Client {
    pub async fn collection(&self, identifiers: Vec<CardIdentifier>) -> Result<Vec<Card>, ClientError> {
        let mut results = vec![];
        for identifiers in identifiers.chunks(75) {
            let post_body = serde_json::to_string(&identifiers).map_err(|x| ClientError::Serialize(x))?;
            let request = self.collection_client().body(post_body);
            let result = self.send_request::<CollectionResult>(request).await?;
            results.extend(result.data.into_iter());
        };
        Ok(results)
    }
    pub async fn collection_by_names(&self, identifiers: Vec<String>) -> Result<Vec<Card>, ClientError>  {
        self.collection(identifiers.into_iter().map(CardIdentifier::name).collect()).await
    }
}
