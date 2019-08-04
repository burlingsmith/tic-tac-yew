#![recursion_limit = "256"]

//! Tic-tac-toe written in Rust using the Yew framework.

mod board;

use board::Position;

use std::collections::HashSet;
use std::iter::FromIterator;

use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use board::Board;

//////////////////////////////////////////////////////////////////////////////
// Auxiliary Structures
//////////////////////////////////////////////////////////////////////////////

/// Two tic-tac-toe player labels.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Player {
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
enum MoveOutcome {
    Win(Player),
    Draw,
    Switch,
    NoChange,
}

//////////////////////////////////////////////////////////////////////////////
// Primary Structure
//////////////////////////////////////////////////////////////////////////////

/// Win/Loss record over the course of multiple games.
#[derive(Debug)]
struct Record {
    xwins: u32,
    owins: u32,
    draws: u32,
}

impl Record {
    fn new() -> Self {
        Self {
            xwins: 0,
            owins: 0,
            draws: 0,
        }
    }
}

/// Tic-tac-toe game state.
#[derive(Debug)]
struct GameState {
    board: Board,
    turn: Player,
    ongoing: bool,
    winner: Option<Player>,
    log: Record,
}

impl GameState {
    //////////////////////////////////
    // Instantiation
    //////////////////////////////////

    /// Setup a new game, with X going first.
    fn new() -> Self {
        Self {
            board: Board::new(),
            turn: Player::X,
            ongoing: true,
            winner: None,
            log: Record::new(),
        }
    }

    //////////////////////////////////
    // Interaction
    //////////////////////////////////

    /// Attempt to make a move on the current board.
    fn play(&mut self, (col, row): Position) -> MoveOutcome {
        if !self.ongoing {
            MoveOutcome::NoChange
        } else if self.board.values[col][row].is_none()
            && col < 3
            && row < 3
        {
            self.board.values[col][row] = Some(self.turn);

            match self.board.winner() {
                Some(player) => {
                    self.ongoing = false;
                    self.winner = Some(player);

                    if player == Player::X {
                        self.log.xwins += 1;
                    } else {
                        self.log.owins += 1;
                    }

                    MoveOutcome::Win(player)
                }
                None => {
                    if self.board.is_full() {
                        self.ongoing = false;
                        self.log.draws += 1;

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

    /// Reset everthing besides the game log.
    fn reset(&mut self) {
        self.board = Board::new();
        self.turn = Player::X;
        self.ongoing = true;
        self.winner = None;
    }
}

//////////////////////////////////////////////////////////////////////////////
// Yew State Machine
//////////////////////////////////////////////////////////////////////////////

type Model = GameState;

#[derive(Debug)]
enum Msg {
    Click(Position),
    Reset,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self::new()
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(pos) => {
                self.play(pos);
                true
            }
            Msg::Reset => {
                self.reset();
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        // CSS labels
        let xteam_label = "chi";
        let oteam_label = "omi";
        let neutr_label = "neutral";

        // Game status rendering
        let (game_status, indicator) = {
            if self.ongoing {
                if self.turn == Player::X {
                    ("Active player: Chi", xteam_label)
                } else {
                    ("Active player: Omi", oteam_label)
                }
            } else {
                match self.winner {
                    Some(Player::X) => ("Chi Wins!", xteam_label),
                    Some(Player::O) => ("Omi Wins!", oteam_label),
                    None => ("It's a draw!", neutr_label),
                }
            }
        };

        // Single-tile rendering
        let view_tile = |pos: &Position| {
            let (col, row) = *pos;

            let col_label = match col {
                0 => "col-0",
                1 => "col-1",
                _ => "col-2",
            };
            let row_label = match row {
                0 => "row-0",
                1 => "row-1",
                _ => "row-2",
            };
            let tile_label = match self.board.values[col][row] {
                Some(Player::X) => "x-tile",
                Some(Player::O) => "o-tile",
                _ => "empty-tile",
            };

            html! {
                <div
                    class=("tile", tile_label, col_label, row_label)
                    onclick=|_| Msg::Click((col, row))
                >
                </div>
            }
        };

        // Complete rendering
        let positions: [Position; 9] = [
            (0, 0), (1, 0), (2, 0),
            (0, 1), (1, 1), (2, 1),
            (0, 2), (1, 2), (2, 2),
        ];

        html! {
            <div>
                <section class="game-container">
                    <div class="record">
                        <div class="col-0">
                            { format!("Chi victories: {}", self.log.xwins) }
                        </div>
                        <div class="col-1">
                            { format!("Omi victories: {}", self.log.owins) }
                        </div>
                        <div class="col-2">
                            { format!("Draws: {}", self.log.draws) }
                        </div>
                    </div>
                    <section class="game-area">
                        <div class="game-board">
                            { for positions.iter().map(view_tile) }
                        </div>
                        <div class="game-status">
                            { game_status }
                        </div>
                        <div class=("indicator", indicator)>
                        </div>
                        <div class="game-buttons">
                            <button
                                type="button"
                                class="reset-button"
                                onclick=|_| Msg::Reset
                            >
                                {
                                    "RESET"
                                }
                            </button>
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
