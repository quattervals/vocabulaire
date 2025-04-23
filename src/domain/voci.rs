use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::Entity;

// Todo:
// - nicer "value()" for TranslationRecord

/// Represents available languages in the system
/// Languages codes according to https://de.wikipedia.org/wiki/Liste_der_ISO-639-2-Codes
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Lang {
    fr, // french
    de, // german
}

#[derive(Debug, PartialEq, Error)]
pub enum TranslationRecordError {
    #[error("Word is Empty")]
    EmptyWord,
    #[error("No Translation")]
    EmptyTranslation,
    #[error("One of the Words in Translations is empty")]
    EmptyWordInTranslation,
    #[error("Unknown Error: {0}")]
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Word {
    word: String,
    lang: Lang,
}

impl Word {
    pub fn new(word: String, lang: Lang) -> Result<Self, TranslationRecordError> {
        if word.is_empty() {
            return Err(TranslationRecordError::EmptyWord);
        }
        Ok(Word { word, lang })
    }
    pub fn value(&self) -> (&String, &Lang) {
        (&self.word, &self.lang)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Translations {
    lang: Lang,
    words: Vec<String>,
}

impl Translations {
    fn new(words: Vec<String>, lang: Lang) -> Result<Self, TranslationRecordError> {
        if words.is_empty() {
            return Err(TranslationRecordError::EmptyTranslation);
        }

        if words.iter().any(|s| s.is_empty()) {
            return Err(TranslationRecordError::EmptyWordInTranslation);
        }

        Ok(Translations { words, lang })
    }

    fn translations(&self) -> &Vec<String> {
        &self.words
    }
    fn value(&self) -> (&Vec<String>, &Lang) {
        (&self.translations(), &self.lang)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranslationRecord {
    word: Word,
    translations: Translations,
}

impl TranslationRecord {
    pub fn new(
        word: String,
        word_lang: Lang,
        translations: Vec<String>,
        translation_lang: Lang,
    ) -> Result<Self, TranslationRecordError> {
        let word = Word::new(word, word_lang)?;
        let translations = Translations::new(translations, translation_lang)?;

        Ok(TranslationRecord {
            word: word,
            translations: translations,
        })
    }

    pub fn flat(&self) -> (&String, &Lang, &Vec<String>, &Lang) {
        let word = &self.word.value();
        let trans = &self.translations.value();
        (word.0, word.1, trans.0, trans.1)
    }
}

impl Entity for TranslationRecord {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_new_ok_input_constructed() {
        let word = Word::new("chien".to_string(), Lang::fr);
        assert_eq!((word.unwrap().value()), (&"chien".to_string(), &Lang::fr));
    }

    #[test]
    fn word_new_bad_input_error() {
        let err_word = Word::new("".to_string(), Lang::fr);
        assert_eq!(err_word.is_err(), true);
        assert_eq!(err_word.unwrap_err(), TranslationRecordError::EmptyWord);
    }

    #[test]
    fn translation_new_ok_input_constructed() {
        let words = vec!["hund".to_string(), "köter".to_string()];

        let translations = Translations::new(words.clone(), Lang::de);
        for (i, translation) in translations.unwrap().translations().into_iter().enumerate() {
            assert_eq!(*translation, words[i]);
        }
    }

    #[test]
    fn translation_new_empty_word_err() {
        let err_words = vec!["".to_string(), "köter".to_string()];

        let err_translations = Translations::new(err_words.clone(), Lang::de);
        assert_eq!(err_translations.is_err(), true);
        assert_eq!(
            err_translations.unwrap_err(),
            TranslationRecordError::EmptyWordInTranslation
        );
    }

    #[test]
    fn translation_new_empty_string_err() {
        let err_words = vec![];

        let err_translations = Translations::new(err_words.clone(), Lang::de);
        assert_eq!(err_translations.is_err(), true);
        assert_eq!(
            err_translations.unwrap_err(),
            TranslationRecordError::EmptyTranslation
        );
    }

    #[test]
    fn translation_record_new_ok_input_constructed() {
        let word = "chien".to_string();
        let word_lang = Lang::fr;

        let translations = vec!["hund".to_string(), "köter".to_string()];
        let translation_lang = Lang::de;

        let chien = TranslationRecord::new(word, word_lang, translations.clone(), translation_lang)
            .unwrap();

        assert_eq!(chien.word.word, "chien");
        assert_eq!(chien.word.lang, Lang::fr);
        assert_eq!(chien.translations.lang, Lang::de);
        for (i, translation) in chien.translations.words.into_iter().enumerate() {
            assert_eq!(translation, translations[i]);
        }
    }

    #[test]
    fn translation_record_new_bad_input_err() {
        let word = "chien".to_string();
        let word_lang = Lang::fr;

        let translations = vec!["hund".to_string(), "".to_string()];
        let translation_lang = Lang::de;

        let chien = TranslationRecord::new(word, word_lang, translations.clone(), translation_lang);

        assert_eq!(chien.is_err(), true);
        assert_eq!(
            chien.unwrap_err(),
            TranslationRecordError::EmptyWordInTranslation
        );
    }
}
