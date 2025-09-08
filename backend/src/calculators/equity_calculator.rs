use poker_eval::eval::seven as seven_eval;
use poker_eval::eval::seven::TableSeven;
use rand::seq::SliceRandom;
use rayon::prelude::*;
use std::sync::Arc;

use crate::poker_core::{card::{Card}, deck::Deck,};
use crate::calculators::calculator_helpers::{Equity, card_to_poker_eval_id};


pub fn calculate_equity(
    player_hand: &[Card],
    opponent_hand: &[Card],
    board: &[Card],
    num_simulations: u32,
    tables: &Arc<TableSeven>,
) -> Equity {
    if board.len() == 5 {
        let player_cards_vec: Vec<usize> = player_hand.iter().chain(board.iter()).map(card_to_poker_eval_id).collect();
        let opponent_cards_vec: Vec<usize> = opponent_hand.iter().chain(board.iter()).map(card_to_poker_eval_id).collect();

        let player_cards: [usize; 7] = player_cards_vec.try_into().expect("Hand should have 7 cards");
        let opponent_cards: [usize; 7] = opponent_cards_vec.try_into().expect("Hand should have 7 cards");

        let player_rank = seven_eval::get_rank(tables, player_cards);
        let opponent_rank = seven_eval::get_rank(tables, opponent_cards);

        let (wins, ties) = if player_rank > opponent_rank {
            (1, 0)
        } else if player_rank < opponent_rank {
            (0, 0)
        } else {
            (0, 1)
        };
        return Equity { wins, ties, total_sims: 1 };
    }

    let remaining_deck = {
        let mut d = Deck::new();
        d.remove_cards(player_hand);
        d.remove_cards(opponent_hand);
        d.remove_cards(board);
        d
    };

    let results = (0..num_simulations)
        .into_par_iter()
        .map_init(
            || rand::rng(),
            |rng, _| {
                let mut deck_copy = remaining_deck.clone();
                deck_copy.cards.shuffle(rng);

                let cards_to_draw = 5 - board.len();
                let mut final_board = board.to_vec();
                final_board.extend(deck_copy.cards.iter().take(cards_to_draw));
                
                let player_eval_hand_vec: Vec<usize> = player_hand.iter().chain(final_board.iter()).map(card_to_poker_eval_id).collect();
                let opponent_eval_hand_vec: Vec<usize> = opponent_hand.iter().chain(final_board.iter()).map(card_to_poker_eval_id).collect();

                let player_eval_hand: [usize; 7] = player_eval_hand_vec.try_into().expect("Hand should have 7 cards");
                let opponent_eval_hand: [usize; 7] = opponent_eval_hand_vec.try_into().expect("Hand should have 7 cards");

                let player_rank = seven_eval::get_rank(tables, player_eval_hand);
                let opponent_rank = seven_eval::get_rank(tables, opponent_eval_hand);

                if player_rank > opponent_rank {
                    (1, 0)
                } else if player_rank == opponent_rank {
                    (0, 1)
                } else {
                    (0, 0)
                }
            },
        )
        .reduce(|| (0, 0), |(total_wins, total_ties), (win, tie)| (total_wins + win, total_ties + tie));

    Equity {
        wins: results.0,
        ties: results.1,
        total_sims: num_simulations,
    }
}