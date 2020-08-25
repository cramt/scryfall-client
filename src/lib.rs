pub mod decklist;
pub mod client;
pub mod search_builder;
pub mod card;
pub mod test;

macro_rules! wait {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

#[cfg(test)]
mod tests {
    use inflector::cases::snakecase::to_snake_case;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let mut map = HashMap::new();
        map.insert("idk", 2);
        println!("{:?}", serde_json::to_string(&map));
    }

    #[test]
    fn idk() {
        let a = wait!(wait!(reqwest::Client::new().get("https://api.scryfall.com/cards/search")
            .form(&[("q", "f:commander t:legendary (t:creature or (t:planewalker o:\"can be your commander\"))")])
            .send())
            .unwrap()
            .text())
            .unwrap();
        println!("{}", a);
    }
}
