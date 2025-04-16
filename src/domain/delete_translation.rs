use crate::domain::voci::{Lang, TranslationRecord, TranslationRecordError, Word};

use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum DeleteError {
    #[error("Bad Input: {0}")]
    WordError(#[from] TranslationRecordError),
    // #[error("Translation not found")]
    // NotFound,
}

//todo
// add side effect of deleting this Translation record
// add deletion via ID
pub fn delete_translation(word: &str, lang: Lang) -> Result<(), DeleteError> {
    let _word = Word::new(word.to_string(), lang)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_utils::shared::*;

    #[test]
    fn delete_ok_word_ok() {
        assert_eq!(delete_translation(WORD, WORD_LANG), Ok(()));
    }
    #[test]
    fn delete_bad_word_err() {
        let del_trans = delete_translation("", WORD_LANG);
        assert_eq!(del_trans.is_err(), true);
        assert_eq!(
            del_trans.unwrap_err(),
            DeleteError::WordError(TranslationRecordError::EmptyWord)
        );
    }
}
