use cucumber::{World, given, then, when};
use reqwest::{Client, Response, StatusCode};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// Outer layer interna
use vocabulaire::domain::ports::TranslationRepository;
use vocabulaire::driven::repository::mongo_repository;

/// Testing utils
use vocabulaire::test_utils::utils::shared;

/// Interna used for testing convenience
use vocabulaire::driving::rest_handler::vocis::CreateTranslationRequest;

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
    let req = read_translation_request_from_file("tests/resources/chien.json").await;
    let client = Client::new();
    let url = "http://localhost:8082/voci/api/v1/translations";
    let response = client.post(url).json(&req).send().await;

    match response {
        Ok(r) => world.server_response = Some(r),
        Err(e) => println!("{:#?}", e),
    }
}

#[then("the operation should succeed")]
async fn is_ok(world: &mut DatabaseWorld) {
    assert_eq!(
        world.server_response.as_ref().unwrap().status(),
        StatusCode::OK
    );
}

#[then("the opration is a client error")]
async fn is_client_error(world: &mut DatabaseWorld) {
    assert!(
        world
            .server_response
            .as_ref()
            .unwrap()
            .status()
            .is_client_error()
    );
}

#[then("is duplicate")]
async fn is_duplicate(world: &mut DatabaseWorld) {
    assert_eq!(
        world.server_response.as_ref().unwrap().status(),
        StatusCode::CONFLICT
    );
}

#[tokio::main]
async fn main() {
    DatabaseWorld::run("tests/features/voci.feature").await;
}

async fn read_translation_request_from_file(file_path: &str) -> CreateTranslationRequest {
    let mut file = File::open(file_path).await.expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .await
        .expect("Unable to read file");

    serde_json::from_str(&contents).expect("Unable to parse JSON")
}
