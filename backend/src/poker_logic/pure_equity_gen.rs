use crate::poker_logic::{card::Card, deck::Deck, equity_calculator};
use rand::seq::SliceRandom;
use rand::{thread_rng};

// Problem struct for pot-equity questions
#[derive(Clone, Debug)]
pub struct PureEqEquityProblem {
    pub player_hand: Vec<Card>,
    pub opponent_hand: Vec<Card>,
    pub board: Vec<Card>,
    pub player_equity: f32,
    pub lower_bound_equity: f32,
    pub upper_bound_equity: f32,
}

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

pub fn generate_pure_eq_problem(allowed_streets_str: Vec<String>) -> PureEqEquityProblem {
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

    let equity_result = equity_calculator::calculate_equity(
        &player_hand,
        &opponent_hand,
        &board,
        25_000,
    );

    let player_equity = equity_result.equity() * 100.0;
    let tolerance = 2.0;
    let lower_bound_equity = player_equity - tolerance;
    let upper_bound_equity = player_equity + tolerance;

    PureEqEquityProblem {
        player_hand,
        opponent_hand,
        board,
        player_equity,
        lower_bound_equity,
        upper_bound_equity,
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