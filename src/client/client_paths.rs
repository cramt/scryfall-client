use crate::client::Client;

impl Client {
    pub(super) fn search_client(&self) -> reqwest::RequestBuilder {
        self.get("https://api.scryfall.com/cards/search")
    }
    pub(super) fn named_client(&self) -> reqwest::RequestBuilder {
        self.get("https://api.scryfall.com/cards/named")
    }
    pub(super) fn autocomplete_client(&self) -> reqwest::RequestBuilder {
        self.get("https://api.scryfall.com/cards/autocomplete")
    }
}
