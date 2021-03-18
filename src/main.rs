fn main() {}


#[derive(Default)]
struct Scoreboard {
    player1: u8,
    player2: u8,
}

impl Scoreboard {
    fn score(&self) -> String {
        let mut score = "".to_string();
        if self.player1 == 3 && self.player2 == 3 {
            score.push_str("Deuce");
            return score;
        }
        score.push_str(self.convert_score_to_label(self.player1));
        score.push_str(" ");
        score.push_str(self.convert_score_to_label(self.player2));
        score
    }

    fn convert_score_to_label(&self, player: u8) -> &str {
        match player {
            0 => "Love",
            1 => "Fifteen",
            2 => "Thirty",
            3 => "Forty",
            _ => panic!()
        }
    }

    fn trigger_player1(&mut self) {
        self.player1 = self.player1 + 1;
    }

    fn trigger_player2(&mut self) {
        self.player2 = self.player2 + 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::Scoreboard;

    #[test]
    fn should_init_scoreboard() {
        let scoreboard: Scoreboard = Default::default();
        assert_eq!(scoreboard.score(), "Love Love")
    }

    #[test]
    fn should_display_fifteen_if_player_one_win_one_time() {
        let mut scoreboard: Scoreboard = Default::default();
        scoreboard.trigger_player1();
        assert_eq!(scoreboard.score(), "Fifteen Love")
    }

    #[test]
    fn should_display_thirty_if_player_one_win_two_times() {
        let mut scoreboard: Scoreboard = Default::default();
        scoreboard.trigger_player1();
        scoreboard.trigger_player1();
        assert_eq!(scoreboard.score(), "Thirty Love")
    }

    #[test]
    fn should_display_forty_if_player_one_win_three_times() {
        let mut scoreboard: Scoreboard = Default::default();
        scoreboard.trigger_player1();
        scoreboard.trigger_player1();
        scoreboard.trigger_player1();
        assert_eq!(scoreboard.score(), "Forty Love")
    }

    #[test]
    fn should_display_fifteen_if_player_two_win_one_time() {
        let mut scoreboard: Scoreboard = Default::default();
        scoreboard.trigger_player2();
        assert_eq!(scoreboard.score(), "Love Fifteen")
    }

    #[test]
    fn should_display_deuce_if_players_win_three_times() {
        let mut scoreboard: Scoreboard = Default::default();
        for _i in 0..3 {
            scoreboard.trigger_player1();
            scoreboard.trigger_player2();
        }
        assert_eq!(scoreboard.score(), "Deuce")
    }
}