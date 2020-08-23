#[cfg(test)]
mod client {
    use crate::client::Client;
    use crate::search_builder::name::Name;
    use crate::search_builder::SearchBuilderTrait;

    #[test]
    fn fetch_ugin() {
        let result = tokio_test::block_on(Client.search(Name::new_exact("Ugin, The Spirit Dragon").stringify()));
        let result = result.ok().unwrap();
        assert_eq!(1, result.len());
        assert_eq!("Ugin, the Spirit Dragon", &result[0].name)
    }
}
