#[cfg(test)]
pub mod repo_double {
    use async_trait::async_trait;
    use std::cell::RefCell;

    use crate::config::PersistenceConfig;
    use crate::domain::voci::{TranslationRecord, Word};
    use crate::driven::repository::{
        RepoCreateError, RepoReadError, Repository,
    };
    use crate::tests::test_utils::shared::*;

    struct Wrap(RefCell<bool>);

    unsafe impl Sync for Wrap {}

    pub struct VociRepoDouble {
        has_error: Wrap,
    }

    impl VociRepoDouble {
        pub fn set_error(&mut self, value: bool) {
            *self.has_error.0.borrow_mut() = value;
        }
    }

    #[async_trait]
    impl Repository<TranslationRecord> for VociRepoDouble {
        fn new(_config: &PersistenceConfig) -> Result<Self, String>
        where
            Self: Sized,
        {
            Ok(VociRepoDouble {
                has_error: Wrap(RefCell::from(false)),
            })
        }

        async fn create(
            &self,
            tr: &TranslationRecord,
        ) -> Result<TranslationRecord, RepoCreateError> {
            if self.has_error.0.take() {
                return Err(RepoCreateError::Unknown(String::from("Error occurred")));
            }

            let (id, word, lang, translations, translation_lang) = tr.flat();

            let s = TranslationRecord::new(
                Some(TRANSLATION_ID.to_string()),
                word.clone(),
                lang.clone(),
                translations.clone(),
                translation_lang.clone(),
            )
            .unwrap();

            Ok(s)
        }

        async fn read_by_word(&self, find_tr: &Word) -> Result<TranslationRecord, RepoReadError> {

            Err(RepoReadError::Unknown("dunno".to_string()))
        }
    }
}
