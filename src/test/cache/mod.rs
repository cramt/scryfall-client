#[cfg(test)]
mod cache {
    use crate::cache::{CacheOptions, ScryfallCache};
    use crate::card::Card;
    use crate::search_builder;
    use crate::search_builder::name::Name;
    use crate::search_builder::SearchBuilderTrait;
    use tokio_test::block_on;

    #[test]
    fn initialize() {
        let cache = ScryfallCache::new_options(
            CacheOptions::new().query(
                search_builder::Builder::new()
                    .add(Name::new_regex("ugin"))
                    .stringify(),
            ),
        );
        block_on(cache.check()).unwrap();
        let types = block_on(cache.iter())
            .unwrap()
            .filter(|x| x.type_line.starts_with("Legendary Planeswalker"))
            .collect::<Vec<Card>>();
    }
}
