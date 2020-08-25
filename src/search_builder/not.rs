use crate::search_builder::SearchBuilderTrait;

pub struct Not {
    inner: Box<dyn SearchBuilderTrait>,
}

impl Not {
    pub fn new<E: 'static>(inner: E) -> Not
    where
        E: SearchBuilderTrait,
    {
        Not {
            inner: Box::new(inner),
        }
    }
}

impl SearchBuilderTrait for Not {
    fn stringify(&self) -> String {
        format!("-{}", self.inner.stringify()).to_string()
    }
}
