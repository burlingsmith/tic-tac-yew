//! Tic-tac-toe written in Rust using the Yew framework.

//////////////////////////////////////////////////////////////////////////////
// Convenience Aliases
//////////////////////////////////////////////////////////////////////////////

pub type Index = (usize, usize);

//////////////////////////////////////////////////////////////////////////////
//
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Player {
    X,
    O,
}

#[derive(Debug)]
pub struct Board {
    values: [[Option<Player>; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Self {
            values: [[None; 3]; 3],
        }
    }

    pub fn get(&self, (col, row): Index) -> Option<Player> {
        if col < 3 && row < 3 {
            self.values[col][row]
        } else {
            None
        }
    }

    pub fn from_array(values: [[Option<Player>; 3]; 3]) -> Self {
        Self {
            values: values,
        }
    }

    pub fn winner(&self) -> Option<Player> {
        unimplemented!();
    }
}

//////////////////////////////////////////////////////////////////////////////
//
//////////////////////////////////////////////////////////////////////////////

fn main() {
    println!("Hello, world!");
}

//////////////////////////////////////////////////////////////////////////////
// Unit Tests
//////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let new_board = Board::new();

        for col in 0..3 {
            for row in 0..3 {
                assert_eq!(new_board.get((col, row)), None);
            }
        }
    }

    #[test]
    fn test_winner() {
        let horizontal_x_win_1 = Board::from_array(
            [
                [Some(Player::X), Some(Player::X), Some(Player::X)],
                [None, None, None],
                [None, None, None],
           ]
        );
        let horizontal_o_win_2 = Board::from_array(
            [
                [None, None, None],
                [Some(Player::O), Some(Player::O), Some(Player::O)],
                [None, None, None],
           ]
        );
        let horizontal_x_win_3 = Board::from_array(
            [
                [None, None, None],
                [None, None, None],
                [Some(Player::X), Some(Player::X), Some(Player::X)],
           ]
        );

        assert_eq!(horizontal_x_win_1.winner(), Some(Player::X));
        assert_eq!(horizontal_o_win_2.winner(), Some(Player::O));
        assert_eq!(horizontal_x_win_3.winner(), Some(Player::X));

        let vertical_o_win_1 = Board::from_array(
            [
                [Some(Player::O), None, None],
                [Some(Player::O), None, None],
                [Some(Player::O), None, None],
            ]
        );
        let vertical_x_win_2 = Board::from_array(
            [
                [None, Some(Player::X), None],
                [None, Some(Player::X), None],
                [None, Some(Player::X), None],
            ]
        );
        let vertical_o_win_3 = Board::from_array(
            [
                [None, None, Some(Player::O)],
                [None, None, Some(Player::O)],
                [None, None, Some(Player::O)],
            ]
        );

        assert_eq!(vertical_o_win_1.winner(), Some(Player::O));
        assert_eq!(vertical_x_win_2.winner(), Some(Player::X));
        assert_eq!(vertical_o_win_3.winner(), Some(Player::O));

        let diagonal_x_win_1 = Board::from_array(
            [
                [Some(Player::X), None, None],
                [None, Some(Player::X), None],
                [None, None, Some(Player::X)],
            ]
        );
        let diagonal_o_win_2 = Board::from_array(
            [
                [None, None, Some(Player::O)],
                [None, Some(Player::O), None],
                [Some(Player::O), None, None],
            ]
        );

        assert_eq!(diagonal_x_win_1.winner(), Some(Player::X));
        assert_eq!(diagonal_o_win_2.winner(), Some(Player::O));
    }

    // test_no_winner corners, center, edges
}
