use rand::{rng};
use rand::prelude::IndexedRandom;
use poker_eval::eval::seven::TableSeven;
use std::sync::Arc;
use tracing::info;

use crate::poker_core::{card::Card, deck::Deck};
use crate::preflop_data::preflop_lookup::PreflopEquity;
use crate::calculators::preflop_scenarios::generate_preflop_scenario;
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
    preflop_data: &[PreflopEquity],
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


    let (player_hand, opponent_hand, board, player_equity) = if *chosen_street == Street::PreFlop {
        //preflop
        generate_preflop_scenario(preflop_data, &mut deck, &mut rng, seven_card_tables)
    } else {
        // post-flop logic
        let player_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
        let opponent_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
        
        let board = draw_board(&mut deck, chosen_street);
        
        let equity_result = equity_calculator::calculate_equity(&player_hand, &opponent_hand, &board, 25_000, seven_card_tables);
        
        (player_hand, opponent_hand, board, equity_result.equity())
    };

    // Convert equity to percentage and calculate bounds.
    let player_equity_percentage = player_equity * 100.0;
    let lower_bound_equity = (player_equity_percentage - tolerance).max(0.0);
    let upper_bound_equity = (player_equity_percentage + tolerance).min(100.0);

    info!("Generated Pure Equity Problem: Player Equity: {:.2}%, Bounds: [{:.2}%, {:.2}%], Directional Hints Active: {}", 
        player_equity_percentage, lower_bound_equity, upper_bound_equity, directional_hints_active);

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