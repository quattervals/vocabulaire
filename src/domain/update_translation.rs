use crate::domain::voci::{Lang, TranslationRecord, TranslationRecordError, Word};

use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum UpdateError {
    #[error("Bad Input: {0}")]
    WordError(#[from] TranslationRecordError),
    // #[error("Translation not found")]
    // NotFound,
}

//todo
// add side effect of actually finding this Translation record
// add finding via ID
// return not found error, if translation record can't be found

/// Updates a translation record for a given word and language by appending additional translations.
///
/// # Arguments
/// * `word` - The word to update the translation record for
/// * `word_lang` - The language of the word to be updated
/// * `extra_translations` - A vector of additional translations to be appended
/// * `extra_translation_lang` - The language of the additional translations
///
/// # Returns
/// * `Result<TranslationRecord, UpdateError>` - The updated translation record if successful,
///   or an error if the input is invalid
///
/// # Errors
/// Returns `UpdateError::WordError` if:
/// * The search word cannot be created
pub fn update_translation(
    word: &str,
    word_lang: Lang,
    extra_translations: &Vec<&str>,
    extra_translation_lang: Lang,
) -> Result<TranslationRecord, UpdateError> {
    let _word = Word::new(word.to_string(), word_lang)?;
    //todo: word is needed to find the record

    let mut translations = vec!["köter".to_string(), "waldi".to_string()];
    let mut extra_translations: Vec<String> =
        extra_translations.iter().map(|s| s.to_string()).collect();
    translations.append(&mut extra_translations);


    Ok(TranslationRecord::new(
        "chien".to_string(),
        Lang::fr,
        translations,
        Lang::de,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_utils::shared::*;

    #[test]
    fn update_ok_word_ok() {
        matches!(update_translation(WORD, WORD_LANG, &TRANSLATIONS.to_vec(), TRANSLATION_LANG), Ok(_));
    }
    #[test]
    fn update_bad_word_err() {
        let upd_trans = update_translation("", WORD_LANG, &TRANSLATIONS.to_vec(), TRANSLATION_LANG);
        assert_eq!(upd_trans.is_err(), true);
        assert_eq!(upd_trans.unwrap_err(), UpdateError::WordError(TranslationRecordError::EmptyWord))
    }
}
