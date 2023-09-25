use crate::teams::{Player, Rating};
use dotenv::dotenv;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    Client, Collection,
};
use std::env;

pub struct Mongo {
    ratings: Collection<Rating>,
    players: Collection<Player>,
}

impl Mongo {
    pub async fn new() -> Mongo {
        dotenv().ok();

        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Could not find MONGO_URI env variable."),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("badminton");
        let ratings: Collection<Rating> = db.collection("ratings");
        let players: Collection<Player> = db.collection("players");

        Mongo { ratings, players }
    }

    pub async fn create_raing(&self, rating: Rating) {
        let rating = Rating {
            id: None,
            rater_id: rating.rater_id,
            ratee_id: rating.ratee_id,
            rating: rating.rating,
        };

        match self
            .ratings
            .delete_one(doc! {"rater_id": rating.rater_id}, None)
            .await
        {
            Ok(_) => {}
            Err(_) => {}
        };

        self.ratings.insert_one(rating, None).await.unwrap();
    }

    pub async fn create_player(&self, player: Player) {
        // Check to see if they are already in database
        match self
            .players
            .delete_one(doc! {"name": &player.name}, None)
            .await
        {
            Ok(_) => {},
            Err(_) => {}
        };

        self
            .players
            .insert_one(player, None)
            .await
            .unwrap();
    }

    pub async fn get_player(&self, id: ObjectId) -> Result<Player, Error> {
        let mut ratings = self
            .ratings
            .find(doc! {"ratee_id": id}, None)
            .await
            .unwrap();

        let mut sum: i32 = 0;
        let mut count: i32 = 0;

        while ratings.advance().await.unwrap() {
            let current = ratings.current();

            sum += current.get("rating").unwrap().unwrap().as_i32().unwrap();
            count += 1;
        }

        let skill = (sum as f64 / count as f64 * 100.0).round() / 100.0;

        let name = self
            .players
            .find_one(doc! {"_id": id}, None)
            .await
            .unwrap()
            .unwrap()
            .name;

        Ok(Player { id: Some(id), name, skill: Some(skill) })
    }

    pub async fn get_player_by_name(&self, name: String) -> Result<Player, Error> {
        let id = self.players.find(doc! {"name": name}, None).await.unwrap();
        let id = id.current();

        self.get_player(id.get_object_id("_id").unwrap()).await
    }

    pub async fn get_players(&self) -> Vec<Player> {
        let mut players: Vec<Player> = vec![];

        let mut cursor = self
            .players
            .find(None, None)
            .await
            .unwrap();

        while cursor.advance().await.unwrap() {
            let current = cursor.current();
            let id = current.get("_id").unwrap().unwrap().as_object_id().unwrap();

            players.push(
                self.get_player(id).await.unwrap()
            );
        }

        players
    }
}
