use crate::card::mana_cost::ManaCost;
use crate::search_builder::SearchBuilderTrait;

pub struct Devotion {
    devotion: ManaCost,
    operator: String,
}

impl Devotion {
    fn internal_new(devotion: ManaCost, operator: &str) -> Devotion {
        Devotion {
            devotion,
            operator: operator.to_string(),
        }
    }
}

impl SearchBuilderTrait for Devotion {
    fn stringify(&self) -> String {
        format!("devotion{}{}", self.operator, self.devotion.to_string())
    }
}

crate::equality_operator_implementer!(Devotion, ManaCost);
