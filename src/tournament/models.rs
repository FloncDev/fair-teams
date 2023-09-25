use crate::teams::Team;

#[derive(Debug)]
pub enum GameStatus {
    Scheduled,
    InGame,
    Done
}

#[derive(Debug)]
pub struct Game {
    pub teams: Vec<Team>,
    pub scores: [i32; 2],
    pub status: GameStatus
}