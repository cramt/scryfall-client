use std::collections::HashMap;
use crate::client::{Client, ClientError};
use crate::decklist::populated_decklist::PopulatedDecklist;

pub struct UnpopulatedDecklist {
    pub main: Vec<(u16, String)>,
    pub sideboard: Vec<(u16, String)>,
}

impl UnpopulatedDecklist {
    pub async fn populate(&self) -> Result<PopulatedDecklist, ClientError> {
        let mut map: HashMap<String, (u16, u16)> = HashMap::new();
        let default = (0u16, 0u16);
        for entry in &self.main {
            let mut existing = map.get(entry.1.as_str()).unwrap_or(&default).clone();
            existing.0 += entry.0;
            map.insert(entry.1.clone(), existing);
        }
        for entry in &self.sideboard {
            let mut existing = map.get(entry.1.as_str()).unwrap_or(&default).clone();
            existing.1 += entry.0;
            map.insert(entry.1.clone(), existing);
        }
        let result = Client.collection_by_names(map.keys().map(|x| x.clone()).collect()).await?;
        let (main, sideboard) = result.into_iter().fold((vec![], vec![]), |(mut main, mut sideboard), x| {
            let possible_names = x.name.split(" // ").chain(vec![x.name.as_str()].into_iter());
            if let Some(values) = possible_names.map(|x| map.get(x)).find(|x| x.is_some()) {
                if let Some((main_amount, sideboard_amount)) = values {
                    let main_amount = main_amount.clone();
                    let sideboard_amount = sideboard_amount.clone();
                    if main_amount != 0 {
                        main.push((main_amount, x.clone()))
                    }
                    if sideboard_amount != 0 {
                        sideboard.push((sideboard_amount, x.clone()))
                    }
                }
            }
            (main, sideboard)
        });
        Ok(PopulatedDecklist {
            main,
            sideboard,
        })
    }

    pub fn to_raw(&self) -> String {
        fn join(v: &Vec<(u16, String)>) -> String {
            v.into_iter().map(|(amount, name)| format!("{} {}", amount, name).to_string()).collect::<Vec<String>>().join("\r\n")
        }
        format!("{}\r\nSideboard:\r\n{}", join(&self.main), join(&self.sideboard))
    }
}
