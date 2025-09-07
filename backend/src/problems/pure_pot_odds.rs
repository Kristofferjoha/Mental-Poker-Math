use rand::Rng;

/// Represents a pure pot odds problem where you have to decide whether to call a bet based on pot odds and equity.
/// Struct Given pot size, bet to call, and equity percentage to user. 
/// Struct also has pot odds and correct decision calculated.
/// Must calculate pot odds and decide if calling is correct.


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

    // Pot size rounded to nearest 1000
    let pot_size = (rng.random_range(10_000..100_000) / 1000) * 1000;

    // Bet to call between 1/4 pot and 2x pot (if allow_overbets), else between 1/4 pot and 9/10 pot
    let min_bet = pot_size / 4;
    let max_bet = if allow_overbets {
        pot_size * 2
    } else {
        pot_size * 9 / 10
    };

    // Ensure minimum bet is at least 5000 to avoid trivial decisions
    let effective_min_bet = if min_bet < 5000 { 5000 } else { min_bet };

    // Bet to call rounded to nearest 1000
    let bet_to_call = (rng.random_range(effective_min_bet..=max_bet) / 1000) * 1000;

    // Equity as a percentage between 10% and 90%, rounded to 2 decimals
    let raw_equity: f64 = rng.random_range(10.0..90.0);
    let equity_percentage = (raw_equity * 100.0).round() / 100.0; // e.g. 42.37 (%)

    // Current pot (after enemy bet) + our call
    let final_pot = pot_size + bet_to_call;

    // Required equity as a fraction (0.0–1.0)
    let required_equity = bet_to_call as f64 / final_pot as f64;

    // Pot odds in percent
    let pot_odds_percentage = required_equity * 100.0; // e.g. 25.0 (%)

    let correct_decision = equity_percentage > pot_odds_percentage;

    PurePotOddsProblem {
        pot_size,
        bet_to_call,
        pot_odds: pot_odds_percentage,
        equity: equity_percentage,
        correct_decision,
    }
}