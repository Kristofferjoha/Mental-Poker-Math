use poker_eval::eval::seven as seven_eval;
use poker_eval::eval::seven::TableSeven;
use rand::seq::SliceRandom;
use rayon::prelude::*;
use std::sync::Arc;

use crate::poker_core::{card::{Card}, deck::Deck,};
use crate::calculators::calculator_helpers::{Equity, card_to_poker_eval_id};

/// Calculates the equity of the player's hand against the opponent's hand given the current board.
/// `player_hand`: The player's hole cards.
/// `opponent_hand`: The opponent's hole cards.
/// `board`: The community cards on the board.
/// `num_simulations`: The number of Monte Carlo simulations to run for estimating equity.
/// `tables`: Precomputed tables for seven-card hand evaluation from poker_eval crate.
/// 
/// Returns an `Equity` struct containing wins, ties, and total simulations.

pub fn calculate_equity(
    player_hand: &[Card],
    opponent_hand: &[Card],
    board: &[Card],
    num_simulations: u32,
    tables: &Arc<TableSeven>,
) -> Equity {
    // If board is complete
    if board.len() == 5 {
        // Converts player + board cards to poker_eval IDs
        let player_cards_vec: Vec<usize> = player_hand.iter().chain(board.iter()).map(card_to_poker_eval_id).collect();
        let opponent_cards_vec: Vec<usize> = opponent_hand.iter().chain(board.iter()).map(card_to_poker_eval_id).collect();

        let player_cards: [usize; 7] = player_cards_vec.try_into().expect("Hand should have 7 cards");
        let opponent_cards: [usize; 7] = opponent_cards_vec.try_into().expect("Hand should have 7 cards");

        let player_rank = seven_eval::get_rank(tables, player_cards); // fx 3232
        let opponent_rank = seven_eval::get_rank(tables, opponent_cards); // fx 3231

        let (wins, ties) = if player_rank > opponent_rank {
            (1, 0)
        } else if player_rank < opponent_rank {
            (0, 0)
        } else {
            (0, 1)
        };
        // Only one simulation needed since outcome is deterministic
        return Equity { wins, ties, total_sims: 1 };
    }

    // Build remaining deck (exclude player hand, opponent hand, and current board)
    let remaining_deck = {
        let mut d = Deck::new();
        d.remove_cards(player_hand);
        d.remove_cards(opponent_hand);
        d.remove_cards(board);
        d
    };

    // Run Monte Carlo simulations in parallel
    let results = (0..num_simulations)
        .into_par_iter() // parallel iterator (Rayon)
        .map_init(
            || rand::rng(), // initialize RNG per worker thread
            |rng, _| {
                // Clone and shuffle the remaining deck
                let mut deck_copy = remaining_deck.clone();
                deck_copy.cards.shuffle(rng);

                // Draw however many cards are missing from the board (flop, turn, river)
                let cards_to_draw = 5 - board.len();
                let mut final_board = board.to_vec();
                final_board.extend(deck_copy.cards.iter().take(cards_to_draw));

                // Build 7-card hands for both players (hole cards + final board)
                let player_eval_hand_vec: Vec<usize> = player_hand
                    .iter()
                    .chain(final_board.iter())
                    .map(card_to_poker_eval_id)
                    .collect();
                let opponent_eval_hand_vec: Vec<usize> = opponent_hand
                    .iter()
                    .chain(final_board.iter())
                    .map(card_to_poker_eval_id)
                    .collect();

                // Convert Vec into arrays of exactly 7 cards
                let player_eval_hand: [usize; 7] = player_eval_hand_vec.try_into().expect("Hand should have 7 cards");
                let opponent_eval_hand: [usize; 7] = opponent_eval_hand_vec.try_into().expect("Hand should have 7 cards");

                // Evaluate hand ranks
                let player_rank = seven_eval::get_rank(tables, player_eval_hand);
                let opponent_rank = seven_eval::get_rank(tables, opponent_eval_hand);

                // Return this simulation's result
                if player_rank > opponent_rank {
                    (1, 0) // win
                } else if player_rank == opponent_rank {
                    (0, 1) // tie
                } else {
                    (0, 0) // loss
                }
            },
        )
        // Reduce all simulation results into totals
        .reduce(
            || (0, 0), // initial accumulator (0 wins, 0 ties)
            |(total_wins, total_ties), (win, tie)| {
                (total_wins + win, total_ties + tie) // add results from each sim
            }
        );

    // Final result: aggregated wins, ties, and total number of simulations
    Equity {
        wins: results.0,
        ties: results.1,
        total_sims: num_simulations,
    }
}