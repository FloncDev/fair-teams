#[macro_use]
extern crate rocket;

use std::{sync::Mutex, collections::HashMap};
use bson::DateTime;
use chrono::{Utc, Duration};
use rocket::{State, response::Redirect};
use rocket_dyn_templates::{Template, context};
use routes::{Session, sessions};
use teams::fair_teams;
use db::Mongo;
use api as routes;

pub mod db;
pub mod teams;
pub mod api;
pub mod tournament;
pub mod static_files;

pub struct AppState {
    pub db: Mongo,
    pub sessions: Mutex<Vec<Session>>
}

impl AppState {
    fn verify_session(&self, token: String) -> Option<Session> {    
        let lock = &self.sessions.lock().unwrap();

        for session in lock.iter() {
            if session.token == token {
                return Some(session.clone());
            }
        }
        None
    }

    fn add_session(&self, session: Session) {
        let mut lock = self.sessions.lock().unwrap();
        lock.push(session);
    }
}

#[launch]
pub async fn rocket() -> _ {
    let mongo = Mongo::new().await;

    let teams = fair_teams(mongo.get_players().await);
    
    for team in teams {
        let players = format!("{} & {}", team.players[0].name, team.players[1].name);
        println!("{} ({})", players, team.skill);
    }

    rocket::build()
        .manage(AppState { db: mongo, sessions: Mutex::new(vec![]) })
        .attach(Template::fairing())
        .mount("/", routes![
                root,
                ratings,
                admin
            ])
        .mount("/", routes![static_files::get_file])
        .mount("/", routes![
                routes::auth::login,
                routes::auth::callback
            ])
        .mount("/teams", routes![
                routes::teams::get_ratings,
                routes::teams::post_ratings
            ])
}

#[get("/")]
async fn root(session: Option<Session>, state: &State<AppState>) -> Result<Template, Redirect> {
    let session = match session {
        Some(session) => session,
        None => {return Err(Redirect::to(uri!("/login")));}
    };

    let mut player: HashMap<&str, String> = HashMap::new();
    let skill = state.db.get_player(session.player.id.unwrap(), None).await.unwrap().skill.unwrap();

    player.insert("name", session.player.name);
    player.insert("rating", format!("{:#?}", skill));

    let last_week_player = state.db.get_player(
        session.player.id.unwrap(),
        Some(DateTime::from_chrono(Utc::now()-Duration::weeks(1)))
    ).await.unwrap();

    let percent = (last_week_player.skill.unwrap() - skill) / session.player.skill.unwrap() * 100.0;
    let change = {
        if percent == 0.0 {
            (String::from("#8C8C8C"), String::from("No change"))
        } else if percent > 0.0 {
            (String::from("#5C8E58"), format!("+{:.1}%", percent))
        } else {
            (String::from("#9E4747"), format!("{:.1}%", percent))
        }
    };

    Ok(Template::render("profile", context! {me: player, change: change}))
}

#[get("/ratings")]
async fn ratings(session: Option<Session>, state: &State<AppState>) -> Result<Template, Redirect> {
    let session = match session {
        Some(session) => session,
        None => {return Err(Redirect::to(uri!("/login")));}
    };

    let ratings = state.db.get_ratings_from_player(&session.player).await;
    Ok(Template::render("ratings", context! {ratings:Vec::from_iter(ratings.iter())}))
}

#[get("/admin")]
async fn admin(session: Option<Session>, state: &State<AppState>) -> Result<Template, Redirect> {
    let session = match session {
        Some(session) => session,
        None => {return Err(Redirect::to(uri!("/login")));}
    };

    if session.player.discord_id.unwrap() != String::from("275709752875548674") {
        return Err(Redirect::to("/"));
    }

    Ok(Template::render("admin", context! {}))
}