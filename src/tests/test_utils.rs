#[cfg(test)]

pub mod shared {

    use actix_web::web::Data;
    use std::path::PathBuf;

    use crate::config::{Config, PersistenceConfig, parse_config};
    use crate::domain::voci::{Lang, TranslationId, TranslationRecord};

    use crate::driven::repository::Repository;
    use crate::tests::voci_repo_double::repo_double::VociRepoDouble;

    /// Constants

    pub const TRANSLATION_ID: &str = "123";
    pub const EMPTY_TRANSLATION_ID: &str = "";
    pub const WORD: &str = "chien";
    pub const WORD_LANG: Lang = Lang::fr;
    pub const TRANSLATIONS: [&str; 2] = ["hund", "köter"];
    pub const TRANSLATION_LANG: Lang = Lang::de;

    pub fn stub_translations() -> Vec<String> {
        TRANSLATIONS.iter().map(|i| i.to_string()).collect()
    }

    pub fn stub_translation_record(with_id: bool) -> TranslationRecord {
        //todo: add bool for ID
        TranslationRecord::new(
            if with_id {
                Some(TRANSLATION_ID.to_string())
            } else {
                None
            },
            WORD.to_string(),
            WORD_LANG,
            stub_translations(),
            TRANSLATION_LANG,
        )
        .unwrap()
    }

    pub fn get_testing_persistence_config() -> PersistenceConfig {
        get_testing_config().persistence
    }

    pub fn get_testing_config() -> Config {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/tests/test_config.toml");
        parse_config(d)
    }

    pub fn double_repo_data() -> Data<VociRepoDouble> {
        let repo = VociRepoDouble::new(&get_testing_persistence_config()).unwrap();
        Data::new(repo)
    }
}
