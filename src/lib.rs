pub mod cache;
pub mod card;
pub mod client;
pub mod decklist;
pub mod search_builder;
pub mod test;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let mut map = HashMap::new();
        map.insert("idk", 2);
        println!("{:?}", serde_json::to_string(&map));
    }
}
