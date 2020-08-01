pub mod produces;
pub mod not;
pub mod color_identity;
pub mod devotion;
pub mod cmc;
pub mod color;
pub mod mana;
pub mod oracle_text;
pub mod or;
pub mod magic_type;
pub mod format;

pub trait SearchBuilderTrait {
    fn stringify(&self) -> String;
}

pub struct Builder {
    children: Vec<Box<dyn SearchBuilderTrait>>
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            children: vec![],
        }
    }

    pub fn add<E: 'static>(mut self, search_builder: E) -> Self where E: SearchBuilderTrait {
        self.children.push(Box::new(search_builder));
        self
    }
}

impl SearchBuilderTrait for Builder {
    fn stringify(&self) -> String {
        self.children.iter().map(|x| x.stringify()).collect::<Vec<String>>().join(" ")
    }
}
