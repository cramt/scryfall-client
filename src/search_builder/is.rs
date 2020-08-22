use crate::search_builder::SearchBuilderTrait;
use inflector::cases::snakecase::to_snake_case;

macro_rules! is_struct {
    ($name:ident) => (
        pub struct $name;

        impl SearchBuilderTrait for $name {
            fn stringify(&self) -> String {
                format!("is:{}", to_snake_case(stringify!($name))).to_string()
            }
        }
    );

    ($name:ident, $second:ident) => (
        pub struct $name;

        impl SearchBuilderTrait for $name {
            fn stringify(&self) -> String {
                format!("is:{}", stringify!($second)).to_string()
            }
        }
    )
}

is_struct!(Hybrid);
is_struct!(Phyrexian);
is_struct!(Split);
is_struct!(Flip);
is_struct!(Transform);
is_struct!(Meld);
is_struct!(Leveler);
is_struct!(Spell);
is_struct!(Permanent);
is_struct!(Historic);
is_struct!(Modal);
is_struct!(Vanilla);
is_struct!(Frenchvanilla);
is_struct!(Funny);
is_struct!(Booster);
is_struct!(PlaneswalkerDeck);
is_struct!(League);
is_struct!(BuyABox, buyabox);
is_struct!(GiftBox, giftbox);
is_struct!(IntroPack);
is_struct!(GameDay, gameday);
is_struct!(PreRelease, prerelease);
is_struct!(Release);
is_struct!(DateStamped, datestamped);
is_struct!(Commander);
is_struct!(Brawler);
is_struct!(Companion);
is_struct!(Reserved);
is_struct!(Reprint);
is_struct!(New);
is_struct!(Old);
is_struct!(Hires);
is_struct!(Foil);
is_struct!(Full);
is_struct!(Nonfoil);
is_struct!(Digital);
is_struct!(Promo);
is_struct!(Spotlight);
is_struct!(Unique);
is_struct!(Dual);
is_struct!(ColorShifted, colorshifted);
is_struct!(Bikeland);
is_struct!(Bounceland);
is_struct!(Canopyland);
is_struct!(Checkland);
is_struct!(Fastland);
is_struct!(Fetchland);
is_struct!(Filterland);
is_struct!(Gainland);
is_struct!(Painland);
is_struct!(Scryland);
is_struct!(Shadowland);
is_struct!(Shockland);
is_struct!(Storageland);
is_struct!(Creatureland);
is_struct!(Triland);
is_struct!(Tangoland);
is_struct!(Masterpiece);
