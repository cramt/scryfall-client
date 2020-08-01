use crate::search_builder::SearchBuilderTrait;

pub struct OracleText {
    text: String
}

impl OracleText {
    pub fn new<S: AsRef<str>>(text: S) -> OracleText {
        OracleText {
            text: String::from(text.as_ref())
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
