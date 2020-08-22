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
}

impl SearchBuilderTrait for Color {
    fn stringify(&self) -> String {
        format!("c{}{}", self.operator, self.colors.iter().map(|x| x.to_string()).collect::<String>())
    }
}

crate::equality_operator_implementer!(Color, Colors);
