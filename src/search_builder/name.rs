use crate::search_builder::SearchBuilderTrait;

pub struct Name {
    query: String,
}

impl Name {
    pub fn new_exact<S: AsRef<str>>(name: S) -> Self {
        Name {
            query: name.as_ref().to_string(),
        }
    }
    pub fn new_regex<S: AsRef<str>>(regex: S) -> Self {
        Name {
            query: format!("/{}/", regex.as_ref()),
        }
    }
    pub fn new_fuzzy<S: AsRef<str>>(name: S) -> Self {
        Self::new_regex(format!("\\b{}\\b", name.as_ref()))
    }
}

impl SearchBuilderTrait for Name {
    fn stringify(&self) -> String {
        format!("name:{}", self.query)
    }
}
