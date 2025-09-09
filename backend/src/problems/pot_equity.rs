use rand::prelude::IndexedRandom;
use rand::{Rng, rng};
use poker_eval::eval::seven::TableSeven;
use std::sync::Arc;


use crate::poker_core::{card::Card, deck::Deck};
use crate::calculators::equity_calculator;
use crate::preflop_data::preflop_lookup::PreflopEquity;
use crate::calculators::preflop_scenarios::generate_preflop_scenario;
use crate::problems::problem_helpers::{draw_board, Street};

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

pub fn generate(
    allowed_streets_str: Vec<String>,
    allow_overbets: bool,
    preflop_data: &[PreflopEquity],
    seven_card_tables: &Arc<TableSeven>,
) -> PotEquityProblem {
    let mut rng = rng();
    let mut deck = Deck::new();
    deck.shuffle(&mut rng);

    // Parse allowed streets from strings, defaulting to all if none valid.
    let mut allowed_streets: Vec<Street> = allowed_streets_str
        .iter()
        .filter_map(|s| Street::from_str(s))
        .collect();

    if allowed_streets.is_empty() {
        allowed_streets = vec![Street::PreFlop, Street::Flop, Street::Turn, Street::River];
    }
    // Randomly choose one of the allowed streets.
    let chosen_street = allowed_streets.choose(&mut rng).unwrap();

    // Generate hands and board based on chosen street.
    let (player_hand, opponent_hand, board, player_equity) = if *chosen_street == Street::PreFlop {
        generate_preflop_scenario(preflop_data, &mut deck, &mut rng, seven_card_tables)
    } else {
        // Postflop: deal two random hands and draw board cards.
        let player_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
        let opponent_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
        let board = draw_board(&mut deck, chosen_street);

        let equity_result = equity_calculator::calculate_equity(
            &player_hand, 
            &opponent_hand, 
            &board, 
            25_000, 
            seven_card_tables
        );
        let player_equity = equity_result.equity();

        (player_hand, opponent_hand, board, player_equity)
    };

    // Generate pot size and bet to call.
    let pot_size = (rng.random_range(10_000..100_000) / 1000) * 1000;
    let bet_to_call = generate_bet_size(pot_size, allow_overbets, &mut rng);
    let pot_odds = bet_to_call as f32 / (pot_size + bet_to_call+bet_to_call) as f32;

    PotEquityProblem {
        player_hand,
        opponent_hand,
        board,
        pot_size,
        bet_to_call,
        player_equity,
        pot_odds,
        correct_decision: player_equity > pot_odds,
    }
}

fn generate_bet_size(pot_size: u32, allow_overbets: bool, rng: &mut impl Rng) -> u32 {
    let min_bet = pot_size / 4;
    let max_bet = if allow_overbets { pot_size * 3 / 2 } else { pot_size * 9 / 10 };
    let effective_min_bet = if min_bet < 5000 { 5000 } else { min_bet };
    (rng.random_range(effective_min_bet..=max_bet) / 1000) * 1000
}