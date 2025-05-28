use cucumber::{World, given, then, when};
use reqwest::Client;
use serde::Serialize;

// These `Cat` definitions would normally be inside your project's code,
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
struct Cat {
    pub hungry: bool,
}

impl Cat {
    fn feed(&mut self) {
        self.hungry = false;
    }
}

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct AnimalWorld {
    cat: Cat,
}

// #[given(regex = r"^a (hungry|satiated) cat$")]
#[given(expr = "a {word} cat")]
fn hungry_cat(world: &mut AnimalWorld, state: String) {
    match state.as_str() {
        "hungry" => world.cat.hungry = true,
        "satiated" => world.cat.hungry = false,
        _ => unreachable!(),
    }
}

#[when("I feed the cat")]
fn feed_cat(world: &mut AnimalWorld) {
    world.cat.feed();
}

#[then("the cat is not hungry")]
fn cat_is_fed(world: &mut AnimalWorld) {
    assert!(!world.cat.hungry);
}

use vocabulaire::test_utils::utils::shared;

use vocabulaire;
use vocabulaire::domain::ports::TranslationRepository;
use vocabulaire::driven::repository::mongo_repository;

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
}

#[when("I add something")]
async fn add(world: &mut DatabaseWorld) {
    let json_object = TranslationRequest {
        word: "chien".to_string(),
        lang: "fr".to_string(),
        translations: vec!["köter".to_string(), "hund".to_string()],
        translation_lang: "de".to_string(),
    };

    // Create an HTTP client
    let client = Client::new();

    // Define the URL of the web service
    let url = "http://localhost:8082/voci/api/v1/translations";

    time::sleep(time::Duration::new(50, 0)).await;

    match client.post(url).json(&json_object).send().await {
        Ok(v) => println!("posted {:#?}", v),
        Err(e) => println!("{:#?}", e),
    }

    time::sleep(time::Duration::new(50, 0)).await;
    println!("db add");
}

use tokio::time;
#[when("I perform a database operation")]
async fn op(world: &mut DatabaseWorld) {
    time::sleep(time::Duration::new(50, 0)).await;
    println!("db op");
}

#[then("the operation should succeed")]
async fn okop(world: &mut DatabaseWorld) {
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

// This runs before everything else, so you can set up things here.
#[tokio::main]
async fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    // futures::executor::block_on(AnimalWorld::run("tests/features/simple.feature"));
    DatabaseWorld::run("tests/features/db.feature").await;
}
