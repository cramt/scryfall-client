use std::collections::HashMap;
use crate::client::{Client, ClientError};
use crate::decklist::populated_decklist::PopulatedDecklist;

pub struct UnpopulatedDecklist {
    pub main: Vec<(u16, String)>,
    pub sideboard: Vec<(u16, String)>,
}

impl UnpopulatedDecklist {
    pub async fn populate(&self) -> Result<PopulatedDecklist, ClientError> {
        let map = vec![
            self.main.iter().map(|(amount, name)| (name.clone(), (amount.clone(), true))).collect::<Vec<(String, (u16, bool))>>(),
            self.sideboard.iter().map(|(amount, name)| (name.clone(), (amount.clone(), false))).collect::<Vec<(String, (u16, bool))>>()
        ].into_iter().flat_map(|s| s.into_iter()).collect::<HashMap<String, (u16, bool)>>();
        let result = Client.collection_by_names(map.keys().map(|x| x.clone()).collect()).await?;
        let (main, sideboard) = result.into_iter().fold((vec![], vec![]), |(mut main, mut sideboard), x| {
            let values = map.get(&x.name);
            if let Some((amount, board)) = values {
                let entry = (amount.clone(), x);
                if *board {
                    main.push(entry)
                } else {
                    sideboard.push(entry)
                }
            }
            (main, sideboard)
        });
        Ok(PopulatedDecklist {
            main,
            sideboard,
        })
    }
}
