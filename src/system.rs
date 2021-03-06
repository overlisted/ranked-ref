use std::collections::BTreeSet;
use std::sync::Arc;

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serenity::model::id::GuildId;
use serenity::prelude::TypeMapKey;

#[derive(Serialize, Deserialize, PartialOrd, Ord, Clone)]
pub struct Player {
    pub score: u32,
    pub increase: i32,
    pub id: String,
}

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        self.id == other.id
    }
}

impl Eq for Player {}

impl Player {
    fn gain(&mut self, score: i32) -> &mut Self {
        self.increase = score;

        let new = self.score as i32 + score;
        self.score = new.max(0) as u32;

        self
    }
}

#[derive(Serialize, Deserialize)]
pub struct Leaderboard(pub(crate) BTreeSet<Player>);

impl Leaderboard {
    pub(crate) fn new() -> Leaderboard {
        Leaderboard(BTreeSet::new())
    }

    pub fn calculate_scores((mut winner, mut loser): (Player, Player)) -> (Player, Player) {
        winner.gain(10);
        loser.gain(-10);
        (winner, loser)
    }

    pub fn rank_of(&self, player: &Player) -> u32 {
        let index = self.0.iter().position(|it| it == player).unwrap_or(0);
        let index = index as i32;
        let len = self.0.len() as i32;

        let index = (len - index).abs();
        index as u32
    }

    pub fn find_player(&self, name: String) -> Option<&Player> {
        self.0.iter().find(|it| it.id == name)
    }

    pub fn insert_player(&mut self, name: String) -> Player {
        let player = Player {
            score: 0,
            increase: 0,
            id: name,
        };

        self.0.insert(player.clone());
        player
    }

    fn expect_player(&self, name: String) -> &Player {
        self.find_player(name).expect("Player isn't in the system!")
    }

    pub fn score(&mut self, winner: String, loser: String) -> (Player, Player) {
        let (winner, loser) = (
            self.expect_player(winner).clone(),
            self.expect_player(loser).clone(),
        );

        let winner = self.0.take(&winner).unwrap();
        let loser = self.0.take(&loser).unwrap();

        let (winner, loser) = Self::calculate_scores((winner, loser));

        self.0.insert(winner.clone());
        self.0.insert(loser.clone());

        (winner, loser)
    }
}

impl TypeMapKey for Leaderboard {
    type Value = Arc<DashMap<GuildId, Leaderboard>>;
}
