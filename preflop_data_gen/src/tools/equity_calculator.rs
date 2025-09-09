use crate::poker_objects::card::{Card, Rank, Suit as MySuit};
use crate::CHUNK_SIZE;
use rs_poker::core::{Hand, Suit, Value};
use rs_poker::holdem::MonteCarloGame;
use rayon::prelude::*;

/// Estimates the equity of a given starting hand against an opponent's hand.
pub fn calculate_equity(
    player_hand: &Hand,
    opponent_hand: &Hand,
    num_simulations: u32,
) -> f32 {
    let hands = vec![player_hand.clone(), opponent_hand.clone()];

    // Run Monte Carlo trials in parallel.
    let results = (0..num_simulations)
        .into_par_iter()
        .chunks(CHUNK_SIZE)
        .map_init(
            // For each worker thread, create a fresh MonteCarloGame instance.
            || MonteCarloGame::new(hands.clone()).unwrap(),
            |game, chunk| {
                let mut wins = 0;
                let mut ties = 0;
                
                // Runs each simulation in the chunk.
                for _ in chunk {
                    let result_mask = game.simulate().0; // `simulate` returns a bitmask of winners
                    game.reset();

                    let num_winners = result_mask.count();
                    
                    if result_mask.get(0) {
                        if num_winners > 1 {
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

    // Equity = (wins + 0.5 * ties) / total simulations
    (wins as f32 + ties as f32 / 2.0) / num_simulations as f32
}


/// Maps the internal `Rank` enum → `rs_poker::core::Value`.
impl From<Rank> for Value {
    fn from(rank: Rank) -> Self {
        match rank {
            Rank::Two => Value::Two,
            Rank::Three => Value::Three,
            Rank::Four => Value::Four,
            Rank::Five => Value::Five,
            Rank::Six => Value::Six,
            Rank::Seven => Value::Seven,
            Rank::Eight => Value::Eight,
            Rank::Nine => Value::Nine,
            Rank::Ten => Value::Ten,
            Rank::Jack => Value::Jack,
            Rank::Queen => Value::Queen,
            Rank::King => Value::King,
            Rank::Ace => Value::Ace,
        }
    }
}

/// Maps the internal `Suit` enum → `rs_poker::core::Suit`.
impl From<MySuit> for Suit {
    fn from(suit: MySuit) -> Self {
        match suit {
            MySuit::Clubs => Suit::Club,
            MySuit::Diamonds => Suit::Diamond,
            MySuit::Hearts => Suit::Heart,
            MySuit::Spades => Suit::Spade,
        }
    }
}

/// Converts a custom `Card` (your type) → `rs_poker::core::Card`.
pub fn to_rs_card(card: &Card) -> rs_poker::core::Card {
    rs_poker::core::Card::new(card.rank.into(), card.suit.into())
}
