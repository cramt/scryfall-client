#[cfg(test)]
mod client {
    use crate::card::magic_set::MagicSet;
    use crate::client::Client;
    use crate::client::ClientError;
    use crate::search_builder::name::Name;
    use crate::search_builder::SearchBuilderTrait;
    use chrono::NaiveDate;
    use rand::seq::IteratorRandom;
    use std::ops::Deref;
    use tokio_test::block_on;

    #[test]
    fn search_ugin() {
        let result = tokio_test::block_on(
            Client.search(Name::new_exact("Ugin, The Spirit Dragon").stringify()),
        )
        .unwrap();
        assert_eq!(1, result.len());
        assert_eq!("Ugin, the Spirit Dragon", &result[0].name)
    }

    #[test]
    fn fetch_ugin() {
        let result = tokio_test::block_on(Client.name("Ugin, the Spirit Dragon", true));
        let result = result.unwrap();
        assert_eq!("Ugin, the Spirit Dragon", result.name)
    }

    #[test]
    fn fetch_error() {
        let result = tokio_test::block_on(Client.name("ifosiofhsdiofhduipbguhihp", true));
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(match err {
            ClientError::Scryfall(scryfall) => scryfall.code == "not_found",
            _ => false,
        })
    }

    #[test]
    fn fetch_azcanta() {
        let result =
            block_on(Client.collection_by_names(vec!["Search for Azcanta".to_string()])).unwrap();
        assert_eq!(1, result.len());
        assert_eq!(
            "Search for Azcanta // Azcanta, the Sunken Ruin",
            &result[0].name
        );
    }

    #[test]
    fn fetch_rtr_set() {
        let result = block_on(Client.set("rtr")).unwrap();
        assert_eq!("Return to Ravnica", result.name)
    }

    #[test]
    fn fetch_pionier() {
        let result = block_on(Client.sets()).unwrap();
        let after = NaiveDate::from_ymd(2012, 10, 05);
        let result = result
            .into_iter()
            .filter(|x| {
                x.released_at.is_some() && x.released_at.as_ref().unwrap().deref().ge(&after)
            })
            .collect::<Vec<MagicSet>>();
        assert!(result.iter().find(|x| x.name == "Time Spiral").is_none());
        assert!(result
            .iter()
            .find(|x| x.name == "Guilds of Ravnica")
            .is_some());
    }

    fn block_party() -> (Vec<MagicSet>, Vec<MagicSet>) {
        let result = block_on(Client.sets()).unwrap();
        let after = NaiveDate::from_ymd(2012, 10, 05);
        let (core_sets, expansion_sets): (Vec<MagicSet>, Vec<MagicSet>) = result
            .into_iter()
            .filter(|x| {
                x.released_at.is_some() && x.released_at.as_ref().unwrap().deref().ge(&after)
            })
            .filter(|x| x.set_type == "core" || x.set_type == "expansion")
            .partition(|x| x.set_type == "core");
        (core_sets, expansion_sets)
    }

    #[test]
    fn block_party_core() {
        let (core_sets, _) = block_party();
        println!("Core Sets:");
        for core_set in &core_sets {
            println!(
                "{}, https://scryfall.com/sets/{}?as=grid&order=set",
                core_set.name, core_set.code
            )
        }
    }

    #[test]
    fn block_party_expansion() {
        let (_, expansion_sets) = block_party();
        let mut rng = rand::thread_rng();
        let result = expansion_sets.into_iter().choose_multiple(&mut rng, 12);
        println!("Expansions:");
        for expansions in &result {
            println!(
                "{}, https://scryfall.com/sets/{}?as=grid&order=set",
                expansions.name, expansions.code
            )
        }
    }
}
