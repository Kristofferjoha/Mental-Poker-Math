use rand::Rng;
use tracing::info;
use rand::prelude::IndexedRandom;

/// Represents a pure pot odds problem where you have to decide whether to call a bet based on pot odds and equity.
/// Struct Given pot size, bet to call, and equity percentage to user. 
/// Struct also has pot odds and correct decision calculated.
/// Must calculate pot odds and decide if calling is correct.

#[derive(Debug, Copy, Clone)] 
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}


#[derive(Debug, Clone)]
pub struct PurePotOddsProblem {
    pub pot_size: u32,
    pub bet_to_call: u32,
    pub pot_odds: f64,
    pub equity: f64,
    pub correct_decision: bool,
}

pub fn generate(allow_overbets: bool) -> PurePotOddsProblem {

    let mut rng = rand::rng();

    const DIFFICULTIES: [Difficulty; 3] = [
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Hard,
    ];

    let difficulty = *DIFFICULTIES.choose(&mut rng).unwrap();
    // Pot size rounded to nearest 1000. 
    let initial_pot = (rng.random_range(10_000..100_000) / 1000) * 1000;

    // Bet to call between 1/4 pot and 2x pot (if allow_overbets)
    let min_bet = initial_pot / 4;
    let max_bet = if allow_overbets {
        initial_pot * 2
    } else {
        initial_pot * 9 / 10
    };

    let effective_min_bet = min_bet.max(5000);
    let call_amount = (rng.random_range(effective_min_bet..=max_bet) / 1000) * 1000;

    // Final pot = (pot before bet) + (opponent's bet) + (our call)
    let final_pot = initial_pot + call_amount+ call_amount;
    let required_equity_raw = call_amount as f64 / final_pot as f64;
    let required_equity = (required_equity_raw * 10000.0).round() / 100.0;

    let is_correct_to_call = rng.random_bool(0.5);

    let (min_margin, max_margin) = match difficulty {
        Difficulty::Easy => (15.0, 30.0),
        Difficulty::Medium => (5.0, 15.0),
        Difficulty::Hard => (0.5, 5.0),
    };
    let margin = rng.random_range(min_margin..max_margin);

    // Calculate player equity based on the correct decision and the margin.
    let player_equity_raw = if is_correct_to_call {
        required_equity + margin
    } else {
        required_equity - margin
    };

    let player_equity = player_equity_raw.clamp(5.0, 95.0).round();

    let correct_decision = player_equity > required_equity;

    info!(
        "Generated Problem (Difficulty::{:?}): initial_pot={}, call_amount={}, required_equity={:.2}%, player_equity={:.2}% -> {}",
        difficulty, initial_pot, call_amount, required_equity, player_equity, if correct_decision {"CALL"} else {"FOLD"}
    );
    // Displayed pot includes the enemy bet to call
    let displyed_pot = initial_pot + call_amount;

    PurePotOddsProblem {
        pot_size: displyed_pot,
        bet_to_call: call_amount,
        pot_odds: required_equity,
        equity: player_equity,
        correct_decision,
    }
}