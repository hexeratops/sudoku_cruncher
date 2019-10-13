#[derive(Debug)]
pub struct GameMove {
    pub x_pos: u8,
    pub y_pos: u8,
    pub grid_index: u8,

    pub value: u8
}

impl GameMove {
    pub fn new(x_pos: u8, y_pos: u8, grid_index: u8, value: u8) -> Self {
        Self {
            x_pos: x_pos,
            y_pos: y_pos,
            grid_index: grid_index,
            value: value
        }
    }
}
