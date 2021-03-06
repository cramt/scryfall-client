use serde::de::{Error, Visitor};
use serde::export::Formatter;
use serde::ser::{Serialize, Serializer};
use serde::{Deserialize, Deserializer};
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ManaCost {
    Green,
    White,
    Blue,
    Black,
    Red,
    Tap,
    Untap,
    EnergyCounter,
    Planeswalker,
    Chaos,
    AcornCounter,
    XGeneric(char),
    Generic(u32),
    InfiniteGeneric,
    Phyrexian,
    Or(Rc<ManaCost>, Rc<ManaCost>),
    Colorless,
    Half(Rc<ManaCost>),
    Snow,
}

impl ManaCost {
    fn from(s: String) -> Result<ManaCost, &'static str> {
        if s.contains("/") {
            let raw_values = s
                .split("/")
                .map(|x| ManaCost::from(x.to_string()))
                .collect::<Vec<Result<ManaCost, &str>>>();
            let mut values = vec![];
            for v in raw_values {
                match v {
                    Ok(cost) => values.push(cost),
                    Err(err) => return Err(&err),
                }
            }

            return Ok(ManaCost::chained_or(values));
        }
        let chars = s.chars().collect::<Vec<char>>();
        if chars[0] == 'H' {
            let result = ManaCost::from(String::from(&s[1..]));
            return match result {
                Ok(cost) => Ok(ManaCost::Half(Rc::new(cost))),
                Err(err) => Err(err),
            };
        }
        Ok(match s.as_str() {
            "G" => ManaCost::Green,
            "R" => ManaCost::Red,
            "U" => ManaCost::Blue,
            "B" => ManaCost::Black,
            "W" => ManaCost::White,
            "T" => ManaCost::Tap,
            "Q" => ManaCost::Untap,
            "E" => ManaCost::EnergyCounter,
            "PW" => ManaCost::Planeswalker,
            "CHAOS" => ManaCost::Chaos,
            "A" => ManaCost::AcornCounter,
            "½" => ManaCost::Half(Rc::new(ManaCost::Generic(1))),
            "∞" => ManaCost::InfiniteGeneric,
            "P" => ManaCost::Phyrexian,
            "C" => ManaCost::Colorless,
            "S" => ManaCost::Snow,
            _value => match _value.parse::<u32>() {
                Err(_) => ManaCost::XGeneric(_value.chars().into_iter().nth(0).unwrap()),
                Ok(n) => ManaCost::Generic(n),
            },
        })
    }

    pub fn chained_or(v: Vec<ManaCost>) -> ManaCost {
        v.into_iter()
            .fold(None, |acc, x| match acc {
                None => Some(x),
                Some(acc) => Some(ManaCost::Or(Rc::new(acc), Rc::new(x))),
            })
            .unwrap()
    }
}

impl ToString for ManaCost {
    fn to_string(&self) -> String {
        match self.clone() {
            ManaCost::Green => "G".to_string(),
            ManaCost::White => "W".to_string(),
            ManaCost::Blue => "U".to_string(),
            ManaCost::Black => "B".to_string(),
            ManaCost::Red => "R".to_string(),
            ManaCost::Tap => "T".to_string(),
            ManaCost::Untap => "Q".to_string(),
            ManaCost::EnergyCounter => "E".to_string(),
            ManaCost::Planeswalker => "PW".to_string(),
            ManaCost::Chaos => "CHAOS".to_string(),
            ManaCost::AcornCounter => "A".to_string(),
            ManaCost::XGeneric(c) => c.to_string(),
            ManaCost::Generic(n) => format!("{}", n),
            ManaCost::InfiniteGeneric => "∞".to_string(),
            ManaCost::Phyrexian => "P".to_string(),
            ManaCost::Or(lhs, rhs) => format!("{}/{}", lhs.to_string(), rhs.to_string()),
            ManaCost::Colorless => "C".to_string(),
            ManaCost::Half(cost) => match cost.as_ref().clone() {
                ManaCost::Generic(n) => {
                    if n == 1 {
                        "½".to_string()
                    } else {
                        format!("{}", n / 2)
                    }
                }
                _cost => format!("H{}", _cost.to_string()),
            },
            ManaCost::Snow => "S".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ManaCostCollection {
    pub costs: Vec<Vec<ManaCost>>,
}

impl Default for ManaCostCollection {
    fn default() -> Self {
        ManaCostCollection::new(vec![])
    }
}

impl ToString for ManaCostCollection {
    fn to_string(&self) -> String {
        self.costs
            .iter()
            .map(|x| {
                x.into_iter()
                    .map(|y| format!("{{{}}}", y.to_string()))
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join(" // ")
    }
}

impl ManaCostCollection {
    pub fn new(costs: Vec<Vec<ManaCost>>) -> ManaCostCollection {
        ManaCostCollection { costs }
    }

    pub fn from(v: String) -> Result<ManaCostCollection, &'static str> {
        if v.is_empty() {
            return Ok(ManaCostCollection::default());
        }
        fn individual_parse(v: String) -> Result<Vec<ManaCost>, &'static str> {
            let unparsed = String::from(&v[1..v.len() - 1])
                .split("}{")
                .map(|s| ManaCost::from(s.to_string()))
                .collect::<Vec<Result<ManaCost, &str>>>();
            if let Some(Err(err)) = unparsed.iter().find(|x| x.is_err()) {
                return Err(err);
            }
            let parsed = unparsed
                .into_iter()
                .map(|x| x.unwrap())
                .collect::<Vec<ManaCost>>();
            Ok(parsed)
        }
        let results = v
            .split(" // ")
            .map(|x| individual_parse(x.to_string()))
            .collect::<Vec<Result<Vec<ManaCost>, &str>>>();
        for result in &results {
            if let Err(err) = result {
                return Err(err);
            }
        }
        Ok(ManaCostCollection::new(
            results.into_iter().map(|x| x.unwrap()).collect(),
        ))
    }
}

impl Serialize for ManaCostCollection {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

struct ManaCostCollectionVisitor;

impl<'de> Visitor<'de> for ManaCostCollectionVisitor {
    type Value = ManaCostCollection;

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
        match ManaCostCollection::from(v) {
            Ok(cost) => Ok(cost),
            Err(error) => Err(Error::missing_field(error)),
        }
    }
}

impl<'de> Deserialize<'de> for ManaCostCollection {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ManaCostCollectionVisitor)
    }
}

impl From<ManaCostCollection> for Vec<Vec<ManaCost>> {
    fn from(x: ManaCostCollection) -> Self {
        x.costs
    }
}
