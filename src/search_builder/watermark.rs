use crate::search_builder::SearchBuilderTrait;

pub struct Watermark {
    search_string: String
}

impl Watermark {
    pub fn new<S: AsRef<str>>(set_id: S) -> Self {
        Watermark {
            search_string: format!("watermark:{}", set_id.as_ref()).to_string()
        }
    }
    pub fn exists() -> Self {
        Watermark {
            search_string: String::from("has:watermark")
        }
    }
}

impl SearchBuilderTrait for Watermark {
    fn stringify(&self) -> String {
        self.search_string.clone()
    }
}
