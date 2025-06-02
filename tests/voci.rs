use cucumber::{World, given, then, when};
use reqwest::{Client, Response, StatusCode};
use std::net::TcpListener;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::oneshot;

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
    server_port: Option<u16>,
    shutdown_tx: Option<oneshot::Sender<()>>,
    server_handle: Option<tokio::task::JoinHandle<()>>,
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
    let repo = world.repo.as_ref().unwrap().clone();
    let port = get_available_port();
    world.server_port = Some(port);

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let handle = tokio::spawn(async move {
        match vocabulaire::server::create_server(repo, port).await {
            Ok(server_future) => {
                tokio::select! {
                    result = server_future => {
                        if let Err(e) = result {
                            eprintln!("Server error on port {port}: {e:?}");
                        }
                    }
                    _ = shutdown_rx => {}
                }
            }
            Err(e) => eprintln!("Error creating server on port {port}: {e:?}"),
        }
    });

    world.shutdown_tx = Some(shutdown_tx);
    world.server_handle = Some(handle);

    wait_for_server_on_port(port).await;
}

#[when("I add a complete translation")]
async fn add(world: &mut DatabaseWorld) {
    let port = world.server_port.unwrap_or(8082);
    let req = read_translation_request_from_file("tests/resources/chien.json").await;
    let client = Client::new();
    let url = format!("http://localhost:{}/voci/api/v1/translations", port); //todo: work with better strings
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
    DatabaseWorld::cucumber()
        .max_concurrent_scenarios(1)
        .after(|_feature, _rule, _scenario, _ev, world| {
            Box::pin(async move {
                if let Some(world) = world {
                    shutdown_server(world).await;
                }
            })
        })
        .run("tests/features/voci.feature")
        .await;
}

async fn read_translation_request_from_file(file_path: &str) -> CreateTranslationRequest {
    let mut file = File::open(file_path).await.expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .await
        .expect("Unable to read file");

    serde_json::from_str(&contents).expect("Unable to parse JSON")
}

fn get_available_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

async fn wait_for_server_on_port(port: u16) {
    let client = Client::new();
    let url = format!("http://localhost:{port}/voci/api/v1/translations"); // todo better url

    for attempt in 1..=30 {
        match client.get(&url).send().await {
            Ok(_) => {
                println!("Server ready on port {} after {} attempts", port, attempt);
                return;
            }
            Err(_) => {
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        }
    }
    panic!("Server failed to start on port {port} within reasonable time");
}

async fn shutdown_server(world: &mut DatabaseWorld) {
    if let Some(shutdown_tx) = world.shutdown_tx.take() {
        let _ = shutdown_tx.send(());
    }

    if let Some(handle) = world.server_handle.take() {
        if let Err(e) = tokio::time::timeout(tokio::time::Duration::from_secs(5), handle).await {
            println!("Server shutdown error {e}");
        }
    }

    // Give the port time to be released
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
}
