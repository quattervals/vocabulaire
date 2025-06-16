use std::ops::Deref;
use thiserror::Error;

use crate::domain::ports::{RepoCreateError, RepoReadError, TranslationRepository};
use crate::domain::voci::{Lang, TranslationRecord, TranslationRecordError};

#[derive(Debug, PartialEq, Error)]
pub enum CreateError {
    #[error("Bad Input: {0}")]
    InvalidInput(#[from] TranslationRecordError),
    #[error("Read Error: {0}")]
    Read(#[from] RepoReadError),
    #[error("Create Error")]
    Create(#[from] RepoCreateError),
    #[error("Duplicate")]
    Duplicate,
}

pub async fn create_translation<S>(
    repository: &impl TranslationRepository,
    word: &str,
    word_lang: &Lang,
    translations: &[S],
    translation_lang: &Lang,
) -> Result<TranslationRecord, CreateError>
where
    S: Deref<Target = str>,
{
    let tr = TranslationRecord::new(None, word, word_lang, translations, translation_lang)?;

    let word = tr.word();

    let does_exist = repository.read_by_word(word).await.is_ok();

    if !does_exist {
        let create_response = repository.create(&tr).await?;
        Ok(create_response)
    } else {
        Err(CreateError::Duplicate)
    }
}
