use crate::cache::cache_iterator::CacheIterator;
use crate::cache::io_cache::IoCache;
use crate::card::Card;
use crate::client::{get_unix_time, Client};
use crate::search_builder;
use crate::search_builder::SearchBuilderTrait;
use eyre::Result;
use std::path::PathBuf;
use std::time::Duration;

pub mod cache_iterator;
mod io_cache;

#[derive(Debug)]
pub struct ScryfallCache {
    io: IoCache,
    expiration: Duration,
    query: String,
}

impl ScryfallCache {
    pub fn new() -> Self {
        Self::new_options(CacheOptions::default())
    }

    pub async fn check(&self) -> Result<()> {
        if match self.io.get_meta()? {
            None => true,
            Some(meta) => meta.download_time + self.expiration.as_millis() > get_unix_time(),
        } {
            let result = Client.search(&self.query).await?;
            self.io.add(result);
        };
        Ok(())
    }

    pub fn new_options(options: CacheOptions) -> Self {
        let (path, expiration, query) = options.unpack();
        Self {
            io: IoCache::new(path).unwrap(),
            expiration,
            query,
        }
    }

    pub async fn get_card_by_name<S: ToString>(&self, name: S) -> Result<Option<Card>> {
        self.check().await?;
        self.io.get(name)
    }

    pub async fn iter<'a>(&'a self) -> Result<CacheIterator<'a>> {
        self.check().await?;
        Ok(CacheIterator {
            inner: &self,
            inner_iter: self.io.get_all()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CacheOptions {
    location: Box<PathBuf>,
    expiration: Duration,
    query: String,
}

impl CacheOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn unpack(self) -> (Box<PathBuf>, Duration, String) {
        (self.location, self.expiration, self.query)
    }

    pub fn location(mut self, path: PathBuf) -> Self {
        self.location = Box::new(path);
        self
    }

    pub fn expiration(mut self, time: Duration) -> Self {
        self.expiration = time;
        self
    }

    pub fn query<S: ToString>(mut self, query: S) -> Self {
        self.query = query.to_string();
        self
    }
}

impl Default for CacheOptions {
    fn default() -> Self {
        Self {
            location: Box::new(std::env::current_dir().unwrap().join("__scryfall_cache")),
            expiration: Duration::from_secs(60 * 60 * 24),
            query: search_builder::Builder::all_cards().stringify(),
        }
    }
}
