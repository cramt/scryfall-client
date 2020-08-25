use crate::decklist::unpopulated_decklist::UnpopulatedDecklist;
use std::collections::VecDeque;
use std::ops::Index;

impl UnpopulatedDecklist {
    pub fn parse_raw<S: ToString>(data: S) -> Option<Self> {
        let mut main = vec![];
        let mut sideboard = vec![];
        let mut writing = true;
        for line in data.to_string().lines() {
            if line.is_empty() {
                continue;
            }
            if line == "Sideboard:" {
                writing = false;
                continue;
            }
            let mut parts = line.split(" ").collect::<VecDeque<&str>>();
            let amount = match match parts.pop_front() {
                Some(str) => str,
                _ => return None
            }.parse::<u16>() {
                Ok(amount) => amount,
                _ => return None
            };
            let name: String = parts.into_iter().collect::<Vec<&str>>().join(" ").to_string();
            let entry = (amount, name);
            if writing {
                main.push(entry)
            } else {
                sideboard.push(entry)
            }
        };
        Some(Self {
            main,
            sideboard,
        })
    }

    pub fn parse_arena<S: ToString>(data: S) -> Option<Self> {
        let mut main = vec![];
        let mut sideboard = vec![];
        let mut writing = true;
        for line in data.to_string().lines() {
            if line.is_empty() {
                writing = false;
                continue;
            }
            let mut parts = line.split(" ").collect::<VecDeque<&str>>();
            let amount = match match parts.pop_front() {
                Some(str) => str,
                _ => return None
            }.parse::<u16>() {
                Ok(amount) => amount,
                _ => return None
            };
            let name: String = parts.into_iter().collect::<Vec<&str>>().join(" ").to_string();
            let entry = (amount, name);
            if writing {
                main.push(entry)
            } else {
                sideboard.push(entry)
            }
        };
        Some(Self {
            main,
            sideboard,
        })
    }
    pub fn parse_csv<S: ToString>(data: S) -> Option<Self> {
        let mut main = vec![];
        let mut sideboard = vec![];
        for result in csv::Reader::from_reader(data.to_string().as_bytes()).records() {
            let record = match result {
                Ok(record) => record,
                _ => return None
            };
            if record.len() != 3 {
                return None;
            }
            if record.index(0) == "Count" {
                continue;
            }
            let amount = match record.index(0).parse::<u16>() {
                Ok(amount) => amount,
                _ => return None
            };
            let name = record.index(1);
            let position = record.index(2);
            let entry = (amount, name.to_string());
            match position {
                "main" => main.push(entry),
                "sideboard" => sideboard.push(entry),
                _ => return None
            }
        }
        Some(Self {
            main,
            sideboard,
        })
    }
}
