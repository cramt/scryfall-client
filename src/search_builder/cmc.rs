use crate::search_builder::SearchBuilderTrait;

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
    pub fn eq(cmc: u32) -> Cmc {
        Cmc::internal_new(cmc, ":")
    }
    pub fn less(cmc: u32) -> Cmc {
        Cmc::internal_new(cmc, "<")
    }
    pub fn less_eq(cmc: u32) -> Cmc {
        Cmc::internal_new(cmc, "<=")
    }
    pub fn greater(cmc: u32) -> Cmc {
        Cmc::internal_new(cmc, ">")
    }
    pub fn greater_eq(cmc: u32) -> Cmc {
        Cmc::internal_new(cmc, ">=")
    }
    pub fn not(cmc: u32) -> Cmc {
        Cmc::internal_new(cmc, "!")
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
