use crate::client::{Client, ClientError};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct AutocompleteResult {
    pub object: String,
    pub total_values: u64,
    pub data: Vec<String>,
}

impl Client{
    pub async fn autocomplete<S: ToString>(&self, name: S) -> Result<Vec<String>, ClientError> {
        let name = name.to_string();
        let query = self.autocomplete_client().form(&[("q", &name)]);
        let result = self.send_request::<AutocompleteResult>(query).await?;
        Ok(result.data)
    }
}
