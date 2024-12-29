use actix_files::NamedFile;
use actix_web::{
    middleware::Logger,
    web::{self, Data, JsonConfig},
    {get, App, HttpRequest, HttpResponse, HttpServer, Responder},
};
use db::{init, init_user_db, user_crud::UserRepository};
use dotenv::dotenv;
use handler::poll::poll_results;
// use handler::middleware::auth_middleware::CheckAuth;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
mod db;
mod handler;
mod models;

use crate::db::{config::DbConfig, poll_crud::PollRepository};
use crate::handler::{
    auth::{finish_authentication, finish_register, start_authentication, start_register},
    poll::{add_polls, cast_vote, close_poll, delete_poll, fetch_polls, reset_vote},
};
use crate::models::{auth_state::AuthenticationState, reg_state::RegistrationState};
use actix_cors::Cors;
use webauthn_rs::prelude::*; // Import the CORS middlewar

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
    dotenv().ok();

    //webauthn setup
    let rp_id = "localhost";
    let rp_origin = Url::parse("http://localhost:3000").expect("Invalid URL");
    let builder = WebauthnBuilder::new(rp_id, &rp_origin).expect("Invalid configuration");

    let webauthn = Data::new(builder.build().expect("Invalid configuration"));

    let reg_state_storage = Data::new(RegistrationState::new());
    let auth_state_storeage = Data::new(AuthenticationState::new());
    let config = DbConfig::new(
        "mongodb",
        env::var("DATABASE_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017/?directConnection=true".to_string()),
        "rustest",
    );

    let poll_repo = init(config.clone()).await;
    let user_repo = init_user_db(config).await;

    let store_arc: Arc<dyn PollRepository> = Arc::new(poll_repo);
    let store_data: Data<dyn PollRepository> = Data::from(store_arc);

    let user_store: Arc<dyn UserRepository> = Arc::new(user_repo);
    let user_data: Data<dyn UserRepository> = Data::from(user_store);
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(store_data.clone())
            .app_data(user_data.clone())
            .app_data(reg_state_storage.clone())
            .app_data(auth_state_storeage.clone())
            .app_data(JsonConfig::default())
            .app_data(webauthn.clone())
            .service(root_handler)
            .service(auth_handler)
            .service(
                web::scope("api/auth")
                    .service(start_register)
                    .service(finish_register)
                    .service(start_authentication)
                    .service(finish_authentication),
            )
            .service(
                web::scope("api")
                    //.wrap(CheckAuth)
                    .service(add_polls)
                    .service(fetch_polls)
                    .service(delete_poll)
                    .service(cast_vote)
                    .service(close_poll)
                    .service(reset_vote)
                    .service(poll_results),
            )
            .wrap(
                Cors::default() // Configure CORS to allow all origins
                    .allow_any_origin() // Allows all origins
                    .allow_any_method() // Allows any HTTP method
                    .allow_any_header()
                    .supports_credentials(), // Allows any header
            )
    })
    .bind(("0.0.0.0", port.parse::<u16>().unwrap()))?
    .run()
    .await
}
