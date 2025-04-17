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
pub fn update_translation(
    word: &str,
    lang: &Lang,
    extra_translations: &Vec<&str>,
    extra_translation_lang: &Lang,
) -> Result<TranslationRecord, UpdateError> {
    let word = Word::new(word.to_string(), lang.clone())?;
    //todo: word is needed to find the record

    let mut translations = vec!["köter".to_string(), "waldi".to_string()];
    let mut extra_translations: Vec<String> =
        extra_translations.iter().map(|s| s.to_string()).collect();
    translations.append(&mut extra_translations);

    let (word, lang) = word.value();

    Ok(TranslationRecord::new(
        word.to_string(),
        lang.clone(),
        translations,
        extra_translation_lang.clone(),
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_utils::shared::*;

    #[test]
    fn update_ok_word_ok() {
        matches!(
            update_translation(WORD, &WORD_LANG, &TRANSLATIONS.to_vec(), &TRANSLATION_LANG),
            Ok(_)
        );
    }
    #[test]
    fn update_bad_word_err() {
        let upd_trans = update_translation("", &WORD_LANG, &TRANSLATIONS.to_vec(), &TRANSLATION_LANG);
        assert_eq!(upd_trans.is_err(), true);
        assert_eq!(
            upd_trans.unwrap_err(),
            UpdateError::WordError(TranslationRecordError::EmptyWord)
        )
    }
}
