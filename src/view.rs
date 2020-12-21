use crate::system::{Leaderboard, Player};

impl Leaderboard {
    pub(crate) fn format_player(&self, player: &Player) -> String {
        format!(
            "**{}** [#{}]: {}",
            player.id,
            self.rank_of(player),
            player.score
        )
    }
}
