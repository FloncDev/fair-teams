use crate::teams::{Player, Rating};
use bson::DateTime;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    Client, Collection, options::{FindOptions, FindOneOptions}
};
use std::{collections::{HashSet, HashMap}, env};
use chrono::Utc;
use dotenv::dotenv;

pub struct Mongo {
    ratings: Collection<Rating>,
    players: Collection<Player>,
}

impl Mongo {
    pub async fn new() -> Mongo {
        dotenv().ok();

        let client = Client::with_uri_str(env::var("MONGO_URI").unwrap()).await.unwrap();
        let db = client.database("badminton");
        let ratings: Collection<Rating> = db.collection("ratings");
        let players: Collection<Player> = db.collection("players");

        Mongo { ratings, players }
    }

    pub async fn create_raing(&self, rating: Rating) {
        // Delete latest rating if within the same day
        match self.ratings.find_one(
            doc! {"ratee_id": rating.ratee_id, "rater_id": rating.rater_id},
            FindOneOptions::builder()
                    .sort(doc!{"timestamp": -1})
                    .build()
        ).await {
            Ok(past_rating) => {
                let past_rating = past_rating.unwrap();

                if rating.rating == past_rating.rating {
                    return;
                }

                if rating.timestamp.date_naive() == past_rating.timestamp.date_naive() {
                    let _ = self.ratings.delete_one(doc!{"_id": past_rating.id}, None).await;
                }
            },
            Err(_) => {}
        }

        let rating = Rating {
            id: None,
            rater_id: rating.rater_id,
            ratee_id: rating.ratee_id,
            rating: rating.rating,
            timestamp: Utc::now()
        };

        self.ratings.insert_one(rating, None).await.unwrap();
    }

    pub async fn create_player(&self, player: Player) {
        // Check to see if they are already in database
        let _ = self
            .players
            .delete_one(doc! {"name": &player.name}, None)
            .await;

        self
            .players
            .insert_one(player, None)
            .await
            .unwrap();
    }

    pub async fn get_player(&self, id: ObjectId, time: Option<DateTime>) -> Result<Player, Error> {
        let filter = match time {
            Some(v) => {
                doc! {
                    "ratee_id": id,
                    "timestamp": doc! {
                        "$lte": v
                    }
                }
            },
            None => {
                doc! {"ratee_id": id}
            }
        };

        let mut ratings = self
            .ratings
            .find(filter, 
                FindOptions::builder()
                    .sort(doc!{"timestamp": -1})
                    .build()
            )
            .await
            .unwrap();

        let mut sum: i32 = 0;
        let mut count: i32 = 0;

        let mut ids: HashSet<ObjectId> = HashSet::new();

        while ratings.advance().await.unwrap() {
            let current = ratings.current();
            let rater_id = current.get_object_id("rater_id").unwrap();

            if ids.contains(&rater_id) { continue; }            
            ids.insert(rater_id);

            sum += match current.get_i32("rating") {
                Ok(val) => {val},
                Err(_) => {current.get_f64("rating").unwrap() as i32}
            };
            count += 1;
        }

        let skill = (sum as f64 / count as f64 * 100.0).round() / 100.0;

        let player = self
            .players
            .find_one(doc! {"_id": id}, None)
            .await
            .unwrap()
            .unwrap();

        Ok(Player { id: Some(id), name: player.name, skill: Some(skill), discord_id: player.discord_id})
    }

    pub async fn get_player_by_name(&self, name: String) -> Result<Player, Error> {
        let obj_id = self.players.find_one(doc! {"name": name}, None).await.unwrap().unwrap().id.unwrap();

        Ok(self.get_player(obj_id, None).await?)
    }

    pub async fn get_player_by_discord_id(&self, id: String) -> Result<Player, Error> {
        let obj_id = self.players.find_one(doc! {"discord_id": id}, None).await.unwrap().unwrap().id.unwrap();

        Ok(self.get_player(obj_id, None).await?)
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
                self.get_player(id, None).await.unwrap()
            );
        }

        players
    }

    pub async fn get_ratings_from_player(&self, player: &Player) -> HashMap<String, f64> {
        let mut ratings: HashMap<String, f64> = HashMap::new();

        let mut cursor = self
            .ratings
            .find(doc! {"rater_id": player.id},
                FindOptions::builder()
                    .sort(doc! {"timestamp": -1})
                    .build()
        ).await.unwrap();

        while cursor.advance().await.unwrap() {
            let current = cursor.deserialize_current().unwrap();
            let ratee = self.get_player(current.ratee_id, None).await.unwrap();

            ratings.insert(ratee.name, current.rating);
        }

        return ratings;
    }
}
