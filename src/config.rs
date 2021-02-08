use std::collections::HashMap;

use crate::model::MyError;
use convert_case::{Case, Casing};
use csv::Reader;

#[derive(Debug)]
pub struct Config {
    pub players: Vec<Player>,
}

impl Config {
    pub fn new(filename: &str) -> Result<Self, MyError> {
        Ok(Self {
            players: create_players(filename)?,
        })
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub expert: Vec<String>,
    pub op: Vec<String>,
}

impl Player {
    fn new(name: String) -> Self {
        Self {
            name,
            expert: vec![],
            op: vec![],
        }
    }
}

fn create_players(filename: &str) -> Result<Vec<Player>, MyError> {
    let mut rdr = Reader::from_path(filename)?;
    let mut players = rdr
        .headers()?
        .iter()
        .enumerate()
        .map(|(i, s)| {
            if s.is_empty() || i > 5 {
                None
            } else {
                Some(Player::new(s.to_owned()))
            }
        })
        .collect::<Vec<_>>();
    for result in rdr.records() {
        let record = result?;
        let mut heroes = String::new();
        for (index, s) in record.iter().enumerate() {
            if index == 0 {
                heroes = s.to_case(Case::Snake);
            } else {
                if let Some(player) = players.get_mut(index).unwrap() {
                    if s.contains("Expert") {
                        player.expert.push(heroes.to_owned());
                    } else if s.contains("Op") {
                        player.op.push(heroes.to_owned());
                    }
                }
            }
        }
    }
    let mut players = players
        .into_iter()
        .filter_map(|player| player)
        .collect::<Vec<Player>>();
    if players.len() == 5 {
        let mut flex = Player::new("Flex pick".to_owned());
        let mut hashmap_support = HashMap::new();
        let mut hashmap_cores = HashMap::new();
        for ( index, Player { expert, op, name: _ }) in players.iter().enumerate() {
            for hero in expert.iter().chain(op.iter()) {
                match index {
                    0 | 1 | 2 => {
                        *hashmap_cores.entry(hero).or_insert(0) += 1;
                    }
                    3 | 4 => {
                        *hashmap_support.entry(hero).or_insert(0) += 1;
                    }
                    n => {
                        println!("{} not supported", n);
                    }
                }
            }
        }
        flex.expert = hashmap_cores
            .iter()
            .filter(|(_, nu)| nu >= &&2)
            .map(|(h, _)| h.to_string())
            .collect::<Vec<_>>();
        flex.op = hashmap_support
            .iter()
            .filter(|(_, nu)| nu >= &&2)
            .map(|(h, _)| h.to_string())
            .collect::<Vec<_>>();
        players.push(flex);
    }
    Ok(players)
}
