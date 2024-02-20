use crate::details::GameDetails;

#[derive(serde::Serialize, Debug)]
pub struct Collection {
    user_id: String,
    games: Vec<GameDetails>
}

pub fn get_collection(user_id: &str)  {
}