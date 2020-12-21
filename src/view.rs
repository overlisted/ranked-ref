use crate::system::{Leaderboard, Player};

fn to_russian_temperature(number: i32) -> String {
    format!("{}{}", if number.is_negative() { "-" } else { "+" }, number.abs())
}

impl Leaderboard {
    pub(crate) fn format_player(&self, player: &Player) -> String {
        format!(
            "**{}** [#{}]: {} ({})",
            player.id,
            self.rank_of(player),
            player.score,
            to_russian_temperature(player.increase)
        )
    }

    pub(crate) fn format(&self) -> String {
        let mut players: Vec<String> = self.0.iter().map(|it| self.format_player(it)).collect();
        if players.is_empty() {
            players.push(String::from("*No players*"));
        }

        format!("**Leaderboard**: \n{}", players.join("\n"))
    }
}
