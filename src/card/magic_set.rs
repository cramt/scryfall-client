use crate::card::magic_date::MagicDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MagicSet {
    pub id: String,
    pub code: String,
    pub mtgo_code: Option<String>,
    pub tcgplayer_id: Option<u32>,
    pub name: String,
    pub set_type: String, //TODO: make set_type and enum
    pub released_at: Option<MagicDate>,
    pub block_code: Option<String>,
    pub block: Option<String>,
    pub parent_set_code: Option<String>,
    pub card_count: u32,
    pub printed_size: Option<u32>,
    pub digital: bool,
    pub foil_only: bool,
    pub nonfoil_only: bool,
    pub scryfall_uri: String,
    pub uri: String,
    pub icon_svg_uri: String,
}
