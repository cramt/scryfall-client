use crate::search_builder::SearchBuilderTrait;
use crate::card::color::Colors;

pub trait ColorArgument: ToString {}

impl ColorArgument for u32 {}

impl ColorArgument for Colors {}

pub struct Color {
    colors: String,
    operator: String,
}

impl Color {
    fn internal_new<T: ColorArgument>(colors: T, operator: &str) -> Color {
        Color {
            colors: colors.to_string(),
            operator: operator.to_string(),
        }
    }
}

impl SearchBuilderTrait for Color {
    fn stringify(&self) -> String {
        format!("c{}{}", self.operator, self.colors)
    }
}

crate::equality_operator_implementer_trait!(Color, ColorArgument);
