use crate::poker_logic::{card::Card, deck::Deck, equity_calculator};
use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Clone, Debug)]
pub enum Street {
    PreFlop,
    Flop,
    Turn,
    River,
}

impl Street {
    fn from_str(s: &str) -> Option<Street> {
        match s {
            "pre-flop" => Some(Street::PreFlop),
            "flop" => Some(Street::Flop),
            "turn" => Some(Street::Turn),
            "river" => Some(Street::River),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PotEquityProblem {
    pub player_hand: Vec<Card>,
    pub opponent_hand: Vec<Card>,
    pub board: Vec<Card>,
    pub pot_size: u32,
    pub bet_to_call: u32,
    pub player_equity: f32,
    pub pot_odds: f32,
    pub correct_decision: bool,
}

pub fn generate_pot_eq_problem(allowed_streets_str: Vec<String>, allow_overbets: bool) -> PotEquityProblem {
    let mut rng = thread_rng();
    let mut deck = Deck::new();
    deck.shuffle(&mut rng);

    let player_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
    let opponent_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];

    let mut allowed_streets: Vec<Street> = allowed_streets_str
        .iter()
        .filter_map(|s| Street::from_str(s))
        .collect();
    
    if allowed_streets.is_empty() {
        allowed_streets = vec![Street::PreFlop, Street::Flop, Street::Turn, Street::River];
    }
    let chosen_street = allowed_streets.choose(&mut rng).unwrap();
    let board = draw_board(&mut deck, chosen_street);

    let pot_size = (rng.gen_range(10_000..100_000) / 1000) * 1000;
    let min_bet = pot_size / 4;
    let max_bet = if allow_overbets {
        pot_size * 3 / 2
    } else {
        pot_size * 9 / 10
    };
    let effective_min_bet = if min_bet < 5000 { 5000 } else { min_bet };
    let bet_to_call = (rng.gen_range(effective_min_bet..=max_bet) / 1000) * 1000;

    let equity_result = equity_calculator::calculate_equity(
        &player_hand,
        &opponent_hand,
        &board,
        10_000,
    );
    let player_equity = equity_result.equity();
    let pot_odds = (bet_to_call as f32) / ((pot_size + bet_to_call) as f32);
    let correct_decision = player_equity > pot_odds;

    PotEquityProblem {
        player_hand,
        opponent_hand,
        board,
        pot_size,
        bet_to_call,
        player_equity,
        pot_odds,
        correct_decision,
    }
}

fn draw_board(deck: &mut Deck, stage: &Street) -> Vec<Card> {
    let num_cards = match stage {
        Street::PreFlop => 0,
        Street::Flop => 3,
        Street::Turn => 4,
        Street::River => 5,
    };
    (0..num_cards)
        .map(|_| deck.cards.pop().unwrap())
        .collect()
}