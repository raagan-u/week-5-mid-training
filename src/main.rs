use actix_files::NamedFile;
use actix_web::{
    middleware::Logger,
    web::{Data, JsonConfig},
    {get, App, HttpRequest, HttpResponse, HttpServer, Responder},
};
use startup::startup;

use std::path::PathBuf;
mod handler;
mod models;
mod startup;
use crate::handler::auth::{
    finish_authentication, finish_register, start_authentication, start_register,
};
use crate::models::{AuthenticationState, RegistrationState};
use actix_cors::Cors; // Import the CORS middlewar

#[get("/")]
pub async fn root_handler(req: HttpRequest) -> impl Responder {
    let path: PathBuf = PathBuf::from("src/static/index.html");
    match NamedFile::open(path) {
        Ok(file) => file.into_response(&req),
        Err(_) => HttpResponse::InternalServerError().body("Could not open static file"),
    }
}

#[get("/api")]
pub async fn auth_handler() -> impl Responder {
    HttpResponse::Ok().json("1.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }

    // Initialize env-logger
    env_logger::init();

    // Generate secret key for cookies.
    // Normally you would read this from a configuration file.

    let (webauthn, webauthn_users) = startup();
    let reg_state_storage = Data::new(RegistrationState::new());
    let auth_state_storeage = Data::new(AuthenticationState::new());
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(reg_state_storage.clone())
            .app_data(auth_state_storeage.clone())
            .app_data(JsonConfig::default())
            .app_data(webauthn.clone())
            .app_data(webauthn_users.clone())
            .service(root_handler)
            .service(auth_handler)
            .service(start_register)
            .service(finish_register)
            .service(start_authentication)
            .service(finish_authentication)
            .wrap(
                Cors::default() // Configure CORS to allow all origins
                    .allow_any_origin() // Allows all origins
                    .allow_any_method() // Allows any HTTP method
                    .allow_any_header()
                    .supports_credentials(), // Allows any header
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
