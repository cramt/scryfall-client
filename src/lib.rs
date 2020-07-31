mod test;
mod card;




mod scryfall_client;

macro_rules! wait {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}


#[cfg(test)]
mod tests {
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
}
