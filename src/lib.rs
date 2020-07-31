mod card;

use serde::{Deserialize, Serialize};
use crate::card::mana_cost::ManaCostCollection;

mod scryfall_client;

macro_rules! wait {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CostTestStruct {
    cost: ManaCostCollection
}

#[cfg(test)]
mod tests {
    use crate::CostTestStruct;
    use crate::card::mana_cost::{ManaCostCollection, ManaCost};
    use std::rc::Rc;
    use crate::card::Card;

    #[test]
    fn it_works() {
        let a = "12345";
        println!("{}", &a[1..]);
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

    #[test]
    fn mana_cost() {
        let json = "{\"cost\": \"{G/P}{R}{½}{4353246}\"}";
        let cost = serde_json::from_str::<CostTestStruct>(json).unwrap().cost;
        assert_eq!(cost.array, vec![
            ManaCost::Or(Rc::new(ManaCost::Green), Rc::new(ManaCost::Phyrexian)),
            ManaCost::Red,
            ManaCost::Half(Rc::new(ManaCost::Generic(1))),
            ManaCost::Generic(4353246)
        ]);
    }

    #[test]
    fn parse(){
        let json = include_str!("./card_test.json");
        let card = serde_json::from_str::<Card>(json).unwrap();
        println!("{:?}", card);
    }
}
