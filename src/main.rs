use env_logger::Env;
use vocabulaire::config::parse_local_config;
use vocabulaire::server;

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(Env::new().filter_or("VOCI_LOG", "debug"));
    let config = parse_local_config();
    if let Err(e) = server::start_server(config).await {
        eprintln!("Failed to start server: {}", e);
    }
}
