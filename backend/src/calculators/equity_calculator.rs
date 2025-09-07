use crate::poker_core::{
    card::{Card, Rank, Suit},
    deck::Deck,
};
use poker_eval::eval::seven as seven_eval;
use poker_eval::eval::seven::TableSeven;
use rand::seq::SliceRandom;
use rayon::prelude::*;
use std::sync::Arc;

pub struct Equity {
    pub wins: u32,
    pub ties: u32,
    pub total_sims: u32,
}

impl Equity {
    pub fn equity(&self) -> f32 {
        if self.total_sims == 0 {
            0.0
        } else {
            (self.wins as f32 + self.ties as f32 * 0.5) / self.total_sims as f32
        }
    }
}

fn card_to_poker_eval_id(card: &Card) -> usize {
    let rank_index = match card.rank {
        Rank::Two => 0, Rank::Three => 1, Rank::Four => 2, Rank::Five => 3,
        Rank::Six => 4, Rank::Seven => 5, Rank::Eight => 6, Rank::Nine => 7,
        Rank::Ten => 8, Rank::Jack => 9, Rank::Queen => 10, Rank::King => 11,
        Rank::Ace => 12,
    };
    let suit_index = match card.suit {
        Suit::Clubs => 0, Suit::Diamonds => 1, Suit::Hearts => 2, Suit::Spades => 3,
    };
    rank_index * 4 + suit_index
}

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