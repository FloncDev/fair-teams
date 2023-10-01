#[macro_use]
extern crate rocket;

use std::sync::Mutex;
use rocket::{State, http::Status, response::Redirect};
use rocket_dyn_templates::{Template, context};
use routes::Session;
use teams::fair_teams;
use db::Mongo;
use api as routes;

pub mod db;
pub mod teams;
pub mod api;
pub mod tournament;

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
        .mount("/", routes![root])
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

    let ratings = state.db.get_ratings_from_player(&session.player).await;

    Ok(Template::render("index", context! {ratings: ratings, me: session.player}))
}