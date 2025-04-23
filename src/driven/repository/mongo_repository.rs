use std::str::FromStr;

use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{Document, doc};
use mongodb::error::Error;
use mongodb::{Client, Collection, bson};
use serde::{Deserialize, Serialize};

use crate::config::PersistenceConfig;
use crate::domain::voci::{Lang, TranslationRecord, TranslationRecordError};
use crate::driven::repository::{RepoCreateError, Repository};

use super::FindTranslationRecord;

#[derive(Debug, Serialize, Deserialize)]
pub struct VociMongo {
    // todo: is this a good name? should include something like TranslationRecord + Mongo
    _id: ObjectId,
    word: String,
    lang: Lang,
    translations: Vec<String>,
    translation_lang: Lang,
}

impl From<TranslationRecord> for VociMongo {
    fn from(tr: TranslationRecord) -> Self {
        let object_id = match tr.id().value() {
            Some(id) => ObjectId::parse_str(id).unwrap(),
            None => ObjectId::new(),
        };

        let (_, word, lang, translations, translation_lang) = tr.flat();

        VociMongo {
            _id: object_id,
            word: word.clone(),
            lang: lang.clone(),
            translations: translations.clone(),
            translation_lang: translation_lang.clone(),
        }
    }
}

impl TryInto<TranslationRecord> for VociMongo {
    type Error = TranslationRecordError;
    fn try_into(self) -> Result<TranslationRecord, Self::Error> {
        TranslationRecord::new(
            Some(self._id.to_string()),
            self.word,
            self.lang,
            self.translations,
            self.translation_lang,
        )
    }
}

#[derive(Clone)]
pub struct VociMongoRepository {
    database: String,
    collection: String,
    conn_uri: String,
}

impl VociMongoRepository {
    async fn open_connection(&self) -> Client {
        let c = Client::with_uri_str(&self.conn_uri);
        c.await
            .expect("Error while opening the connection to MongoDB")
    }

    async fn get_collection(&self) -> Collection<VociMongo> {
        let client = self.open_connection().await;
        client.database(&self.database).collection(&self.collection)
    }

    // todo: needed to find a document
    //fn compose_document_from_translationrecord(&self, tr: FindTranslationRecord) -> Result<Document, Error>
    // {}
}

#[async_trait]
impl Repository<TranslationRecord> for VociMongoRepository {
    fn new(config: &PersistenceConfig) -> Result<Self, String>
    where
        Self: Sized,
    {
        config.validate()?;
        let config = config.clone();

        let conn_uri = create_connection_uri(&config);

        Ok(VociMongoRepository {
            database: config.database,
            collection: config.schema_collection,
            conn_uri: conn_uri,
        })
    }

    async fn create(&self, tr: TranslationRecord) -> Result<TranslationRecord, RepoCreateError> {
        let voci_mongo = VociMongo::from(tr.clone());
        let translation_collection = self.get_collection().await;

        let result = translation_collection.insert_one(voci_mongo).await;

        let inserted_id = match result {
            Ok(id) => id.inserted_id.as_object_id().unwrap(),
            Err(e) => return Err(RepoCreateError::Unknown(e.to_string())),
        };

        let (_, word, lang, translations, translation_lang) = tr.flat();

        let created_tr = TranslationRecord::new(
            Some(inserted_id.to_string()),
            word.clone(),
            lang.clone(),
            translations.clone(),
            translation_lang.clone(),
        )
        .unwrap();
        Ok(created_tr)
    }
}

/// create connection uri
fn create_connection_uri(config: &PersistenceConfig) -> String {
    format!(
        "mongodb://{}:{}@{}/{}",
        config.user,
        config.password,
        match config.port {
            None => config.host.to_string(),
            Some(port) => config.host.clone() + ":" + &port.to_string(),
        },
        config.auth_db
    )
}

#[cfg(test)]
mod tests {
    use crate::tests::test_utils::shared::{
        get_testing_persistence_config, stub_translation_record,
    };
    use serial_test::serial;

    use super::*;

    #[test]
    #[serial]
    fn new_repo_ok_config_repo_created() {
        let config = get_testing_persistence_config();
        let repo: VociMongoRepository = Repository::<TranslationRecord>::new(&config).unwrap();
        assert_eq!(
            repo.conn_uri,
            "mongodb://root:tran5lation5@localhost:27017/admin"
        );
        assert_eq!(repo.collection, "test_translation")
    }

    #[test]
    #[serial]
    fn new_repo_bad_config_error() {
        let mut config = get_testing_persistence_config();
        config.host = "".to_string();
        let result: Result<VociMongoRepository, String> =
            Repository::<TranslationRecord>::new(&config);

        assert_eq!(result.is_err(), true);
    }

    #[serial]
    #[actix_rt::test]
    async fn create_ok_parameters_record_created() {
        let repo: VociMongoRepository =
            Repository::<TranslationRecord>::new(&get_testing_persistence_config()).unwrap();

        let result = repo.create(stub_translation_record(false)).await.unwrap();
        let result = result.flat();
        let expected = stub_translation_record(true);
        let expected = expected.flat();

        assert_eq!(result.1, expected.1);
        assert_eq!(result.2, expected.2);
        assert_eq!(result.3, expected.3);
        assert_eq!(result.4, expected.4);
    }
}
