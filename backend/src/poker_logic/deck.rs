use crate::poker_logic::card::{Card, Rank, Suit};

use rand::seq::SliceRandom;
use rand::Rng;
use strum::IntoEnumIterator;

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

    pub fn shuffle<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.cards.shuffle(rng);
    }

    // function that removes cards in argument from the deck
    pub fn remove_cards(&mut self, cards: &[Card]) {
        self.cards.retain(|c| !cards.contains(c));
    }

}