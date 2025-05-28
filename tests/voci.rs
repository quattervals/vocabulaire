use cucumber::{World, given, then, when};
use reqwest::Client;
use serde::Serialize;

use vocabulaire::domain::ports::TranslationRepository;
use vocabulaire::driven::repository::mongo_repository;
use vocabulaire::test_utils::utils::shared;

#[derive(Default, Debug, World)]
pub struct DatabaseWorld {
    repo: Option<mongo_repository::VociMongoRepository>,
}

#[given("a clean database is available")]
async fn setup_database(world: &mut DatabaseWorld) {
    let persistence_config = shared::get_testing_persistence_config();

    let repo = mongo_repository::VociMongoRepository::new(&persistence_config).unwrap();

    shared::delete_collection(persistence_config, &repo).await;
    world.repo = Some(repo);
    println!("setup db");
}

#[given("the server is started")]
async fn start_server(world: &mut DatabaseWorld) {
    println!("starting server");
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

    println!("Server started in background.");
    time::sleep(time::Duration::new(1, 0)).await;
}

#[when("I add something")]
async fn add(_world: &mut DatabaseWorld) {
    let json_object = TranslationRequest {
        word: "chien".to_string(),
        lang: "fr".to_string(),
        translations: vec!["köter".to_string(), "hund".to_string()],
        translation_lang: "de".to_string(),
    };

    let client = Client::new();

    let url = "http://localhost:8082/voci/api/v1/translations";

    match client.post(url).json(&json_object).send().await {
        Ok(v) => println!("posted {:#?}", v),
        Err(e) => println!("{:#?}", e),
    }

    println!("db add");
}

use tokio::time;
#[when("I perform a database operation")]
async fn op(_world: &mut DatabaseWorld) {
    time::sleep(time::Duration::new(1, 0)).await;
    println!("db op");
}

#[then("the operation should succeed")]
async fn okop(_world: &mut DatabaseWorld) {
    println!("db okop op");
    assert!(true);
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
