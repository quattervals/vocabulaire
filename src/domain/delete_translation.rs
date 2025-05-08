use actix_web::web;
use thiserror::Error;

use crate::Repository;
use crate::domain::voci::{Lang, TranslationRecord, TranslationRecordError, Word};
use crate::driven::repository::{RepoDeleteError, RepoReadError};

#[derive(Debug, PartialEq, Error)]
pub enum DeleteError {
    #[error("Bad Input: {0}")]
    WordError(#[from] TranslationRecordError),
    #[error("Read Error: {0}")]
    ReadError(#[from] RepoReadError),
    #[error("Delete Error:")]
    DeleteError(#[from] RepoDeleteError),
}

pub async fn delete_translation<T: Repository<TranslationRecord>>(
    repository: web::Data<T>,
    word: &str,
    lang: &Lang,
) -> Result<(), DeleteError> {
    let word = Word::new(word.to_string(), lang.clone())?;

    let tr_to_be_deleted = repository.read_by_word(&word).await?;

    let delete_response = repository.delete(&tr_to_be_deleted.id()).await?;

    Ok(delete_response)
}

#[cfg(test)]
mod tests {
    use actix_web::web::Data;

    use super::*;
    use crate::tests::{test_utils::shared::*, voci_repo_double::repo_double::VociRepoDouble};

    #[actix_rt::test]
    async fn delete_ok_word_ok() {
        let repo = VociRepoDouble::new(&get_testing_persistence_config()).unwrap();

        let response = delete_translation(Data::new(repo), WORD, &WORD_LANG).await;

        assert_eq!(response, Ok(()));
    }

    #[actix_rt::test]
    async fn delete_bad_word_err() {
        let repo = VociRepoDouble::new(&get_testing_persistence_config()).unwrap();

        let response = delete_translation(Data::new(repo), "", &WORD_LANG).await;

        assert_eq!(response.is_err(), true);
        assert_eq!(
            response.unwrap_err(),
            DeleteError::WordError(TranslationRecordError::EmptyWord)
        );
    }

    #[actix_rt::test]
    async fn delete_manually_provoked_error_err() {
        let mut  repo = VociRepoDouble::new(&get_testing_persistence_config()).unwrap();
        repo.set_error(true);

        let response = delete_translation(Data::new(repo), WORD, &WORD_LANG).await;

        assert_eq!(response.is_err(), true);
        assert_eq!(
            response.unwrap_err(),
            DeleteError::ReadError(RepoReadError::Unknown)
        );
    }
}
