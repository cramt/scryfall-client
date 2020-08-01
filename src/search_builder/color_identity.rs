use crate::search_builder::SearchBuilderTrait;
use crate::card::color::Colors;

pub struct ColorIdentity {
    colors: Colors,
    operator: String,
}

impl ColorIdentity {
    fn internal_new(colors: Colors, operator: &str) -> ColorIdentity {
        ColorIdentity {
            colors,
            operator: operator.to_string(),
        }
    }
    pub fn eq(colors: Colors) -> ColorIdentity {
        ColorIdentity::internal_new(colors, ":")
    }
    pub fn less(colors: Colors) -> ColorIdentity {
        ColorIdentity::internal_new(colors, "<")
    }
    pub fn less_eq(colors: Colors) -> ColorIdentity {
        ColorIdentity::internal_new(colors, "<=")
    }
    pub fn greater(colors: Colors) -> ColorIdentity {
        ColorIdentity::internal_new(colors, ">")
    }
    pub fn greater_eq(colors: Colors) -> ColorIdentity {
        ColorIdentity::internal_new(colors, ">=")
    }
    pub fn not(colors: Colors) -> ColorIdentity {
        ColorIdentity::internal_new(colors, "!")
    }
}

impl SearchBuilderTrait for ColorIdentity {
    fn stringify(&self) -> String {
        format!("c{}{}", self.operator, self.colors.iter().map(|x| x.to_string()).collect::<String>())
    }
}
