use chrono::NaiveDate;
use serde::de::Visitor;
use serde::export::Formatter;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct MagicDate(NaiveDate);

impl Deref for MagicDate {
    type Target = NaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToString for MagicDate {
    fn to_string(&self) -> String {
        self.deref().to_string()
    }
}

impl MagicDate {
    pub fn parse<S: AsRef<str>>(str: S) -> chrono::format::ParseResult<MagicDate> {
        Ok(MagicDate(NaiveDate::parse_from_str(
            str.as_ref(),
            "%Y-%m-%d",
        )?))
    }
}

impl Serialize for MagicDate {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

struct MagicDateVisitor;

impl<'de> Visitor<'de> for MagicDateVisitor {
    type Value = MagicDate;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("standard mtg date formatting")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        MagicDate::parse(v).map_err(|_| serde::de::Error::missing_field("couldnt parse date"))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(v.as_str())
    }
}

impl<'de> Deserialize<'de> for MagicDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(MagicDateVisitor)
    }
}
