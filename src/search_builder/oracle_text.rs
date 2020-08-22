use crate::search_builder::SearchBuilderTrait;

pub struct OracleText {
    text: String
}

impl OracleText {
    pub fn new<S: AsRef<str>>(text: S) -> Self {
        Self {
            text: String::from(text.as_ref())
        }
    }
    pub fn new_regex<S: AsRef<str>>(regex: S) -> Self {
        Self {
            text: format!("/{}/", regex.as_ref()).to_string()
        }
    }
}

impl SearchBuilderTrait for OracleText {
    fn stringify(&self) -> String {
        return if self.text.contains(" ") {
            format!("o:\"{}\"", self.text).to_string()
        } else {
            format!("o:{}", self.text).to_string()
        };
    }
}
