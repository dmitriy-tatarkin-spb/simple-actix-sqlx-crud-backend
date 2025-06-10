mod auth_validator;
mod base;
mod config;
mod db;
mod dto;
mod errors;
mod todos;

use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;

use auth_validator::{validator, AppState};
use config::{load, Config};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Config = load();
    let app_state = web::Data::new(AppState {
        jwt_secret: config.secret_key.to_string(),
    });

    let auth_bearer = HttpAuthentication::bearer(validator);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(auth_bearer.clone())
            .configure(base::routes)
            .configure(todos::routes)
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
