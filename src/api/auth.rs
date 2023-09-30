use rocket::{http::{Status, CookieJar, Cookie}, serde::json::Json, State};
use serde::Deserialize;
use crate::{AppState, Session, teams::Player};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString, PasswordHash, PasswordVerifier
    },
    Argon2
};

#[derive(Deserialize)]
pub struct LoginUser {
    pub name: String,
    pub password: String
}

#[post("/login", format="json", data="<user>")]
pub async fn login(user: Json<LoginUser>, state: &State<AppState>, cookies: &CookieJar<'_>) -> Status {
    let name = user.name.to_owned();
    let player = match state.db.get_player_by_name(name).await {
        Ok(player) => player,
        Err(_) => {return Status::Unauthorized}
    };

    let password = player.password.clone().unwrap();

    let parsed_hash = PasswordHash::new(&password).unwrap();
    match Argon2::default().verify_password(user.password.as_bytes(), &parsed_hash) {
        Ok(_) => {},
        Err(_) => return Status::Unauthorized
    }

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

#[post("/user", format="json", data="<user>")]
pub async fn create_user(session: Session, state: &State<AppState>, user: Json<LoginUser>) -> Status {
    if session.player.id.unwrap().to_string() != String::from("65184673563ab00301020578") {
        return Status::Unauthorized;
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(user.password.as_bytes(), &salt).unwrap().to_string();
    
    state.db.create_player(
        Player {
            id: None,
            name: user.name.to_string(),
            skill: None,
            password: Some(password_hash)
        }
    ).await;

    Status::Created
}