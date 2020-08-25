mod name;
mod search;
mod client_paths;

use crate::card::Card;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, Duration};
use std::sync::Mutex;
use std::thread::sleep;
use std::ops::Deref;
use lazy_static::*;

#[derive(Debug)]
pub enum ClientError {
    Request(reqwest::Error),
    TextDecode(reqwest::Error),
    Deserialize(serde_json::Error),
    Scryfall(ScryfallError),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScryfallError {
    pub object: String,
    pub code: String,
    pub status: u16,
    pub details: String,
}

pub struct Client;

lazy_static! {
    static ref LAST_WAIT_FOR_RATE_LIMIT_CALL: Mutex<u128> = Mutex::new(0);
}

pub fn get_unix_time() -> u128 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
}

pub async fn wait_for_rate_limit() -> () {
    fn wait_time(last: u128, now: u128) -> u64 {
        if now > last {
            0
        } else {
            (last - now) as u64
        }
    }
    let mut value = LAST_WAIT_FOR_RATE_LIMIT_CALL.lock().unwrap();
    sleep(Duration::from_millis(wait_time(value.deref().clone(), get_unix_time())));
    *value = get_unix_time() + 100;
    ()
}
