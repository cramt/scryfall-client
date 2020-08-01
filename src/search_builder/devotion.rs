use crate::search_builder::SearchBuilderTrait;
use crate::card::mana_cost::ManaCost;

pub struct Devotion {
    devotion: ManaCost,
    operator: String,
}

impl Devotion {
    fn internal_new(devotion: ManaCost, operator: &str) -> Devotion {
        Devotion {
            devotion,
            operator: operator.to_string()
        }
    }
    pub fn eq(devotion: ManaCost) -> Devotion {
        Devotion::internal_new(devotion, ":")
    }
    pub fn less(devotion: ManaCost) -> Devotion {
        Devotion::internal_new(devotion, "<")
    }
    pub fn less_eq(devotion: ManaCost) -> Devotion {
        Devotion::internal_new(devotion, "<=")
    }
    pub fn greater(devotion: ManaCost) -> Devotion {
        Devotion::internal_new(devotion, ">")
    }
    pub fn greater_eq(devotion: ManaCost) -> Devotion {
        Devotion::internal_new(devotion, ">=")
    }
    pub fn not(devotion: ManaCost) -> Devotion {
        Devotion::internal_new(devotion, "!")
    }
}

impl SearchBuilderTrait for Devotion {
    fn stringify(&self) -> String {
        format!("devotion{}{}", self.operator, self.devotion.to_string())
    }
}
