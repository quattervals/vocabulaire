use async_trait::async_trait;
use thiserror::Error;

use crate::config::PersistenceConfig;
use crate::domain::voci::{TranslationId, TranslationRecord, Word};

#[derive(Error, Debug, PartialEq)]
pub enum RepoCreateError {
    #[error("Unknown")]
    Unknown,
}

#[derive(Error, Debug, PartialEq)]
pub enum RepoReadError {
    #[error("Not found")]
    NotFound,
    #[error("Unknown")]
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
    #[error("Unknown")]
    Unknown,
}

#[async_trait]
pub trait TranslationRepository: Send + Sync + Clone + 'static {
    /// Creation of a repository
    fn new(config: &PersistenceConfig) -> Result<Self, String>
    where
        Self: Sized;

    /// Insert the received TranslationRecord in the persistence system
    async fn create(&self, tr: &TranslationRecord) -> Result<TranslationRecord, RepoCreateError>;

    /// Read/find a TranslationRecord given a Word
    async fn read_by_word(&self, word: &Word) -> Result<TranslationRecord, RepoReadError>;

    /// Update a TranslationRecord given a TranslationRecord
    ///
    /// The TranslationId in the argument is used to identify the TranslationRecord.
    /// Translations within it, are used to update the existing translations
    async fn update(&self, tr: &TranslationRecord) -> Result<TranslationRecord, RepoUpdateError>;

    /// Delete a TranslationRecord given an ID
    async fn delete(&self, id: &TranslationId) -> Result<(), RepoDeleteError>;
}
