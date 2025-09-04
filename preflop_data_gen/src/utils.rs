use crate::poker_objects::card::{Card, Rank, Suit};

/// Utility functions for handling starting hands strings into [`Card`] structs.
/// "s" suffix means suited (`AKs` → Ace♣, King♣).
/// "o" suffix means offsuit (`AK` → Ace♣, King♦).
/// Pairs are represented without a suffix (`AA` → Ace♣, Ace♦).

/// Takes in two string hands and returns hand as [`Card`] structs.
/// Checks if some cards are the same.
/// Returns hands as vectors of cards
pub fn hands_from_strings(hand1_str: &str, hand2_str: &str) -> (Vec<Card>, Vec<Card>) {
    let hand1 = hand_string_to_cards(hand1_str, 0);
    let hand2 = hand_string_to_cards(hand2_str, 1);

    if hand1[0] == hand2[0] || hand1[0] == hand2[1] || hand1[1] == hand2[0] || hand1[1] == hand2[1] {
        let hand2_reassigned = hand_string_to_cards(hand2_str, 2);
        return (hand1, hand2_reassigned);
    }

    (hand1, hand2)
}

/// Gives string hands values of Suit and Rank.
/// Uses Clubs and Diamonds for suit first. If overlap between hands, gives Hearts and Spades to hand 2
fn hand_string_to_cards(hand_str: &str, suit_variant: u8) -> Vec<Card> {
    let chars: Vec<char> = hand_str.chars().collect();
    let r1 = char_to_rank(chars[0]);
    let r2 = char_to_rank(chars[1]);

    let is_pair = r1 == r2;
    let is_suited = chars.len() == 3 && chars[2] == 's';

    let (s1, s2) = if is_pair {
        // AA -> Ac Ad  or  Ah As
        match suit_variant {
            0 => (Suit::Clubs, Suit::Diamonds),
            _ => (Suit::Hearts, Suit::Spades),
        }
    } else if is_suited {
        // AKs -> Ac Kc or Ad Kd
        let suit = match suit_variant {
            0 => Suit::Clubs,
            _ => Suit::Diamonds,
        };
        (suit, suit)
    } else { // Offsuit
        // AKo -> Ac Kd or Ad Ks
        match suit_variant {
            0 => (Suit::Clubs, Suit::Diamonds),
            _ => (Suit::Hearts, Suit::Spades),
        }
    };
    
    vec![Card { rank: r1, suit: s1 }, Card { rank: r2, suit: s2 }]
}

/// Converting letter to rank
fn char_to_rank(c: char) -> Rank {
    match c {
        'A' => Rank::Ace, 'K' => Rank::King, 'Q' => Rank::Queen, 'J' => Rank::Jack,
        'T' => Rank::Ten, '9' => Rank::Nine, '8' => Rank::Eight, '7' => Rank::Seven,
        '6' => Rank::Six, '5' => Rank::Five, '4' => Rank::Four, '3' => Rank::Three,
        '2' => Rank::Two, _ => panic!("Invalid rank character"),
    }
}