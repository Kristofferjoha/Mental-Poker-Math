use rand::{prelude::*, Rng};
use tracing::{error, info};

use crate::calculators::equity_calculator;
use crate::poker_core::{card::Card, deck::Deck};
use crate::preflop_data::helpers::parse_specific_hand;
use crate::preflop_data::preflop_lookup::PreflopEquity;

pub fn generate_preflop_scenario(
    preflop_data: &[PreflopEquity],
    deck: &mut Deck,
    rng: &mut impl Rng,
) -> (Vec<Card>, Vec<Card>, Vec<Card>, f32) {
    let matchup = preflop_data.choose(rng).expect("Preflop equity data is empty");
    info!("Selected matchup: {} vs {}, equity: {}", matchup.hand1, matchup.hand2, matchup.equity);

    let hands: Option<(Vec<Card>, Vec<Card>)> = match (
        parse_specific_hand(&matchup.hand1),
        parse_specific_hand(&matchup.hand2),
    ) {
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

    let (player_hand, opponent_hand, equity) = if let Some((hand1, hand2)) = hands {
        let player_is_hand1 = rng.random_bool(0.5);

        if player_is_hand1 {
            let eq = matchup.equity / 100.0;
            info!("Parsed hands successfully: {:?} vs {:?}, equity: {}", &hand1, &hand2, eq);
            (hand1, hand2, eq)
        } else {
            let eq = (100.0 - matchup.equity) / 100.0;
            info!("Parsed hands successfully: {:?} vs {:?}, equity: {}", &hand2, &hand1, eq);
            (hand2, hand1, eq)
        }
    } else {
        error!("Generating random hands as a fallback.");
        deck.shuffle(rng);
        let player_h = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
        let opponent_h = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];

        let equity_result =
            equity_calculator::calculate_equity(&player_h, &opponent_h, &[], 25_000);
        (player_h, opponent_h, equity_result.equity())
    };

    (player_hand, opponent_hand, vec![], equity)
}

