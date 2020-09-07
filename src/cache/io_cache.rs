use crate::card::Card;
use crate::client::get_unix_time;
use eyre::Result;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::fs::{create_dir_all, read_dir, remove_dir_all, remove_file, File};
use std::io::Write;
use std::ops::Deref;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct MetaData {
    pub download_time: u128,
}

#[derive(Debug)]
pub(crate) struct IoCache {
    dir_location: Box<PathBuf>,
}

impl IoCache {
    pub fn new(dir_location: Box<PathBuf>) -> Result<Self> {
        if !dir_location.exists() {
            create_dir_all(dir_location.deref())?;
        }
        let gitignore = dir_location.join(".gitignore");
        if !gitignore.exists() {
            let mut file = File::create(gitignore)?;
            file.write_all(b"*");
        }
        Ok(Self { dir_location })
    }

    fn write_to_file<S: ToString, T: ?Sized + Serialize>(&self, name: S, value: &T) -> Result<()> {
        let name = self.name_format(name.to_string());
        let path = self.dir_location.join(name);
        if path.exists() {
            remove_file(path.clone())?;
        };
        let mut file = File::create(path)?;
        let bytes = bincode::serialize(value)?;
        file.write_all(bytes.as_slice())?;
        Ok(())
    }

    fn add_individual(&self, card: Card) -> Result<()> {
        self.write_to_file(&card.name, &card);
        Ok(())
    }

    pub fn add(&self, cards: Vec<Card>) -> Result<()> {
        if let Some(err) = cards
            .into_iter()
            .map(|x| self.add_individual(x))
            .find(|x| x.is_err())
        {
            return err;
        }
        let meta_data = MetaData {
            download_time: get_unix_time(),
        };
        self.write_to_file("__meta", &meta_data)
    }

    fn read_file<S: ToString, T: DeserializeOwned>(&self, name: S) -> Result<Option<T>> {
        let path = self.dir_location.join(name.to_string());
        if !path.exists() {
            return Ok(None);
        }
        let file = File::open(path)?;
        let result: T = bincode::deserialize_from(file)?;
        Ok(Some(result))
    }

    pub fn get<S: ToString>(&self, name: S) -> Result<Option<Card>> {
        self.read_file(name)
    }

    pub fn get_meta(&self) -> Result<Option<MetaData>> {
        self.read_file("__meta")
    }

    pub fn get_all(&self) -> Result<Box<dyn Iterator<Item = OsString>>> {
        Ok(Box::new(
            read_dir(self.dir_location.as_path())?.map(|x| x.unwrap().file_name()),
        ))
    }

    pub fn delete(&self) -> Result<()> {
        remove_dir_all(self.dir_location.deref())?;
        Ok(())
    }

    fn name_format(&self, name: String) -> String {
        //characters that needs to be escaped \ / : ? " |
        name.replace("\\", "{back_slash}")
            .replace("/", "{slash}")
            .replace(":", "{colon}")
            .replace("?", "{question_mark}")
            .replace("\"", "{double_qoute}")
            .replace("|", "{line}")
    }
}
