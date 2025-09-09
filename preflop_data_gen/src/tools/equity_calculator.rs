// src/equity_calculator.rs

use crate::poker_objects::card::{Card, Rank, Suit as MySuit};
use rayon::prelude::*;
use rs_poker::core::{Hand, Suit, Value};
use rs_poker::holdem::MonteCarloGame;

const CHUNK_SIZE: usize = 1024;

/// Estimates the equity of a given starting hand against an opponent's hand.
pub fn calculate_equity(
    player_hand: &Hand,
    opponent_hand: &Hand,
    num_simulations: u32,
) -> f32 {
    let hands = vec![player_hand.clone(), opponent_hand.clone()];

    let results = (0..num_simulations)
        .into_par_iter()
        .chunks(CHUNK_SIZE.min(num_simulations as usize))
        .map_init(
            || MonteCarloGame::new(hands.clone()).expect("Failed to create MonteCarloGame"),
            |game, chunk| {
                let mut wins = 0;
                let mut ties = 0;
                for _ in chunk {
                    let result_mask = game.simulate().0;
                    game.reset();
                    if result_mask.get(0) { // Check if player 1 (index 0) is a winner
                        if result_mask.count() > 1 {
                            ties += 1;
                        } else {
                            wins += 1;
                        }
                    }
                }
                (wins, ties)
            },
        )
        .reduce(|| (0, 0), |(w1, t1), (w2, t2)| (w1 + w2, t1 + t2));

    let wins = results.0;
    let ties = results.1;

    (wins as f32 + ties as f32 / 2.0) / num_simulations as f32
}

impl From<Rank> for Value {
    fn from(rank: Rank) -> Self {
        match rank {
            Rank::Two => Value::Two, Rank::Three => Value::Three, Rank::Four => Value::Four,
            Rank::Five => Value::Five, Rank::Six => Value::Six, Rank::Seven => Value::Seven,
            Rank::Eight => Value::Eight, Rank::Nine => Value::Nine, Rank::Ten => Value::Ten,
            Rank::Jack => Value::Jack, Rank::Queen => Value::Queen, Rank::King => Value::King,
            Rank::Ace => Value::Ace,
        }
    }
}

impl From<MySuit> for Suit {
    fn from(suit: MySuit) -> Self {
        match suit {
            MySuit::Clubs => Suit::Club, MySuit::Diamonds => Suit::Diamond,
            MySuit::Hearts => Suit::Heart, MySuit::Spades => Suit::Spade,
        }
    }
}

pub fn to_rs_card(card: &Card) -> rs_poker::core::Card {
    rs_poker::core::Card::new(card.rank.into(), card.suit.into())
}