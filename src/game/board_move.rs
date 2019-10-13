use super::game_move::GameMove;

#[derive(Debug)]
pub struct BoardMove {
    pub game_move: GameMove,
    pub move_type: BoardMoveType
}

#[derive(Debug)]
pub enum BoardMoveType {
    Setup,
    Reversable
}