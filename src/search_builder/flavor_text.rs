use crate::search_builder::SearchBuilderTrait;

pub struct Name {
    query: String
}

impl Name {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Name {
            query: name.as_ref().to_string()
        }
    }
    pub fn new_regex<S: AsRef<str>>(regex: S) -> Self {
        Name {
            query: format!("/{}/", regex.as_ref())
        }
    }
}

impl SearchBuilderTrait for Name {
    fn stringify(&self) -> String {
        format!("flavor:{}", self.query)
    }
}
