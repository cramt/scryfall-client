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

    #[test]
    fn it_works() {
        println!("{}", "idk");
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
