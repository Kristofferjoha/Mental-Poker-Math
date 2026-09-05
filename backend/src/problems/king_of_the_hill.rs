use poker_eval::eval::seven::TableSeven;
use rand::prelude::IndexedRandom;
use rand::rng;

use crate::calculators::equity_calculator::calculate_equity;
use crate::poker_core::{card::Card, deck::Deck};
use crate::problems::problem_helpers::{draw_board, Street};

pub const MIN_HANDS: usize = 4;
pub const MAX_HANDS: usize = 6;

const TIE_EPSILON: f64 = 1e-9;

fn tied(a: f64, b: f64) -> bool {
    (a - b).abs() < TIE_EPSILON
}

#[derive(Clone, Debug)]
pub struct KingOfHillProblem {
    pub board: Vec<Card>,
    pub hands: Vec<Vec<Card>>,
    pub equities: Vec<f64>,
    pub correct_order: Vec<usize>
}

#[derive(Clone, Debug, PartialEq)]
pub struct Score {
    pub correct_pairs: usize,
    pub total_pairs: usize,
    pub kendall_tau: f64
}

pub fn score(guess: &[usize], equities: &[f64]) -> Score {
    let n = equities.len();
    let mut position = vec![0usize; n];
    for (slot, &hand) in guess.iter().enumerate() {
        position[hand] = slot;
    }

    let mut correct_pairs = 0;
    let mut total_pairs = 0;
    for a in 0..n {
        for b in (a + 1)..n {
            total_pairs += 1;
            let agrees = if tied(equities[a], equities[b]) {
                true
            } else {
                (position[a] < position[b]) == (equities[a] > equities[b])
            };
            if agrees {
                correct_pairs += 1;
            }
        }
    }

    let ratio = if total_pairs == 0 {
        1.0
    } else {
        correct_pairs as f64 / total_pairs as f64
    };

    Score {
        correct_pairs,
        total_pairs,
        kendall_tau: 2.0 * ratio - 1.0
    }
}

pub fn generate(
    num_hands: usize,
    allowed_streets_str: Vec<String>,
    tables: &TableSeven
) -> KingOfHillProblem {
    debug_assert!((MIN_HANDS..=MAX_HANDS).contains(&num_hands));

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

    let hands: Vec<Vec<Card>> = (0..num_hands)
        .map(|_| vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()])
        .collect();
    let board = draw_board(&mut deck, chosen_street);

    let seats: Vec<&[Card]> = hands.iter().map(|h| h.as_slice()).collect();
    let equities = calculate_equity(&seats, &board, tables).equities;

    let mut correct_order: Vec<usize> = (0..num_hands).collect();
    correct_order.sort_by(|&a, &b| {
        equities[b]
            .partial_cmp(&equities[a])
            .expect("equities are never NaN")
    });

    KingOfHillProblem {
        board,
        hands,
        equities,
        correct_order
    }
}
