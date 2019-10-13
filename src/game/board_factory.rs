use super::game_move::GameMove;
use super::board_move::{BoardMove, BoardMoveType};
use super::validation_state::ValidationState;
use super::board_indexing;
use super::board::GameBoard;

use std::error::Error;

pub struct BoardFactory {
    /// Contains the dimensions of one square of the puzzle.
    /// ie: the typical 9-square grid is represented by 3.
    grid_dim: u8,

    moves: Vec<BoardMove>
}

impl BoardFactory {
    pub fn new(grid_size: u8) -> Self {
        Self {
            grid_dim: grid_size,
            moves: Vec::new()
        }
    }

    pub fn populate_grid(&mut self, grid: &str) -> Result<(), &'static str> {
        // Validate the input
        let expected_dim = board_indexing::expected_size(self.grid_dim);
        board_indexing::validate_size(expected_dim, &grid)?;

        // Build the game board
        self.moves.reserve(expected_dim as usize);

        for (i, c) in grid.chars().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                let digit = digit as u8;
                let i :u8 = i as u8;
                let (x, y, grid) = board_indexing::from_index(self.grid_dim, i);

                if digit > self.grid_dim * self.grid_dim {
                    return Err("Invalid value discovered in puzzle input");
                }

                let game_move = BoardMove {
                    move_type: BoardMoveType::Setup,
                    game_move: GameMove::new(x, y, grid, digit)
                };

                self.moves.push(game_move);
            }
        }

        Ok(())
    }

    pub fn iterate(&mut self) -> Result<ValidationState, Box<dyn Error>> {
        let board = GameBoard::populated(self.grid_dim, &self.moves)?;

        let mut status = match board.validate() {
            Ok(_) => ValidationState::NextMove,
            Err(_) => match self.moves.last().ok_or("Game board is empty")?.move_type {
                BoardMoveType::Setup => ValidationState::Unsolvable,
                BoardMoveType::Reversable => ValidationState::BadMove,
            }
        };

        if let ValidationState::NextMove = status {
            match board.next_index() {
                Some((x, y, grid)) => {
                    let game_move = BoardMove {
                        move_type: BoardMoveType::Reversable,
                        game_move: GameMove::new(x, y, grid, 1)
                    };
                    self.moves.push(game_move);
                },
                None => {
                    status = ValidationState::Complete;
                }
            }
        } else if let ValidationState::BadMove = status {
            let bad_move = self.moves.pop().ok_or("Game board is empty")?;
            if bad_move.game_move.value != self.grid_dim * self.grid_dim {
                let game_move = BoardMove {
                    move_type: BoardMoveType::Reversable,
                    game_move: GameMove::new(bad_move.game_move.x_pos, bad_move.game_move.y_pos, bad_move.game_move.grid_index, bad_move.game_move.value + 1)
                };
                self.moves.push(game_move);
            } else {
                loop {
                    match self.moves.pop() {
                        Some(prev_move) => {
                            if prev_move.game_move.value != self.grid_dim * self.grid_dim {
                                let game_move = BoardMove {
                                    move_type: BoardMoveType::Reversable,
                                    game_move: GameMove::new(prev_move.game_move.x_pos, prev_move.game_move.y_pos, prev_move.game_move.grid_index, prev_move.game_move.value + 1)
                                };
                                self.moves.push(game_move);
                                break;
                            }
                        },
                        None => {
                            status = ValidationState::Unsolvable;
                            break;
                        }
                    }
                }
            }
        }
        
        Ok(status)
    }
}
