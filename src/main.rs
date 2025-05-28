use env_logger::Env;
use vocabulaire::config::parse_local_config;
use vocabulaire::domain::ports::TranslationRepository;
use vocabulaire::driven::repository::mongo_repository::VociMongoRepository;
use vocabulaire::server;

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(Env::new().filter_or("VOCI_LOG", "debug"));

    let config = parse_local_config();

    let repo = VociMongoRepository::new(&config.persistence).unwrap();

    server::create_server(repo)
        .await
        .unwrap()
        .await
        .expect("An error occurred while starting the web application");
}
