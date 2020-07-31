use crate::mana_cost::ManaCostCollection;
use serde::{Deserialize, Serialize};

mod mana_cost;
mod color;
mod legality;
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
        let json = "{\"cost\": \"{G/P}\"}";
        let cost = serde_json::from_str::<CostTestStruct>(json).unwrap().cost;
        println!("{:?}", cost);
    }
}
