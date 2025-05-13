use async_trait::async_trait;
use thiserror::Error;

use crate::config::PersistenceConfig;
use crate::domain::voci::{TranslationId, Word};

pub mod mongo_repository;

#[derive(Error, Debug, PartialEq)]
pub enum RepoCreateError {
    #[error("Invalid Data")]
    InvalidData(String),
    #[error("Unknown repo create error")]
    Unknown,
}

#[derive(Error, Debug, PartialEq)]
pub enum  RepoReadError {
    #[error("Not found")]
    NotFound,
    #[error("Unknown repo read error")]
    Unknown,
}

#[derive(Error, Debug, PartialEq)]
pub enum RepoUpdateError {
    #[error("malformed id")]
    BadId,
    #[error("Not Found")]
    NotFound,
    #[error("Unknown")]
    Unknown,
}

#[derive(Error, Debug, PartialEq)]
pub enum RepoDeleteError {
    #[error("malformed id")]
    BadId,
    #[error("Not Found")]
    NotFound,
    #[error("Unknown repo delete error")]
    Unknown,
}

#[async_trait]
pub trait Repository<T>
{
    /// Creation of a repository
    fn new(config: &PersistenceConfig) -> Result<Self, String>
    where
        Self: Sized;

    /// Insert the received TranslationRecord in the persistence system
    async fn create(&self, tr: &T) -> Result<T, RepoCreateError>;

    /// Read/find a TranslationRecord given a Word
    async fn read_by_word(&self, word: &Word) -> Result<T, RepoReadError>;

    /// Update a TranslationRecord given a TranslationRecord
    ///
    /// The TranslationId in the argument is used to identify the TranslationRecord.
    /// Translations within it, are used to update the existing translations
    async fn update(&self, tr: &T) -> Result<T, RepoUpdateError>;

    /// Delete a TranslationRecord given an ID
    async fn delete(&self, id: &TranslationId) -> Result<(), RepoDeleteError>;
}
