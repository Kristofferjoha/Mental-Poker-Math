use rand::prelude::IndexedRandom;
use rand::{Rng, rng};
use poker_eval::eval::seven::TableSeven;
use std::sync::Arc;

use crate::poker_core::{card::Card, deck::Deck};
use crate::calculators::equity_calculator;
use crate::problems::problem_helpers::{draw_board, Street};

#[derive(Clone, Debug)]
pub struct PotEquityProblem {
    pub hands: Vec<Vec<Card>>,
    pub board: Vec<Card>,
    pub pot_size: u32,
    pub bet_to_call: u32,
    pub player_equity: f32,
    pub pot_odds: f32,
    pub correct_decision: bool
}

pub fn generate(
    allowed_streets_str: Vec<String>,
    num_players: usize,
    allow_overbets: bool,
    seven_card_tables: &Arc<TableSeven>
) -> PotEquityProblem {
    let mut rng = rng();
    let mut deck = Deck::new();
    deck.shuffle(&mut rng);

    let mut allowed_streets: Vec<Street> = allowed_streets_str
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    allowed_streets.retain(|s| *s != Street::River);
    if allowed_streets.is_empty() {
        allowed_streets = vec![Street::PreFlop, Street::Flop, Street::Turn];
    }
    let chosen_street = allowed_streets.choose(&mut rng).unwrap();

    let max_multiple = if allow_overbets { 2.0f32 } else { 1.0 };
    let cheapest = MIN_BET_MULTIPLE / (1.0 + 2.0 * MIN_BET_MULTIPLE);
    let dearest = max_multiple / (1.0 + 2.0 * max_multiple);
    let (floor, ceiling) = (cheapest + 0.02, dearest - 0.02);

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

    let miss = |equity: f32| {
        if equity < floor {
            floor - equity
        } else if equity > ceiling {
            equity - ceiling
        } else {
            0.0
        }
    };

    let mut best = deal();
    for _ in 0..48 {
        if miss(best.2) == 0.0 {
            break;
        }
        let candidate = deal();
        if miss(candidate.2) < miss(best.2) {
            best = candidate;
        }
    }
    let (hands, board, player_equity) = best;

    let pot_size = (rng.random_range(10_000..100_000) / 1000) * 1000;
    let bet_to_call = bet_for_close_decision(pot_size, player_equity, allow_overbets, &mut rng);
    let pot_odds = bet_to_call as f32 / (pot_size + bet_to_call+bet_to_call) as f32;

    PotEquityProblem {
        hands,
        board,
        pot_size,
        bet_to_call,
        player_equity,
        pot_odds,
        correct_decision: player_equity > pot_odds
    }
}

const MIN_BET_MULTIPLE: f32 = 0.05;

fn bet_for_close_decision(
    pot_size: u32,
    equity: f32,
    allow_overbets: bool,
    rng: &mut impl Rng
) -> u32 {
    let margin = rng.random_range(0.015f32..0.09);
    let target = if rng.random_bool(0.5) { equity - margin } else { equity + margin };
    let target = target.clamp(0.04, 0.45);

    let ideal = target * pot_size as f32 / (1.0 - 2.0 * target);

    let max_bet = if allow_overbets { pot_size * 2 } else { pot_size };
    let min_bet = (pot_size as f32 * MIN_BET_MULTIPLE) as u32;
    let bet = (ideal.round() as i64).clamp(min_bet as i64, max_bet as i64) as u32;
    (bet / 1000).max(1) * 1000
}
