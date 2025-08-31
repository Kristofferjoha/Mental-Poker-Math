use crate::poker_logic::{
    card::{Card, Rank},
    deck::Deck,
    equity_calculator,
    preflop_lookup::PreflopEquity,
};
use rand::{rng, Rng};
use rand::prelude::IndexedRandom;

// Remove unused HashSet import
// use std::collections::HashSet;

// FIX: Add PartialEq to allow for direct comparison (==)
#[derive(Clone, Debug, PartialEq)]
pub enum Street {
    PreFlop,
    Flop,
    Turn,
    River,
}

impl Street {
    fn from_str(s: &str) -> Option<Street> {
        match s {
            "pre-flop" => Some(Street::PreFlop),
            "flop" => Some(Street::Flop),
            "turn" => Some(Street::Turn),
            "river" => Some(Street::River),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PotEquityProblem {
    pub player_hand: Vec<Card>,
    pub opponent_hand: Vec<Card>,
    pub board: Vec<Card>,
    pub pot_size: u32,
    pub bet_to_call: u32,
    pub player_equity: f32,
    pub pot_odds: f32,
    pub correct_decision: bool,
}

pub fn generate_pot_eq_problem(
    allowed_streets_str: Vec<String>,
    allow_overbets: bool,
    preflop_data: &[PreflopEquity],
) -> PotEquityProblem {
    let mut rng = rng();

    let mut allowed_streets: Vec<Street> = allowed_streets_str
        .iter()
        .filter_map(|s| Street::from_str(s))
        .collect();
    if allowed_streets.is_empty() {
        allowed_streets = vec![Street::PreFlop, Street::Flop, Street::Turn, Street::River];
    }
    let chosen_street = allowed_streets.choose(&mut rng).unwrap();

    if *chosen_street == Street::PreFlop {
        let matchup = preflop_data.choose(&mut rng).expect("Preflop equity data is empty");
        let player_is_hand1 = rng.random_bool(0.5);
        let (player_hand, opponent_hand) = if player_is_hand1 {
            hands_from_strings(&matchup.hand1, &matchup.hand2)
        } else {
            hands_from_strings(&matchup.hand2, &matchup.hand1)
        };

        let pot_size = (rng.random_range(10_000..100_000) / 1000) * 1000;
        let bet_to_call = generate_bet_size(pot_size, allow_overbets, &mut rng);
        let pot_odds = bet_to_call as f32 / (pot_size + bet_to_call) as f32;
        let player_equity = if player_is_hand1 {
            matchup.equity / 100.0
        } else {
            (100.0 - matchup.equity) / 100.0
        };

        PotEquityProblem {
            player_hand,
            opponent_hand,
            board: vec![],
            pot_size,
            bet_to_call,
            player_equity,
            pot_odds,
            correct_decision: player_equity > pot_odds,
        }
    } else {
        let mut deck = Deck::new();
        deck.shuffle(&mut rng);
        let player_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
        let opponent_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
        let board = draw_board(&mut deck, chosen_street);

        let pot_size = (rng.random_range(10_000..100_000) / 1000) * 1000;
        let bet_to_call = generate_bet_size(pot_size, allow_overbets, &mut rng);

        let equity_result = equity_calculator::calculate_equity(&player_hand, &opponent_hand, &board, 10_000);
        let player_equity = equity_result.equity();
        let pot_odds = bet_to_call as f32 / (pot_size + bet_to_call) as f32;

        PotEquityProblem {
            player_hand,
            opponent_hand,
            board,
            pot_size,
            bet_to_call,
            player_equity,
            pot_odds,
            correct_decision: player_equity > pot_odds,
        }
    }
}

fn generate_bet_size(pot_size: u32, allow_overbets: bool, rng: &mut impl Rng) -> u32 {
    let min_bet = pot_size / 4;
    let max_bet = if allow_overbets { pot_size * 3 / 2 } else { pot_size * 9 / 10 };
    let effective_min_bet = if min_bet < 5000 { 5000 } else { min_bet };
    (rng.random_range(effective_min_bet..=max_bet) / 1000) * 1000 // Use random_range
}

fn draw_board(deck: &mut Deck, stage: &Street) -> Vec<Card> {
    let num_cards = match stage {
        Street::PreFlop => 0,
        Street::Flop => 3,
        Street::Turn => 4,
        Street::River => 5,
    };
    (0..num_cards).map(|_| deck.cards.pop().unwrap()).collect()
}

fn hands_from_strings(hand1_str: &str, hand2_str: &str) -> (Vec<Card>, Vec<Card>) {
    let mut deck = Deck::new();
    let hand1 = hand_string_to_cards(hand1_str, &deck.cards);
    deck.remove_cards(&hand1);
    let hand2 = hand_string_to_cards(hand2_str, &deck.cards);
    (hand1, hand2)
}

fn hand_string_to_cards(hand_str: &str, available_cards: &[Card]) -> Vec<Card> {
    let chars: Vec<char> = hand_str.chars().collect();
    let r1 = char_to_rank(chars[0]);
    let r2 = char_to_rank(chars[1]);

    if r1 == r2 {
        return available_cards.iter().filter(|c| c.rank == r1).take(2).cloned().collect();
    }
    let is_suited = chars.len() == 3 && chars[2] == 's';
    if is_suited {
        let card1 = available_cards.iter().find(|c| c.rank == r1).unwrap();
        let card2 = available_cards.iter().find(|c| c.rank == r2 && c.suit == card1.suit).unwrap();
        vec![*card1, *card2]
    } else {
        let card1 = available_cards.iter().find(|c| c.rank == r1).unwrap();
        let card2 = available_cards.iter().find(|c| c.rank == r2 && c.suit != card1.suit).unwrap();
        vec![*card1, *card2]
    }
}

fn char_to_rank(c: char) -> Rank {
    match c {
        'A' => Rank::Ace,
        'K' => Rank::King,
        'Q' => Rank::Queen,
        'J' => Rank::Jack,
        'T' => Rank::Ten,
        '9' => Rank::Nine,
        '8' => Rank::Eight,
        '7' => Rank::Seven,
        '6' => Rank::Six,
        '5' => Rank::Five,
        '4' => Rank::Four,
        '3' => Rank::Three,
        '2' => Rank::Two,
        _ => panic!("Invalid rank character"),
    }
}