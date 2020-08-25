use crate::search_builder::SearchBuilderTrait;

pub struct MagicType {
    format_name: String,
}

impl MagicType {
    pub fn new<S: AsRef<str>>(format_name: S) -> MagicType {
        MagicType {
            format_name: String::from(format_name.as_ref()),
        }
    }
}

impl SearchBuilderTrait for MagicType {
    fn stringify(&self) -> String {
        format!("t:{}", self.format_name).to_string()
    }
}
