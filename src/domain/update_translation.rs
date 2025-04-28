use actix_web::web;
use thiserror::Error;

use crate::Repository;
use crate::domain::voci::{Lang, TranslationRecord, TranslationRecordError, Word};
use crate::driven::repository::{RepoUpdateError, RepoReadError};


#[derive(Debug, PartialEq, Error)]
pub enum UpdateError {
    #[error("Bad Input: {0}")]
    WordError(#[from] TranslationRecordError),
    #[error("Translation not found")]
    NotFound,
    #[error("Read Error: {0}")]
    ReadError(#[from] RepoReadError),
}

/// Updates a translation record for a given word and language.
///
/// # Arguments
/// * `word` - The word to update the translation for
/// * `lang` - The language of the word
///
/// # Returns
/// * `Result<TranslationRecord, UpdateError>` - The updated translation record if successful,
///   or an error if the input is invalid
///
/// # Errors
/// Returns `UpdateError::WordError` if:
/// * The word is empty or invalid
/// * The language specification is invalid
///
///
pub async fn update_translation<T: Repository<TranslationRecord>>(
    repository: web::Data<T>,
    word: &str,
    lang: &Lang,
    extra_translations: &Vec<&str>,
    extra_translation_lang: &Lang,
) -> Result<TranslationRecord, UpdateError> {
    let word = Word::new(word.to_string(), lang.clone())?;

    let result =
    repository.read_by_word(&word).await?;






     let mut translations = vec![];
    // let mut extra_translations: Vec<String> =
    //     extra_translations.iter().map(|s| s.to_string()).collect();
    // translations.append(&mut extra_translations);

    // let (word, lang) = word.value();

    Ok(TranslationRecord::new(
        None,
        "".to_string(),
        lang.clone(),
        translations,
        extra_translation_lang.clone(),
    )?)


}

#[cfg(test)]
mod tests {
    use actix_web::web::Data;

    use super::*;
    use crate::tests::{test_utils::shared::*, voci_repo_double::repo_double::VociRepoDouble};



    #[actix_rt::test]
    async fn update_existing_record_with_no_extra_word() {
        let repo = VociRepoDouble::new(&get_testing_persistence_config()).unwrap();



        let updated_tr = update_translation(
            Data::new(repo),
            &WORD,
            &WORD_LANG,
            &[].to_vec(),
            &TRANSLATION_LANG,
        ).await;

        let updated_translation = updated_tr.unwrap();
        let (_, _, _, actual_translations, _) = updated_translation.flat();
        assert_on_translations(actual_translations, &TRANSLATIONS.map(|t| t.to_string()).to_vec());
    }

    #[actix_rt::test]
    async fn update_existing_record() {
        let repo = VociRepoDouble::new(&get_testing_persistence_config()).unwrap();

        let extra_translations = TranslationRecord::new(
            None,
            WORD.to_string(),
            WORD_LANG.clone(),
            ADDITONAL_TRANSLATIONS.map(|t| t.to_string()).to_vec(),
            TRANSLATION_LANG.clone(),
        );

        let mut additional_tranlations = ADDITONAL_TRANSLATIONS.map(|t| t.to_string()).to_vec();

        let mut translations = TRANSLATIONS.map(|t| t.to_string()).to_vec();

        translations.append(&mut additional_tranlations);

        let updated_tr = update_translation(
            Data::new(repo),
            &WORD,
            &WORD_LANG,
            &ADDITONAL_TRANSLATIONS.to_vec(),
            &TRANSLATION_LANG,
        ).await;

        let updated_translation = updated_tr.unwrap();
        let (_, _, _, actual_translations, _) = updated_translation.flat();
        assert_on_translations(actual_translations, &translations);
    }

    // #[test]
    // fn update_ok_word_ok() {
    //     matches!(
    //         update_translation(WORD, &WORD_LANG, &TRANSLATIONS.to_vec(), &TRANSLATION_LANG),
    //         Ok(_)
    //     );
    // }
    // #[test]
    // fn update_bad_word_err() {
    //     let upd_trans =
    //         update_translation("", &WORD_LANG, &TRANSLATIONS.to_vec(), &TRANSLATION_LANG);
    //     assert_eq!(upd_trans.is_err(), true);
    //     assert_eq!(
    //         upd_trans.unwrap_err(),
    //         UpdateError::WordError(TranslationRecordError::EmptyWord)
    //     )
    // }
}
