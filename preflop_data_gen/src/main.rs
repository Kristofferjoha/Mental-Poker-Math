mod tools;
mod poker_objects;
mod starting_hands;
mod utils;

use tools::generator::preflop_equity_generation;

const CHUNK_SIZE: usize = 50;

/// Command line program for calculating preflop equity for starting hands.

fn main() -> std::io::Result<()> {
    const NUM_OF_SIMULATIONS: u32 = 500;

    preflop_equity_generation(NUM_OF_SIMULATIONS)?;
    Ok(())
}