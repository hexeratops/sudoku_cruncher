mod game;
mod file_reader;

use std::env;
use game::board_factory::BoardFactory;
use game::validation_state::ValidationState;

fn collect_args(args: &[String]) -> Result<(u8, String), &'static str> {
    if args.len() != 3 {
        return Err("Provided invalid CLI arguments");
    }

    let grid_dims: u8 = args[1].parse().expect("Failed to convert grid dimensions to numeric value.");
    let filename: String = args[2].to_owned();
    Ok((grid_dims, filename))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_args: Vec<String> = env::args().collect();
    let args = collect_args(&raw_args)?;

    let mut gb_factory = BoardFactory::new(args.0);
    let game_grid = file_reader::read_file(&args.1).expect("Failed to read provided game board");

    gb_factory.populate_grid(&game_grid)?;

    loop {
        match gb_factory.iterate()? {
            ValidationState::Complete => {
                println!("Board complete~");
                break;
            },
            ValidationState::Unsolvable => { 
                println!("Puzzle impossible!!");
                break;
            },
            _ => (),
        }
    }

    Ok(())
}
