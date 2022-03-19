use crate::game::rule::{FieldStates, Players, Rule};
use crate::game::view::View;

pub struct GameView {
    current_point: usize,
}

impl View for GameView {
    fn render(&self, game_rule: &Rule) -> String {
        let mut terminal_view = String::new();
        terminal_view.push_str("Player ");

        let mut color = String::new();
        let current_player = match game_rule.current_player {
            Players::A => {
                color.push_str(Self::PRIMARY_COLOR);
                "A"
            }
            Players::B => {
                color.push_str(Self::SECONDARY_COLOR);
                "B"
            }
        };

        terminal_view.push_str(current_player);
        terminal_view.push_str("\n");

        let mut cnt = 0;
        for value in game_rule.fields.iter() {
            if self.current_point == cnt {
                terminal_view.push_str(&color);
            }
            terminal_view.push_str("[");
            match value {
                FieldStates::Empty => terminal_view.push_str(" "),
                FieldStates::O => terminal_view.push_str("O"),
                FieldStates::X => terminal_view.push_str("X"),
            }
            terminal_view.push_str("]");
            terminal_view.push_str(Self::DEFAULT_COLOR);
            cnt += 1;
            if cnt % 3 == 0 {
                terminal_view.push_str("\n");
            }
        }
        let mut s = self.fill_space(&terminal_view);
        s.push_str(&self.reset_cursor());
        s
    }
}

impl GameView {
    pub fn new() -> GameView {
        // Initial GameView
        // Player A
        // [ ][ ][ ]
        // [ ][ ][ ]
        // [ ][ ][ ]
        GameView { current_point: 0 }
    }

    pub fn push_w(&mut self) {
        let current_point: usize = self.current_point;
        if 2 < current_point {
            self.current_point = current_point - 3;
        }
    }

    pub fn push_a(&mut self) {
        let current_point: usize = self.current_point;
        if current_point % 3 != 0 {
            self.current_point = current_point - 1;
        }
    }

    pub fn push_s(&mut self) {
        let current_point: usize = self.current_point;
        if current_point < 6 {
            self.current_point = current_point + 3;
        }
    }

    pub fn push_d(&mut self) {
        let current_point: usize = self.current_point;
        if current_point % 3 != 2 {
            self.current_point = current_point + 1;
        }
    }

    pub fn push_space(&self, rule: &mut Rule) {
        if rule.fields[self.current_point] == FieldStates::Empty {
            if rule.current_player == Players::A {
                rule.fields[self.current_point] = FieldStates::O;
            } else {
                rule.fields[self.current_point] = FieldStates::X;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::rule::Winner;
    use crate::game::view::Controller;
    #[test]
    fn game_view_render_test() {
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
            winner: Winner::None,
            end_of_game: false,
        };
        let game_view = GameView::new();
        print!("{}", game_view.render(&game));
        print!("\n\n\n\n");
    }

    #[test]
    fn key_down_test() {
        let game_view = GameView::new();
        println!("input 'a'");
        let c = game_view.key_down();
        assert!(matches!(c, Controller::A));
    }

    #[test]
    fn push_space_test() {
        let mut game = Rule::new();
        let game_view = GameView::new();
        game_view.push_space(&mut game);
    }
}
