use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::config::PersistenceConfig;
use crate::domain::Entity;
use crate::domain::voci::Word;

pub mod mongo_repository;

#[derive(Debug)]
pub enum RepoCreateError {
    InvalidData(String),
    Unknown(String),
}

/// Structure to specify agreed format for passing the lookup value through the port.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindTranslationRecord {
    pub word: Word,
}

#[async_trait]
pub trait Repository<T>
where
    T: Entity,
{
    /// Creation of a repository
    fn new(config: &PersistenceConfig) -> Result<Self, String>
    where
        Self: Sized;

    /// Insert the received entity in the persistence system
    async fn create(&self, voci: T) -> Result<T, RepoCreateError>;
}
