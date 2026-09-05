use rand::{rng, Rng};
use rand::prelude::IndexedRandom;
use poker_eval::eval::seven::TableSeven;
use std::sync::Arc;

use crate::poker_core::{card::Card, deck::Deck};
use crate::calculators::equity_calculator;
use crate::problems::problem_helpers::{draw_board, Street};

#[derive(Clone, Debug)]
pub struct PureEqEquityProblem {
    pub hands: Vec<Vec<Card>>,
    pub board: Vec<Card>,
    pub player_equity: f32,
    pub lower_bound_equity: f32,
    pub upper_bound_equity: f32
}

pub fn generate(
    allowed_streets_str: Vec<String>,
    num_players: usize,
    tolerance: f32,
    seven_card_tables: &Arc<TableSeven>
) -> PureEqEquityProblem {
    let mut rng = rng();
    let mut deck = Deck::new();
    deck.shuffle(&mut rng);

    let mut allowed_streets: Vec<Street> = allowed_streets_str
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    if allowed_streets.is_empty() {
        allowed_streets = vec![Street::PreFlop, Street::Flop, Street::Turn, Street::River];
    }

    let chosen_street = allowed_streets.choose(&mut rng).unwrap();

    const MIN_INTERESTING: f32 = 0.02;
    const MAX_INTERESTING: f32 = 0.98;

    let river = *chosen_street == Street::River;
    let want_win = rng.random_bool(0.5);
    let worth_posing = |equity: f32| {
        if river {
            (equity > 0.99) == want_win
        } else {
            (MIN_INTERESTING..=MAX_INTERESTING).contains(&equity)
        }
    };

    let mut deal = || {
        let mut deck = Deck::new();
        deck.shuffle(&mut rng);
        let hands: Vec<Vec<Card>> = (0..num_players)
            .map(|_| vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()])
            .collect();
        let board = draw_board(&mut deck, chosen_street);
        let seats: Vec<&[Card]> = hands.iter().map(|h| h.as_slice()).collect();
        let equity =
            equity_calculator::calculate_equity(&seats, &board, seven_card_tables).hero() as f32;
        (hands, board, equity)
    };

    let attempts = if river { 400 } else { 32 };
    let mut dealt = deal();
    for _ in 0..attempts {
        if worth_posing(dealt.2) {
            break;
        }
        dealt = deal();
    }
    let (hands, board, player_equity) = dealt;

    let player_equity_percentage = player_equity * 100.0;
    let lower_bound_equity = (player_equity_percentage - tolerance).max(0.0);
    let upper_bound_equity = (player_equity_percentage + tolerance).min(100.0);

    PureEqEquityProblem {
        hands,
        board,
        player_equity: player_equity_percentage,
        lower_bound_equity,
        upper_bound_equity
    }
}
