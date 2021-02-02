use std::collections::HashMap;

use crate::model::MyError;
use convert_case::{Case, Casing};
use csv::Reader;

#[derive(Debug)]
pub struct Config {
    pub players: Vec<Player>,
}

impl Config {
    pub fn new() -> Result<Self, MyError> {
        Ok(Self {
            players: create_players()?,
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

fn create_players() -> Result<Vec<Player>, MyError> {
    let mut rdr = Reader::from_path("heroes.csv")?;
    let mut players = rdr
        .headers()?
        .iter()
        .map(|s| {
            if s.is_empty() || s.contains("Das") {
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
        let mut flex = Player::new("Flex".to_owned());
        let mut hashmap = HashMap::new();
        for Player { expert, op, .. } in players.iter() {
            for hero in expert {
                *hashmap.entry(hero).or_insert(0) += 1;
            }
            for hero in op {
                *hashmap.entry(hero).or_insert(0) += 1;
            }
        }
        flex.expert = hashmap
            .iter()
            .filter(|(_, nu)| nu >= &&3)
            .map(|(h, _)| h.to_string())
            .collect::<Vec<_>>();
        flex.op = hashmap
            .iter()
            .filter(|(_, nu)| nu == &&2)
            .map(|(h, _)| h.to_string())
            .collect::<Vec<_>>();
        players.push(flex);
    }
    Ok(players)
}
