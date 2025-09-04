use crate::poker_objects::card::{Card, Rank, Suit as MySuit};
use crate::CHUNK_SIZE;
use rs_poker::core::{Hand, Suit, Value};
use rs_poker::holdem::MonteCarloGame;
use rayon::prelude::*;

/// Estimates the equity of a given starting hand against an opponent's hand
/// The result represents the probability that the hero wins, including half the probability of ties.
/// This is done using rayon parallelization and rs_poker for hand lookups.
/// rs_poker has MonteCarloGame which runs monte carlo simulations and has hand lookup to quickly find winner


pub fn calculate_equity(
    player_hand: &[Card],
    opponent_hand: &[Card],
    board: &[Card],
    num_simulations: u32,
) -> f32 {
    // Convert to rs_poker types
    let hero_cards: Vec<rs_poker::core::Card> = player_hand.iter().map(to_rs_card).collect();
    let villain_cards: Vec<rs_poker::core::Card> = opponent_hand.iter().map(to_rs_card).collect();
    let _board_cards: Vec<rs_poker::core::Card> = board.iter().map(to_rs_card).collect();

    // Wrap cards into rs_poker Hand objects
    let hero_hand = Hand::new_with_cards(hero_cards);
    let villain_hand = Hand::new_with_cards(villain_cards);
    let hands = vec![hero_hand, villain_hand];

    // Run Monte Carlo trials in parallel.
    // `CHUNK_SIZE` controls how many simulations each worker does before combining results.
    let results = (0..num_simulations)
        .into_par_iter() // Parallel iterator over simulation trials
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

                    let mut hero_won = false;
                    let mut num_winners = 0;

                    // Check all winners for this trial.
                    for winner_index in result_mask.ones() {
                        num_winners += 1;
                        if winner_index == 0 {
                            hero_won = true;
                        }
                    }

                    if hero_won {
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
        // Reduce partial results from all threads into a single (wins, ties) tuple.
        .reduce(|| (0, 0), |(w1, t1), (w2, t2)| (w1 + w2, t1 + t2));

    let wins = results.0;
    let ties = results.1;

    // Equity = (wins + 0.5 * ties) / total simulations
    (wins as f32 + ties as f32 / 2.0) / num_simulations as f32
}

/// Maps your internal `Rank` enum → `rs_poker::core::Value`.
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

/// Maps your internal `Suit` enum → `rs_poker::core::Suit`.
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
fn to_rs_card(card: &Card) -> rs_poker::core::Card {
    rs_poker::core::Card::new(card.rank.into(), card.suit.into())
}
