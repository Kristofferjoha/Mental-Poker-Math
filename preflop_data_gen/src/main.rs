use serde::Serialize;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

mod poker_logic;
use poker_logic::{
    card::{Card, Rank, Suit},
    deck::Deck,
    equity_calculator::calculate_equity,
};

#[derive(Serialize)]
struct PreflopEquity {
    hand1: String,
    hand2: String,
    equity: f32,
}

const STARTING_HANDS: [&str; 169] = [
    "AA", "KK", "QQ", "JJ", "TT", "99", "88", "77", "66", "55", "44", "33", "22", "AKs", "AQs",
    "AJs", "ATs", "A9s", "A8s", "A7s", "A6s", "A5s", "A4s", "A3s", "A2s", "KQs", "KJs", "KTs",
    "K9s", "K8s", "K7s", "K6s", "K5s", "K4s", "K3s", "K2s", "QJs", "QTs", "Q9s", "Q8s", "Q7s",
    "Q6s", "Q5s", "Q4s", "Q3s", "Q2s", "JTs", "J9s", "J8s", "J7s", "J6s", "J5s", "J4s", "J3s",
    "J2s", "T9s", "T8s", "T7s", "T6s", "T5s", "T4s", "T3s", "T2s", "98s", "97s", "96s", "95s",
    "94s", "93s", "92s", "87s", "86s", "85s", "84s", "83s", "82s", "76s", "75s", "74s", "73s",
    "72s", "65s", "64s", "63s", "62s", "54s", "53s", "52s", "43s", "42s", "32s", "AKo", "AQo",
    "AJo", "ATo", "A9o", "A8o", "A7o", "A6o", "A5o", "A4o", "A3o", "A2o", "KQo", "KJo", "KTo",
    "K9o", "K8o", "K7o", "K6o", "K5o", "K4o", "K3o", "K2o", "QJo", "QTo", "Q9o", "Q8o", "Q7o",
    "Q6o", "Q5o", "Q4o", "Q3o", "Q2o", "JTo", "J9o", "J8o", "J7o", "J6o", "J5o", "J4o", "J3o",
    "J2o", "T9o", "T8o", "T7o", "T6o", "T5o", "T4o", "T3o", "T2o", "98o", "97o", "96o", "95o",
    "94o", "93o", "92o", "87s", "86s", "85s", "84s", "83s", "82s", "76o", "75o", "74o", "73o",
    "72o", "65o", "64o", "63o", "62o", "54o", "53o", "52o", "43o", "42o", "32o",
];

fn main() {
    println!("Starting equity generation using rs-poker.");
    const NUM_SIMULATIONS: u32 = 500000;

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
                NUM_SIMULATIONS,
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

fn hands_from_strings(
    hand1_str: &str,
    hand2_str: &str,
    deck_cards: &[Card],
) -> (Vec<Card>, Vec<Card>) {
    let mut used_cards = HashSet::new();

    let hand1 = hand_string_to_cards(hand1_str, deck_cards, &mut used_cards);

    let available_deck: Vec<Card> = deck_cards
        .iter()
        .cloned()
        .filter(|c| !used_cards.contains(c))
        .collect();

    let hand2 = hand_string_to_cards(hand2_str, &available_deck, &mut used_cards);

    (hand1, hand2)
}

fn hand_string_to_cards(hand_str: &str, deck: &[Card], used: &mut HashSet<Card>) -> Vec<Card> {
    let chars: Vec<char> = hand_str.chars().collect();
    let r1 = char_to_rank(chars[0]);
    let r2 = char_to_rank(chars[1]);

    let is_pair = r1 == r2;
    let is_suited = chars.len() == 3 && chars[2] == 's';

    if is_pair {
        let cards: Vec<_> = deck
            .iter()
            .filter(|c| c.rank == r1)
            .take(2)
            .cloned()
            .collect();
        for c in &cards {
            used.insert(*c);
        }
        return cards;
    }

    if is_suited {
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            let c1 = deck.iter().find(|c| c.rank == r1 && c.suit == suit);
            let c2 = deck.iter().find(|c| c.rank == r2 && c.suit == suit);
            if let (Some(c1), Some(c2)) = (c1, c2) {
                used.insert(*c1);
                used.insert(*c2);
                return vec![*c1, *c2];
            }
        }
        panic!("No suited combination available for {}", hand_str);
    } else { 
        if let Some(c1) = deck.iter().find(|c| c.rank == r1) {
            if let Some(c2) = deck.iter().find(|c| c.rank == r2 && c.suit != c1.suit) {
                used.insert(*c1);
                used.insert(*c2);
                return vec![*c1, *c2];
            }
        }
        panic!("No offsuit combination available for {}", hand_str);
    }
}

fn char_to_rank(c: char) -> Rank {
    match c {
        'A' => Rank::Ace, 'K' => Rank::King, 'Q' => Rank::Queen, 'J' => Rank::Jack,
        'T' => Rank::Ten, '9' => Rank::Nine, '8' => Rank::Eight, '7' => Rank::Seven,
        '6' => Rank::Six, '5' => Rank::Five, '4' => Rank::Four, '3' => Rank::Three,
        '2' => Rank::Two, _ => panic!("Invalid rank character"),
    }
}