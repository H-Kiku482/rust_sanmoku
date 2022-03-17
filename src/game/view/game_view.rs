use crate::game::rule::FieldStates;
use crate::game::rule::Players;
use crate::game::rule::Rule;
use crate::game::view::View;

pub struct GameView;

impl View for GameView {
    fn render(&self, game_rule: &Rule) -> String {
        let mut terminal_view = String::new();
        terminal_view.push_str("Player ");

        let current_player = match game_rule.current_player {
            Players::A => "A",
            Players::B => "B",
        };
        terminal_view.push_str(current_player);
        terminal_view.push_str("\n");

        let mut cnt = 0;
        for value in game_rule.fields.iter() {
            terminal_view.push_str("[");
            match value {
                FieldStates::Empty => terminal_view.push_str(" "),
                FieldStates::O => terminal_view.push_str("O"),
                FieldStates::X => terminal_view.push_str("X"),
            }
            terminal_view.push_str("]");
            cnt += 1;
            if cnt % 3 == 0 {
                terminal_view.push_str("\n");
            }
        }
        terminal_view
    }
}

impl GameView {
    pub fn new() -> GameView {
        // Initial GameView
        // Player A
        // [ ][ ][ ]
        // [ ][ ][ ]
        // [ ][ ][ ]
        GameView {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn render_test() {
        let game = Rule {
            fields: [
                FieldStates::Empty,
                FieldStates::Empty,
                FieldStates::Empty,
                FieldStates::O,
                FieldStates::O,
                FieldStates::O,
                FieldStates::Empty,
                FieldStates::Empty,
                FieldStates::Empty,
            ],
            current_player: Players::A,
            end_of_game: false,
        };
        let game_view = GameView::new();
        print!("{}", game_view.render(&game));
    }
}
