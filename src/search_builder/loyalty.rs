use crate::search_builder::SearchBuilderTrait;

pub struct Loyalty {
    rhs: u32,
    operator: String,
}

impl Loyalty {
    fn internal_new(rhs: u32, operator: &str) -> Loyalty {
        Loyalty {
            rhs,
            operator: operator.to_string(),
        }
    }
}

impl SearchBuilderTrait for Loyalty {
    fn stringify(&self) -> String {
        format!("loy{}{}", self.operator, self.rhs.to_string())
    }
}

crate::equality_operator_implementer!(Loyalty);
