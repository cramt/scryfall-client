use crate::card::mana_cost::ManaCost;
use crate::search_builder::SearchBuilderTrait;

pub struct Mana {
    mana_cost: ManaCost,
    operator: String,
}

impl Mana {
    fn internal_new(mana_cost: ManaCost, operator: &str) -> Mana {
        Mana {
            mana_cost,
            operator: operator.to_string(),
        }
    }
}

impl SearchBuilderTrait for Mana {
    fn stringify(&self) -> String {
        format!("m{}{}", self.operator, self.mana_cost.to_string())
    }
}

crate::equality_operator_implementer!(Mana, ManaCost);
