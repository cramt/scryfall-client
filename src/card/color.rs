use serde::de::{Error, Visitor};
use serde::export::Formatter;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Green,
    White,
    Blue,
    Black,
    Red,
    Colorless,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::Green => "G",
            Color::White => "W",
            Color::Black => "B",
            Color::Blue => "U",
            Color::Red => "R",
            Color::Colorless => "C",
        }
        .to_string()
    }
}

impl Color {
    pub fn from(s: String) -> Result<Color, ()> {
        match s.as_str() {
            "G" => Ok(Color::Green),
            "W" => Ok(Color::White),
            "B" => Ok(Color::Black),
            "U" => Ok(Color::Blue),
            "R" => Ok(Color::Red),
            "C" => Ok(Color::Colorless),
            _ => Err(()),
        }
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

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
        match Color::from(v) {
            Ok(cost) => Ok(cost),
            Err(_) => Err(Error::missing_field("couldnt parse mtg color")),
        }
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ColorVisitor)
    }
}

pub type Colors = Vec<Color>;
