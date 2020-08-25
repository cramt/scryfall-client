use serde::export::Formatter;

pub mod flavor_text;
pub mod name;
pub mod watermark;
pub mod artist;
pub mod block;
pub mod collector_number;
pub mod set;
pub mod include_extras;
pub mod cmc;
pub mod rarity;
pub mod loyalty;
pub mod toughness;
pub mod power;
pub mod is;
pub mod produces;
pub mod not;
pub mod color_identity;
pub mod devotion;
pub mod color;
pub mod mana;
pub mod oracle_text;
pub mod or;
pub mod magic_type;
pub mod format;

pub trait SearchBuilderTrait {
    fn stringify(&self) -> String;
}

impl std::fmt::Display for Box<dyn SearchBuilderTrait> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
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

    pub fn all_cards() -> Builder {
        Builder::new()
            .add(color::Color::greater_eq(0))
            .add(include_extras::IncludeExtras)
    }
}

impl SearchBuilderTrait for Builder {
    fn stringify(&self) -> String {
        self.children.iter().map(|x| x.stringify()).collect::<Vec<String>>().join(" ")
    }
}

#[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! equality_operator_implementer {
        ($name:ident, $t:tt) => (
            impl $name {
                pub fn eq(value: $t) -> $name {
                    $name::internal_new(value, "=")
                }
                pub fn less(value: $t) -> $name {
                    $name::internal_new(value, "<")
                }
                pub fn less_eq(value: $t) -> $name {
                    $name::internal_new(value, "<=")
                }
                pub fn greater(value: $t) -> $name {
                    $name::internal_new(value, ">")
                }
                pub fn greater_eq(value: $t) -> $name {
                    $name::internal_new(value, ">=")
                }
                pub fn not(value: $t) -> $name {
                    $name::internal_new(value, "!")
                }
            }
        );
        ($name:ident) => (
            crate::equality_operator_implementer!($name, u32);
        )
    }

    #[macro_export]
    macro_rules! equality_operator_implementer_trait {
        ($name:ident, $t:tt) => (
            impl $name {
                pub fn eq<T: $t>(value: T) -> $name {
                    $name::internal_new(value, "=")
                }
                pub fn less<T: $t>(value: T) -> $name {
                    $name::internal_new(value, "<")
                }
                pub fn less_eq<T: $t>(value: T) -> $name {
                    $name::internal_new(value, "<=")
                }
                pub fn greater<T: $t>(value: T) -> $name {
                    $name::internal_new(value, ">")
                }
                pub fn greater_eq<T: $t>(value: T) -> $name {
                    $name::internal_new(value, ">=")
                }
                pub fn not<T: $t>(value: T) -> $name {
                    $name::internal_new(value, "!")
                }
            }
        )
    }
}
