mod game_view;

use crate::game::rule::Rule;

pub trait View {
    const MAX_WIDTH: usize = 48;
    const MAX_HEIGHT: usize = 4;
    fn render(&self, game_rule: &Rule) -> String;
    fn clear(&mut self) -> String {
        let mut s = String::new();
        for _ in 0..Self::MAX_HEIGHT {
            for _ in 0..Self::MAX_WIDTH {
                s.push_str(" ");
            }
            s.push_str("\n");
        }
        s
    }

    fn fill_space(&self, terminal_view: &str) -> String {
        let mut terminal_lines: Vec<&str> = terminal_view.split("\n").collect();
        let mut terminal_string = String::new();
        if terminal_lines.len() < Self::MAX_HEIGHT {
            while Self::MAX_HEIGHT != terminal_lines.len() {
                terminal_lines.push("");
            }
        } else if Self::MAX_HEIGHT < terminal_lines.len() {
            while Self::MAX_HEIGHT != terminal_lines.len() {
                terminal_lines.pop();
            }
        }
        for line in terminal_lines.iter() {
            let mut s = String::new();
            s.push_str(line);
            if Self::MAX_WIDTH < s.len() {
                s.replace_range(Self::MAX_WIDTH.., "");
            } else {
                for _ in 0..(Self::MAX_WIDTH - s.len()) {
                    s.push_str(" ");
                }
            }
            terminal_string.push_str(&s);
            terminal_string.push_str("\n");
        }
        terminal_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::view::game_view::GameView;
    #[test]
    fn fill_space() {
        let game_view = GameView::new();
        let terminal_view = "\
12345678901234567890123456789012345678901234567890
1234567890123456789012345678901234567890123456789012345678901234567890
123456789012345678901234567890
12345678901234567890123456789012345678901234567890
12345678901234567890123456789012345678901234567890
12345678901234567890123456789012345678901234567890";
        let truth = "\
123456789012345678901234567890123456789012345678
123456789012345678901234567890123456789012345678
123456789012345678901234567890                  
123456789012345678901234567890123456789012345678
";
        let tl = game_view.fill_space(terminal_view);
        assert_eq!(truth, tl);

        let terminal_view = "\
12345678901234567890123456789012345678901234567890
1234567890123456789012345678901234567890123456789012345678901234567890
123456789012345678901234567890";
        let truth = "\
123456789012345678901234567890123456789012345678
123456789012345678901234567890123456789012345678
123456789012345678901234567890                  
                                                
";
        let tl = game_view.fill_space(terminal_view);
        assert_eq!(truth, tl);
    }
}
