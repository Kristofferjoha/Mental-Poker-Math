use crate::tools::equity_calculator::{calculate_equity, to_rs_card};
use crate::starting_hands::STARTING_HANDS;
use crate::utils::parse_specific_hand;
use rayon::prelude::*;
use rs_poker::core::Hand;
use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

#[derive(Serialize)]
pub struct PreflopEquity<'a> {
    pub hand1: &'a str,
    pub hand2: &'a str,
    pub equity: f32,
}

/// Top-level function: generates equity data and writes to file.
pub fn preflop_equity_generation(number_of_simulations: u32) -> std::io::Result<()> {
    println!("Starting equity generation for {} matchups...", (STARTING_HANDS.len() * (STARTING_HANDS.len() -1) / 2));
    let start_time = Instant::now();

    let equities = generate_equities(number_of_simulations, &start_time);

    println!(
        "Generation complete in {:.2?}. Found {} valid matchups.",
        start_time.elapsed(),
        equities.len()
    );
    write_equities_to_file(&equities, "preflop_equity.json")?;

    println!(
        "Successfully saved {} matchups to preflop_equity.json",
        equities.len()
    );
    Ok(())
}

/// Generate all preflop equities by parallelizing the matchup creation.
fn generate_equities(num_sims: u32, start_time: &Instant) -> Vec<PreflopEquity<'static>> {
    let parsed_hands: Vec<_> = STARTING_HANDS
        .iter()
        .map(|s| {
            let hand_vec = parse_specific_hand(s).expect("Failed to parse hand from const array");
            let rs_cards: Vec<rs_poker::core::Card> = hand_vec.iter().map(to_rs_card).collect();
            let rs_hand = Hand::new_with_cards(rs_cards);
            (*s, hand_vec, rs_hand)
        })
        .collect();

    let processed_count = AtomicUsize::new(0);

    let all_equities: Vec<PreflopEquity<'static>> = parsed_hands
        .par_iter()
        .enumerate()
        .flat_map(|(i, (hand1_str, hand1_vec, hand1_rs))| {
            parsed_hands
                .par_iter()
                .skip(i + 1)
                .filter_map(|(hand2_str, hand2_vec, hand2_rs)| {
                    if hand1_vec[0] == hand2_vec[0]
                        || hand1_vec[0] == hand2_vec[1]
                        || hand1_vec[1] == hand2_vec[0]
                        || hand1_vec[1] == hand2_vec[1]
                    {
                        return None;
                    }

                    let equity_result = calculate_equity(hand1_rs, hand2_rs, num_sims);

                    let current_count = processed_count.fetch_add(1, Ordering::Relaxed);
                    if current_count > 0 && current_count % 3000 == 0 {
                        print_progress(current_count, start_time);
                    }

                    Some(PreflopEquity {
                        hand1: hand1_str,
                        hand2: hand2_str,
                        equity: equity_result * 100.0,
                    })
                })
        })
        .collect();

    println!(
        "Processed a total of {} valid matchups.",
        processed_count.load(Ordering::Relaxed)
    );
    all_equities
}

/// Write results to JSON file using a BufWriter for better performance.
fn write_equities_to_file(equities: &[PreflopEquity], path: &str) -> std::io::Result<()> {
    println!("Writing results to {}...", path);
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, equities)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

/// Print progress every N matchups.
fn print_progress(processed: usize, start: &Instant) {
    println!(
        "Processed {} valid matchups so far... elapsed: {:.2?}",
        processed,
        start.elapsed()
    );
}