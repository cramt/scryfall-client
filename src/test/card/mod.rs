mod mana_cost;

#[cfg(test)]
mod card {
    use crate::card::Card;

    #[test]
    fn json_parse() {
        let json = include_str!("card_test.json");
        let card = serde_json::from_str::<Card>(json).unwrap();
        assert_eq!(card.name, "Austere Command")
    }
}
