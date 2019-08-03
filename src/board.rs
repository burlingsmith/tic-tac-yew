//! 3x3 tic-tac-toe grid and associated methods.

use super::*;

//////////////////////////////////////////////////////////////////////////////
// Convenience Aliases
//////////////////////////////////////////////////////////////////////////////

/// Tuple of the form (column, row) used for accessing board cells.
pub type Position = (usize, usize);

//////////////////////////////////////////////////////////////////////////////
// Primary Structure
//////////////////////////////////////////////////////////////////////////////

/// 3x3 tic-tac-toe board.
#[derive(Debug)]
pub(crate) struct Board {
    pub(crate) values: [[Option<Player>; 3]; 3],
}

impl Board {
    //////////////////////////////////
    // Instantiation
    //////////////////////////////////

    /// Create a new 3x3 grid with each cell instantiated to `None`.
    pub(crate) fn new() -> Self {
        Self {
            values: [[None; 3]; 3],
        }
    }

    /// Create a new 3x3 grid with a given set of values.
    pub(crate) fn from_array(values: [[Option<Player>; 3]; 3]) -> Self {
        Self {
            values: values,
        }
    }

    //////////////////////////////////
    // Evaluation
    //////////////////////////////////

    /// Check for winners on a given board, either `Some(Player)` or `None`.
    pub(crate) fn winner(&self) -> Option<Player> {
        let mut result = None;

        let only_has = |set: &HashSet<Option<Player>>, player| {
            set.len() == 1 && set.contains(&Some(player))
        };

        // Vertical checks
        for col in 0..3 {
            let column: HashSet<Option<Player>> = {
                HashSet::from_iter(self.values[col].iter().cloned())
            };

            if only_has(&column, Player::X) {
                result = Some(Player::X);
                break;
            } else if only_has(&column, Player::O) {
                result = Some(Player::O);
                break;
            }
        }

        // Horizontal checks
        if result.is_none() {
            'outer: for row in 0..3 {
                let mut row_vals: HashSet<Option<Player>> = HashSet::new();

                for col in 0..3 {
                    row_vals.insert(self.values[col][row]);
                }

                if only_has(&row_vals, Player::X) {
                    result = Some(Player::X);
                    break 'outer;
                } else if only_has(&row_vals, Player::O) {
                    result = Some(Player::O);
                    break 'outer;
                }
            }
        }

        // Diagonal checks
        if result.is_none() {
            let mut s1: HashSet<Option<Player>> = HashSet::new();
            let mut s2: HashSet<Option<Player>> = HashSet::new();

            for n in 0..3 {
                s1.insert(self.values[n][n]);
                s2.insert(self.values[n][2 - n]);
            }

            if only_has(&s1, Player::X) || only_has(&s2, Player::X) {
                result = Some(Player::X);
            } else if only_has(&s1, Player::O) || only_has(&s2, Player::O) {
                result = Some(Player::O);
            }
        }

        result
    }

    /// Check for a full board.
    pub(crate) fn is_full(&self) -> bool {
        let mut result = true;

        for col in 0..3 {
            let column: HashSet<Option<Player>> = {
                HashSet::from_iter(self.values[col].iter().cloned())
            };

            if column.contains(&Some(Player::X))
                || column.contains(&Some(Player::O))
            {
                result = false;
                break;
            }
        }

        result
    }
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
                assert_eq!(new_board.values[col][row], None);
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

    #[test]
    fn test_no_winner() {
        let center = Board::from_array(
            [
                [None, None, None],
                [None, Some(Player::O), None],
                [None, None, None],
            ]
        );
        let corners = Board::from_array(
            [
                [Some(Player::X), None, Some(Player::X)],
                [None, None, None],
                [Some(Player::X), None, Some(Player::X)],
            ]
        );
        let diamond = Board::from_array(
            [
                [None, Some(Player::O), None],
                [Some(Player::O), None, Some(Player::O)],
                [None, Some(Player::O), None],
            ]
        );
        let triangles = Board::from_array(
            [
                [Some(Player::X), Some(Player::X), None],
                [Some(Player::X), None, Some(Player::O)],
                [None, Some(Player::O), Some(Player::O)],
            ]
        );
        let draw = Board::from_array(
            [
                [Some(Player::X), Some(Player::O), Some(Player::O)],
                [Some(Player::O), Some(Player::X), Some(Player::X)],
                [Some(Player::X), Some(Player::X), Some(Player::O)],
            ]
        );

        assert_eq!(center.winner(), None);
        assert_eq!(corners.winner(), None);
        assert_eq!(diamond.winner(), None);
        assert_eq!(triangles.winner(), None);
        assert_eq!(draw.winner(), None);

        let mixed_horizontal_1 = Board::from_array(
            [
                [Some(Player::O), Some(Player::X), Some(Player::X)],
                [None, None, None],
                [None, None, None],
            ]
        );
        let mixed_horizontal_2 = Board::from_array(
            [
                [None, None, None],
                [Some(Player::O), Some(Player::X), Some(Player::O)],
                [None, None, None],
            ]
        );
        let mixed_horizontal_3 = Board::from_array(
            [
                [None, None, None],
                [None, None, None],
                [Some(Player::X), Some(Player::X), Some(Player::O)],
            ]
        );

        assert_eq!(mixed_horizontal_1.winner(), None);
        assert_eq!(mixed_horizontal_2.winner(), None);
        assert_eq!(mixed_horizontal_3.winner(), None);

        let mixed_vertical_1 = Board::from_array(
            [
                [Some(Player::O), None, None],
                [Some(Player::X), None, None],
                [Some(Player::X), None, None],
            ]
        );
        let mixed_vertical_2 = Board::from_array(
            [
                [None, Some(Player::X), None],
                [None, Some(Player::X), None],
                [None, Some(Player::O), None],
            ]
        );
        let mixed_vertical_3 = Board::from_array(
            [
                [None, None, Some(Player::X)],
                [None, None, Some(Player::O)],
                [None, None, Some(Player::X)],
            ]
        );

        assert_eq!(mixed_vertical_1.winner(), None);
        assert_eq!(mixed_vertical_2.winner(), None);
        assert_eq!(mixed_vertical_3.winner(), None);

        let mixed_diagonal_1 = Board::from_array(
            [
                [None, None, Some(Player::O)],
                [None, Some(Player::O), None],
                [Some(Player::X), None, None],
            ]
        );
        let mixed_diagonal_2 = Board::from_array(
            [
                [Some(Player::X), None, None],
                [None, Some(Player::O), None],
                [None, None, Some(Player::O)],
            ]
        );
        let mixed_diagonal_3 = Board::from_array(
            [
                [None, None, Some(Player::O)],
                [None, Some(Player::X), None],
                [Some(Player::O), None, None],
            ]
        );

        assert_eq!(mixed_diagonal_1.winner(), None);
        assert_eq!(mixed_diagonal_2.winner(), None);
        assert_eq!(mixed_diagonal_3.winner(), None);
    }
}