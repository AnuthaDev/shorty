use actix_web::web;
use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/")
                .route(web::get().to(handlers::index))
        )
        .service(
            web::resource("/health")
                .route(web::get().to(handlers::health))
        )
        .service(
            web::resource("/api/shorten")
                .route(web::post().to(handlers::shorten_url))
        )
        .service(
            web::resource("/{short_code}")
                .route(web::get().to(handlers::redirect))
        );
}
