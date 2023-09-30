use std::collections::HashMap;
use serde::Deserialize;
use rocket::{http::{Status, CookieJar, Cookie}, State, response::Redirect};
use crate::{AppState, Session};
use reqwest;

#[derive(Debug, Deserialize)]
struct OAuthResponse {
    access_token: String
}

#[derive(Debug, Deserialize)]
struct IdentityResponse {
    id: String
}

#[get("/callback?<code>")]
pub async fn callback(state: &State<AppState>, cookies: &CookieJar<'_>, code: &str) -> Status {
    let mut data = HashMap::new();
    data.insert("client_id", dotenv!("DISCORD_CLIENT_ID"));
    data.insert("client_secret", dotenv!("DISCORD_CLIENT_SECRET"));
    data.insert("grant_type", "authorization_code");
    data.insert("code", code);
    data.insert("redirect_uri", dotenv!("DISCORD_REDIRECT_URI"));

    let client = reqwest::Client::new();
    let res = client.post("https://discord.com/api/oauth2/token")
        .form(&data)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await
        .unwrap()
        .json::<OAuthResponse>()
        .await
        .unwrap();


    let res = client.get("https://discord.com/api/users/@me")
        .header("Authorization", format!("Bearer {}", res.access_token))
        .send()
        .await
        .unwrap()
        .json::<IdentityResponse>()
        .await
        .unwrap();

    let player = state.db.get_player_by_discord_id(res.id).await.unwrap();

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

#[get("/login")]
pub async fn login() -> Redirect {
    Redirect::to(dotenv!("DISCORD_OAUTH_URI"))
}
