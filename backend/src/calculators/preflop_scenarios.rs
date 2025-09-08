use rand::{prelude::*, Rng};
use tracing::{error, info};
use poker_eval::eval::seven::TableSeven;
use std::sync::Arc;

use crate::calculators::equity_calculator::calculate_equity;
use crate::poker_core::{card::Card, deck::Deck};
use crate::preflop_data::helpers::parse_specific_hand;
use crate::preflop_data::preflop_lookup::PreflopEquity;

/// Generates a pre-flop scenario based on the provided pre-flop equity data.
/// `preflop_data`: Slice of `PreflopEquity` structs containing hand
/// `deck`: Mutable reference to a `Deck` struct representing the current deck of cards.
/// `rng`: Mutable reference to a random number generator implementing the `Rng` trait.
/// `seven_card_tables`: Precomputed tables for seven-card hand evaluation from poker_eval crate.
/// Returns a tuple containing:
/// - Player's hand (Vec<Card>)
/// - Opponent's hand (Vec<Card>)
/// - Board cards (Vec<Card>) - empty for pre-flop
/// - Player's equity (f32) against the opponent's hand

pub fn generate_preflop_scenario(
    preflop_data: &[PreflopEquity],
    deck: &mut Deck,
    rng: &mut impl Rng,
    seven_card_tables: &Arc<TableSeven>,
) -> (Vec<Card>, Vec<Card>, Vec<Card>, f32) {
    // Randomly select a matchup from the preflop equity data
    let matchup = preflop_data.choose(rng).expect("Preflop equity data is empty");

    // Parse the specific hands from the matchup strings into Card structs
    let hands: Option<(Vec<Card>, Vec<Card>)> = match (
        parse_specific_hand(&matchup.hand1),
        parse_specific_hand(&matchup.hand2),
    ) {
        // If both hands parse successfully, remove them from the deck.
        (Ok(h1_arr), Ok(h2_arr)) => {
            let h1 = h1_arr.to_vec();
            let h2 = h2_arr.to_vec();

            deck.remove_cards(&h1);
            deck.remove_cards(&h2);
            Some((h1, h2))
        }
        (Err(e1), _) => {
            error!("Failed to parse hand1 {}: {}", matchup.hand1, e1);
            None
        }
        (_, Err(e2)) => {
            error!("Failed to parse hand2 {}: {}", matchup.hand2, e2);
            None
        }
    };

    // If both hands parsed successfully.
    let (player_hand, opponent_hand, equity) = if let Some((hand1, hand2)) = hands {
        // Randomly decide whether the player gets `hand1` or `hand2`.
        let player_is_hand1 = rng.random_bool(0.5);

        if player_is_hand1 {
            // Player gets hand1, opponent gets hand2.
            let eq = matchup.equity / 100.0; // Equity stored as percentage, converted to 0–1.
            info!(
                "Parsed hands successfully: {:?} vs {:?}, equity: {}",
                &hand1, &hand2, eq
            );
            (hand1, hand2, eq)
        } else {
            // Swap perspective: opponent is hand1, player is hand2.
            let eq = (100.0 - matchup.equity) / 100.0;
            info!(
                "Parsed hands successfully: {:?} vs {:?}, equity: {}",
                &hand2, &hand1, eq
            );
            (hand2, hand1, eq)
        }
    } else {
        // If parsing fails, log, and give random hands.
        error!("Generating random hands as a fallback.");
        deck.shuffle(rng);

        // Deal two cards each to player and opponent.
        let player_h = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
        let opponent_h = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];

        // Run Monte Carlo equity calculation the slow way with sims
        let equity_result = calculate_equity(&player_h, &opponent_h, &[], 25_000, seven_card_tables);
        (player_h, opponent_h, equity_result.equity())
    };

    // Board is empty since this is a preflop scenario.
    (player_hand, opponent_hand, vec![], equity)
}