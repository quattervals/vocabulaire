use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::ops::Deref;
use thiserror::Error;

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
    #[error("Translation language mismatch")]
    TranslationLanguageMismatch,
    #[error("Update with same items")]
    UpdateWithSameItems,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranslationId(Option<String>);

impl TranslationId {
    pub fn value(&self) -> &Option<String> {
        &self.0
    }
}

impl From<&str> for TranslationId {
    fn from(id: &str) -> Self {
        if id.is_empty() {
            Self(None)
        } else {
            Self(Some(id.to_string()))
        }
    }
}

impl From<Option<&str>> for TranslationId {
    fn from(opt: Option<&str>) -> Self {
        match opt {
            Some(val) => {
                if val.is_empty() {
                    Self(None)
                } else {
                    Self(Some(val.to_string()))
                }
            }
            None => Self(None),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Word {
    word: String,
    lang: Lang,
}

impl Word {
    pub fn new(word: &str, lang: &Lang) -> Result<Self, TranslationRecordError> {
        if word.is_empty() {
            return Err(TranslationRecordError::EmptyWord);
        }
        Ok(Word {
            word: word.to_string(),
            lang: lang.clone(),
        })
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
    fn new<S>(words: &[S], lang: &Lang) -> Result<Self, TranslationRecordError>
    where
        S: Deref<Target = str>,
    {
        if words.is_empty() {
            return Err(TranslationRecordError::EmptyTranslation);
        }

        if words.iter().any(|s| s.is_empty()) {
            return Err(TranslationRecordError::EmptyWordInTranslation);
        }

        Ok(Translations {
            words: words.iter().map(|s| s.to_string()).collect(),
            lang: lang.clone(),
        })
    }

    fn translations(&self) -> &Vec<String> {
        &self.words
    }
    fn value(&self) -> (&Vec<String>, &Lang) {
        (self.translations(), &self.lang)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranslationRecord {
    id: TranslationId,
    word: Word,
    translations: Translations,
}

impl TranslationRecord {
    pub fn new<S>(
        id: Option<&str>,
        word: &str,
        word_lang: &Lang,
        translations: &[S],
        translation_lang: &Lang,
    ) -> Result<Self, TranslationRecordError>
    where
        S: Deref<Target = str>,
    {
        let id = TranslationId::from(id);
        let word = Word::new(word, word_lang)?;
        let translations = Translations::new(translations, translation_lang)?;

        Ok(TranslationRecord {
            id,
            word,
            translations,
        })
    }

    pub fn id(&self) -> &TranslationId {
        &self.id
    }

    pub fn word(&self) -> &Word {
        &self.word
    }

    pub fn update(
        &mut self,
        translations: Vec<String>,
        lang: Lang,
    ) -> Result<(), TranslationRecordError> {
        if self.translations.lang != lang {
            return Err(TranslationRecordError::TranslationLanguageMismatch);
        }

        if vectors_are_equal(&self.translations.words, &translations) {
            return Err(TranslationRecordError::UpdateWithSameItems);
        }

        let mut seen: HashSet<String> = self.translations.words.iter().cloned().collect();

        for t in translations {
            if !t.is_empty() && !seen.contains(&t) {
                seen.insert(t.clone());
                self.translations.words.push(t);
            }
        }

        Ok(())
    }

    pub fn flat(&self) -> (&Option<String>, &String, &Lang, &Vec<String>, &Lang) {
        let id = &self.id.value();
        let word = &self.word.value();
        let trans = &self.translations.value();
        (id, word.0, word.1, trans.0, trans.1)
    }
}

fn vectors_are_equal<T: Ord + Clone>(vec1: &[T], vec2: &[T]) -> bool {
    if vec1.len() != vec2.len() {
        return false;
    }

    let mut sorted_vec1 = vec1.to_vec();
    let mut sorted_vec2 = vec2.to_vec();

    sorted_vec1.sort();
    sorted_vec2.sort();

    sorted_vec1 == sorted_vec2
}

#[cfg(test)]
mod tests {
    use crate::test_utils::utils::shared::{
        ADDITONAL_TRANSLATIONS, TRANSLATIONS, stub_translation_record,
    };

    use super::*;

    #[test]
    fn word_new_ok_input_constructed() {
        let word = Word::new("chien", &Lang::fr);
        assert_eq!((word.unwrap().value()), (&"chien".to_string(), &Lang::fr));
    }

    #[test]
    fn word_new_bad_input_error() {
        let err_word = Word::new("", &Lang::fr);
        assert!(err_word.is_err());
        assert_eq!(err_word.unwrap_err(), TranslationRecordError::EmptyWord);
    }

    #[test]
    fn translation_new_ok_input_constructed() {
        let words = vec!["hund", "köter"];

        let translations = Translations::new(&words, &Lang::de);
        for (i, translation) in translations.unwrap().translations().iter().enumerate() {
            assert_eq!(*translation, words[i]);
        }
    }

    #[test]
    fn translation_new_empty_word_err() {
        let err_words = &["", "köter"];

        let err_translations = Translations::new(err_words, &Lang::de);
        assert!(err_translations.is_err());
        assert_eq!(
            err_translations.unwrap_err(),
            TranslationRecordError::EmptyWordInTranslation
        );
    }

    #[test]
    fn translation_new_empty_string_err() {
        let err_words: [&str; 0] = [];
        let err_translations = Translations::new(&err_words, &Lang::de);
        assert!(err_translations.is_err());
        assert_eq!(
            err_translations.unwrap_err(),
            TranslationRecordError::EmptyTranslation
        );
    }

    #[test]
    fn translation_record_new_ok_input_constructed() {
        let id = "1234";

        let word = "chien";
        let word_lang = Lang::fr;
        let translations = vec!["hund", "köter"];
        let translation_lang = Lang::de;

        let chien =
            TranslationRecord::new(Some(id), word, &word_lang, &translations, &translation_lang)
                .unwrap();

        assert_eq!(*chien.id.value(), Some(id.to_string()));
        assert_eq!(chien.word.word, "chien");
        assert_eq!(chien.word.lang, Lang::fr);
        assert_eq!(chien.translations.lang, Lang::de);
        for (i, translation) in chien.translations.words.into_iter().enumerate() {
            assert_eq!(translation, translations[i]);
        }
    }

    #[test]
    fn translation_record_new_bad_input_err() {
        let word = "chien";
        let word_lang = Lang::fr;

        let translations = vec!["hund", ""];
        let translation_lang = Lang::de;

        let chien =
            TranslationRecord::new(None, word, &word_lang, &translations, &translation_lang);

        assert!(chien.is_err());
        assert_eq!(
            chien.unwrap_err(),
            TranslationRecordError::EmptyWordInTranslation
        );
    }

    #[test]
    fn translation_record_update_correct_translation_record() {
        let mut tr = stub_translation_record(true);
        let extra_translations = ADDITONAL_TRANSLATIONS.map(|r| r.to_string()).to_vec();
        let mut expected = tr.flat().3.clone();
        expected.append(&mut extra_translations.clone());

        let _ = tr.update(extra_translations, Lang::de);

        assert_eq!(tr.flat().3, &expected);
    }

    #[test]
    fn translation_record_update_different_language_no_update_and_error() {
        let mut tr = stub_translation_record(true);
        let extra_translations = ADDITONAL_TRANSLATIONS.map(|r| r.to_string()).to_vec();
        let expected = tr.flat().3.clone();

        let result = tr.update(extra_translations, Lang::fr);

        assert_eq!(tr.flat().3, &expected);
        assert_eq!(
            result.unwrap_err(),
            TranslationRecordError::TranslationLanguageMismatch
        );
    }

    #[test]
    fn translation_record_update_same_word_as_already_in_no_update() {
        let mut tr = stub_translation_record(true);
        let extra_translations = vec![TRANSLATIONS[0].to_string()];
        let expected = tr.flat().3.clone();

        let _ = tr.update(extra_translations, Lang::de);

        assert_eq!(tr.flat().3, &expected);
    }

    #[test]
    fn translation_record_update_same_word_twice_in_update_no_update() {
        let mut tr = stub_translation_record(true);
        let extra_translations = vec![TRANSLATIONS[0].to_string(), TRANSLATIONS[0].to_string()];
        let expected = tr.flat().3.clone();

        let _ = tr.update(extra_translations, Lang::de);

        assert_eq!(tr.flat().3, &expected);
    }

    #[test]
    fn translation_record_update_with_existing_words_no_update_error() {
        let mut tr = stub_translation_record(true);
        let extra_translations = vec![TRANSLATIONS[1].to_string(), TRANSLATIONS[0].to_string()];
        let expected = tr.flat().3.clone();

        let result = tr.update(extra_translations, Lang::de);

        assert_eq!(tr.flat().3, &expected);
        assert_eq!(
            result.unwrap_err(),
            TranslationRecordError::UpdateWithSameItems
        );
    }
}
