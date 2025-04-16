use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain;
use crate::domain::create_translation::CreateError;
use crate::domain::voci::{Lang, TranslationRecord};

use crate::driving::rest_handler::errors::ApiError;
use crate::driving::rest_handler::validate::validate;

/// Helper function to reduce boilerplate of an OK/Json response
fn respond_json<T>(data: T) -> Result<Json<T>, ApiError>
where
    T: Serialize,
{
    Ok(Json(data))
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TranslationResponse {
    pub word: String,
    pub lang: Lang,
    pub translations: Vec<String>,
    pub translation_lang: Lang,
}
impl From<TranslationRecord> for TranslationResponse {
    fn from(s: TranslationRecord) -> Self {
        let (word, lang, translations, translation_lang) = s.flat();
        TranslationResponse {
            word: word.clone(),
            lang: lang.clone(),
            translations: translations.clone(),
            translation_lang: translation_lang.clone(),
        }
    }
}


#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateTranslationRequest {
    #[validate(length(min = 1, message = "name is required and must be at least 1 character"))]
    pub word: String,

    pub lang: Lang,

    #[validate(length(
        min = 1,
        message = "ingredients is required and must be at least 1 item"
    ))]
    pub translations: Vec<String>,

    pub translation_lang: Lang,
}

pub async fn create_translation(
    request: Json<CreateTranslationRequest>,
) -> Result<Json<TranslationResponse>, ApiError> {
    validate(&request)?;

    let result = domain::create_translation::create_translation(
        &request.word,
        &request.lang,
        &request.translations.iter().map(|s| s.as_str()).collect(), //todo split this to helper function
        &request.translation_lang,
    );

    result
        .map(|v| respond_json(TranslationResponse::from(v)))
        .map_err(|e| match e {
            CreateError::Unknown(m) => ApiError::Unknown(m),
            CreateError::InvalidInput(m) => ApiError::InvalidData(m.to_string()),
        })?
}
