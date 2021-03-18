fn main() {}


#[derive(Default)]
struct Scoreboard {
    player1: u8,
    player2: u8,
}

impl Scoreboard {
    fn score(&self) -> String {
        let mut score = "".to_string();
        if self.player1 == 0 {
            score.push_str("Love")
        }
        score.push_str(" ");
        if self.player2 == 0 {
            score.push_str("Love")
        }
        score
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
}