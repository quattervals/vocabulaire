use crate::domain::voci::{Lang, TranslationRecord, TranslationRecordError, Word};

use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum ReadError {
    #[error("Bad Input: {0}")]
    QueryWord(#[from] TranslationRecordError),
    // #[error("Translation not found")]
    // NotFound,
}

//todo
// add side effect of actually finding this Translation record
// add finding via ID
pub fn read_translation(word: &str, lang: &Lang) -> Result<TranslationRecord, ReadError> {
    let _word = Word::new(word.to_string(), lang.clone())?;

    Ok(TranslationRecord::new(
        None,
        "chien".to_string(),
        Lang::fr,
        vec!["hund".to_string(), "köter".to_string()],
        Lang::de,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_utils::shared::*;

    #[test]
    fn read_ok_word_ok() {
        matches!(read_translation(WORD, &WORD_LANG), Ok(_));
    }
    #[test]
    fn read_bad_word_err() {
        let read_trans = read_translation("", &WORD_LANG);
        assert_eq!(read_trans.is_err(), true);
        assert_eq!(read_trans.unwrap_err(), ReadError::QueryWord(TranslationRecordError::EmptyWord));
    }
}
