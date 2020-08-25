use crate::client::Client;

impl Client {
    pub(super) fn search_client(&self) -> reqwest::RequestBuilder {
        reqwest::Client::new().get("https://api.scryfall.com/cards/search")
    }
    pub(super) fn named_client(&self) -> reqwest::RequestBuilder {
        reqwest::Client::new().get("https://api.scryfall.com/cards/named")
    }
}
