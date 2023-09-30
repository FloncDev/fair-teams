use std::collections::HashMap;
use rocket::{State, http::Status, serde::json::Json};
use serde_json::Map;
use serde::Deserialize;
use rocket::serde::json::Value;
use crate::{AppState, Session, teams::Rating};
use chrono::Utc;

#[get("/ratings")]
pub async fn get_ratings(_session: Session, state: &State<AppState>) -> Value {
    let mut ratings = Map::new();

    for player in state.db.get_players().await {
        ratings.insert(player.name, Value::from(player.skill.unwrap()));
    }

   Value::from(ratings)
}

#[derive(Deserialize)]
pub struct ApiRatings {
    ratings: HashMap<String, f64>
}

#[post("/ratings", data="<ratings>")]
pub async fn post_ratings(session: Session, state: &State<AppState>, ratings: Json<ApiRatings>) -> Status {
    let rater_id = session.player.id.unwrap();

    for (name, rating) in &ratings.ratings {
        let player = state.db.get_player_by_name(name.to_owned()).await.unwrap();

        state.db.create_raing(
            Rating {
                id: None,
                rater_id,
                ratee_id: player.id.unwrap(),
                rating: rating.to_owned(),
                timestamp: Utc::now()
            }
        ).await;
    }

    Status::Ok
}