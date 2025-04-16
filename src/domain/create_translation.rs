use crate::domain::voci::{Lang, TranslationRecord, TranslationRecordError};

use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum CreateError {
    #[error("Bad Input: {0}")]
    InvalidInput(#[from] TranslationRecordError),
}

//todo
// add side effect of storing this Translation record
// maybe return ID from storing
pub fn create_translation(
    word: &str,
    word_lang: Lang,
    translations: &Vec<&str>,
    translation_lang: Lang,
) -> Result<TranslationRecord, CreateError> {
    Ok(TranslationRecord::new(
        word.to_string(),
        Lang::fr,
        translations
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>(),
        Lang::de,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_utils::shared::*;

    #[test]
    fn create_translation_ok_input_no_error() {
        create_translation(WORD, WORD_LANG, &TRANSLATIONS.to_vec(), TRANSLATION_LANG)
            .expect("Faulty creation");
    }

    #[test]
    fn create_translation_bad_input_error() {
        let create_trans =
            create_translation("", WORD_LANG, &TRANSLATIONS.to_vec(), TRANSLATION_LANG);

        assert_eq!(create_trans.is_err(), true);
        assert_eq!(
            create_trans.unwrap_err(),
            CreateError::InvalidInput(TranslationRecordError::EmptyWord)
        );
    }
}
