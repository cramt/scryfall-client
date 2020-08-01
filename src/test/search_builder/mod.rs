#[cfg(test)]
mod search_builder {
    use crate::search_builder::{Builder, SearchBuilderTrait};
    use crate::search_builder::format::Format;
    use crate::search_builder::or::Or;
    use crate::search_builder::magic_type::MagicType;
    use crate::search_builder::not::Not;
    use crate::search_builder::cmc::Cmc;

    #[test]
    fn empty() {
        assert_eq!(Builder::new().stringify(), "");
    }

    #[test]
    fn format() {
        assert_eq!(Builder::new()
                       .add(Format::new("standard"))
                       .stringify(), "f:standard");
    }

    #[test]
    fn magic_type() {
        assert_eq!(Builder::new()
                       .add(MagicType::new("legendary"))
                       .stringify(), "t:legendary");
    }

    #[test]
    fn or() {
        assert_eq!(Builder::new()
                       .add(Or::new(
                           Format::new("standard"),
                           MagicType::new("legendary")
                       ))
                       .stringify(), "(f:standard or t:legendary)");
    }

    #[test]
    fn not() {
        assert_eq!(Builder::new()
                       .add(Not::new(MagicType::new("legendary")))
                       .stringify(), "-t:legendary");
    }

    #[test]
    fn cmc() {
        assert_eq!(Builder::new()
                       .add(Cmc::even())
                       .stringify(), "cmc:even");
    }
}
