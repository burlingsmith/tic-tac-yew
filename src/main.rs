//! Tic-tac-toe written in Rust using the Yew framework.

mod board;

pub use board::Position;

use std::collections::HashSet;
use std::iter::FromIterator;

use board::Board;

//////////////////////////////////////////////////////////////////////////////
// Auxiliary Structures
//////////////////////////////////////////////////////////////////////////////

/// Two tic-tac-toe player labels.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Player {
    X,
    O,
}

/// Result of an attempted move.
#[derive(Debug)]
pub enum MoveOutcome {
    Win(Player),
    Draw,
    Switch,
    NoChange,
}

//////////////////////////////////////////////////////////////////////////////
// Primary Structure
//////////////////////////////////////////////////////////////////////////////

/// Tic-tac-toe game state.
#[derive(Debug)]
struct GameState {
    board: Board,
    turn: Player,
}

impl GameState {
    /// Setup a new game, with X going first.
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn: Player::X,
        }
    }

    /// Attempt to make a move on the current board.
    pub fn play(&mut self, (col, row): Position) -> MoveOutcome {
        if col < 3 && row < 3 && self.board.values[col][row].is_none() {
            if self.board.is_full() {
                match self.board.winner() {
                    None => MoveOutcome::Draw,
                    Some(player) => MoveOutcome::Win(player),
                }
            } else {
                MoveOutcome::Switch
                // update board;
            }
        } else {
            MoveOutcome::NoChange
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//
//////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut game = GameState::new();

    let game_result = loop {
        let player_move: Position = unimplemented!();  // get_player_move()

        match game.play(player_move) {
            MoveOutcome::Switch | MoveOutcome::NoChange => (),
            game_result => break game_result,
        }
    };

    unimplemented!();
}

//////////////////////////////////////////////////////////////////////////////
// Unit Tests
//////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_win_for_x() {
        let mut game = GameState::new();

        unimplemented!();
    }

    #[test]
    fn test_demo_win_for_o() {
        let mut game = GameState::new();

        unimplemented!();
    }

    #[test]
    fn test_demo_draw() {
        let mut game = GameState::new();

        unimplemented!();
    }
}
