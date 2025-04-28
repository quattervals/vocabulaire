use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::config::PersistenceConfig;
use crate::domain::Entity;
use crate::domain::voci::{TranslationId, Word};

pub mod mongo_repository;

#[derive(Debug, PartialEq)]
pub enum RepoCreateError {
    InvalidData(String),
    Unknown(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum RepoReadError {
    #[error("Not found")]
    NotFound,
    #[error("Unknown repo read error")]
    Unknown(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum RepoUpdateError {
    #[error("Not Found")]
    NotFound,
    #[error("Nothing found")]
    NoChange,
    #[error("Unknown")]
    Unknown,
}

// /// Structure to specify agreed format for passing the lookup value through the port.
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct FindTranslationRecord {
//     pub word: Word,
// }

// /// Structure to specify agreed format for passing the lookup value through the port.
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum FindTranslationRecordx{
//     ByWord(Word),
//     ById(TranslationId),
// }

#[async_trait]
pub trait Repository<T>
where
    T: Entity,
{
    /// Creation of a repository
    fn new(config: &PersistenceConfig) -> Result<Self, String>
    where
        Self: Sized;

    /// Insert the received TranslationRecord in the persistence system
    async fn create(&self, tr: &T) -> Result<T, RepoCreateError>;

    /// Read/find a TranslationRecord given a Word
    async fn read_by_word(&self, find_tr: &Word) -> Result<T, RepoReadError>;

    /// Update a TranslationRecord given a TranslationRecord
    ///
    /// The Word in the argument is used to identify the TranslationRecord.
    /// Translations within it, are used to update the existing translations
    async fn update(&self, tr: &T) -> Result<T, RepoUpdateError>;
}
