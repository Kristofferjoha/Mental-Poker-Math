use std::fs::File;
use std::io::Write;
use std::time::Instant;
use serde::Serialize;
use rs_poker::core::Hand;

use crate::starting_hands::STARTING_HANDS;
use crate::tools::equity_calculator::{calculate_equity, to_rs_card};
use crate::utils::parse_specific_hand;

#[derive(Serialize)]
pub struct PreflopEquity<'a> {
    pub hand1: &'a str,
    pub hand2: &'a str,
    pub equity: f32,
}

/// Top-level function: generates equity data and writes to file.
pub fn preflop_equity_generation(number_of_simulations: u32) -> std::io::Result<()> {
    println!("Starting equity generation...");
    let start_time = Instant::now();

    let equities = generate_equities(number_of_simulations, &start_time);

    println!("Generation complete in {:.2?}", start_time.elapsed());
    write_equities_to_file(&equities, "preflop_equity.json")?;

    println!(
        "Successfully saved {} matchups to preflop_equity.json",
        equities.len()
    );
    Ok(())
}

/// Generate all preflop equities.
fn generate_equities(num_sims: u32, start_time: &Instant) -> Vec<PreflopEquity<'static>> {
    let mut parsed_hands = Vec::with_capacity(STARTING_HANDS.len());
    parsed_hands.extend(STARTING_HANDS.iter().map(|s| {
        let hand_vec = parse_specific_hand(s).expect("Failed to parse hand from const array");
        let rs_cards: Vec<rs_poker::core::Card> = hand_vec.iter().map(to_rs_card).collect();
        let rs_hand = Hand::new_with_cards(rs_cards);
        (*s, hand_vec, rs_hand)
    }));

    const EXPECTED_MATCHUPS: usize = 812_175;
    let mut all_equities = Vec::with_capacity(EXPECTED_MATCHUPS);
    
    let mut processed_valid_matchups = 0;

    for (i, (hand1_str, hand1_vec, hand1_rs)) in parsed_hands.iter().enumerate() {
        for (hand2_str, hand2_vec, hand2_rs) in parsed_hands.iter().skip(i + 1) {
            // Direct comparison for two-card hands
            if hand1_vec[0] == hand2_vec[0] || hand1_vec[0] == hand2_vec[1] ||
            hand1_vec[1] == hand2_vec[0] || hand1_vec[1] == hand2_vec[1] {
                continue;
            }
            let equity_result = calculate_equity(hand1_rs, hand2_rs, num_sims);
            all_equities.push(PreflopEquity {
                hand1: hand1_str,
                hand2: hand2_str,
                equity: equity_result * 100.0,
            });
            processed_valid_matchups += 1;
            if processed_valid_matchups % 1000 == 0 {
                print_progress(processed_valid_matchups, start_time);
            }
        }
    }
    all_equities
}


/// Write results to JSON file.
fn write_equities_to_file(equities: &[PreflopEquity], path: &str) -> std::io::Result<()> {
    println!("Writing results to {}...", path);
    let json_output = serde_json::to_string_pretty(equities)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let mut file = File::create(path)?;
    file.write_all(json_output.as_bytes())?;
    Ok(())
}

/// Print progress every N matchups.
fn print_progress(processed: usize, start: &Instant) {
    println!(
        "Processed {} valid matchups so far... elapsed: {:.2?}",
        processed,
        start.elapsed()
    );
}
