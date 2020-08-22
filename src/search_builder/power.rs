use crate::search_builder::SearchBuilderTrait;

pub struct Power {
    rhs: String,
    operator: String,
}

impl Power {
    fn internal_new(rhs: String, operator: &str) -> Power {
        Power {
            rhs,
            operator: operator.to_string(),
        }
    }
    pub fn eq(rhs: u32) -> Power {
        Power::internal_new(rhs.to_string(), ":")
    }
    pub fn less(rhs: u32) -> Power {
        Power::internal_new(rhs.to_string(), "<")
    }
    pub fn less_eq(rhs: u32) -> Power {
        Power::internal_new(rhs.to_string(), "<=")
    }
    pub fn greater(rhs: u32) -> Power {
        Power::internal_new(rhs.to_string(), ">")
    }
    pub fn greater_eq(rhs: u32) -> Power {
        Power::internal_new(rhs.to_string(), ">=")
    }
    pub fn not(rhs: u32) -> Power {
        Power::internal_new(rhs.to_string(), "!")
    }
    pub fn eq_tou() -> Power {
        Power::internal_new("tou".to_string(), ":")
    }
    pub fn less_tou() -> Power {
        Power::internal_new("tou".to_string(), "<")
    }
    pub fn less_eq_tou() -> Power {
        Power::internal_new("tou".to_string(), "<=")
    }
    pub fn greater_tou() -> Power {
        Power::internal_new("tou".to_string(), ">")
    }
    pub fn greater_eq_tou() -> Power {
        Power::internal_new("tou".to_string(), ">=")
    }
    pub fn not_tou() -> Power {
        Power::internal_new("tou".to_string(), "!")
    }
}

impl SearchBuilderTrait for Power {
    fn stringify(&self) -> String {
        format!("pow{}{}", self.operator, self.rhs.to_string())
    }
}
