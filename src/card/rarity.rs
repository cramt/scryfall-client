use serde::de::{Error, Visitor};
use serde::export::Formatter;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Copy, Clone, Debug)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Mythic
}

impl ToString for Rarity {
    fn to_string(&self) -> String {
        match self {
            Rarity::Common => "common",
            Rarity::Uncommon => "uncommon",
            Rarity::Rare => "rare",
            Rarity::Mythic => "mythic",
        }
        .to_string()
    }
}

impl Rarity {
    pub fn from(s: String) -> Result<Rarity, ()> {
        match s.as_str() {
            "common" => Ok(Rarity::Common),
            "uncommon" => Ok(Rarity::Uncommon),
            "rare" => Ok(Rarity::Rare),
            "mythic" => Ok(Rarity::Mythic),
            _ => Err(()),
        }
    }
}

impl Serialize for Rarity {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

struct RarityVisitor;

impl<'de> Visitor<'de> for RarityVisitor {
    type Value = Rarity;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("standard mtg cost formatting")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(v.to_string())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Rarity::from(v) {
            Ok(cost) => Ok(cost),
            Err(_) => Err(Error::missing_field("couldnt parse mtg color")),
        }
    }
}

impl<'de> Deserialize<'de> for Rarity {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RarityVisitor)
    }
}
