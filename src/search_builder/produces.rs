use crate::search_builder::SearchBuilderTrait;
use crate::card::color::Colors;

pub struct Produces {
    colors: Colors,
    operator: String,
}

impl Produces {
    fn internal_new(colors: Colors, operator: &str) -> Produces {
        Produces {
            colors,
            operator: operator.to_string(),
        }
    }
    pub fn eq(colors: Colors) -> Produces {
        Produces::internal_new(colors, ":")
    }
    pub fn less(colors: Colors) -> Produces {
        Produces::internal_new(colors, "<")
    }
    pub fn less_eq(colors: Colors) -> Produces {
        Produces::internal_new(colors, "<=")
    }
    pub fn greater(colors: Colors) -> Produces {
        Produces::internal_new(colors, ">")
    }
    pub fn greater_eq(colors: Colors) -> Produces {
        Produces::internal_new(colors, ">=")
    }
    pub fn not(colors: Colors) -> Produces {
        Produces::internal_new(colors, "!")
    }
}

impl SearchBuilderTrait for Produces {
    fn stringify(&self) -> String {
        format!("produces{}{}", self.operator, self.colors.iter().map(|x| x.to_string()).collect::<String>())
    }
}
