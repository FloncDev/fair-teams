use crate::{tournament::{Game, GameStatus}, teams::Team};
use itertools::Itertools;

#[derive(Debug)]
pub struct Tournament {
    pub matches: Vec<Game>
}

impl Tournament {
    pub fn new(teams: Vec<Team>, courts: u8) -> Tournament {
        let mut matches: Vec<Game> = vec![];

        for perm in teams.iter().combinations(2) {

            let perm: Vec<Team> = {
                let mut out: Vec<Team> = vec![];
                for team in perm {
                    out.push(team.to_owned());
                }
                out
            };

            matches.push(
                Game {
                    teams: perm,
                    scores: [0, 0],
                    status: GameStatus::Scheduled
                }
            )
        }

        println!("{:#?} {:#?}", matches, courts);

        Tournament {
            matches
        }
    }
}