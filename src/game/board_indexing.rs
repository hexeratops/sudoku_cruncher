pub fn from_index(grid_dim: u8, pos: u8) -> (u8, u8, u8) {
    let grid_pos :u8 = ((pos / (grid_dim * grid_dim * grid_dim)) * grid_dim) + ((pos / grid_dim) % grid_dim);
    let y_pos :u8 = (pos / (grid_dim * grid_dim)) % grid_dim;
    let x_pos :u8 = pos % grid_dim;

    (x_pos, y_pos, grid_pos)
}


pub fn from_position(grid_size: u8, grid: u8, x_pos: u8, y_pos: u8) -> u8 {
    let grid_x_offset = (grid % grid_size) * grid_size;
    let grid_y_offset = grid_size * grid_size * grid_size * (grid / grid_size);
    let y_index = y_pos * grid_size * grid_size;
    let x_index = x_pos;

    grid_x_offset + grid_y_offset + y_index + x_index
}


pub fn expected_size(grid_dim: u8) -> u16 {
    (grid_dim * grid_dim * grid_dim * grid_dim).into()
}


pub fn validate_size(expected_dim: u16, grid: &str) -> Result<(), &'static str> {
    if grid.chars().count() != expected_dim as usize {
        return Err("Requested size does not match the provided puzzle.");
    }
    Ok(())
}