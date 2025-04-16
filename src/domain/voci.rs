/// Represents available languages in the system
/// Languages codes according to https://de.wikipedia.org/wiki/Liste_der_ISO-639-2-Codes
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Lang {
    fr, // french
    de, // german
}

pub struct Word {
    word: String,
    lang: Lang,
}

pub struct Translations {
    lang: Lang,
    words: Vec<String>,
}

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
    ) -> Self {
        let word = Word {
            word: word,
            lang: word_lang,
        };

        let translations = Translations {
            lang: translation_lang,
            words: translations,
        };
        TranslationRecord {
            word: word,
            translations: translations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_translation_record() {
        let word = "chien".to_string();
        let word_lang = Lang::fr;

        let translations = vec!["hund".to_string(), "köter".to_string()];
        let translation_lang = Lang::de;

        let chien = TranslationRecord::new(word, word_lang, translations.clone(), translation_lang);

        assert_eq!(chien.word.word, "chien");
        assert_eq!(chien.word.lang, Lang::fr);
        assert_eq!(chien.translations.lang, Lang::de);
        for (i, translation) in chien.translations.words.into_iter().enumerate() {
            assert_eq!(translation, translations[i]);
        }
    }

    // no translations? aka empty translations
}
