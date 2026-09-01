use crate::poker_core::card::{Card, Rank, Suit};

// Converts a `Card` struct to the corresponding PokerEval ID (0-51).
pub fn card_to_poker_eval_id(card: &Card) -> usize {
    let rank_index = match card.rank {
        Rank::Two => 0, Rank::Three => 1, Rank::Four => 2, Rank::Five => 3,
        Rank::Six => 4, Rank::Seven => 5, Rank::Eight => 6, Rank::Nine => 7,
        Rank::Ten => 8, Rank::Jack => 9, Rank::Queen => 10, Rank::King => 11,
        Rank::Ace => 12,
    };
    let suit_index = match card.suit {
        Suit::Clubs => 0, Suit::Diamonds => 1, Suit::Hearts => 2, Suit::Spades => 3,
    };
    rank_index * 4 + suit_index
}