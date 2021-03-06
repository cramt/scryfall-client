mod autocomplete;
mod card_identifier;
mod client_paths;
mod collection;
mod name;
mod random;
mod search;
mod set;
mod sets;

use lazy_static::*;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub enum ClientError {
    Request(reqwest::Error),
    TextDecode(reqwest::Error),
    Deserialize(serde_json::Error),
    Serialize(serde_json::Error),
    Scryfall(ScryfallError),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::Request(e) => e.fmt(f),
            ClientError::TextDecode(e) => e.fmt(f),
            ClientError::Deserialize(e) => e.fmt(f),
            ClientError::Serialize(e) => e.fmt(f),
            ClientError::Scryfall(e) => format!("{:?}", e).fmt(f),
        }
    }
}

impl std::error::Error for ClientError {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScryfallError {
    pub object: String,
    pub code: String,
    pub status: u16,
    pub details: String,
}

pub struct Client;

impl Client {
    fn get<S: IntoUrl>(&self, path: S) -> reqwest::RequestBuilder {
        reqwest::Client::new().get(path)
    }
    fn post<S: IntoUrl>(&self, path: S) -> reqwest::RequestBuilder {
        reqwest::Client::new().post(path)
    }
    async fn send_request<T: DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<T, ClientError> {
        wait_for_rate_limit().await;
        let result = request.send().await.map_err(|x| ClientError::Request(x))?;
        let text = result
            .text()
            .await
            .map_err(|x| ClientError::TextDecode(x))?;
        let json: T = serde_json::from_reader(text.as_bytes()).map_err(|x| {
            let json_error: serde_json::error::Result<ScryfallError> =
                serde_json::from_reader(text.as_bytes());
            match json_error {
                Ok(scryfall) => ClientError::Scryfall(scryfall),
                _ => ClientError::Deserialize(x),
            }
        })?;
        Ok(json)
    }
}

lazy_static! {
    static ref LAST_WAIT_FOR_RATE_LIMIT_CALL: Mutex<u128> = Mutex::new(0);
}

const RATE_LIMIT_WAIT: u32 = 100;

pub fn get_unix_time() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
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
    sleep(Duration::from_millis(wait_time(
        value.deref().clone(),
        get_unix_time(),
    )));
    *value = get_unix_time() + RATE_LIMIT_WAIT as u128;
    ()
}
