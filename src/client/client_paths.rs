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
    pub(super) fn random_client(&self) -> reqwest::RequestBuilder {
        self.get("https://api.scryfall.com/cards/random")
    }
    pub(super) fn collection_client(&self) -> reqwest::RequestBuilder {
        self.post("https://api.scryfall.com/cards/collection")
            .header("Content-Type", "application/json")
    }
}
