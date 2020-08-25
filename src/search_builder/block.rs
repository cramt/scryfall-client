use crate::search_builder::SearchBuilderTrait;

pub struct Block {
    set_id: String,
}

impl Block {
    pub fn new<S: AsRef<str>>(set_id: S) -> Self {
        Block {
            set_id: set_id.as_ref().to_string(),
        }
    }
}

impl SearchBuilderTrait for Block {
    fn stringify(&self) -> String {
        format!("block:{}", self.set_id)
    }
}
