#[macro_use]
extern crate rocket;

use crate::db::Mongo;

pub mod db;
pub mod teams;

pub struct State {
    pub db: Mongo,
}

#[launch]
pub async fn rocket() -> _ {
    let mongo = Mongo::new().await;

    rocket::build()
        .manage(State { db: mongo })
        .mount("/", routes![])
}
