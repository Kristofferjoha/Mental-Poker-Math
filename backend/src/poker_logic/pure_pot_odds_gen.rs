use rand::Rng;

#[derive(Debug, Clone)]
pub struct PurePotOddsProblem {
    pub pot_size: u32,
    pub bet_to_call: u32,
    pub pot_odds: f64,
    pub equity: f64,
    pub correct_decision: bool,
}

pub fn generate_pure_pot_odds_problem() -> PurePotOddsProblem {
    let mut rng = rand::thread_rng();

    let equity = rng.gen_range(10.0..90.0); 

    let pot_size = (rng.gen_range(10_000..100_000) / 1000) * 1000;

    let min_bet = pot_size / 4;
    let max_bet = pot_size * 3 / 2; 

    let effective_min_bet = if min_bet < 5000 { 5000 } else { min_bet };

    let bet_to_call = (rng.gen_range(effective_min_bet..=max_bet) / 1000) * 1000;
    let pot_odds = (bet_to_call as f64) / ((pot_size + bet_to_call) as f64) * 100.0;
    
    let correct_decision = equity > pot_odds;

    PurePotOddsProblem {
        pot_size,
        bet_to_call,
        pot_odds,
        equity,
        correct_decision,
    }
}