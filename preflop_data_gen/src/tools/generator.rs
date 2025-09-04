use std::fs::File;
use std::io::Write;
use std::time::Instant;
use serde::Serialize;

use crate::starting_hands::STARTING_HANDS;
use crate::tools::equity_calculator::calculate_equity;
use crate::utils::hands_from_strings;

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
    let mut processed = 0;
    let total = (STARTING_HANDS.len() * (STARTING_HANDS.len() + 1)) / 2;

    for (i, hand1_str) in STARTING_HANDS.iter().enumerate() {
        for hand2_str in STARTING_HANDS.iter().skip(i) {
            let entry = process_matchup(hand1_str, hand2_str, num_sims);
            all_equities.push(entry);

            processed += 1;
            if processed % 100 == 0 {
                print_progress(processed, total, start_time);
            }
        }
    }
    all_equities
}

/// Process matchup.
fn process_matchup(hand1_str: &str, hand2_str: &str, num_sims: u32) -> PreflopEquity {
    let (hand1, hand2) = hands_from_strings(hand1_str, hand2_str);
    let equity_result = calculate_equity(&hand1, &hand2, &[], num_sims);

    PreflopEquity {
        hand1: hand1_str.to_string(),
        hand2: hand2_str.to_string(),
        equity: equity_result * 100.0,
    }
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
fn print_progress(processed: usize, total: usize, start: &Instant) {
    println!(
        "Processed {} / {} matchups, elapsed: {:.2?}",
        processed,
        total,
        start.elapsed()
    );
}
