use crate::card::color::Colors;
use crate::search_builder::SearchBuilderTrait;

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
}

impl SearchBuilderTrait for ColorIdentity {
    fn stringify(&self) -> String {
        format!(
            "id{}{}",
            self.operator,
            self.colors
                .iter()
                .map(|x| x.to_string())
                .collect::<String>()
        )
    }
}

crate::equality_operator_implementer!(ColorIdentity, Colors);
