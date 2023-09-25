use crate::teams::{Player, Team};
use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn fair_teams(mut players: Vec<Player>) -> Vec<Team> {
    if players.len() % 2 != 0 {
        players.push(Player {
            id: None,
            name: "[empty]".into(),
            skill: Some(0.0),
        })
    }

    players.shuffle(&mut thread_rng());
    
    // Sort players
    let mut sorted: Vec<Player> = vec![];
    
    for player in players {
        if &sorted.len() == &0 {
            sorted.push(player);
            continue;
        }
        
        for (index, sorted_player) in sorted.iter().enumerate() {
            // Check next player
            if sorted_player.skill > player.skill {
                sorted.insert(index, player);
                break;
            } else if index == sorted.len()-1 {
                sorted.push(player);
                break;
            }
        }
    }

    sorted.reverse();
    
    let mut teams: Vec<Team> = vec![];

    for player in &mut sorted.clone()[..sorted.len()/2].to_vec() {
        teams.push(
            Team::new(vec![player.clone(), sorted.pop().unwrap()])
        );
    }
    
    teams
}
