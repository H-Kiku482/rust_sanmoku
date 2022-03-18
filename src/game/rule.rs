// field status
// e.g...
// [O][X][E]
// [E][X][E]
// [E][O][E]
#[derive(PartialEq, Clone)]
pub enum FieldStates {
    Empty,
    O,
    X,
}

#[derive(PartialEq, Clone)]
pub enum Players {
    A,
    B,
}

#[derive(PartialEq, Clone)]
pub enum Winner {
    A,
    B,
    None,
}

#[derive(Clone)]
pub struct Rule {
    // field
    // use as...
    // [0][1][2]
    // [3][4][5]
    // [6][7][8]
    pub fields: [FieldStates; 9],
    pub current_player: Players,
    pub winner: Winner,
    pub end_of_game: bool,
}

impl Rule {
    pub fn new() -> Rule {
        Rule {
            fields: [
                FieldStates::Empty,
                FieldStates::Empty,
                FieldStates::Empty,
                FieldStates::Empty,
                FieldStates::Empty,
                FieldStates::Empty,
                FieldStates::Empty,
                FieldStates::Empty,
                FieldStates::Empty,
            ],
            current_player: Players::A,
            winner: Winner::None,
            end_of_game: false,
        }
    }

    pub fn judge(&mut self) {
        let mut field_numbers: [i32; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

        // cast to i32 array
        let mut cnt = 0;
        for field in self.fields.iter() {
            let num: i32 = match field {
                FieldStates::Empty => 0,
                FieldStates::O => 1,
                FieldStates::X => 2,
            };
            field_numbers[cnt] = num;
            cnt += 1;
        }

        // check 8 lines
        if field_numbers[0] == field_numbers[1] && field_numbers[1] == field_numbers[2] {
            if field_numbers[0] != 0 {
                self.end_of_game = true;
                self.set_winner(field_numbers[0]);
            }
        } else if field_numbers[3] == field_numbers[4] && field_numbers[4] == field_numbers[5] {
            if field_numbers[3] != 0 {
                self.end_of_game = true;
                self.set_winner(field_numbers[3]);
            }
        } else if field_numbers[6] == field_numbers[7] && field_numbers[7] == field_numbers[8] {
            if field_numbers[6] != 0 {
                self.end_of_game = true;
                self.set_winner(field_numbers[6]);
            }
        } else if field_numbers[0] == field_numbers[3] && field_numbers[3] == field_numbers[6] {
            if field_numbers[0] != 0 {
                self.end_of_game = true;
                self.set_winner(field_numbers[0]);
            }
        } else if field_numbers[1] == field_numbers[4] && field_numbers[4] == field_numbers[7] {
            if field_numbers[1] != 0 {
                self.end_of_game = true;
                self.set_winner(field_numbers[1]);
            }
        } else if field_numbers[2] == field_numbers[5] && field_numbers[5] == field_numbers[8] {
            if field_numbers[2] != 0 {
                self.end_of_game = true;
                self.set_winner(field_numbers[2]);
            }
        } else if field_numbers[0] == field_numbers[4] && field_numbers[4] == field_numbers[8] {
            if field_numbers[0] != 0 {
                self.end_of_game = true;
                self.set_winner(field_numbers[0]);
            }
        } else if field_numbers[2] == field_numbers[4] && field_numbers[4] == field_numbers[6] {
            if field_numbers[2] != 0 {
                self.end_of_game = true;
                self.set_winner(field_numbers[2]);
            }
        } else {
            let mut cnt = 0;
            for i in self.fields.iter() {
                if *i != FieldStates::Empty {
                    cnt += 1;
                }
            }
            if cnt == 9 {
                self.end_of_game = true;
            }
        }
    }

    pub fn toggle_player(&mut self) {
        if self.current_player == Players::A {
            self.current_player = Players::B
        } else {
            self.current_player = Players::A
        }
    }

    fn set_winner(&mut self, winner: i32) {
        if winner == 1 {
            self.winner = Winner::A
        } else if winner == 2 {
            self.winner = Winner::B
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn judge_test() {
        let mut game = Rule {
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
        game.judge();
        assert_eq!(game.end_of_game, true);
    }
}
