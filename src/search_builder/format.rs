use crate::search_builder::SearchBuilderTrait;

pub struct Format {
    format_name: String,
}

impl Format {
    pub fn new<S: AsRef<str>>(format_name: S) -> Format {
        Format {
            format_name: String::from(format_name.as_ref()),
        }
    }
}

impl SearchBuilderTrait for Format {
    fn stringify(&self) -> String {
        format!("f:{}", self.format_name).to_string()
    }
}
