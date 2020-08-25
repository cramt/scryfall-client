use crate::card::Card;
use crate::client::card_identifier::CardIdentifier;
use crate::client::{Client, ClientError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CollectionResult {
    pub object: String,
    pub data: Vec<Card>,
}

impl Client {
    pub async fn collection(
        &self,
        identifiers: Vec<CardIdentifier>,
    ) -> Result<Vec<Card>, ClientError> {
        let identifiers = identifiers
            .into_iter()
            .map(|x| x.to_map())
            .collect::<Vec<HashMap<String, String>>>();
        let mut results = vec![];
        for identifiers in identifiers.chunks(74) {
            let post_body =
                serde_json::to_string(&identifiers).map_err(|x| ClientError::Serialize(x))?;
            let post_body = format!("{{ \"identifiers\": {}}}", post_body);
            let request = self.collection_client().body(post_body);
            let result = self.send_request::<CollectionResult>(request).await?;
            results.extend(result.data.into_iter());
        }
        Ok(results)
    }
    pub async fn collection_by_names(
        &self,
        identifiers: Vec<String>,
    ) -> Result<Vec<Card>, ClientError> {
        self.collection(identifiers.into_iter().map(CardIdentifier::name).collect())
            .await
    }
}
