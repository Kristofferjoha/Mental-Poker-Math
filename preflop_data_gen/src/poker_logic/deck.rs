use crate::poker_logic::card::{Card, Rank, Suit};

use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card { suit: suit, rank: rank });
            }
        }
        Deck { cards }
    }

}