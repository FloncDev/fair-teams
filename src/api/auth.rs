use rocket::{http::{Status, CookieJar, Cookie}, serde::json::Json, State};
use serde::Deserialize;
use crate::{AppState, Session};

#[derive(Deserialize)]
pub struct LoginUser {
    name: String
}

#[post("/login", format="json", data="<user>")]
pub async fn login(user: Json<LoginUser>, state: &State<AppState>, cookies: &CookieJar<'_>) -> Status {
    let name = user.name.to_owned();
    let player = match state.db.get_player_by_name(name).await {
        Ok(player) => player,
        Err(_) => {return Status::Unauthorized}
    };

    let session = Session::new(player);

    state.add_session(session.clone());

    cookies.add_private(
        Cookie::build("token", session.token)
            .secure(true)
            .http_only(true)
            .finish()
    );

    Status::Ok
}