pub struct Player {
    pub id: String,
    pub name: String,
    pub skill: f64
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
