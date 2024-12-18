use crate::db::poll_crud::PollRepository;
use crate::models::poll::Poll;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

#[get("/{poll_id}")]
pub async fn fetch_polls(db: Data<dyn PollRepository>, path: Path<i64>) -> HttpResponse {
    let poll_id = path.into_inner();
    match db.get_poll(poll_id).await {
        Ok(polls) => HttpResponse::Ok().json(polls),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/")]
pub async fn add_polls(db: Data<dyn PollRepository>, request: Json<Poll>) -> HttpResponse {
    match db.create_poll(request.into_inner()).await {
        Ok(poll) => HttpResponse::Ok().json(poll),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//#[post("/{poll_id}/vote")]
//pub async fn cast_vote(db: Data<dyn PollRepository>) -> HttpResponse {}

#[put("update-poll")]
pub async fn update_poll(db: Data<dyn PollRepository>, request: Json<Poll>) -> HttpResponse {
    match db.update_poll(request.into_inner()).await {
        Ok(poll) => HttpResponse::Ok().json(poll),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
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
