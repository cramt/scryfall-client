use crate::search_builder::SearchBuilderTrait;

pub struct FlavorText {
    query: String,
}

impl FlavorText {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        FlavorText {
            query: name.as_ref().to_string(),
        }
    }
    pub fn new_regex<S: AsRef<str>>(regex: S) -> Self {
        FlavorText {
            query: format!("/{}/", regex.as_ref()),
        }
    }
}

impl SearchBuilderTrait for FlavorText {
    fn stringify(&self) -> String {
        if self.query.contains(" ")
            && match self.query.chars().nth(0) {
                Some(c) => c != '/',
                None => true,
            }
        {
            format!("flavor:\"{}\"", self.query)
        } else {
            format!("flavor:{}", self.query)
        }
    }
}
