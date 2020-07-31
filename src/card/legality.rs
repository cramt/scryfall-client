use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Visitor, Error};
use serde::export::Formatter;

#[derive(Copy, Clone, Debug)]
pub enum Legality {
    Legal,
    NotLegal,
    Banned,
    Restricted,
}

impl ToString for Legality {
    fn to_string(&self) -> String {
        match self {
            Legality::Legal => "legal",
            Legality::Banned => "banned",
            Legality::NotLegal => "not_legal",
            Legality::Restricted => "restricted"
        }.to_string()
    }
}

impl Legality {
    pub fn from(s: String) -> Result<Legality, ()> {
        match s.as_str() {
            "legal" => Ok(Legality::Legal),
            "banned" => Ok(Legality::Banned),
            "not_legal" => Ok(Legality::NotLegal),
            "restricted" => Ok(Legality::Restricted),
            _ => Err(())
        }
    }
}

impl Serialize for Legality {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str(self.to_string().as_str())
    }
}

struct LegalityVisitor;

impl<'de> Visitor<'de> for LegalityVisitor {
    type Value = Legality;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("standard mtg cost formatting")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
        E: serde::de::Error, {
        self.visit_string(v.to_string())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where
        E: serde::de::Error, {
        match Legality::from(v) {
            Ok(cost) => Ok(cost),
            Err(_) => {
                Err(Error::missing_field("couldnt parse mtg color"))
            }
        }
    }
}

impl<'de> Deserialize<'de> for Legality {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_str(LegalityVisitor)
    }
}
