use actix_web::web;
use thiserror::Error;

use crate::Repository;
use crate::domain::voci::{Lang, TranslationRecord, TranslationRecordError};
use crate::driven::repository::RepoCreateError;

#[derive(Debug, PartialEq, Error)]
pub enum CreateError {
    #[error("Bad Input: {0}")]
    InvalidInput(#[from] TranslationRecordError),
    #[error("Invalid Data to DB")]
    InvalidData(String),
    #[error("Unknown")]
    Unknown(String),
    //error if this translation item already exists
}

pub async fn create_translation<T: Repository<TranslationRecord>>(
    repository: web::Data<T>,
    word: &str,
    word_lang: &Lang,
    translations: &Vec<&str>,
    translation_lang: &Lang,
) -> Result<TranslationRecord, CreateError> {
    let tr = TranslationRecord::new(
        None,
        word.to_string(),
        word_lang.clone(),
        translations
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>(),
        translation_lang.clone(),
    )
    .map_err(|e| CreateError::InvalidInput(e))?;

    //todo: check if this translation already exists

    repository.create(&tr).await.map_err(|e| {
        return match e {
            RepoCreateError::InvalidData(e) => CreateError::InvalidData(e),
            RepoCreateError::Unknown(e) => CreateError::Unknown(e),
        };
    })
}

#[cfg(test)]
mod tests {
    use actix_web::web::Data;

    use super::*;
    use crate::tests::{test_utils::shared::*, voci_repo_double::repo_double::VociRepoDouble};

    #[actix_rt::test]
    async fn create_translation_ok_input_no_error() {
        let repo = VociRepoDouble::new(&get_testing_persistence_config()).unwrap();

        let result = create_translation(
            Data::new(repo),
            WORD,
            &WORD_LANG,
            &TRANSLATIONS.to_vec(),
            &TRANSLATION_LANG,
        )
        .await
        .unwrap();


        // assert_eq!(result.id().value().is_some(), true); //todo check if ID exists

        assert_eq!(result, stub_translation_record(true));
    }
    //     create_translation(WORD, &WORD_LANG, &TRANSLATIONS.to_vec(), &TRANSLATION_LANG)
    //         .expect("Faulty creation");
    // }

    // #[test]
    // fn create_translation_bad_input_error() {
    //     let create_trans =
    //         create_translation("", &WORD_LANG, &TRANSLATIONS.to_vec(), &TRANSLATION_LANG);

    //     assert_eq!(create_trans.is_err(), true);
    //     assert_eq!(
    //         create_trans.unwrap_err(),
    //         CreateError::InvalidInput(TranslationRecordError::EmptyWord)
    //     );
    // }
}
