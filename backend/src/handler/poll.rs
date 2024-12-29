use crate::db::poll_crud::PollRepository;
use crate::db::user_crud::UserRepository;
use crate::models::poll::{Poll, ResultsQuery, ServerEvents};
use crate::models::user::Votes;
use actix_web::body::MessageBody;
use actix_web::{
    delete, get, post,
    web::{Data, Json, Path, Query},
    HttpResponse,
};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

#[post("polls")]
pub async fn add_polls(db: Data<dyn PollRepository>, request: Json<Poll>) -> HttpResponse {
    println!("Received Poll Data: {:#?}", request);
    match db.create_poll(request.into_inner()).await {
        Ok(poll) => HttpResponse::Ok().json(poll),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("polls/{poll_id}")]
pub async fn fetch_polls(db: Data<dyn PollRepository>, path: Path<i64>) -> HttpResponse {
    let poll_id = path.into_inner();
    if poll_id == 0 {
        match db.fetch_all().await {
            Ok(polls) => HttpResponse::Ok().json(polls),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    } else {
        match db.get_poll(poll_id).await {
            Ok(polls) => HttpResponse::Ok().json(polls),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct VoteOption {
    option_id: i64,
    username: String,
}

#[post("polls/{poll_id}/vote")]
pub async fn cast_vote(
    db: Data<dyn PollRepository>,
    db2: Data<dyn UserRepository>,
    path: Path<i64>,
    query: Query<VoteOption>,
) -> HttpResponse {
    let query_opts = query;
    let option_id = query_opts.option_id;
    let username = query_opts.username.to_string();
    let poll_id = path.into_inner();
    let vote = Votes {
        poll_id: poll_id,
        option_id: option_id,
    };
    db2.update_user(username.clone(), vote).await.unwrap();
    match db.vote_poll(poll_id, option_id, username).await {
        Ok(_) => HttpResponse::Ok().body("Vote casted successfully"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[post("polls/{poll_id}/reset")]
pub async fn reset_vote(db: Data<dyn PollRepository>, path: Path<i64>) -> HttpResponse {
    let poll_id = path.into_inner();
    match db.update_poll(poll_id, "reset".to_string()).await {
        Ok(_) => HttpResponse::Ok().body("Poll reset successful"),
        Err(e) => {
            eprintln!("Error deleting poll: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete poll")
        }
    }
}

#[post("polls/{poll_id}/close")]
pub async fn close_poll(db: Data<dyn PollRepository>, path: Path<i64>) -> HttpResponse {
    let poll_id = path.into_inner();
    match db.update_poll(poll_id, "close".to_string()).await {
        Ok(_) => HttpResponse::Ok().body("Poll closed successfully"),
        Err(e) => {
            eprintln!("Error deleting poll: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete poll")
        }
    }
}

#[get("/polls/{poll_id}/results")]
async fn poll_results(
    db: Data<dyn PollRepository>,
    path: Path<i64>,
    query: Query<ResultsQuery>,
) -> HttpResponse {
    let poll_id = path.into_inner();
    let db_clone = db.clone();
    if query.live {
        let (tx, rx) = mpsc::channel(1024);

        tokio::spawn(async move {
            loop {
                match db_clone.get_poll(poll_id).await {
                    Ok(poll) => {
                        tx.send(format!(
                            "data: {}\n\n",
                            serde_json::to_string(&poll).unwrap()
                        ))
                        .await
                        .unwrap();
                        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    }
                    Err(_) => {
                        tx.send("Poll not found".to_string()).await.unwrap();
                    }
                }
            }
        });

        // Creating the ServerEvents stream
        let stream = ServerEvents { events: rx };
        let body = stream.map(|event| event.unwrap().try_into_bytes());

        // Returning the response with the correct streaming headers
        return HttpResponse::Ok()
            .insert_header(("Content-Type", "text/event-stream"))
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .streaming(body);
    }

    // If not live, return the current poll data
    match db.get_poll(poll_id).await {
        Ok(poll) => HttpResponse::Ok().json(poll),
        Err(_) => HttpResponse::NotFound().json(json!({
            "error": "Poll not found"
        })),
    }
}

#[delete("polls/delete-poll/{poll_id}")]
pub async fn delete_poll(db: Data<dyn PollRepository>, path: Path<i64>) -> HttpResponse {
    let poll_id = path.into_inner(); // Extract the poll_id from the path

    match db.delete_poll(poll_id).await {
        Ok(_) => HttpResponse::Ok().body("Poll deleted successfully"),
        Err(e) => {
            eprintln!("Error deleting poll: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete poll")
        }
    }
}
