use crate::poker_logic::{
    card::Card,
    deck::Deck,
    hand_evaluator::evaluate_hand,
};
use rand::thread_rng;
use rand::seq::SliceRandom;
use rayon::prelude::*;
use std::cmp::Ordering;

pub struct Equity {
    pub wins: u32,
    pub ties: u32,
    pub total_sims: u32,
}

impl Equity {
    pub fn equity(&self) -> f32 {
        (self.wins as f32 + self.ties as f32 / 2.0) / self.total_sims as f32
    }
}

pub fn calculate_equity(
    player_hand: &[Card],
    opponent_hand: &[Card],
    board: &[Card],
    num_simulations: u32,
) -> Equity {
    let base_deck = {
    let mut d = Deck::new();
    d.remove_cards(player_hand);
    d.remove_cards(opponent_hand);
    d.remove_cards(board);
    d
};

let results = (0..num_simulations)
    .into_par_iter()
    .map_init(
        || (thread_rng(), base_deck.clone()),
        |(rng, deck), _| {
            let mut local_deck = deck.clone();
            local_deck.cards.shuffle(rng);

            let cards_to_deal = 5 - board.len();
            let simulated_board: Vec<_> = board.iter()
                .cloned()
                .chain(local_deck.cards.iter().take(cards_to_deal).cloned())
                .collect();

            let mut player_hand_buf = [Card::default(); 7];
            player_hand_buf[..2].copy_from_slice(player_hand);
            player_hand_buf[2..].copy_from_slice(&simulated_board);

            let mut opponent_hand_buf = [Card::default(); 7];
            opponent_hand_buf[..2].copy_from_slice(opponent_hand);
            opponent_hand_buf[2..].copy_from_slice(&simulated_board);

            let player_rank = evaluate_hand(&player_hand_buf);
            let opponent_rank = evaluate_hand(&opponent_hand_buf);

            match player_rank.cmp(&opponent_rank) {
                Ordering::Greater => (1, 0),
                Ordering::Equal => (0, 1),
                Ordering::Less => (0, 0),
            }
        }
    )
    .reduce(|| (0, 0), |(w1, t1), (w2, t2)| (w1 + w2, t1 + t2));


    Equity {
        wins: results.0,
        ties: results.1,
        total_sims: num_simulations,
    }
}
