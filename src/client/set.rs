use crate::card::magic_set::MagicSet;
use crate::client::{Client, ClientError};

impl Client {
    pub async fn set<S: AsRef<str>>(&self, id: S) -> Result<MagicSet, ClientError> {
        self.send_request::<MagicSet>(self.set_client(id)).await
    }
}
