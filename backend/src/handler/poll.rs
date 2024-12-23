use crate::db::poll_crud::PollRepository;
use crate::models::poll::Poll;
use actix_web::{
    delete, get, post,
    web::{Data, Json, Path, Query},
    HttpResponse,
};
use serde::Deserialize;

#[post("/")]
pub async fn add_polls(db: Data<dyn PollRepository>, request: Json<Poll>) -> HttpResponse {
    match db.create_poll(request.into_inner()).await {
        Ok(poll) => HttpResponse::Ok().json(poll),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/{poll_id}")]
pub async fn fetch_polls(db: Data<dyn PollRepository>, path: Path<i64>) -> HttpResponse {
    let poll_id = path.into_inner();
    match db.get_poll(poll_id).await {
        Ok(polls) => HttpResponse::Ok().json(polls),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[derive(Debug, Deserialize)]
pub struct VoteOption {
    option_id: i64,
}

#[post("/{poll_id}/vote")]
pub async fn cast_vote(
    db: Data<dyn PollRepository>,
    path: Path<i64>,
    query: Query<VoteOption>,
) -> HttpResponse {
    let option_id = query.option_id;
    let poll_id = path.into_inner();
    match db.vote_poll(poll_id, option_id).await {
        Ok(_) => HttpResponse::Ok().body("Vote casted successfully"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[post("/{poll_id}/reset")]
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

#[post("/{poll_id}/close")]
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

#[delete("delete-poll/{poll_id}")]
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
