#[cfg(test)]
mod card {
    use crate::card::mana_cost::{ManaCost, ManaCostCollection};
    use serde::{Deserialize, Serialize};
    use std::rc::Rc;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct CostTestStruct {
        cost: ManaCostCollection,
    }

    #[test]
    fn mana_cost_json_parsing() {
        let json = "{\"cost\": \"{G/P}{R}{½}{4353246}\"}";
        let cost = serde_json::from_str::<CostTestStruct>(json).unwrap().cost;
        assert_eq!(
            cost.costs,
            vec![vec![
                ManaCost::Or(Rc::new(ManaCost::Green), Rc::new(ManaCost::Phyrexian)),
                ManaCost::Red,
                ManaCost::Half(Rc::new(ManaCost::Generic(1))),
                ManaCost::Generic(4353246)
            ]]
        );
    }
}
