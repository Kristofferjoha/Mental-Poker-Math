use std::str::FromStr;

use crate::poker_core::{card::Card, deck::Deck};

#[derive(Clone, Debug, PartialEq)]
pub enum Street {
    PreFlop,
    Flop,
    Turn,
    River
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnknownStreet;

impl FromStr for Street {
    type Err = UnknownStreet;

    fn from_str(s: &str) -> Result<Street, UnknownStreet> {
        match s {
            "pre-flop" => Ok(Street::PreFlop),
            "flop" => Ok(Street::Flop),
            "turn" => Ok(Street::Turn),
            "river" => Ok(Street::River),
            _ => Err(UnknownStreet)
        }
    }
}

pub fn draw_board(deck: &mut Deck, stage: &Street) -> Vec<Card> {
    let num_cards = match stage {
        Street::PreFlop => 0,
        Street::Flop => 3,
        Street::Turn => 4,
        Street::River => 5
    };
    (0..num_cards).map(|_| deck.cards.pop().unwrap()).collect()
}
