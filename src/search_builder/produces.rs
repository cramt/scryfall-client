use crate::card::color::Colors;
use crate::search_builder::SearchBuilderTrait;

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
}

impl SearchBuilderTrait for Produces {
    fn stringify(&self) -> String {
        format!(
            "produces{}{}",
            self.operator,
            self.colors
                .iter()
                .map(|x| x.to_string())
                .collect::<String>()
        )
    }
}

crate::equality_operator_implementer!(Produces, Colors);
