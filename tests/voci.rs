use cucumber::{World, given, then, when};
use reqwest::{Client, Response, StatusCode};
use serde::Serialize;

use vocabulaire::domain::ports::TranslationRepository;
use vocabulaire::driven::repository::mongo_repository;
use vocabulaire::test_utils::utils::shared;

#[derive(Default, Debug, World)]
pub struct DatabaseWorld {
    repo: Option<mongo_repository::VociMongoRepository>,
    server_response: Option<Response>,
}

#[given("a clean database is available")]
async fn setup_database(world: &mut DatabaseWorld) {
    let persistence_config = shared::get_testing_persistence_config();

    let repo = mongo_repository::VociMongoRepository::new(&persistence_config).unwrap();

    shared::delete_collection(persistence_config, &repo).await;
    world.repo = Some(repo);
}

#[given("the server is started")]
async fn start_server(world: &mut DatabaseWorld) {
    let repo_clone = world.repo.as_ref().unwrap().clone();

    tokio::spawn(async move {
        match vocabulaire::server::create_server(repo_clone).await {
            Ok(server_future) => {
                if let Err(e) = server_future.await {
                    eprintln!("Error while running the server: {:?}", e);
                }
            }
            Err(e) => eprintln!("Error during server creation: {:?}", e),
        }
    });
}

#[when("I add a complete translation")]
async fn add(world: &mut DatabaseWorld) {
    let json_object = TranslationRequest {
        word: "chien".to_string(),
        lang: "fr".to_string(),
        translations: vec!["köter".to_string(), "hund".to_string()],
        translation_lang: "de".to_string(),
    };

    let client = Client::new();

    let url = "http://localhost:8082/voci/api/v1/translations";

    let response = client.post(url).json(&json_object).send().await;

    match response {
        Ok(r) => world.server_response = Some(r),
        Err(e) => println!("{:#?}", e),
    }
}

#[when("I perform a database operation")]
async fn op(_world: &mut DatabaseWorld) {
    println!("db op");
}

#[then("the operation should succeed")]
async fn okop(world: &mut DatabaseWorld) {
    let resp = world.server_response.as_ref().unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[derive(Serialize)]
struct TranslationRequest {
    word: String,
    lang: String,
    translations: Vec<String>,
    translation_lang: String,
}

#[tokio::main]
async fn main() {
    DatabaseWorld::run("tests/features/voci.feature").await;
}
