use crate::card::Card;

pub struct PopulatedDecklist {
    pub main: Vec<(u16, Card)>,
    pub sideboard: Vec<(u16, Card)>,
}


