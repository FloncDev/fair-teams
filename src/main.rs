use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{io, collections::HashMap};

fn read_json() -> HashMap<f32> {
    let file = io::read_to_string("rankings.json");
}

fn main() {
    println!("Hello, world!");
}
