//! "King of the Hill" -- rank 4-6 hands by equity, highest first.
//!
//! Scoring is pairwise agreement with the true order (Kendall's tau), so a
//! near-miss still earns credit. See [`score`] for why the raw ratio alone is a
//! misleading number to show a player.

use poker_eval::eval::seven::TableSeven;
use rand::prelude::IndexedRandom;
use rand::rng;

use crate::calculators::equity_calculator::calculate_equity;
use crate::poker_core::{card::Card, deck::Deck};
use crate::problems::problem_helpers::{draw_board, Street};

pub const MIN_HANDS: usize = 4;
pub const MAX_HANDS: usize = 6;

/// Equities closer together than this are the same hand strength.
///
/// Comparing exactly is wrong. Two hands can be mathematically identical and
/// still differ in the last bits of an `f64`: 2d8d and 2h8h are the same hand
/// against a table that is unchanged by swapping those two suits, but the
/// enumeration accumulates their wins in a different order. Measured, that pair
/// comes back exactly equal at two hands and at four, and *unequal* with a lone
/// AsKs alongside -- same value to ten decimal places, different f64.
///
/// The threshold sits far below the smallest real difference there can be, which
/// is a single board out of the millions enumerated.
const TIE_EPSILON: f64 = 1e-9;

fn tied(a: f64, b: f64) -> bool {
    (a - b).abs() < TIE_EPSILON
}

#[derive(Clone, Debug)]
pub struct KingOfHillProblem {
    pub board: Vec<Card>,
    /// Presented unordered; the player supplies the ranking.
    pub hands: Vec<Vec<Card>>,
    /// Exact equity per hand, in the same index space as `hands`.
    pub equities: Vec<f64>,
    /// Indices of `hands`, strongest first.
    pub correct_order: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Score {
    pub correct_pairs: usize,
    pub total_pairs: usize,
    /// Pairwise agreement rescaled so chance sits at zero: +1 perfect, 0 random,
    /// -1 exactly backwards.
    pub kendall_tau: f64,
}

/// Scores a submitted order against the true equities.
///
/// The raw ratio `correct_pairs / total_pairs` is kept because it is what the
/// reveal shows, but it is a poor *score*: any pair is either right or wrong, so
/// a random ordering averages exactly 0.5 regardless of hand count. That makes
/// the bottom half of the range unreachable by anyone genuinely trying, and
/// reports "50%" to a player who knew nothing. Kendall's tau is the same count
/// rescaled to put chance at zero.
///
/// Hands with equal equity are counted as agreeing in either order -- with ties
/// there is no wrong answer to give.
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
                // the player ranked `a` above `b` exactly when it is stronger
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
        kendall_tau: 2.0 * ratio - 1.0,
    }
}

pub fn generate(
    num_hands: usize,
    allowed_streets_str: Vec<String>,
    tables: &TableSeven,
) -> KingOfHillProblem {
    debug_assert!((MIN_HANDS..=MAX_HANDS).contains(&num_hands));

    let mut rng = rng();
    let mut deck = Deck::new();
    deck.shuffle(&mut rng);

    let mut allowed_streets: Vec<Street> = allowed_streets_str
        .iter()
        .filter_map(|s| Street::from_str(s))
        .collect();
    // The river is not offered: with a finished board there are only two
    // equities on the table -- the winner's 100% and everyone else's 0% -- so
    // two thirds of the pairs are ties that agree whichever way round they go.
    // Measured over 400 six-hand spots, a random shuffle scored tau +0.63 there
    // against +0.01 on the flop, where chance is meant to sit. The question
    // collapses to "who won", and the ranking is free.
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

    // strongest first; ties keep their dealt order, which is arbitrary anyway
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
        correct_order,
    }
}
