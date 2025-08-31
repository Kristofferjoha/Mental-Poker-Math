use serde::{Serialize, Deserialize};
use strum_macros::EnumIter;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)] 
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

// repr helps with u8 conversion
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
#[repr(u8)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

// Converting rank to u8, used for hand evals
impl Rank {
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}

impl Default for Suit {
    fn default() -> Self {
        Suit::Spades
    }
}

impl Default for Rank {
    fn default() -> Self {
        Rank::Ace
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank_str = match self.rank {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        let suit_char = match self.suit {
            Suit::Hearts => 'h',
            Suit::Diamonds => 'd',
            Suit::Clubs => 'c',
            Suit::Spades => 's',
        };
        write!(f, "{}{}", rank_str, suit_char)
    }
}