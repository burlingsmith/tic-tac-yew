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
    values: [[Option<Player>; 3]; 3]
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
                assert_eq!(
                    new_board.get((col, row)),
                    None,
                    "found 'Some(_)' in instantiated board; expected 'None'"
                );
            }
        }
    }
}
