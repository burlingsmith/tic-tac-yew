//! Tic-tac-toe written in Rust using the Yew framework.

use std::collections::HashSet;

/// Index of the form (column, row), where 0, 0 is the Northwestern corner.
type Index = (usize, usize);

/// Player identifiers.
#[derive(Debug, Hash, Eq, PartialEq)]
enum Player {
    X,
    O,
    Nil,
}

/// Game state.
#[derive(Debug, Hash, Eq, PartialEq)]
struct TableTop {
    active_player: Player,
    board: [[Player; 3]; 3],
}

impl TableTop {
    /// Set the value of a position on a tic-tac-toe grid.
    fn set_tile(&mut self, (col, row): Index, value: Player) {
        if col < 3 && row < 3 {
            self.board[col][row] = value;
        }
    }

    /// Check for a column win condition.
    fn col_winner(self) -> Player {
        let mut result = Nil;

        for c in 0..3 {
            let mut checking = HashSet::new();

            for r in 0..3 {
                checking.insert(self.board[c][r]);
            }

            if checking.len() == 1 {
                result = {
                    checking
                    .get(Player::X)
                    .or(checking.get(Player::O))
                    .unwrap_or(Nil)
                };
            }

            if result != Nil {
                break;
            }
        }

        result
    }

    /// Check for a row win condition.
    fn row_winner(self) -> Player {
        unimplemented!();
    }

    /// Check for a diagonal win condition
    fn diag_winner(self) -> Player {
        unimplemented!();
        /*
        let mut checking_1 = HashSet::new();
        let mut checking_2 = HashSet::new();

        for x in 0..3 {
            checking_1.insert(self.board[x][x].unwrap_or(None));
            checking_2.insert(self.board[x][2 - x].unwrap_or(None));
        }

        let mut result = None;
        let mut xpres = checking_1.get(Player::X);
        let mut opres = checking_1.get(Player::O);

        result = xpres.or(opres);

        xpres = checking_2.get(Player::X);
        opres = checking_2.get(Player::O);

        if result.is_none() {
            xpres.or(opres)
        } else {
            result
        }
        */
    }

    /// Report any winners with a given board state
    fn winner(self) -> Player {
        let mut result = self.col_winner();

        if result.is_none() {
            result = self.row_winner();
        }

        // Diagonal checks
        if result.is_none() {
            checking.clear();

            unimplemented!();
        }

        result
    }
}

fn main() {
    println!("Hello, world!");
}
