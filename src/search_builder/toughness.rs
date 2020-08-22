use crate::search_builder::SearchBuilderTrait;

pub struct Toughness {
    rhs: String,
    operator: String,
}

impl Toughness {
    fn internal_new(rhs: String, operator: &str) -> Toughness {
        Toughness {
            rhs,
            operator: operator.to_string(),
        }
    }
    pub fn eq(rhs: u32) -> Toughness {
        Toughness::internal_new(rhs.to_string(), ":")
    }
    pub fn less(rhs: u32) -> Toughness {
        Toughness::internal_new(rhs.to_string(), "<")
    }
    pub fn less_eq(rhs: u32) -> Toughness {
        Toughness::internal_new(rhs.to_string(), "<=")
    }
    pub fn greater(rhs: u32) -> Toughness {
        Toughness::internal_new(rhs.to_string(), ">")
    }
    pub fn greater_eq(rhs: u32) -> Toughness {
        Toughness::internal_new(rhs.to_string(), ">=")
    }
    pub fn not(rhs: u32) -> Toughness {
        Toughness::internal_new(rhs.to_string(), "!")
    }
    pub fn eq_pow() -> Toughness {
        Toughness::internal_new("pow".to_string(), ":")
    }
    pub fn less_pow() -> Toughness {
        Toughness::internal_new("pow".to_string(), "<")
    }
    pub fn less_eq_pow() -> Toughness {
        Toughness::internal_new("pow".to_string(), "<=")
    }
    pub fn greater_pow() -> Toughness {
        Toughness::internal_new("pow".to_string(), ">")
    }
    pub fn greater_eq_pow() -> Toughness {
        Toughness::internal_new("pow".to_string(), ">=")
    }
    pub fn not_pow() -> Toughness {
        Toughness::internal_new("pow".to_string(), "!")
    }
}

impl SearchBuilderTrait for Toughness {
    fn stringify(&self) -> String {
        format!("tou{}{}", self.operator, self.rhs.to_string())
    }
}
