use crate::search_builder::SearchBuilderTrait;

pub struct CollectorNumber {
    number: u64
}

impl CollectorNumber {
    pub fn new(number: u64) -> Self {
        CollectorNumber {
            number
        }
    }
}

impl SearchBuilderTrait for CollectorNumber {
    fn stringify(&self) -> String {
        format!("number:{}", self.number)
    }
}
