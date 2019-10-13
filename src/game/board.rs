use super::board_move::BoardMove;
use super::board_move::BoardMoveType;
use super::board_indexing;
use std::error::Error;
use std::collections::HashSet;

#[derive(Debug)]
pub struct GameBoard {
    board: Vec<u8>,
    grid_dim: u8,
}

impl GameBoard {
    pub fn populated(grid_size: u8, board: &Vec<BoardMove>) -> Result<Self, Box<dyn Error>> {
        let expected_dim: usize = (grid_size * grid_size * grid_size * grid_size).into();
        let mut game_board: Vec<u8> = vec![0; expected_dim];

        for cell in board.iter() {
            match cell.move_type {
                BoardMoveType::Setup => {
                    let game_move = &cell.game_move;
                    let index = board_indexing::from_position(grid_size, game_move.grid_index, game_move.x_pos, game_move.y_pos);
                    let pos = game_board.get_mut(index as usize).ok_or(format!("Failed to get index {} (x: {}, y: {}, g:{}, gs: {})", index, game_move.x_pos, game_move.y_pos, game_move.grid_index, grid_size))?;
                    *pos = game_move.value;
                },
                BoardMoveType::Reversable => {
                    let game_move = &cell.game_move;
                    let index = board_indexing::from_position(grid_size, game_move.grid_index, game_move.x_pos, game_move.y_pos);
                    let pos = game_board.get_mut(index as usize).ok_or(format!("Failed to get index {} (x: {}, y: {}, g:{}, gs: {})", index, game_move.x_pos, game_move.y_pos, game_move.grid_index, grid_size))?;
                    *pos = game_move.value;
                }
            }
        }


        Ok(Self {
            board: game_board,
            grid_dim: grid_size
        })
    }

    fn validate_rows(&self) -> Result<(), &'static str> {
        for col in 0..(self.grid_dim) {
            for col_grid in 0..(self.grid_dim) {
                let mut row_vals = HashSet::new();

                for row_grid in 0..(self.grid_dim) {
                    for row in 0..(self.grid_dim) {
                        let index = board_indexing::from_position(self.grid_dim, row_grid + (col_grid * self.grid_dim), row, col);
                        let cell_val = self.board[index as usize];

                        if cell_val == 0 {
                            continue;
                        }

                        if row_vals.contains(&cell_val) {
                            return Err("Found row containing duplicate value");
                        } else {
                            row_vals.insert(cell_val);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_cols(&self) -> Result<(), &'static str> {
        for row in 0..(self.grid_dim) {
            for row_grid in 0..(self.grid_dim) {
                let mut col_vals = HashSet::new();

                for col_grid in 0..(self.grid_dim) {
                    for col in 0..(self.grid_dim) {
                        let index = board_indexing::from_position(self.grid_dim, row_grid + (col_grid * self.grid_dim), row, col);
                        let cell_val = self.board[index as usize];

                        if cell_val == 0 {
                            continue;
                        }

                        if col_vals.contains(&cell_val) {
                            return Err("Found column containing duplicate value");
                        } else {
                            col_vals.insert(cell_val);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_grids(&self) -> Result<(), &'static str> {
        for row_grid in 0..(self.grid_dim) {
            for col_grid in 0..(self.grid_dim) {
                let mut col_vals = HashSet::new();

                for row in 0..(self.grid_dim) {
                    for col in 0..(self.grid_dim) {
                        let index = board_indexing::from_position(self.grid_dim, row_grid + (col_grid * self.grid_dim), row, col);
                        let cell_val = self.board[index as usize];
                        
                        if cell_val == 0 {
                            continue;
                        }

                        if col_vals.contains(&cell_val) {
                            return Err("Found column containing duplicate value");
                        } else {
                            col_vals.insert(cell_val);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        self.validate_rows()?;
        self.validate_cols()?;
        self.validate_grids()?;
        Ok(())
    }

    pub fn next_index(&self) -> Option<(u8, u8, u8)> {
        for (pos, e) in self.board.iter().enumerate() {
            if *e == 0 {
                return Some(board_indexing::from_index(self.grid_dim, pos as u8));
            }
        }
        self.dump_board();
        None // We've finished iterating! Everything's set to a value.
    }

    pub fn dump_board(&self) {
        for col_grid in 0..(self.grid_dim) {
            for col in 0..(self.grid_dim) {
                for row_grid in 0..(self.grid_dim) {
                    if row_grid == 0 {
                        print!("|");
                    }
                    
                    for row in 0..(self.grid_dim) {
                        let index = board_indexing::from_position(self.grid_dim, row_grid + (col_grid * self.grid_dim), row, col);
                        let cell_val = self.board[index as usize];

                        print!("{}", cell_val);
                    }
                    print!("|");
                }
                println!("");
            }
            println!("");
        }
    }
}
