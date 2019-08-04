#![recursion_limit = "256"]

//! Tic-tac-toe written in Rust using the Yew framework.

mod board;

pub use board::Position;

use std::collections::HashSet;
use std::iter::FromIterator;

//use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

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
    ongoing: bool,
    winner: Option<Player>,
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
            ongoing: true,
            winner: None,
        }
    }

    //////////////////////////////////
    // Interaction
    //////////////////////////////////

    /// Attempt to make a move on the current board.
    pub fn play(&mut self, (col, row): Position) -> MoveOutcome {
        if self.ongoing {
            MoveOutcome::NoChange
        } else if self.board.values[col][row].is_none()
            && col < 3
            && row < 3
        {
            self.board.values[col][row] = Some(self.turn);

            match self.board.winner() {
                Some(player) => {
                    self.winner = Some(player);

                    MoveOutcome::Win(player)
                }
                None => {
                    if self.board.is_full() {
                        MoveOutcome::Draw
                    } else {
                        self.turn = self.turn.other();
                        MoveOutcome::Switch
                    }
                }
            }
        } else {
            MoveOutcome::NoChange
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Yew State Machine
//////////////////////////////////////////////////////////////////////////////

type Model = GameState;

#[derive(Debug)]
enum Msg {
    Click(Position),
    Feedback(MoveOutcome),
    Nil,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self::new()
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => true,
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let positions: [Position; 9] = [
            (0, 0), (1, 0), (2, 0),
            (0, 1), (1, 1), (2, 1),
            (0, 2), (1, 2), (2, 2),
        ];
        let game_status = {
            if self.ongoing {
                if self.turn == Player::X {
                    "Active player: Chi"
                } else {
                    "Active player: Omi"
                }
            } else {
                match self.winner {
                    Some(Player::X) => "Chi Wins!",
                    Some(Player::O) => "Omi Wins!",
                    None => "It's a draw!",
                }
            }
        };

        let view_tile = |pos: &Position| {
            let (col, row) = *pos;

            let val = match self.board.values[col][row] {
                Some(Player::X) => "x-tile",
                Some(Player::O) => "o-tile",
                _ => "empty-tile",
            };

            html! {
                <div class=("tile", val) onclick=|_| Msg::Click((col, row))>
                    {
                        "placeholder"
                    }
                </div>
            }
        };

        html! {
            <div>
                <section class="game-container">
                    <section class="game-area">
                        <div class="game-board">
                            { for positions.iter().map(view_tile) }
                        </div>
                        <div class="game-status">
                            { game_status }
                        </div>
                    </section>
                </section>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
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
            (0, 0), // X
            (1, 0), // O
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
            (0, 0), // X
            (1, 0), // O
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
            (0, 0), // X
            (1, 0), // O
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
            (3, 0), // X
            (0, 3),
            (0, 0),
            (1, 0), // O
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
            (0, 0), // X
            (3, 0), // O
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
            (3, 3), // X
            (0, 0),
            (0, 0), // O
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
