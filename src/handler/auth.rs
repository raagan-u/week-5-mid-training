use crate::handler::{Error, WebResult};
use crate::models::jwt::encode_jwt;
use crate::models::{auth_state::AuthenticationState, reg_state::RegistrationState};
use crate::startup::UserData;
use actix_web::error::ErrorInternalServerError;
use actix_web::post;
use actix_web::web::{Data, Json, Path};
use actix_web::HttpResponse;
use log::info;
use serde_json::json;
use tokio::sync::Mutex;
use webauthn_rs::prelude::*;

#[post("start_reg/{username}")]
pub(crate) async fn start_register(
    username: Path<String>,
    reg_state_storage: Data<RegistrationState>,
    webauthn_users: Data<Mutex<UserData>>,
    webauthn: Data<Webauthn>,
) -> WebResult<Json<CreationChallengeResponse>> {
    info!("Start register");
    reg_state_storage.remove("reg_state".to_string()).await;
    let user_unique_id = {
        let users_guard = webauthn_users.lock().await;
        users_guard
            .name_to_id
            .get(username.as_str())
            .copied()
            .unwrap_or_else(Uuid::new_v4)
    };
    println!("user unique id : {}", user_unique_id);

    let exclude_credentials = {
        let users_guard = webauthn_users.lock().await;
        users_guard
            .keys
            .get(&user_unique_id)
            .map(|keys| keys.iter().map(|sk| sk.cred_id().clone()).collect())
    };
    println!("exclude creds : {:#?}", exclude_credentials);

    let (ccr, reg_state) = webauthn
        .start_passkey_registration(user_unique_id, &username, &username, exclude_credentials)
        .map_err(|e| {
            info!("challenge_register -> {:?}", e);
            Error::Unknown(e)
        })?;
    info!(
        "Inserting reg_state into session: {:?}",
        (username.clone(), user_unique_id, reg_state.clone())
    );

    reg_state_storage
        .insert((username.to_string(), user_unique_id, reg_state.clone()))
        .await;

    println!("{:#?}", reg_state);
    info!("Registration Successful!");
    Ok(Json(ccr))
}

#[post("finish_reg")]
pub(crate) async fn finish_register(
    req: Json<RegisterPublicKeyCredential>,
    reg_state_storage: Data<RegistrationState>,
    webauthn_users: Data<Mutex<UserData>>,
    webauthn: Data<Webauthn>,
) -> WebResult<HttpResponse> {
    println!("Entered finsih reg");
    let registration_state = reg_state_storage
        .get("reg_state".to_string())
        .await
        .ok_or_else(|| {
            eprintln!("Registration state not found for user ID:");
            Error::CorruptSession
        })?;
    let (username, user_unique_id, reg_state) = registration_state;

    let sk = webauthn
        .finish_passkey_registration(&req, &reg_state)
        .map_err(|e| {
            info!("challenge_register -> {:?}", e);
            Error::BadRequest(e)
        })?;

    let mut users_guard = webauthn_users.lock().await;

    users_guard
        .keys
        .entry(user_unique_id)
        .and_modify(|keys| keys.push(sk.clone()))
        .or_insert_with(|| vec![sk.clone()]);

    users_guard.name_to_id.insert(username, user_unique_id);
    Ok(HttpResponse::Ok().body("Registration Successful"))
}

#[post("start_auth/{username}")]
pub(crate) async fn start_authentication(
    username: Path<String>,
    webauthn_users: Data<Mutex<UserData>>,
    auth_state_store: Data<AuthenticationState>,
    webauthn: Data<Webauthn>,
) -> WebResult<Json<RequestChallengeResponse>> {
    info!("Start Authentication");
    auth_state_store.remove("auth_state".to_string()).await;

    // Get the set of keys that the user possesses
    let users_guard = webauthn_users.lock().await;

    // Look up their unique id from the username
    let user_unique_id = users_guard
        .name_to_id
        .get(username.as_str())
        .copied()
        .ok_or(Error::UserNotFound)?;

    let allow_credentials = users_guard
        .keys
        .get(&user_unique_id)
        .ok_or(Error::UserHasNoCredentials)?;

    let (rcr, auth_state) = webauthn
        .start_passkey_authentication(allow_credentials)
        .map_err(|e| {
            info!("challenge_authenticate -> {:?}", e);
            Error::Unknown(e)
        })?;

    drop(users_guard);

    auth_state_store
        .insert((user_unique_id, auth_state.clone()))
        .await;
    println!("FINISHED AUTH START \n\n\n\n{:#?}", auth_state);
    Ok(Json(rcr))
}

#[post("/finish_auth")]
pub(crate) async fn finish_authentication(
    auth: Json<PublicKeyCredential>,
    auth_state_store: Data<AuthenticationState>,
    webauthn_users: Data<Mutex<UserData>>,
    webauthn: Data<Webauthn>,
) -> WebResult<HttpResponse> {
    println!("Etnered finish auth");
    let auth_state = auth_state_store
        .get("auth_state".to_string())
        .await
        .ok_or_else(|| {
            eprintln!("Registration state not found for user ID:");
            Error::CorruptSession
        })?;

    let (user_unique_id, auth_state) = auth_state;

    let auth_result = webauthn
        .finish_passkey_authentication(&auth, &auth_state)
        .map_err(|e| {
            info!("challenge_register -> {:?}", e);
            Error::BadRequest(e)
        })?;

    let mut users_guard = webauthn_users.lock().await;

    users_guard
        .keys
        .get_mut(&user_unique_id)
        .map(|keys| {
            keys.iter_mut().for_each(|sk| {
                sk.update_credential(&auth_result);
            })
        })
        .ok_or(Error::UserHasNoCredentials)?;

    info!("Authentication Successful!");
    let token =
        encode_jwt(&user_unique_id).map_err(|err| ErrorInternalServerError(err.to_string()));
    let resp_body = match token {
        Ok(token) => {
            json!({
               "token": token,
            })
        }
        Err(e) => {
            json!({
                "error": format!("Error generating token : {}", e),
            })
        }
    };
    println!("{:#?}", resp_body);

    Ok(HttpResponse::Ok().json(resp_body))
}
