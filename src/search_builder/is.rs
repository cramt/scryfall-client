use crate::search_builder::SearchBuilderTrait;
use inflector::cases::snakecase::to_snake_case;

macro_rules! is_struct {
    ($name:ident) => (
        pub struct $name;

        impl SearchBuilderTrait for $name {
            fn stringify(&self) -> String {
                format!("is:{}", to_snake_case(&stringify!($name)[2..])).to_string()
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

is_struct!(IsHybrid);
is_struct!(IsPhyrexian);
is_struct!(IsSplit);
is_struct!(IsFlip);
is_struct!(IsTransform);
is_struct!(IsMeld);
is_struct!(IsLeveler);
is_struct!(IsSpell);
is_struct!(IsPermanent);
is_struct!(IsHistoric);
is_struct!(IsModal);
is_struct!(IsVanilla);
is_struct!(IsFrenchvanilla);
is_struct!(IsFunny);
is_struct!(IsBooster);
is_struct!(IsPlaneswalkerDeck);
is_struct!(IsLeague);
is_struct!(IsBuyABox, buyabox);
is_struct!(IsGiftBox, giftbox);
is_struct!(IsIntroPack);
is_struct!(IsGameDay, gameday);
is_struct!(IsPreRelease, prerelease);
is_struct!(IsRelease);
is_struct!(IsDateStamped, datestamped);
is_struct!(IsCommander);
is_struct!(IsBrawler);
is_struct!(IsCompanion);
is_struct!(IsReserved);
is_struct!(IsReprint);
is_struct!(IsNew);
is_struct!(IsOld);
is_struct!(IsHires);
is_struct!(IsFoil);
is_struct!(IsFull);
is_struct!(IsNonfoil);
is_struct!(IsDigital);
is_struct!(IsPromo);
is_struct!(IsSpotlight);
is_struct!(IsUnique);
is_struct!(IsDual);
is_struct!(IsColorShifted, colorshifted);
is_struct!(IsBikeland);
is_struct!(IsBounceland);
is_struct!(IsCanopyland);
is_struct!(IsCheckland);
is_struct!(IsFastland);
is_struct!(IsFetchland);
is_struct!(IsFilterland);
is_struct!(IsGainland);
is_struct!(IsPainland);
is_struct!(IsScryland);
is_struct!(IsShadowland);
is_struct!(IsShockland);
is_struct!(IsStorageland);
is_struct!(IsCreatureland);
is_struct!(IsTriland);
is_struct!(IsTangoland);
is_struct!(IsMasterpiece);
