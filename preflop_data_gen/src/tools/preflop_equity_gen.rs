use std::fs::File;
use std::io::Write;
use std::time::Instant;

use crate::starting_hands::STARTING_HANDS;
use crate::poker_logic::deck::Deck;
use crate::tools::equity_calculator::calculate_equity;
use crate::utils::{hands_from_strings};
use crate::PreflopEquity;
use crate::NUM_OF_SIMULATIONS;

pub fn run_program() {
    println!("Starting equity generation using rs-poker.");
    let full_deck = Deck::new();
    let start_time = Instant::now();
    let mut processed = 0;
    let total_matchups = (STARTING_HANDS.len() * (STARTING_HANDS.len() + 1)) / 2;

    let mut all_equities: Vec<PreflopEquity> = Vec::new();

    for (i, hand1_str) in STARTING_HANDS.iter().enumerate() {
        for hand2_str in STARTING_HANDS.iter().skip(i) {
            let (hand1, hand2) = hands_from_strings(hand1_str, hand2_str, &full_deck.cards);

            let equity_result = calculate_equity(
                &hand1,
                &hand2,
                &[], 
                NUM_OF_SIMULATIONS,
            );

            let equity_entry = PreflopEquity {
                hand1: hand1_str.to_string(),
                hand2: hand2_str.to_string(),
                equity: equity_result.equity() * 100.0,
            };

            all_equities.push(equity_entry);

            processed += 1;
            if processed % 100 == 0 {
                let elapsed = start_time.elapsed();
                println!(
                    "Processed {} / {} matchups, elapsed: {:.2?}",
                    processed, total_matchups, elapsed
                );
            }
        }
    }

    println!("Generation complete in {:.2?}", start_time.elapsed());

    println!("Writing results to file...");
    let json_output =
        serde_json::to_string_pretty(&all_equities).expect("Failed to serialize to JSON");
    let mut file = File::create("preflop_equity.json").expect("Failed to create file");
    file.write_all(json_output.as_bytes())
        .expect("Failed to write to file");

    println!(
        "Successfully saved {} matchups to preflop_equity.json",
        all_equities.len()
    );
}