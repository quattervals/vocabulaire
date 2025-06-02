use cucumber::{World, given, then, when};
use reqwest::{Client, StatusCode};
use std::net::TcpListener;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::oneshot;
use std::path::{Path, PathBuf};

/// Outer layer interna
use vocabulaire::domain::ports::TranslationRepository;
use vocabulaire::driven::repository::mongo_repository;

/// Testing utils
use vocabulaire::test_utils::utils::shared;

/// Interna used for testing convenience
use vocabulaire::driving::rest_handler::vocis::CreateTranslationRequest;

const SERVER_URL: &str = "http://localhost";
const API_ROUTE: &str = "voci/api/v1/translations";
const TEST_FILES: &str = "tests/features";
const TEST_RESOURCES: &str = "tests/resources";

#[derive(Default, Debug, World)]
pub struct DatabaseWorld {
    repo: Option<mongo_repository::VociMongoRepository>,
    connection_port: Option<u16>,

    shutdown_tx: Option<oneshot::Sender<()>>,
    server_handle: Option<tokio::task::JoinHandle<()>>,

    server_bytes: Option<actix_web::web::Bytes>,
    server_status: StatusCode,
    sent_json: Option<serde_json::Value>,
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
    world.connection_port = Some(port);

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
    let port = world.connection_port.unwrap_or(8082);
    let client = Client::new();

    let request = json_from_file(Path::new(TEST_RESOURCES).join("chien.json")).await;
    let req = json_to_translation_request(request.clone()).await;

    let url = format!("{SERVER_URL}:{port}/{API_ROUTE}");

    let response = client.post(url).json(&req).send().await;

    match response {
        Ok(r) => {
            let status_code = r.status();
            let bytes = r.bytes().await;

            world.server_status = status_code;
            world.server_bytes = bytes.ok();
        }
        Err(e) => println!("{:#?}", e),
    }
    world.sent_json = Some(request);
}

#[when("I read an existing word")]
async fn read_existing_word(world: &mut DatabaseWorld) {}

#[then(expr = r"the http response is {string}")]
async fn http_response(world: &mut DatabaseWorld, status_code: String) {
    let code = match status_code.as_ref() {
        "OK" => StatusCode::OK,
        "CONFLICT" => StatusCode::CONFLICT,
        _ => StatusCode::NOT_IMPLEMENTED,
    };

    assert_eq!(world.server_status, code);
}

#[then(expr = r"the http response class is {string}")]
async fn http_response_class(world: &mut DatabaseWorld, status_class: String) {
    let status_fn: fn(&StatusCode) -> bool = match status_class.as_ref() {
        "Client Error" => StatusCode::is_client_error,
        _ => StatusCode::is_success,
    };

    assert!(status_fn(&world.server_status));
}

#[then("the same translation record is returned")]
async fn got_same_translation(world: &mut DatabaseWorld) {
    let json_value: Option<serde_json::Value> = world
        .server_bytes
        .as_ref()
        .and_then(|b| serde_json::from_slice(b).ok());

    let original_request = world.sent_json.as_ref();

    let keys_equal = ["word", "lang", "translations"];
    let fields_equal = compare_fields_by_key(
        original_request.unwrap(),
        json_value.as_ref().unwrap(),
        &keys_equal,
    );

    assert!(fields_equal);
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
        .run(format!("{TEST_FILES}/create.feature"))
        .await;

    DatabaseWorld::cucumber()
        .max_concurrent_scenarios(1)
        .after(|_feature, _rule, _scenario, _ev, world| {
            Box::pin(async move {
                if let Some(world) = world {
                    shutdown_server(world).await;
                }
            })
        })
       .run(format!("{TEST_FILES}/rud.feature"))
        .await;
}

async fn json_from_file(file_path: PathBuf) -> serde_json::Value {
    let mut file = File::open(file_path).await.expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .await
        .expect("Unable to read file");

    serde_json::from_str(&contents).expect("Unable to parse JSON")
}

async fn json_to_translation_request(tr: serde_json::Value) -> CreateTranslationRequest {
    serde_json::from_value(tr).expect("unable to convert from json to request")
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
    let url = format!("{SERVER_URL}:{port}/{API_ROUTE}");

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

fn compare_fields_by_key(
    json1: &serde_json::Value,
    json2: &serde_json::Value,
    keys: &[&str],
) -> bool {
    for &key in keys {
        let value1 = json1.get(key);
        let value2 = json2.get(key);

        let are_equal = match (value1, value2) {
            (Some(v1), Some(v2)) => v1 == v2,
            (None, None) => false,
            _ => false,
        };

        if !are_equal {
            return false;
        }
    }
    true
}
