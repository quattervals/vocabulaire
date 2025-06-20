pub mod shared {

    use mongodb::Collection;
    use mongodb::bson::doc;
    use std::path::PathBuf;

    use crate::config::{Config, PersistenceConfig, parse_config};
    use crate::domain::ports::TranslationRepository;
    use crate::domain::voci::{Lang, TranslationRecord};
    use crate::driven::repository::mongo_repository::VociMongoRepository;

    /// Constants
    pub const TRANSLATION_ID: &str = "123";
    pub const _EMPTY_TRANSLATION_ID: &str = "";
    pub const WORD: &str = "chien";
    pub const WORD_LANG: Lang = Lang::fr;
    pub const TRANSLATIONS: [&str; 2] = ["hund", "köter"];
    pub const ADDITONAL_TRANSLATIONS: [&str; 2] = ["Schäfer", "Jagdhund"];
    pub const TRANSLATION_LANG: Lang = Lang::de;

    pub fn stub_translation_record(with_id: bool) -> TranslationRecord {
        TranslationRecord::new(
            if with_id { Some(TRANSLATION_ID) } else { None },
            WORD,
            &WORD_LANG,
            &TRANSLATIONS,
            &TRANSLATION_LANG,
        )
        .unwrap()
    }

    pub fn assert_on_translations(actual: &[String], expected: &[String]) {
        assert_eq!(actual.len(), expected.len());
        for (i, item) in expected.iter().enumerate() {
            assert_eq!(&actual[i], item);
        }
    }

    pub fn assert_on_translation_record(
        actual: &TranslationRecord,
        expected: &TranslationRecord,
        check_id: bool,
    ) {
        let actual = actual.flat();
        let expected: (&Option<String>, &String, &Lang, &Vec<String>, &Lang) = expected.flat();

        if check_id {
            assert_eq!(actual.0.as_ref().unwrap(), expected.0.as_ref().unwrap());
        }
        assert_eq!(actual.1, expected.1);
        assert_eq!(actual.2, expected.2);
        assert_on_translations(actual.3, expected.3);
        assert_eq!(actual.2, expected.2);
    }

    pub fn get_testing_persistence_config() -> PersistenceConfig {
        get_testing_config().persistence
    }

    pub fn get_testing_config() -> Config {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/test_utils/test_config.toml");
        parse_config(d)
    }

    pub async fn setup_repo() -> VociMongoRepository {
        let config = get_testing_persistence_config();
        let repo: VociMongoRepository = VociMongoRepository::new(&config).unwrap();

        delete_collection(config, &repo).await;

        repo
    }

    pub async fn delete_collection(config: PersistenceConfig, repo: &VociMongoRepository) {
        let collection = repo.get_collection().await;
        let coll: Collection<VociMongoRepository> = collection
            .client()
            .database(&config.database)
            .collection(&config.schema_collection);
        coll.delete_many(doc! {}).await.unwrap();
    }
}
