pub mod config;
pub mod domain;
pub mod driven;
pub mod driving;
pub mod test_utils;

pub mod server {

    use actix_web::dev::Server;
    use actix_web::middleware::Logger;
    use actix_web::{App, HttpServer, web, web::Data};

    use crate::config;
    use crate::domain::ports::TranslationRepository;
    use crate::driven::repository::mongo_repository::VociMongoRepository;
    use crate::driving::rest_handler;

    pub async fn start_server(config: config::Config) -> Result<Server, std::io::Error> {
        let repo = VociMongoRepository::new(&config.persistence).unwrap();

        create_server(repo).await
    }

    async fn create_server(repo: impl TranslationRepository) -> Result<Server, std::io::Error> {
        let server = HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(Data::new(repo.clone()))
                .configure(routes)
        })
        .bind(("127.0.0.1", 8080))?
        .run();
        Ok(server)
    }

    fn routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/voci").service(
                web::scope("/api/v1")
                    .service(
                        web::resource("translations")
                            .route(
                                web::get().to(rest_handler::vocis::read_translation::<
                                    VociMongoRepository,
                                >),
                            )
                            .route(
                                web::post().to(rest_handler::vocis::create_translation::<
                                    VociMongoRepository,
                                >),
                            )
                            .route(
                                web::delete().to(rest_handler::vocis::delete_translation::<
                                    VociMongoRepository,
                                >),
                            )
                            .route(web::put().to(rest_handler::vocis::update_translation::<
                                VociMongoRepository,
                            >)),
                    )
                    .service(web::resource("translations/{id}")),
            ),
        );
    }
}
