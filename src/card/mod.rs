use crate::card::color::{Color, Colors};
use std::collections::{HashMap, VecDeque};
use crate::card::mana_cost::ManaCostCollection;
use crate::card::legality::Legality;
use serde::{Serialize, Deserialize};

pub mod mana_cost;
pub mod legality;
pub mod color;

//https://scryfall.com/docs/api/cards
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Card {
    //Core Card Fields
    arena_id: Option<u32>,
    id: String,
    lang: String,
    mtgo_id: Option<u32>,
    mtgo_foil_id: Option<u32>,
    multiverse_ids: Vec<u32>,
    tcgplayer_id: Option<u32>,
    object: String,
    oracle_id: String,
    prints_search_uri: String,
    rulings_uri: String,
    scryfall_uri: String,
    uri: String,

    //Gameplay Fields
    all_parts: Option<Vec<RelatedCard>>,
    card_faces: Option<Vec<CardFace>>,
    cmc: f32,
    color_identity: Colors,
    color_indicator: Option<Colors>,
    colors: Option<Colors>,
    edhrec_rank: Option<u32>,
    foil: bool,
    hand_modifier: Option<String>,
    keywords: Vec<String>,
    layout: String,
    legalities: HashMap<String, Legality>,
    life_modifier: Option<String>,
    loyalty: Option<String>,
    mana_cost: ManaCostCollection,
    name: String,
    nonfoil: bool,
    oracle_text: Option<String>,
    oversized: bool,
    power: Option<String>,
    produced_mana: Option<Colors>,
    reserved: bool,
    toughness: Option<String>,
    type_line: String,

    //print fields
    artist: Option<String>,
    booster: bool,
    border_color: String,
    //TODO: only valid strings here are black, borderless, gold, silver, or white, make it an enum
    card_back_id: String,
    collector_number: String,
    content_warning: Option<bool>,
    digital: bool,
    flavor_name: Option<String>,
    flavor_text: Option<String>,
    frame_effects: Option<Vec<String>>,
    frame: String,
    full_art: bool,
    games: Vec<String>,
    highres_image: bool,
    illustration_id: Option<String>,
    image_uris: Option<HashMap<String, String>>,
    prices: HashMap<String, Option<String>>,
    printed_name: Option<String>,
    printed_text: Option<String>,
    printed_type_line: Option<String>,
    promo: bool,
    promo_types: Option<Vec<String>>,
    purchase_uris: HashMap<String, String>,
    rarity: String,
    //TODO: make this an enum, options are common, uncommon, rare, or mythic
    related_uris: HashMap<String, String>,
    released_at: String,
    //TODO: make this a date object
    reprint: bool,
    scryfall_set_uri: String,
    set_name: String,
    set_search_uri: String,
    set_type: String,
    set_uri: String,
    set: String,
    story_spotlight: bool,
    textless: bool,
    variation: bool,
    variation_of: Option<String>,
    watermark: Option<String>,
    //TODO: preview.previewed_at, preview.source_uri, preview.source, which they failed at NOT PUTTING AN ILLEGAL JSON CHARACTER IN
}

//https://scryfall.com/docs/api/cards#related-card-objects
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelatedCard {
    id: String,
    object: String,
    component: String,
    name: String,
    type_line: String,
    uri: String,
}

//https://scryfall.com/docs/api/cards#card-face-objects
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardFace {
    artist: Option<String>,
    color_indicator: Option<Colors>,
    colors: Option<Colors>,
    flavor_text: Option<String>,
    illustration_id: Option<String>,
    image_uris: Option<HashMap<String, String>>,
    loyalty: Option<String>,
    mana_cost: String,
    name: String,
    object: String,
    oracle_text: Option<String>,
    power: Option<String>,
    printed_name: Option<String>,
    printed_text: Option<String>,
    printed_type_line: Option<String>,
    toughness: Option<String>,
    type_line: String,
    watermark: Option<String>,
}
