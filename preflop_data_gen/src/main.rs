use serde::Serialize;

mod tools;
mod poker_logic;
mod starting_hands;
mod utils;

use tools::preflop_equity_gen::run_program;


#[derive(Serialize)]
pub struct PreflopEquity {
    hand1: String,
    hand2: String,
    equity: f32,
}

const NUM_OF_SIMULATIONS: u32 = 500;

fn main() {
    run_program();
}