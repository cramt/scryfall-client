use crate::search_builder::SearchBuilderTrait;

pub struct Set {
    set: String,
    inclusive: bool,
}

impl Set {
    pub fn new_inclusive<S: AsRef<str>>(set: S) -> Self {
        Set {
            set: set.as_ref().to_string(),
            inclusive: true,
        }
    }

    pub fn new_exclusive<S: AsRef<str>>(set: S) -> Self {
        Set {
            set: set.as_ref().to_string(),
            inclusive: false,
        }
    }
}

impl SearchBuilderTrait for Set {
    fn stringify(&self) -> String {
        format!("{}:{}", if self.inclusive { "in" } else { "set" }, self.set)
    }
}
