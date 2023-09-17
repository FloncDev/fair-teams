use serde_json::Value;
use std::{fs, collections::HashMap};

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

struct Team {
    players: Vec<String>,
    ranking: f64
}

fn main() {
    let rankings = read_json();
    let mut teams: Vec<Team> = vec![];

    // https://softwareengineering.stackexchange.com/a/339534
}
