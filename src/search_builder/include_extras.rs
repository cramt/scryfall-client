use crate::search_builder::SearchBuilderTrait;

pub struct IncludeExtras;

impl SearchBuilderTrait for IncludeExtras {
    fn stringify(&self) -> String {
        String::from("include:extras")
    }
}
