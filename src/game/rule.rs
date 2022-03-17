// field status
// e.g...
// [O][X][E]
// [E][X][E]
// [E][O][E]
pub enum FieldStates {
    Empty,
    O,
    X,
}

pub enum Players {
    A,
    B,
}

pub struct Rule {
    // field
    // use as...
    // [0][1][2]
    // [3][4][5]
    // [6][7][8]
    pub fields: [FieldStates; 9],
    pub current_player: Players,
    pub end_of_game: bool,
}

impl Rule {
    fn new() -> Rule {
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
            end_of_game: false,
        }
    }

    fn judge(&mut self) {
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
        {
            if field_numbers[0] == field_numbers[1] && field_numbers[1] == field_numbers[2] {
                if field_numbers[0] != 0 {
                    self.end_of_game = true;
                }
            }
            if field_numbers[3] == field_numbers[4] && field_numbers[4] == field_numbers[5] {
                if field_numbers[3] != 0 {
                    self.end_of_game = true;
                }
            }
            if field_numbers[6] == field_numbers[7] && field_numbers[7] == field_numbers[8] {
                if field_numbers[6] != 0 {
                    self.end_of_game = true;
                }
            }

            if field_numbers[0] == field_numbers[3] && field_numbers[3] == field_numbers[6] {
                if field_numbers[0] != 0 {
                    self.end_of_game = true;
                }
            }
            if field_numbers[1] == field_numbers[4] && field_numbers[4] == field_numbers[7] {
                if field_numbers[1] != 0 {
                    self.end_of_game = true;
                }
            }
            if field_numbers[2] == field_numbers[5] && field_numbers[5] == field_numbers[8] {
                if field_numbers[2] != 0 {
                    self.end_of_game = true;
                }
            }

            if field_numbers[0] == field_numbers[4] && field_numbers[4] == field_numbers[8] {
                if field_numbers[0] != 0 {
                    self.end_of_game = true;
                }
            }
            if field_numbers[2] == field_numbers[4] && field_numbers[4] == field_numbers[6] {
                if field_numbers[2] != 0 {
                    self.end_of_game = true;
                }
            }
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
            end_of_game: false,
        };
        game.judge();
        assert_eq!(game.end_of_game, true);
    }
}
