use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain;
use crate::domain::create_translation::CreateError;
use crate::domain::delete_translation::DeleteError;
use crate::domain::ports::TranslationRepository;
use crate::domain::read_translation::ReadError;
use crate::domain::update_translation::UpdateError;
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
    pub id: Option<String>,
    pub word: String,
    pub lang: Lang,
    pub translations: Vec<String>,
    pub translation_lang: Lang,
}
impl From<TranslationRecord> for TranslationResponse {
    fn from(s: TranslationRecord) -> Self {
        let (id, word, lang, translations, translation_lang) = s.flat();
        TranslationResponse {
            id: id.clone(),
            word: word.clone(),
            lang: lang.clone(),
            translations: translations.clone(),
            translation_lang: translation_lang.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateTranslationRequest {
    pub id: Option<String>,
    #[validate(length(min = 1, message = "Word is required and must be at least 1 character"))]
    pub word: String,
    pub lang: Lang,
    #[validate(length(
        min = 1,
        message = "ingredients is required and must be at least 1 item"
    ))]
    pub translations: Vec<String>,
    pub translation_lang: Lang,
}

pub async fn create_translation<T: TranslationRepository>(
    repository: web::Data<T>,
    request: Json<CreateTranslationRequest>,
) -> Result<Json<TranslationResponse>, ApiError> {
    validate(&request)?;

    let result = domain::create_translation::create_translation(
        repository.get_ref(),
        &request.word,
        &request.lang,
        &request.translations.iter().map(|s| s.as_str()).collect(),
        &request.translation_lang,
    )
    .await;

    result
        .map(|v| respond_json(TranslationResponse::from(v)))
        .map_err(|e| match e {
            CreateError::InvalidInput(s) => ApiError::InvalidInput(s.to_string()),
            CreateError::Read(s) => ApiError::NotFound(s.to_string()),
            CreateError::Create(s) => ApiError::BadRequest(s.to_string()),
            CreateError::Duplicate => ApiError::Conflict(e.to_string()),
        })?
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct RequestTranslationByWord {
    #[validate(length(min = 1, message = "Word is required and must be at least 1 character"))]
    pub word: String,
    pub lang: Lang,
}

pub async fn read_translation<T: TranslationRepository>(
    repository: web::Data<T>,
    request: Json<RequestTranslationByWord>,
) -> Result<Json<TranslationResponse>, ApiError> {
    validate(&request)?;

    let result: Result<TranslationRecord, ReadError> = domain::read_translation::read_translation(
        repository.get_ref(),
        &request.word,
        &request.lang,
    )
    .await;

    result
        .map(|v| respond_json(TranslationResponse::from(v)))
        .map_err(|e| match e {
            ReadError::QueryWord(s) => ApiError::InvalidInput(s.to_string()),
            ReadError::RecordNotFound => ApiError::NotFound(e.to_string()),
            ReadError::Unknown => ApiError::Unknown(e.to_string()),
        })?
}

pub async fn update_translation<T: TranslationRepository>(
    repository: web::Data<T>,
    request: Json<CreateTranslationRequest>,
) -> Result<Json<TranslationResponse>, ApiError> {
    validate(&request)?;

    let result = domain::update_translation::update_translation(
        repository.get_ref(),
        &request.word,
        &request.lang,
        &request.translations.iter().map(|s| s.as_str()).collect(),
        &request.translation_lang,
    )
    .await;

    result
        .map(|v| respond_json(TranslationResponse::from(v)))
        .map_err(|e| match e {
            UpdateError::Word(s) => ApiError::InvalidInput(s.to_string()),
            UpdateError::Read(s) => ApiError::NotFound(s.to_string()),
            UpdateError::Update(s) => ApiError::NotFound(s.to_string()),
        })?
}

pub async fn delete_translation<T: TranslationRepository>(
    repository: web::Data<T>,
    request: Json<RequestTranslationByWord>,
) -> Result<HttpResponse, ApiError> {
    validate(&request)?;

    let result = domain::delete_translation::delete_translation(
        repository.get_ref(),
        &request.word,
        &request.lang,
    )
    .await;

    result
        .map(|_| Ok(HttpResponse::Ok().finish()))
        .map_err(|e| match e {
            DeleteError::Word(s) => ApiError::InvalidInput(s.to_string()),
            DeleteError::Read(s) => ApiError::InvalidInput(s.to_string()),
            DeleteError::Delete(s) => ApiError::Unknown(s.to_string()),
        })?
}
