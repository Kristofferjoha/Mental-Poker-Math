use crate::poker_core::card::{Card, Rank, Suit};

pub fn card_to_poker_eval_id(card: &Card) -> usize {
    let rank_index = match card.rank {
        Rank::Two => 0, Rank::Three => 1, Rank::Four => 2, Rank::Five => 3,
        Rank::Six => 4, Rank::Seven => 5, Rank::Eight => 6, Rank::Nine => 7,
        Rank::Ten => 8, Rank::Jack => 9, Rank::Queen => 10, Rank::King => 11,
        Rank::Ace => 12
    };
    let suit_index = match card.suit {
        Suit::Clubs => 0, Suit::Diamonds => 1, Suit::Hearts => 2, Suit::Spades => 3
    };
    rank_index * 4 + suit_index
}
pub fn card_from_poker_eval_id(id: usize) -> Card {
    debug_assert!(id < 52, "card id {id} is out of range");
    let rank = match id / 4 {
        0 => Rank::Two, 1 => Rank::Three, 2 => Rank::Four, 3 => Rank::Five,
        4 => Rank::Six, 5 => Rank::Seven, 6 => Rank::Eight, 7 => Rank::Nine,
        8 => Rank::Ten, 9 => Rank::Jack, 10 => Rank::Queen, 11 => Rank::King,
        _ => Rank::Ace
    };
    let suit = match id % 4 {
        0 => Suit::Clubs,
        1 => Suit::Diamonds,
        2 => Suit::Hearts,
        _ => Suit::Spades
    };
    Card { rank, suit }
}
