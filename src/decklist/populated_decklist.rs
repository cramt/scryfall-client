use crate::card::Card;
use crate::decklist::unpopulated_decklist::UnpopulatedDecklist;

pub struct PopulatedDecklist {
    pub main: Vec<(u16, Card)>,
    pub sideboard: Vec<(u16, Card)>,
}

impl PopulatedDecklist {
    pub fn unpopulate(&self) -> UnpopulatedDecklist {
        fn reduce(v: &Vec<(u16, Card)>) -> Vec<(u16, String)> {
            v.into_iter().map(|(amount, card)| {
                if let Some(card_face) = card.card_faces.clone() {
                    if card_face.len() == 2 {
                        let first = &card_face[0];
                        let second = &card_face[1];
                        if let Some(oracle_text) = &second.oracle_text {
                            if oracle_text.contains(format!("(Transforms from {}.)", &first.name).as_str()) {
                                if let Some(oracle_text) = &first.oracle_text {
                                    if oracle_text.contains("transform") {
                                        return (amount.clone(), card.name.split(" // ").collect::<Vec<&str>>()[0].to_string());
                                    }
                                }
                            }
                        }
                    }
                }
                (amount.clone(), card.name.to_string())
            }).collect()
        }
        let main = reduce(&self.main);
        let sideboard = reduce(&self.sideboard);
        UnpopulatedDecklist {
            main,
            sideboard,
        }
    }
}
