use crate::search_builder::SearchBuilderTrait;

pub struct Or {
    lhs: Box<dyn SearchBuilderTrait>,
    rhs: Box<dyn SearchBuilderTrait>,
}

impl Or {
    pub fn new<E1: 'static, E2: 'static>(lhs: E1, rhs: E2) -> Or
    where
        E1: SearchBuilderTrait,
        E2: SearchBuilderTrait,
    {
        Or {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

impl SearchBuilderTrait for Or {
    fn stringify(&self) -> String {
        format!("({} or {})", self.lhs.stringify(), self.rhs.stringify()).to_string()
    }
}
