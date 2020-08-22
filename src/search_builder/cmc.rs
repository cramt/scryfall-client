use crate::search_builder::SearchBuilderTrait;
use crate::search_builder::macros::*;

pub struct Cmc {
    cmc: String,
    operator: String,
}

impl Cmc {
    fn internal_new(cmc: u32, operator: &str) -> Cmc {
        Cmc {
            cmc: cmc.to_string(),
            operator: operator.to_string(),
        }
    }
    pub fn odd() -> Cmc {
        Cmc {
            cmc: "odd".to_string(),
            operator: ":".to_string(),
        }
    }
    pub fn even() -> Cmc {
        Cmc {
            cmc: "even".to_string(),
            operator: ":".to_string(),
        }
    }
}

impl SearchBuilderTrait for Cmc {
    fn stringify(&self) -> String {
        format!("cmc{}{}", self.operator, self.cmc)
    }
}

crate::equality_operator_implementer!(Cmc);
