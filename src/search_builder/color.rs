use crate::search_builder::SearchBuilderTrait;
use crate::card::color::Colors;

pub struct Color {
    colors: Colors,
    operator: String,
}

impl Color {
    fn internal_new(colors: Colors, operator: &str) -> Color {
        Color {
            colors,
            operator: operator.to_string(),
        }
    }
    pub fn eq(colors: Colors) -> Color {
        Color::internal_new(colors, ":")
    }
    pub fn less(colors: Colors) -> Color {
        Color::internal_new(colors, "<")
    }
    pub fn less_eq(colors: Colors) -> Color {
        Color::internal_new(colors, "<=")
    }
    pub fn greater(colors: Colors) -> Color {
        Color::internal_new(colors, ">")
    }
    pub fn greater_eq(colors: Colors) -> Color {
        Color::internal_new(colors, ">=")
    }
    pub fn not(colors: Colors) -> Color {
        Color::internal_new(colors, "!")
    }
}

impl SearchBuilderTrait for Color {
    fn stringify(&self) -> String {
        format!("c{}{}", self.operator, self.colors.iter().map(|x| x.to_string()).collect::<String>())
    }
}
