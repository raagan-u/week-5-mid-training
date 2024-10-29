use crate::handler::{Error, WebResult};
use crate::models::{AuthenticationState, RegistrationState};
use crate::startup::UserData;
use actix_web::post;
use actix_web::web::{Data, Json, Path};
use actix_web::HttpResponse;
use log::info;
use tokio::sync::Mutex;
/*
 * Webauthn RS auth handlers.
 * These files use webauthn to process the data received from each route, and are closely tied to actix_web
 */

// 1. Import the prelude - this contains everything needed for the server to function.
use webauthn_rs::prelude::*;

#[post("/auth/start_reg/{username}")]
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

    // Remove any previous registrations that may have occurred from the session.

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

// 3. The browser has completed it's steps and the user has created a public key
// on their device. Now we have the registration options sent to us, and we need
// to verify these and persist them.

#[post("/auth/finish_reg")]
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

    //TODO: This is where we would store the credential in a db, or persist them in some other way.

    users_guard
        .keys
        .entry(user_unique_id)
        .and_modify(|keys| keys.push(sk.clone()))
        .or_insert_with(|| vec![sk.clone()]);

    users_guard.name_to_id.insert(username, user_unique_id);
    Ok(HttpResponse::Ok().body("Registration Successful"))
}

// 4. Now that our public key has been registered, we can authenticate a user and verify
// that they are the holder of that security token. The work flow is similar to registration.
//
//          ┌───────────────┐     ┌───────────────┐      ┌───────────────┐
//          │ Authenticator │     │    Browser    │      │     Site      │
//          └───────────────┘     └───────────────┘      └───────────────┘
//                  │                     │                      │
//                  │                     │     1. Start Auth    │
//                  │                     │─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─▶│
//                  │                     │                      │
//                  │                     │     2. Challenge     │
//                  │                     │◀ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┤
//                  │                     │                      │
//                  │  3. Select Token    │                      │
//             ─ ─ ─│◀ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│                      │
//  4. Verify │     │                     │                      │
//                  │    4. Yield Sig     │                      │
//            └ ─ ─▶│─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─▶                      │
//                  │                     │    5. Send Auth      │
//                  │                     │        Opts          │
//                  │                     │─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─▶│─ ─ ─
//                  │                     │                      │     │ 5. Verify
//                  │                     │                      │          Sig
//                  │                     │                      │◀─ ─ ┘
//                  │                     │                      │
//                  │                     │                      │
//
// The user indicates the wish to start authentication and we need to provide a challenge.

#[post("/auth/start_auth/{username}")]
pub(crate) async fn start_authentication(
    username: Path<String>,
    webauthn_users: Data<Mutex<UserData>>,
    auth_state_store: Data<AuthenticationState>,
    webauthn: Data<Webauthn>,
) -> WebResult<Json<RequestChallengeResponse>> {
    info!("Start Authentication");
    auth_state_store.remove("auth_state".to_string()).await;
    // We get the username from the URL, but you could get this via form submission or
    // some other process.

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

    // Drop the mutex to allow the mut borrows below to proceed
    drop(users_guard);

    // Note that due to the session store in use being a server side memory store, this is
    // safe to store the auth_state into the session since it is not client controlled and
    // not open to replay attacks. If this was a cookie store, this would be UNSAFE.
    auth_state_store
        .insert((user_unique_id, auth_state.clone()))
        .await;
    println!("FINISHED AUTH START \n\n\n\n{:#?}", auth_state);
    Ok(Json(rcr))
}

// 5. The browser and user have completed their part of the processing. Only in the
// case that the webauthn authenticate call returns Ok, is authentication considered
// a success. If the browser does not complete this call, or *any* error occurs,
// this is an authentication failure.

#[post("/auth/finish_auth")]
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

    // Update the credential counter, if possible.
    users_guard
        .keys
        .get_mut(&user_unique_id)
        .map(|keys| {
            keys.iter_mut().for_each(|sk| {
                // This will update the credential if it's the matching
                // one. Otherwise it's ignored. That is why it is safe to
                // iterate this over the full list.
                sk.update_credential(&auth_result);
            })
        })
        .ok_or(Error::UserHasNoCredentials)?;

    info!("Authentication Successful!");
    Ok(HttpResponse::Ok().body("Auth Success"))
}
