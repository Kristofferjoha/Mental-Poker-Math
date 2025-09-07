use std::fs::File;
use std::io::Write;
use std::time::Instant;
use serde::Serialize;

use crate::starting_hands::STARTING_HANDS;
use crate::tools::equity_calculator::calculate_equity;
use crate::utils::parse_specific_hand;

/// Holds the equity result of a preflop hand matchup.

#[derive(Serialize)]
pub struct PreflopEquity {
    pub hand1: String,
    pub hand2: String,
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
fn generate_equities(num_sims: u32, start_time: &Instant) -> Vec<PreflopEquity> {
    let mut all_equities = Vec::new();
    let mut processed_valid_matchups = 0;
    let total_pairs_to_check = (STARTING_HANDS.len() * (STARTING_HANDS.len() - 1)) / 2;

    println!("Total pairs to check for conflicts: {}", total_pairs_to_check);

    for (i, hand1_str_ref) in STARTING_HANDS.iter().enumerate() {
        // Dereference the reference from the iterator
        let hand1_str = *hand1_str_ref;
        let hand1 = parse_specific_hand(hand1_str).expect("Failed to parse hand1 from const array"); 

        // Iterate through all subsequent hands to form pairs
        for hand2_str_ref in STARTING_HANDS.iter().skip(i + 1) {
            let hand2_str = *hand2_str_ref;
            let hand2 = parse_specific_hand(hand2_str).expect("Failed to parse hand2 from const array"); // string into vector of "Card"'s

            // If any card in hand1 is also in hand2, the matchup is invalid.
            if hand1.iter().any(|c1| hand2.contains(c1)) {
                continue; // Skip this pair
            }

            let equity_result = calculate_equity(&hand1, &hand2, &[], num_sims);
            all_equities.push(PreflopEquity {
                hand1: hand1_str.to_string(),
                hand2: hand2_str.to_string(),
                equity: equity_result * 100.0,
            });

            processed_valid_matchups += 1;
            if processed_valid_matchups % 10000 == 0 {
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