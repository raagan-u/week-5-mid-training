pub struct CastedVotes {
    poll_id: i64,
    option_id: i64,
}

pub struct User {
    user_id: i64,
    email: String,
    votes: Vec<CastedVotes>,
}
