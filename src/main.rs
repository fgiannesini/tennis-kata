fn main() {}


#[derive(Default)]
struct Scoreboard {
    player1: u8,
    player2: u8,
}

impl Scoreboard {
    fn score(&self) -> String {
        let mut score = "".to_string();
        match self.player1 {
            0 => score.push_str("Love"),
            1 => score.push_str("Fifteen"),
            2 => score.push_str("Thirty"),
            3 => score.push_str("Forty"),
            _ => panic!()
        }
        score.push_str(" ");
        if self.player2 == 0 {
            score.push_str("Love")
        }
        score
    }

    fn trigger_player1(&mut self) {
        self.player1 = self.player1 + 1;
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
}