use crate::game::rule::{FieldStates, Rule, Winner};
use crate::game::view::View;

pub struct ResultView;

impl View for ResultView {
    fn render(&self, game_rule: &Rule) -> String {
        let mut terminal_view = String::new();
        let color = match game_rule.winner {
            Winner::A => {
                let color = Self::PRIMARY_COLOR.to_string();
                terminal_view.push_str(&color);
                terminal_view.push_str("Winner A");
                color
            }
            Winner::B => {
                let color = Self::SECONDARY_COLOR.to_string();
                terminal_view.push_str(&color);
                terminal_view.push_str("Winner B");
                color
            }
            Winner::None => {
                let color = Self::DEFAULT_COLOR.to_string();
                terminal_view.push_str(&color);
                terminal_view.push_str("D R A W");
                color
            }
        };
        terminal_view.push_str(Self::DEFAULT_COLOR);
        terminal_view.push_str("\n");

        let pos = self.get_position(game_rule);
        let mut ptr: usize = 0b100_000_000;
        let mut cnt = 0;
        for value in game_rule.fields.iter() {
            if pos & ptr == ptr {
                terminal_view.push_str(&color);
            }

            ptr >>= 1;

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

        self.fill_space(&terminal_view)
    }
}

impl ResultView {
    pub fn new() -> ResultView {
        ResultView {}
    }
    fn get_position(&self, game_rule: &Rule) -> usize {
        let mut position = 0b000_000_000;

        if game_rule.fields[0] == game_rule.fields[1] && game_rule.fields[1] == game_rule.fields[2]
        {
            if game_rule.fields[0] != FieldStates::Empty {
                position += 0b111000000;
            }
        }
        if game_rule.fields[3] == game_rule.fields[4] && game_rule.fields[4] == game_rule.fields[5]
        {
            if game_rule.fields[3] != FieldStates::Empty {
                position += 0b000111000;
            }
        }
        if game_rule.fields[6] == game_rule.fields[7] && game_rule.fields[7] == game_rule.fields[8]
        {
            if game_rule.fields[6] != FieldStates::Empty {
                position += 0b000000111;
            }
        }
        if game_rule.fields[0] == game_rule.fields[3] && game_rule.fields[3] == game_rule.fields[6]
        {
            if game_rule.fields[0] != FieldStates::Empty {
                position += 0b100100100;
            }
        }
        if game_rule.fields[1] == game_rule.fields[4] && game_rule.fields[4] == game_rule.fields[7]
        {
            if game_rule.fields[1] != FieldStates::Empty {
                position += 0b010010010;
            }
        }
        if game_rule.fields[2] == game_rule.fields[5] && game_rule.fields[5] == game_rule.fields[8]
        {
            if game_rule.fields[2] != FieldStates::Empty {
                position += 0b001001001;
            }
        }
        if game_rule.fields[0] == game_rule.fields[4] && game_rule.fields[4] == game_rule.fields[8]
        {
            if game_rule.fields[0] != FieldStates::Empty {
                position += 0b100010001;
            }
        }
        if game_rule.fields[2] == game_rule.fields[4] && game_rule.fields[4] == game_rule.fields[6]
        {
            if game_rule.fields[2] != FieldStates::Empty {
                position += 0b001010100;
            }
        }

        position
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::rule::Players;

    #[test]
    fn result_view_render_test() {
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
            winner: Winner::A,
            end_of_game: false,
        };
        let result_view = ResultView::new();
        print!("{}", result_view.render(&game));
        print!("\n\n\n\n");
    }
}
