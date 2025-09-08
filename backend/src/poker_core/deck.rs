use rand::seq::SliceRandom;
use rand::Rng;
use strum::IntoEnumIterator;

use crate::poker_core::card::{Card, Rank, Suit};

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

    // shuffle the deck using rng
    pub fn shuffle<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.cards.shuffle(rng);
    }

    // function that removes cards in argument from the deck
    pub fn remove_cards(&mut self, cards: &[Card]) {
        self.cards.retain(|c| !cards.contains(c));
    }

}