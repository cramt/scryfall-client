#[cfg(test)]
mod client {
    use crate::client::Client;
    use crate::search_builder::name::Name;
    use crate::search_builder::SearchBuilderTrait;
    use crate::client::ClientError;
    use tokio_test::block_on;

    #[test]
    fn search_ugin() {
        let result = tokio_test::block_on(
            Client.search(Name::new_exact("Ugin, The Spirit Dragon").stringify()))
            .unwrap();
        assert_eq!(1, result.len());
        assert_eq!("Ugin, the Spirit Dragon", &result[0].name)
    }

    #[test]
    fn fetch_ugin() {
        let result = tokio_test::block_on(
            Client.name("Ugin, the Spirit Dragon", true));
        let result = result.unwrap();
        assert_eq!("Ugin, the Spirit Dragon", result.name)
    }

    #[test]
    fn fetch_error() {
        let result = tokio_test::block_on(
            Client.name("ifosiofhsdiofhduipbguhihp", true)
        );
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(match err {
            ClientError::Scryfall(scryfall) => scryfall.code == "not_found",
            _ => false
        })
    }

    #[test]
    fn fetch_azcanta() {
        let result = block_on(Client.collection_by_names(vec!["Search for Azcanta".to_string()])).unwrap();
        assert_eq!(1, result.len());
        assert_eq!("Search for Azcanta // Azcanta, the Sunken Ruin", &result[0].name);
    }
}
