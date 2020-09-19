use crate::cache::ScryfallCache;
use crate::card::Card;
use std::ffi::OsString;

pub struct CacheIterator<'a> {
    pub(super) inner: &'a ScryfallCache,
    pub(super) inner_iter: Box<dyn Iterator<Item = OsString>>,
}

impl<'a> Iterator for CacheIterator<'a> {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result: Option<Card> = None;
        loop {
            let os_string = self.inner_iter.next()?;
            result = match self.inner.io.get(os_string.to_str().unwrap()) {
                Err(_) => None,
                Ok(o) => o,
            };
            if result.is_some() {
                break;
            }
        }
        result
    }
}
