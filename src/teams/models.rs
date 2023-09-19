use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    #[serde(skip_serializing)]
    pub skill: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rating {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub rater_id: ObjectId,
    pub ratee_id: ObjectId,
    pub rating: f64
}

pub struct Team {
    pub players: Vec<Player>,
    pub skill: f64
}

impl Team {
    pub fn new(players: Vec<Player>) -> Team {
        let mut skill_sum = 0.0;

        for player in &players {
            skill_sum += player.skill;
        }

        // Round to 1 d.p.
        let skill = (skill_sum / players.len() as f64 * 10.0).round() / 10.0;

        Team {
            players,
            skill
        }
    }
}
