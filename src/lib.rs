pub mod game;

use crate::game::rule::Rule;
use crate::game::view::game_view::GameView;
use crate::game::view::result_view::ResultView;
use crate::game::view::{Controller, View};

pub fn play() {
    let mut rule = Rule::new();
    let mut game_view = GameView::new();
    let mut forced_stop_flag = false;

    print! {"{}", game_view.render(&rule)}

    while !(rule.end_of_game || forced_stop_flag) {
        let before = rule.fields.clone();
        key_input(&mut rule, &mut game_view, &mut forced_stop_flag);

        if before != rule.fields {
            rule.judge();
            rule.toggle_player();
        }

        print! {"{}", game_view.render(&rule)}
    }
    if forced_stop_flag != true {
        let view = ResultView::new();
        print!("{}", view.render(&rule))
    } else {
        print!("\n\n\n\n");
    }
}

fn key_input(rule: &mut Rule, game_view: &mut GameView, forced_stop_flag: &mut bool) {
    match game_view.key_down() {
        Controller::W => game_view.push_w(),
        Controller::A => game_view.push_a(),
        Controller::S => game_view.push_s(),
        Controller::D => game_view.push_d(),
        Controller::Space => game_view.push_space(rule),
        Controller::Esc => {
            *forced_stop_flag = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_test() {
        play();
    }
}
