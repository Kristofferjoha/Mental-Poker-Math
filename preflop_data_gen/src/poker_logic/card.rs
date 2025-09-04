use serde::{Serialize, Deserialize};
use strum_macros::EnumIter;

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