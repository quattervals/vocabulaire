use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};

use crate::driving::rest_handler;

mod domain;
mod driving;
mod driven;
mod tests;
mod config;

#[actix_web::main]
async fn main() {
    unsafe {
        std::env::set_var("RUST_LOG", "debug");

    env_logger::init();}

    create_server()
        .await
        .unwrap()
        .await
        .expect("An error occurred while starting the web application");
}


async fn create_server() -> Result<Server, std::io::Error> {

    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes)
    }).bind(("127.0.0.1", 8080))?
        .run();
    Ok(server)
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/voci")
                .service(
                    web::scope("/api/v1")
                        .service(
                            web::resource("translations")
                                .route(web::get().to(rest_handler::vocis::read_translation))
                                .route(web::post().to(rest_handler::vocis::create_translation))
                                .route(web::delete().to(rest_handler::vocis::delete_translation))
                                .route(web::put().to(rest_handler::vocis::update_translation))
                        ).service(
                        web::resource("translations/{id}")
                            // .route(web::get().to(rest_handler::sandwiches::get_by_id))
                            // .route(web::delete().to(rest_handler::sandwiches::delete_one_sandwich))
                    )
                )
        );
}

#[cfg(test)]
mod unittests {
    #[test]
    fn test_hello_world() {
        assert!(true); // Placeholder test
    }
}
