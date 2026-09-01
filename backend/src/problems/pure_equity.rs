use rand::{rng};
use rand::prelude::IndexedRandom;
use poker_eval::eval::seven::TableSeven;
use std::sync::Arc;

use crate::poker_core::{card::Card, deck::Deck};
use crate::calculators::equity_calculator;
use crate::problems::problem_helpers::{draw_board, Street};

#[derive(Clone, Debug)]
pub struct PureEqEquityProblem {
    pub player_hand: Vec<Card>,
    pub opponent_hand: Vec<Card>,
    pub board: Vec<Card>,
    pub player_equity: f32,
    pub lower_bound_equity: f32,
    pub upper_bound_equity: f32,
    pub directional_hint_active: bool,
}


pub fn generate(
    allowed_streets_str: Vec<String>,
    tolerance: f32,
    directional_hints_active: bool,
    seven_card_tables: &Arc<TableSeven>,
) -> PureEqEquityProblem {
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


    let player_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
    let opponent_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
    let board = draw_board(&mut deck, chosen_street);

    let player_equity = equity_calculator::calculate_equity(
        &[&player_hand, &opponent_hand],
        &board,
        seven_card_tables,
    )
    .hero() as f32;

    // Convert equity to percentage and calculate bounds.
    let player_equity_percentage = player_equity * 100.0;
    let lower_bound_equity = (player_equity_percentage - tolerance).max(0.0);
    let upper_bound_equity = (player_equity_percentage + tolerance).min(100.0);

    PureEqEquityProblem {
        player_hand,
        opponent_hand,
        board,
        player_equity: player_equity_percentage,
        lower_bound_equity,
        upper_bound_equity,
        directional_hint_active: directional_hints_active,
    }
}