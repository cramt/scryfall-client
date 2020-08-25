use crate::search_builder::SearchBuilderTrait;

pub struct Artist {
    search_string: String,
}

impl Artist {
    pub fn new<S: AsRef<str>>(set_id: S) -> Self {
        Artist {
            search_string: format!(":{}", set_id.as_ref()).to_string(),
        }
    }
    pub fn greater_than(n: u32) -> Self {
        Artist {
            search_string: format!(">{}", n),
        }
    }
}

impl SearchBuilderTrait for Artist {
    fn stringify(&self) -> String {
        format!("artist{}", self.search_string)
    }
}
