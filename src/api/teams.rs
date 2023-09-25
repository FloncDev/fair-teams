use rocket::State;
use serde_json::Map;
use rocket::serde::json::Value;
use crate::{AppState, Session};

#[get("/ratings")]
pub async fn get_ratings(session: Session, state: &State<AppState>) -> Value {
    println!("{}", session.player.name);
    let mut ratings = Map::new();

    for player in state.db.get_players().await {
        ratings.insert(player.name, Value::from(player.skill.unwrap()));
    }

   Value::from(ratings)
}