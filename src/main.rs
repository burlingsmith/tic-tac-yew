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

impl Player {
    /// Return the other player label.
    fn other(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

/// Result of an attempted move.
#[derive(Debug, Eq, PartialEq)]
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
    //////////////////////////////////
    // Instantiation
    //////////////////////////////////

    /// Setup a new game, with X going first.
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn: Player::X,
        }
    }

    //////////////////////////////////
    // Interaction
    //////////////////////////////////

    /// Attempt to make a move on the current board.
    pub fn play(&mut self, (col, row): Position) -> MoveOutcome {
        if col < 3 && row < 3 && self.board.values[col][row].is_none() {
            self.board.values[col][row] = Some(self.turn);

            match self.board.winner() {
                None => {
                    if self.board.is_full() {
                        MoveOutcome::Draw
                    } else {
                        self.turn = self.turn.other();
                        MoveOutcome::Switch
                    }
                }
                Some(player) => MoveOutcome::Win(player),
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
    fn test_fast_win_for_x() {
        let mut game = GameState::new();
        let plays = [
            (0, 0),             // X
                    (1, 0),     // O
            (0, 1),
                    (1, 1),
            (0, 2),
        ];

        //  X   O   .
        //  X   O   .
        //  X   .   .

        assert_eq!(game.play(plays[0]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[1]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[2]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[3]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[4]), MoveOutcome::Win(Player::X));
    }

    #[test]
    fn test_fast_win_for_o() {
        let mut game = GameState::new();
        let plays = [
            (0, 0),             // X
                    (1, 0),     // O
            (0, 1),
                    (1, 1),
            (2, 2),
                    (1, 2),
        ];

        //  X   O   .
        //  X   O   .
        //  .   O   X

        assert_eq!(game.play(plays[0]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[1]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[2]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[3]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[4]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[5]), MoveOutcome::Win(Player::O));
    }

    #[test]
    fn test_draw() {
        let mut game = GameState::new();
        let plays = [
            (0, 0),             // X
                    (1, 0),     // O
            (2, 0),
                    (2, 1),
            (0, 1),
                    (0, 2),
            (1, 1),
                    (2, 2),
            (1, 2),
        ];

        //  X   O   X
        //  X   X   O
        //  O   X   O

        assert_eq!(game.play(plays[0]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[1]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[2]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[3]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[4]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[5]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[6]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[7]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[8]), MoveOutcome::Draw);
    }

    #[test]
    fn test_invalid_plays_ending_in_win_for_x() {
        let mut game = GameState::new();
        let plays = [
            (3, 0),             // X
            (0, 3),
            (0, 0),
                    (1, 0),     // O
            (1, 0),
            (0, 0),
            (0, 1),
                    (1, 1),
            (0, 2),
        ];

        //  X   O   .
        //  X   O   .
        //  X   .   .

        assert_eq!(game.play(plays[0]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[1]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[2]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[3]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[4]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[5]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[6]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[7]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[8]), MoveOutcome::Win(Player::X));
    }

    #[test]
    fn test_invalid_plays_ending_in_win_for_o() {
        let mut game = GameState::new();
        let plays = [
            (0, 0),             // X
                    (3, 0),     // O
                    (0, 3),
                    (1, 0),
            (0, 1),
                    (0, 1),
                    (1, 0),
                    (1, 1),
            (2, 2),
                    (1, 2),
        ];

        //  X   O   .
        //  X   O   .
        //  .   O   X

        assert_eq!(game.play(plays[0]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[1]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[2]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[3]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[4]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[5]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[6]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[7]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[8]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[9]), MoveOutcome::Win(Player::O));
    }

    #[test]
    fn test_invalid_plays_ending_in_draw() {
        let mut game = GameState::new();
        let plays = [
            (3, 3),             // X
            (0, 0),
                    (0, 0),     // O
                    (1, 0),
            (0, 0),
            (2, 0),
                    (1, 0),
                    (2, 1),
            (2, 1),
            (0, 1),
                    (2, 0),
                    (0, 2),
            (0, 1),
            (1, 1),
                    (2, 1),
                    (2, 2),
            (1, 1),
            (1, 2),
        ];

        //  X   O   X
        //  X   X   O
        //  O   X   O

        assert_eq!(game.play(plays[0]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[1]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[2]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[3]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[4]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[5]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[6]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[7]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[8]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[9]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[10]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[11]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[12]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[13]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[14]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[15]), MoveOutcome::Switch);
        assert_eq!(game.play(plays[16]), MoveOutcome::NoChange);
        assert_eq!(game.play(plays[17]), MoveOutcome::Draw);
    }
}
