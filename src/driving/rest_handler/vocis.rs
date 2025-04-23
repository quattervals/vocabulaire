use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::create_translation::CreateError;
use crate::domain::read_translation::ReadError;
use crate::domain::update_translation::UpdateError;
use crate::domain::voci::{Lang, TranslationRecord};
use crate::{Repository, domain};

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

pub async fn create_translation<T: Repository<TranslationRecord>>(
    repository: web::Data<T>,
    request: Json<CreateTranslationRequest>,
) -> Result<Json<TranslationResponse>, ApiError> {
    validate(&request)?;

    let result = domain::create_translation::create_translation(
        repository,
        &request.word,
        &request.lang,
        &request.translations.iter().map(|s| s.as_str()).collect(), //todo split this to helper function
        &request.translation_lang,
    )
    .await;

    result
        .map(|v| respond_json(TranslationResponse::from(v)))
        .map_err(|e| match e {
            CreateError::Unknown(s) => ApiError::Unknown(s),
            CreateError::InvalidData(s) => ApiError::InvalidData(s.to_string()),
            CreateError::InvalidInput(m) => ApiError::InvalidData(m.to_string()),
        })?
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct RequestTranslationByWord {
    #[validate(length(min = 1, message = "name is required and must be at least 1 character"))]
    pub word: String,
    pub lang: Lang,
}

pub async fn delete_translation(
    request: Json<RequestTranslationByWord>,
) -> Result<HttpResponse, ApiError> {
    validate(&request)?;

    match domain::delete_translation::delete_translation(&request.word, &request.lang) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(ApiError::InvalidData(e.to_string())),
    }
}

pub async fn read_translation(
    request: Json<RequestTranslationByWord>,
) -> Result<Json<TranslationResponse>, ApiError> {
    validate(&request)?;

    let result: Result<TranslationRecord, ReadError> =
        domain::read_translation::read_translation(&request.word, &request.lang);

    result
        .map(|v| respond_json(TranslationResponse::from(v)))
        .map_err(|e| match e {
            ReadError::WordError(e) => ApiError::InvalidData(e.to_string()),
        })?
}

pub async fn update_translation(
    request: Json<CreateTranslationRequest>,
) -> Result<Json<TranslationResponse>, ApiError> {
    validate(&request)?;

    let result = domain::update_translation::update_translation(
        &request.word,
        &request.lang,
        &request.translations.iter().map(|s| s.as_str()).collect(), //todo split this to helper function
        &request.translation_lang,
    );

    result
        .map(|v| respond_json(TranslationResponse::from(v)))
        .map_err(|e| match e {
            UpdateError::WordError(m) => ApiError::InvalidData(m.to_string()),
        })?
}

#[cfg(test)]
mod tests {
    use actix_web::test::TestRequest;
    use actix_web::{App, FromRequest, Handler, Responder, Route, test, web::Data};

    use super::*;
    use crate::driven::repository::mongo_repository::VociMongoRepository;
    use crate::tests::test_utils::shared::*;

    #[actix_web::test]
    async fn create_translation_ok_input_same_translation_returned() {
        let repo = VociMongoRepository::new(&get_testing_persistence_config()).unwrap();

        let create_req = CreateTranslationRequest {
            word: WORD.to_string(),
            lang: WORD_LANG,
            translations: stub_translations(),
            translation_lang: TRANSLATION_LANG,
        };

        let resp: TranslationResponse = execute(
            &repo,
            "/",
            None,
            web::post(),
            TestRequest::post(),
            create_translation::<VociMongoRepository>,
            Some(create_req),
        )
        .await;

        let expected = TranslationRecord::new(
            WORD.to_string(),
            WORD_LANG,
            stub_translations(),
            TRANSLATION_LANG,
        )
        .unwrap();

        assert_on_translation_response(&resp, &expected);
    }

    #[actix_web::test]
    async fn delete_translation_ok_input_http_success() {

        let repo = VociMongoRepository::new(&get_testing_persistence_config()).unwrap();
        let del_req = RequestTranslationByWord {
            word: WORD.to_string(),
            lang: WORD_LANG,
        };

        let r = execute_http(
            &repo,
            "/",
            None,
            web::delete(),
            TestRequest::delete(),
            delete_translation,
            Some(del_req),
        )
        .await;

        assert_eq!(r.status().is_success(), true);
    }

    #[actix_web::test]
    async fn delete_translation_bad_input_http_client_error() {

        let repo = VociMongoRepository::new(&get_testing_persistence_config()).unwrap();
        let del_req = RequestTranslationByWord {
            word: "".to_string(),
            lang: WORD_LANG,
        };

        let r = execute_http(
            &repo,
            "/",
            None,
            web::delete(),
            TestRequest::delete(),
            delete_translation,
            Some(del_req),
        )
        .await;

        assert_eq!(r.status().is_client_error(), true);
    }

