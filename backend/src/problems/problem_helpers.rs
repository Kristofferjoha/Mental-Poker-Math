use crate::poker_core::{card::Card, deck::Deck};

#[derive(Clone, Debug, PartialEq)]
pub enum Street {
    PreFlop,
    Flop,
    Turn,
    River,
}

// Converts a string to a Street enum variant.
impl Street {
    pub fn from_str(s: &str) -> Option<Street> {
        match s {
            "pre-flop" => Some(Street::PreFlop),
            "flop" => Some(Street::Flop),
            "turn" => Some(Street::Turn),
            "river" => Some(Street::River),
            _ => None,
        }
    }
}

// Draws the appropriate number of board cards based on the game stage.
pub fn draw_board(deck: &mut Deck, stage: &Street) -> Vec<Card> {
    let num_cards = match stage {
        Street::PreFlop => 0,
        Street::Flop => 3,
        Street::Turn => 4,
        Street::River => 5,
    };
    (0..num_cards).map(|_| deck.cards.pop().unwrap()).collect()
}
