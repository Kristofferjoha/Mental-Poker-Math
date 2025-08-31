use rand::Rng;

#[derive(Debug, Clone)]
pub struct PurePotOddsProblem {
    pub pot_size: u32,
    pub bet_to_call: u32,
    pub pot_odds: f64,
    pub equity: f64,
    pub correct_decision: bool,
}

pub fn generate_pure_pot_odds_problem(allow_overbets: bool) -> PurePotOddsProblem {
    let mut rng = rand::thread_rng();

    let pot_size = (rng.gen_range(10_000..100_000) / 1000) * 1000;

    let min_bet = pot_size / 4;
    let max_bet = if allow_overbets {
        pot_size * 2
    } else {
        pot_size * 9 / 10
    };

    let effective_min_bet = if min_bet < 5000 { 5000 } else { min_bet };
    let bet_to_call = (rng.gen_range(effective_min_bet..=max_bet) / 1000) * 1000;

    let raw_equity: f64 = rng.gen_range(10.0..90.0);
    let equity = (raw_equity * 100.0).round() / 100.0;

    let raw_pot_odds = (bet_to_call as f64) / ((pot_size + bet_to_call) as f64) * 100.0;
    let pot_odds = (raw_pot_odds * 100.0).round() / 100.0;
    
    let correct_decision = equity > pot_odds;

    PurePotOddsProblem {
        pot_size,
        bet_to_call,
        pot_odds,
        equity,
        correct_decision,
    }
}