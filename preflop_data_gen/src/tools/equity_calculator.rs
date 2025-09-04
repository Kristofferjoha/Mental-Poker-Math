use crate::poker_logic::card::{Card, Rank, Suit as MySuit};
use rs_poker::core::{Hand, Suit, Value};
use rs_poker::holdem::MonteCarloGame;
use rayon::prelude::*;

pub struct Equity {
    pub wins: u32,
    pub ties: u32,
    pub total_sims: u32,
}

impl Equity {
    pub fn equity(&self) -> f32 {
        (self.wins as f32 + self.ties as f32 / 2.0) / self.total_sims as f32
    }
}

pub fn calculate_equity(
    player_hand: &[Card],
    opponent_hand: &[Card],
    board: &[Card],
    num_simulations: u32,
) -> Equity {
    let hero_cards: Vec<rs_poker::core::Card> = player_hand.iter().map(to_rs_card).collect();
    let villain_cards: Vec<rs_poker::core::Card> = opponent_hand.iter().map(to_rs_card).collect();
    let _board_cards: Vec<rs_poker::core::Card> = board.iter().map(to_rs_card).collect();

    let hero_hand = Hand::new_with_cards(hero_cards);
    let villain_hand = Hand::new_with_cards(villain_cards);
    let hands = vec![hero_hand, villain_hand];

    let chunk_size = 50;
    let results = (0..num_simulations)
        .into_par_iter()
        .chunks(chunk_size)
        .map_init(
            || MonteCarloGame::new(hands.clone()).unwrap(),
            |game, chunk| {
                let mut wins = 0;
                let mut ties = 0;

                for _ in chunk {
                    let result_mask = game.simulate().0;
                    game.reset();

                    let mut hero_won = false;
                    let mut num_winners = 0;

                    for winner in result_mask.ones() {
                        num_winners += 1;
                        if winner == 0 {
                            hero_won = true;
                        }
                    }

                    if hero_won {
                        if num_winners > 1 {
                            ties += 1;
                        } else {
                            wins += 1;
                        }
                    }
                }

                (wins, ties)
            },
        )
        .reduce(|| (0, 0), |(w1, t1), (w2, t2)| (w1 + w2, t1 + t2));

    Equity {
        wins: results.0,
        ties: results.1,
        total_sims: num_simulations,
    }
}

impl From<Rank> for Value {
    fn from(rank: Rank) -> Self {
        match rank {
            Rank::Two => Value::Two,
            Rank::Three => Value::Three,
            Rank::Four => Value::Four,
            Rank::Five => Value::Five,
            Rank::Six => Value::Six,
            Rank::Seven => Value::Seven,
            Rank::Eight => Value::Eight,
            Rank::Nine => Value::Nine,
            Rank::Ten => Value::Ten,
            Rank::Jack => Value::Jack,
            Rank::Queen => Value::Queen,
            Rank::King => Value::King,
            Rank::Ace => Value::Ace,
        }
    }
}

impl From<MySuit> for Suit {
    fn from(suit: MySuit) -> Self {
        match suit {
            MySuit::Clubs => Suit::Club,
            MySuit::Diamonds => Suit::Diamond,
            MySuit::Hearts => Suit::Heart,
            MySuit::Spades => Suit::Spade,
        }
    }
}

fn to_rs_card(card: &Card) -> rs_poker::core::Card {
    rs_poker::core::Card::new(card.rank.into(), card.suit.into())
}
