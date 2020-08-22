use crate::search_builder::SearchBuilderTrait;
use crate::card::Card;

use crate::card::rarity::Rarity as RarityEnum;

pub struct Rarity {
    rhs: String,
    operator: String,
}

impl Rarity {
    fn internal_new(rhs: RarityEnum, operator: &str) -> Rarity {
        Rarity {
            rhs: rhs.to_string(),
            operator: operator.to_string(),
        }
    }
}

impl SearchBuilderTrait for Rarity {
    fn stringify(&self) -> String {
        format!("r{}{}", self.operator, self.rhs.to_string())
    }
}

crate::equality_operator_implementer!(Rarity, RarityEnum);
