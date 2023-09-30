use chrono::{Utc, DateTime};
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Player {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    #[serde(skip_serializing)]
    pub skill: Option<f64>,
    pub discord_id: Option<String>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Rating {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub rater_id: ObjectId,
    pub ratee_id: ObjectId,
    pub rating: f64,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub timestamp: DateTime<Utc>
}

#[derive(Debug, Clone)]
pub struct Team {
    pub players: Vec<Player>,
    pub skill: f64
}

impl Team {
    pub fn new(players: Vec<Player>) -> Team {
        let mut skill_sum = 0.0;

        for player in &players {
            skill_sum += player.skill.unwrap();
        }

        // Round to 1 d.p.
        let skill = (skill_sum / players.len() as f64 * 10.0).round() / 10.0;

        Team {
            players,
            skill
        }
    }
}
