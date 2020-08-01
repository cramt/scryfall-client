use crate::search_builder::SearchBuilderTrait;
use crate::card::mana_cost::ManaCost;

pub struct Mana {
    mana_cost: ManaCost,
    operator: String,
}

impl Mana {
    fn internal_new(mana_cost: ManaCost, operator: &str) -> Mana {
        Mana {
            mana_cost,
            operator: operator.to_string()
        }
    }
    pub fn eq(mana_cost: ManaCost) -> Mana {
        Mana::internal_new(mana_cost, ":")
    }
    pub fn less(mana_cost: ManaCost) -> Mana {
        Mana::internal_new(mana_cost, "<")
    }
    pub fn less_eq(mana_cost: ManaCost) -> Mana {
        Mana::internal_new(mana_cost, "<=")
    }
    pub fn greater(mana_cost: ManaCost) -> Mana {
        Mana::internal_new(mana_cost, ">")
    }
    pub fn greater_eq(mana_cost: ManaCost) -> Mana {
        Mana::internal_new(mana_cost, ">=")
    }
    pub fn not(mana_cost: ManaCost) -> Mana {
        Mana::internal_new(mana_cost, "!")
    }
}

impl SearchBuilderTrait for Mana {
    fn stringify(&self) -> String {
        format!("m{}{}", self.operator, self.mana_cost.to_string())
    }
}
