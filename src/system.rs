use std::collections::BTreeSet;
use std::sync::Arc;

use dashmap::DashMap;
use serenity::model::id::GuildId;
use serenity::prelude::TypeMapKey;

#[derive(PartialOrd, Ord, Clone)]
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

pub struct Leaderboard(BTreeSet<Player>);

impl Leaderboard {
    pub(crate) fn new() -> Leaderboard {
        Leaderboard(BTreeSet::new())
    }

    pub fn calculate_scores((mut winner, mut loser): (Player, Player)) -> (Player, Player) {
        winner.gain(10);
        loser.gain(-10);
        (winner, loser)
    }

    pub fn rank_of(&self, player: &Player) -> usize {
        self.0
            .iter()
            .position(|it| it == player)
            .unwrap_or(self.0.len())
    }

    pub fn find_player(&self, name: String) -> Option<&Player> {
        self.0.iter().find(|it| it.id == name)
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
