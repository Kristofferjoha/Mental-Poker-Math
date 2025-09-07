use crate::poker_objects::card::{Card, Rank, Suit};

/// Parses a 4-character string representation of a specific two-card hand (e.g., "AsKd")
/// into a Vec of two `Card` structs.
///
/// Returns an error if the string format is invalid.
pub fn parse_specific_hand(hand_str: &str) -> Result<Vec<Card>, &'static str> {
    let chars: Vec<char> = hand_str.chars().collect();
    if chars.len() != 4 {
        return Err("Invalid hand string length. Expected 4 characters.");
    }

    // Parse all four characters into ranks and suits
    let r1 = char_to_rank(chars[0])?;
    let s1 = char_to_suit(chars[1])?;
    let r2 = char_to_rank(chars[2])?;
    let s2 = char_to_suit(chars[3])?;

    let card1 = Card { rank: r1, suit: s1 };
    let card2 = Card { rank: r2, suit: s2 };

    if card1 == card2 {
        return Err("Invalid hand string: cards cannot be identical.");
    }

    Ok(vec![card1, card2])
}

/// Converts a rank character ('A', 'K', 'T', '7', etc.) to a `Rank` enum.
fn char_to_rank(c: char) -> Result<Rank, &'static str> {
    match c {
        'A' => Ok(Rank::Ace),
        'K' => Ok(Rank::King),
        'Q' => Ok(Rank::Queen),
        'J' => Ok(Rank::Jack),
        'T' => Ok(Rank::Ten),
        '9' => Ok(Rank::Nine),
        '8' => Ok(Rank::Eight),
        '7' => Ok(Rank::Seven),
        '6' => Ok(Rank::Six),
        '5' => Ok(Rank::Five),
        '4' => Ok(Rank::Four),
        '3' => Ok(Rank::Three),
        '2' => Ok(Rank::Two),
        _ => Err("Invalid rank character"),
    }
}

/// Converts a suit character ('s', 'h', 'd', 'c') to a `Suit` enum.
fn char_to_suit(c: char) -> Result<Suit, &'static str> {
    match c {
        's' => Ok(Suit::Spades),
        'h' => Ok(Suit::Hearts),
        'd' => Ok(Suit::Diamonds),
        'c' => Ok(Suit::Clubs),
        _ => Err("Invalid suit character"),
    }
}