    #[actix_web::test]
    async fn read_translation_good_input_http_translation_returned() {

        let repo = VociMongoRepository::new(&get_testing_persistence_config()).unwrap();
        let read_req = RequestTranslationByWord {
            word: "chien".to_string(),
            lang: WORD_LANG,
        };

        let resp: TranslationResponse = execute(
            &repo,
            "/",
            None,
            web::get(),
            TestRequest::get(),
            read_translation,
            Some(read_req),
        )
        .await;

        let expected = TranslationRecord::new(
            WORD.to_string(),
            WORD_LANG,
            stub_translations(),
            TRANSLATION_LANG,
        )
        .unwrap();

        assert_on_translation_response(&resp, &expected);
    }

    #[actix_web::test]
    async fn update_translation_good_input_extended_translation_returned() {

        let repo = VociMongoRepository::new(&get_testing_persistence_config()).unwrap();
        let update_req = CreateTranslationRequest {
            word: WORD.to_string(),
            lang: WORD_LANG,
            translations: stub_translations(),
            translation_lang: TRANSLATION_LANG,
        };

        let resp: TranslationResponse = execute(
            &repo,
            "/",
            None,
            web::put(),
            TestRequest::put(),
            update_translation,
            Some(update_req),
        )
        .await;

        let mut updated_translations = vec!["köter".to_string(), "waldi".to_string()];
        updated_translations.append(&mut stub_translations());

        let expected = TranslationRecord::new(
            WORD.to_string(),
            WORD_LANG,
            updated_translations,
            TRANSLATION_LANG,
        )
        .unwrap();

        assert_on_translation_response(&resp, &expected);
    }

    /// Execute a test request and return HttpResponse
    async fn execute_http<F, Args, R>(
        repo: &R,
        path: &str,
        uri_to_call: Option<&str>,
        http_method: Route,
        test_req: TestRequest,
        handler: F,
        recipe_req: Option<impl Serialize>,
    ) -> HttpResponse
    where
        R: Repository<TranslationRecord> + Send + Sync + 'static + Clone,
        F: Handler<Args>,
        Args: FromRequest + 'static,
        F::Output: Responder,
    {
        // init the service
        let app = test::init_service(App::new().app_data(Data::new(repo.clone())).route(path, http_method.to(handler))).await;


        // Set URI
        let req = match uri_to_call {
            Some(uri) => test_req.uri(uri),
            None => test_req,
        };

        // Set the JSON body if provided
        let req = match recipe_req {
            Some(ref r) => req.set_json(recipe_req.unwrap()),
            None => req,
        };

        // Call the service and get the response
        let response = test::call_service(&app, req.to_request()).await;

        // Extract the HttpResponse from the ServiceResponse
        response.into_parts().1
    }

    /// execute a test request
    async fn execute<F, Args, R, Ret>(
        repo: &R,
        path: &str,
        uri_to_call: Option<&str>,
        http_method: Route,
        test_req: TestRequest,
        handler: F,
        recipe_req: Option<impl Serialize>,
    ) -> Ret
    where
        R: Repository<TranslationRecord> + Send + Sync + 'static + Clone,
        F: Handler<Args>,
        Args: FromRequest + 'static,
        F::Output: Responder,
        Ret: for<'de> Deserialize<'de>,
    {
        // init service
        let app = test::init_service(App::new().app_data(Data::new(repo.clone())).route(path, http_method.to(handler))).await;

        // set uri
        let req = match uri_to_call {
            Some(uri) => test_req.uri(uri),
            None => test_req,
        };

        // Set json body
        let req = match recipe_req {
            Some(ref r) => req.set_json(recipe_req.unwrap()),
            None => req,
        };

        test::call_and_read_body_json(&app, req.to_request()).await
    }

    fn assert_on_translation_response(actual: &TranslationResponse, expected: &TranslationRecord) {
        let (word, lang, translations, translation_lang) = expected.flat();
        assert_eq!(&actual.word, word);
        assert_eq!(&actual.lang, lang);
        assert_on_translations(&actual.translations, &translations);
        assert_eq!(&actual.translation_lang, translation_lang);
    }

    fn assert_on_translations(actual: &Vec<String>, expected: &Vec<String>) {
        assert_eq!(actual.len(), expected.len());
        for (i, item) in expected.iter().enumerate() {
            assert_eq!(&actual[i], item);
        }
    }
}
