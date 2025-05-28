use thiserror::Error;

use crate::domain::ports::{RepoDeleteError, RepoReadError, TranslationRepository};
use crate::domain::voci::{Lang, TranslationRecordError, Word};

#[derive(Debug, PartialEq, Error)]
pub enum DeleteError {
    #[error("Bad Input: {0}")]
    Word(#[from] TranslationRecordError),
    #[error("Read Error: {0}")]
    Read(#[from] RepoReadError),
    #[error("Delete Error:")]
    Delete(#[from] RepoDeleteError),
}

pub async fn delete_translation(
    repository: &impl TranslationRepository,
    word: &str,
    lang: &Lang,
) -> Result<(), DeleteError> {
    let word = Word::new(word.to_string(), lang.clone())?;

    let tr_to_be_deleted = repository.read_by_word(&word).await?;

    repository.delete(tr_to_be_deleted.id()).await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_utils::{test_utils::shared::*, voci_repo_double::repo_double::VociRepoDouble};

    #[actix_rt::test]
    async fn delete_ok_word_ok() {
        let repo = VociRepoDouble::new(&get_testing_persistence_config()).unwrap();

        let response = delete_translation(&repo, WORD, &WORD_LANG).await;

        assert_eq!(response, Ok(()));
    }

    #[actix_rt::test]
    async fn delete_bad_word_err() {
        let repo = VociRepoDouble::new(&get_testing_persistence_config()).unwrap();

        let response = delete_translation(&repo, "", &WORD_LANG).await;

        assert!(response.is_err());
        assert_eq!(
            response.unwrap_err(),
            DeleteError::Word(TranslationRecordError::EmptyWord)
        );
    }

    #[actix_rt::test]
    async fn delete_manually_provoked_error_err() {
        let mut repo = VociRepoDouble::new(&get_testing_persistence_config()).unwrap();
        repo.set_error(true);

        let response = delete_translation(&repo, WORD, &WORD_LANG).await;

        assert!(response.is_err());
        assert_eq!(
            response.unwrap_err(),
            DeleteError::Read(RepoReadError::Unknown)
        );
    }
}
