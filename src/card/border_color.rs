use serde::de::{Error, Visitor};
use serde::export::Formatter;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Copy, Clone, Debug)]
pub enum BorderColor {
    Black,
    Borderless,
    Gold,
    Silver,
    White,
}

impl ToString for BorderColor {
    fn to_string(&self) -> String {
        match self {
            BorderColor::Black => "black",
            BorderColor::Borderless => "borderless",
            BorderColor::Gold => "gold",
            BorderColor::Silver => "silver",
            BorderColor::White => "white",
        }.to_string()
    }
}

impl BorderColor {
    pub fn from(s: String) -> Result<BorderColor, ()> {
        match s.as_str() {
            "black" => Ok(BorderColor::Black),
            "borderless" => Ok(BorderColor::Borderless),
            "gold" => Ok(BorderColor::Gold),
            "silver" => Ok(BorderColor::Silver),
            "white" => Ok(BorderColor::White),
            _ => Err(()),
        }
    }
}

impl Serialize for BorderColor {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

struct BorderColorVisitor;

impl<'de> Visitor<'de> for BorderColorVisitor {
    type Value = BorderColor;

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
        match BorderColor::from(v) {
            Ok(cost) => Ok(cost),
            Err(_) => Err(Error::missing_field("couldnt parse mtg color")),
        }
    }
}

impl<'de> Deserialize<'de> for BorderColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_str(BorderColorVisitor)
    }
}
