use serde::{Serialize, Deserialize};
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
#[derive(Default)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    #[default]
    Spades
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
#[derive(Default)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    #[default]
    Ace
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}

pub fn parse_cards(s: &str) -> Result<Vec<Card>, &'static str> {
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() || !chars.len().is_multiple_of(2) {
        return Err("Card string length must be a non-zero multiple of 2.");
    }

    let mut cards = Vec::with_capacity(chars.len() / 2);
    for pair in chars.chunks_exact(2) {
        let card = Card {
            rank: char_to_rank(pair[0])?,
            suit: char_to_suit(pair[1])?
        };
        if cards.contains(&card) {
            return Err("Duplicate card.");
        }
        cards.push(card);
    }
    Ok(cards)
}

pub fn parse_hand(hand_str: &str) -> Result<Vec<Card>, &'static str> {
    let cards = parse_cards(hand_str)?;
    if cards.len() != 2 {
        return Err("A hand is exactly two cards.");
    }
    Ok(cards)
}

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
        _ => Err("Invalid rank character")
    }
}

fn char_to_suit(c: char) -> Result<Suit, &'static str> {
    match c {
        's' => Ok(Suit::Spades),
        'h' => Ok(Suit::Hearts),
        'd' => Ok(Suit::Diamonds),
        'c' => Ok(Suit::Clubs),
        _ => Err("Invalid suit character")
    }
}
