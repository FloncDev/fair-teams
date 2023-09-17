use serde_json::Value;
use std::{fs, collections::HashMap};
use itertools::Itertools;

fn read_json() -> HashMap<String, f64> {
    let file = fs::read_to_string("rankings.json").expect("Could not find file.");

    let json: Value = serde_json::from_str(file.as_str()).expect("Json not well formatted");
    let raw_rankings = json.as_object().unwrap();

    let mut rankings: HashMap<String, f64> = HashMap::new();

    for (name, rank) in raw_rankings {
        let rank: f64 = rank.as_f64().unwrap();
        let name = name.to_owned();

        rankings.insert(name, rank);
    } 

    rankings
}

#[derive(Debug)]
struct Team {
    players: Vec<String>,
    ranking: f64
}

fn main() {
    let rankings = read_json();

    let permutations = rankings.keys().combinations(2);
    let mut teams: Vec<Team> = vec![];

    for permutation in permutations {
        let permutation: Vec<&String> = permutation;
        let mut player_rankings: Vec<f64> = vec![];
        let mut team_ranking: f64 = 0.0;

        for name in &permutation {
            let ranking = rankings.get(name.to_owned()).unwrap();
            player_rankings.push(ranking.to_owned());

            team_ranking += ranking.to_owned();
        }

        let mut players: Vec<String> = vec![];

        for player in permutation {
            players.push(player.to_owned());
        }

        teams.push(
            Team {
                ranking: (team_ranking / players.len() as f64 * 10.0).round() / 10.0,
                players: players
            }
        )
    }

    println!("{:#?}", teams);
}
