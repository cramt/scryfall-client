pub mod border_color;
pub mod rarity;

use crate::card::color::Colors;
use crate::card::legality::Legality;
use crate::card::mana_cost::ManaCostCollection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::card::rarity::Rarity;
use crate::card::border_color::BorderColor;

pub mod color;
pub mod legality;
pub mod mana_cost;

//https://scryfall.com/docs/api/cards
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Card {
    //Core Card Fields
    pub arena_id: Option<u32>,
    pub id: String,
    pub lang: String,
    pub mtgo_id: Option<u32>,
    pub mtgo_foil_id: Option<u32>,
    pub multiverse_ids: Vec<u32>,
    pub tcgplayer_id: Option<u32>,
    pub object: String,
    pub oracle_id: String,
    pub prints_search_uri: String,
    pub rulings_uri: String,
    pub scryfall_uri: String,
    pub uri: String,

    //Gameplay Fields
    pub all_parts: Option<Vec<RelatedCard>>,
    pub card_faces: Option<Vec<CardFace>>,
    pub cmc: f32,
    pub color_identity: Colors,
    pub color_indicator: Option<Colors>,
    pub colors: Option<Colors>,
    pub edhrec_rank: Option<u32>,
    pub foil: bool,
    pub hand_modifier: Option<String>,
    pub keywords: Vec<String>,
    pub layout: String,
    pub legalities: HashMap<String, Legality>,
    pub life_modifier: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: ManaCostCollection,
    pub name: String,
    pub nonfoil: bool,
    pub oracle_text: Option<String>,
    pub oversized: bool,
    pub power: Option<String>,
    pub produced_mana: Option<Colors>,
    pub reserved: bool,
    pub toughness: Option<String>,
    pub type_line: String,

    //print fields
    pub artist: Option<String>,
    pub booster: bool,
    pub border_color: BorderColor,
    pub card_back_id: String,
    pub collector_number: String,
    pub content_warning: Option<bool>,
    pub digital: bool,
    pub flavor_name: Option<String>,
    pub flavor_text: Option<String>,
    pub frame_effects: Option<Vec<String>>,
    pub frame: String,
    pub full_art: bool,
    pub games: Vec<String>,
    pub highres_image: bool,
    pub illustration_id: Option<String>,
    pub image_uris: Option<HashMap<String, String>>,
    pub prices: HashMap<String, Option<String>>,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub promo: bool,
    pub promo_types: Option<Vec<String>>,
    pub purchase_uris: HashMap<String, String>,
    pub rarity: Rarity,
    pub related_uris: HashMap<String, String>,
    pub released_at: String,
    //TODO: make this a date object
    pub reprint: bool,
    pub scryfall_set_uri: String,
    pub set_name: String,
    pub set_search_uri: String,
    pub set_type: String,
    pub set_uri: String,
    pub set: String,
    pub story_spotlight: bool,
    pub textless: bool,
    pub variation: bool,
    pub variation_of: Option<String>,
    pub watermark: Option<String>,
    //TODO: preview.previewed_at, preview.source_uri, preview.source, which they failed at NOT PUTTING AN ILLEGAL JSON CHARACTER IN
}

//https://scryfall.com/docs/api/cards#related-card-objects
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelatedCard {
    pub id: String,
    pub object: String,
    pub component: String,
    pub name: String,
    pub type_line: String,
    pub uri: String,
}

//https://scryfall.com/docs/api/cards#card-face-objects
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardFace {
    pub artist: Option<String>,
    pub color_indicator: Option<Colors>,
    pub colors: Option<Colors>,
    pub flavor_text: Option<String>,
    pub illustration_id: Option<String>,
    pub image_uris: Option<HashMap<String, String>>,
    pub loyalty: Option<String>,
    pub mana_cost: String,
    pub name: String,
    pub object: String,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub toughness: Option<String>,
    pub type_line: String,
    pub watermark: Option<String>,
}
