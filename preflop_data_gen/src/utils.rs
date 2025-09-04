use std::collections::HashSet;
use crate::poker_logic::card::{Card, Rank, Suit};

pub fn hands_from_strings(
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

fn hand_string_to_cards(
    hand_str: &str,
    full_deck: &[Card],
    used: &mut HashSet<Card>,
) -> Vec<Card> {
    let chars: Vec<char> = hand_str.chars().collect();
    let r1 = char_to_rank(chars[0]);
    let r2 = char_to_rank(chars[1]);

    let is_pair = r1 == r2;
    let is_suited = chars.len() == 3 && chars[2] == 's';

    if is_pair {
        // Find two cards of the given rank that are not already in the 'used' set.
        let cards: Vec<_> = full_deck
            .iter()
            .filter(|c| c.rank == r1 && !used.contains(c))
            .take(2)
            .cloned()
            .collect();

        if cards.len() == 2 {
            used.insert(cards[0]);
            used.insert(cards[1]);
            return cards;
        } else {
            // This would happen if, for example, 3 cards of a rank are already used.
            panic!("Could not find an available pair for {}", hand_str);
        }
    }

    if is_suited {
        // Iterate through all four suits to find one where both cards are available.
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            let c1_opt = full_deck.iter().find(|c| c.rank == r1 && c.suit == suit);
            let c2_opt = full_deck.iter().find(|c| c.rank == r2 && c.suit == suit);

            if let (Some(c1), Some(c2)) = (c1_opt, c2_opt) {
                // Check if this specific suited combination is available.
                if !used.contains(c1) && !used.contains(c2) {
                    used.insert(*c1);
                    used.insert(*c2);
                    return vec![*c1, *c2];
                }
            }
        }
        panic!("No available suited combination for {}", hand_str);
    } else { // Offsuit
        // This is more complex; we must find any two cards of the correct ranks
        // with different suits that are both available.
        for c1 in full_deck.iter().filter(|c| c.rank == r1) {
            // Only proceed if the first card is available
            if used.contains(c1) {
                continue;
            }

            for c2 in full_deck.iter().filter(|c| c.rank == r2 && c.suit != c1.suit) {
                // Check if the second card is also available
                if !used.contains(c2) {
                    used.insert(*c1);
                    used.insert(*c2);
                    return vec![*c1, *c2];
                }
            }
        }
        panic!("No available offsuit combination for {}", hand_str);
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