use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CardIdentifier {
    key: String,
    value: String,
}

impl CardIdentifier {
    pub fn id<S: ToString>(uuid: S) -> Self {
        Self {
            key: String::from("id"),
            value: uuid.to_string(),
        }
    }
    pub fn mtgo_id(id: u64) -> Self {
        Self {
            key: String::from("mtgo_id"),
            value: id.to_string(),
        }
    }
    pub fn multiverse_id(id: u64) -> Self {
        Self {
            key: String::from("multiverse_id"),
            value: id.to_string(),
        }
    }
    pub fn oracle_id<S: ToString>(uuid: S) -> Self {
        Self {
            key: String::from("oracle_id"),
            value: uuid.to_string(),
        }
    }
    pub fn illustration_id<S: ToString>(uuid: S) -> Self {
        Self {
            key: String::from("illustration_id"),
            value: uuid.to_string(),
        }
    }
    pub fn name<S: ToString>(name: S) -> Self {
        Self {
            key: String::from("name"),
            value: name.to_string(),
        }
    }

    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(self.key.clone(), self.value.clone());
        map
    }
}
