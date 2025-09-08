use crate::poker_core::card::{Card, Rank, Suit};

/// Represents the results of equity calculations between two hands.
/// `wins`: number of simulations won by the player.
/// `ties`: number of simulations that resulted in a tie.
/// `total_sims`: total number of simulations run.
/// `equity()`: calculates the equity as (wins + ties * 0.5) / total_sims.

pub struct Equity {
    pub wins: u32,
    pub ties: u32,
    pub total_sims: u32,
}

impl Equity {
    pub fn equity(&self) -> f32 {
        if self.total_sims == 0 {
            0.0
        } else {
            (self.wins as f32 + self.ties as f32 * 0.5) / self.total_sims as f32
        }
    }
}

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