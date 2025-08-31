use crate::poker_logic::{
    card::{Card, Rank, Suit},
    deck::Deck,
    equity_calculator,
    preflop_lookup::PreflopEquity,
};
use rand::{rng, Rng};
use rand::seq::SliceRandom;
use tracing::{error, info};
use rand::prelude::IndexedRandom;

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
        info!("Selected matchup: {} vs {}, equity: {}", matchup.hand1, matchup.hand2, matchup.equity);
        let player_is_hand1 = rng.gen_bool(0.5);
        let (player_hand, opponent_hand, used_equity) = match hands_from_strings(&matchup.hand1, &matchup.hand2, &mut rng) {
            Ok((h1, h2)) => {
                let (player_hand, opponent_hand) = if player_is_hand1 { (h1, h2) } else { (h2, h1) };
                let equity = if player_is_hand1 {
                    matchup.equity / 100.0
                } else {
                    (100.0 - matchup.equity) / 100.0
                };
                info!("Parsed hands successfully: {:?} vs {:?}, equity: {}", player_hand, opponent_hand, equity);
                (player_hand, opponent_hand, equity)
            }
            Err(e) => {
                error!("Failed to parse hands {} vs {}: {}, generating random hands", matchup.hand1, matchup.hand2, e);
                let mut deck = Deck::new();
                deck.shuffle(&mut rng);
                let player_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
                let opponent_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
                let equity_result = equity_calculator::calculate_equity(&player_hand, &opponent_hand, &vec![], 25_000);
                (player_hand, opponent_hand, equity_result.equity())
            }
        };

        let pot_size = (rng.gen_range(10_000..100_000) / 1000) * 1000;
        let bet_to_call = generate_bet_size(pot_size, allow_overbets, &mut rng);
        let pot_odds = bet_to_call as f32 / (pot_size + bet_to_call) as f32;

        PotEquityProblem {
            player_hand,
            opponent_hand,
            board: vec![],
            pot_size,
            bet_to_call,
            player_equity: used_equity,
            pot_odds,
            correct_decision: used_equity > pot_odds,
        }
    } else {
        let mut deck = Deck::new();
        deck.shuffle(&mut rng);
        let player_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
        let opponent_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
        let board = draw_board(&mut deck, chosen_street);

        let pot_size = (rng.gen_range(10_000..100_000) / 1000) * 1000;
        let bet_to_call = generate_bet_size(pot_size, allow_overbets, &mut rng);

        let equity_result = equity_calculator::calculate_equity(&player_hand, &opponent_hand, &board, 1_000_000);
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
    (rng.gen_range(effective_min_bet..=max_bet) / 1000) * 1000
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

fn hands_from_strings(hand1_str: &str, hand2_str: &str, rng: &mut impl Rng) -> Result<(Vec<Card>, Vec<Card>), &'static str> {
    let mut deck = Deck::new();
    let chars1: Vec<char> = hand1_str.chars().collect();
    let chars2: Vec<char> = hand2_str.chars().collect();
    let is_suited1 = chars1.len() == 3 && chars1[2] == 's';
    let is_suited2 = chars2.len() == 3 && chars2[2] == 's';
    let is_pair1 = chars1.len() == 2 && chars1[0] == chars1[1];
    let is_pair2 = chars2.len() == 2 && chars2[0] == chars2[1];

    // Handle cases where one hand is a pair and the other is suited with the same second rank
    let hand1 = if (is_pair1 && is_suited2 && chars1[0] == chars2[1]) || (is_suited1 && is_pair2 && chars1[1] == chars2[0]) {
        let (pair_str, suited_str, pair_is_hand1) = if is_pair1 { (hand1_str, hand2_str, true) } else { (hand2_str, hand1_str, false) };
        let r_pair = char_to_rank(pair_str.chars().next().ok_or("Invalid pair string")?)?;
        let r_suited1 = char_to_rank(suited_str.chars().next().ok_or("Invalid suited string")?)?;
        let r_suited2 = char_to_rank(suited_str.chars().nth(1).ok_or("Invalid suited string")?)?;

        // Parse the pair first
        let pair: Vec<Card> = deck.cards.iter().filter(|c| c.rank == r_pair).take(2).cloned().collect();
        if pair.len() < 2 {
            return Err("Not enough cards for pair");
        }
        deck.remove_cards(&pair);

        // For the suited hand, choose a suit that has the second rank available
        let available_suits: Vec<Suit> = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades]
            .into_iter()
            .filter(|s| deck.cards.iter().any(|c| c.rank == r_suited2 && c.suit == *s))
            .collect();
        if available_suits.is_empty() {
            return Err("No suits available for suited hand");
        }
        let suit = *available_suits.choose(rng).ok_or("Failed to select suit for suited hand")?;
        let card1 = deck.cards.iter().find(|c| c.rank == r_suited1 && c.suit == suit).ok_or("No card found for first rank in suited hand")?;
        let card2 = deck.cards.iter().find(|c| c.rank == r_suited2 && c.suit == suit).ok_or("No suited card found for second rank")?;
        info!("{}: Selected suit {:?}", suited_str, suit);
        let suited = vec![*card1, *card2];

        if pair_is_hand1 { pair } else { suited }
    } else if is_suited1 && is_suited2 && chars1[1] == chars2[1] {
        // Handle suited hands with shared second rank
        let r1 = char_to_rank(chars1[0])?;
        let r2 = char_to_rank(chars1[1])?;
        let available_suits: Vec<Suit> = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades]
            .into_iter()
            .filter(|s| deck.cards.iter().any(|c| c.rank == r2 && c.suit == *s))
            .collect();
        if available_suits.len() < 2 {
            return Err("Not enough suits for shared second rank");
        }
        let suit = *available_suits.choose(rng).ok_or("Failed to select suit for hand1")?;
        let card1 = deck.cards.iter().find(|c| c.rank == r1 && c.suit == suit).ok_or("No card found for first rank in hand1")?;
        let card2 = deck.cards.iter().find(|c| c.rank == r2 && c.suit == suit).ok_or("No suited card found for second rank in hand1")?;
        info!("Hand1 {}: Selected suit {:?}", hand1_str, suit);
        vec![*card1, *card2]
    } else {
        hand_string_to_cards(hand1_str, &deck.cards, rng)?
    };

    deck.remove_cards(&hand1);

    let hand2 = if (is_pair1 && is_suited2 && chars1[0] == chars2[1]) || (is_suited1 && is_pair2 && chars1[1] == chars2[0]) {
        let (pair_str, suited_str, pair_is_hand1) = if is_pair1 { (hand1_str, hand2_str, true) } else { (hand2_str, hand1_str, false) };
        let r_suited1 = char_to_rank(suited_str.chars().next().ok_or("Invalid suited string")?)?;
        let r_suited2 = char_to_rank(suited_str.chars().nth(1).ok_or("Invalid suited string")?)?;
        let available_suits: Vec<Suit> = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades]
            .into_iter()
            .filter(|s| deck.cards.iter().any(|c| c.rank == r_suited2 && c.suit == *s))
            .collect();
        if available_suits.is_empty() {
            return Err("No suits available for suited hand");
        }
        let suit = *available_suits.choose(rng).ok_or("Failed to select suit for suited hand")?;
        let card1 = deck.cards.iter().find(|c| c.rank == r_suited1 && c.suit == suit).ok_or("No card found for first rank in suited hand")?;
        let card2 = deck.cards.iter().find(|c| c.rank == r_suited2 && c.suit == suit).ok_or("No suited card found for second rank")?;
        info!("{}: Selected suit {:?}", suited_str, suit);
        vec![*card1, *card2]
    } else if is_suited1 && is_suited2 && chars1[1] == chars2[1] {
        let r1 = char_to_rank(chars2[0])?;
        let r2 = char_to_rank(chars2[1])?;
        let used_suit = hand1[0].suit;
        let available_suits: Vec<Suit> = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades]
            .into_iter()
            .filter(|s| *s != used_suit && deck.cards.iter().any(|c| c.rank == r2 && c.suit == *s))
            .collect();
        if available_suits.is_empty() {
            return Err("No different suit available for hand2");
        }
        let suit = *available_suits.choose(rng).ok_or("Failed to select suit for hand2")?;
        let card1 = deck.cards.iter().find(|c| c.rank == r1 && c.suit == suit).ok_or("No card found for first rank in hand2")?;
        let card2 = deck.cards.iter().find(|c| c.rank == r2 && c.suit == suit).ok_or("No suited card found for second rank in hand2")?;
        info!("Hand2 {}: Selected suit {:?}", hand2_str, suit);
        vec![*card1, *card2]
    } else {
        hand_string_to_cards(hand2_str, &deck.cards, rng)?
    };

    Ok((hand1, hand2))
}

fn hand_string_to_cards(hand_str: &str, available_cards: &[Card], rng: &mut impl Rng) -> Result<Vec<Card>, &'static str> {
    let chars: Vec<char> = hand_str.chars().collect();
    info!("Parsing hand: {}", hand_str);
    if chars.len() < 2 || chars.len() > 3 {
        return Err("Invalid hand string format");
    }
    let r1 = char_to_rank(chars[0])?;
    let r2 = char_to_rank(chars[1])?;

    if r1 == r2 {
        let pair: Vec<Card> = available_cards.iter().filter(|c| c.rank == r1).take(2).cloned().collect();
        if pair.len() < 2 {
            return Err("Not enough cards for pair");
        }
        return Ok(pair);
    }

    let is_suited = chars.len() == 3 && chars[2] == 's';
    let card1_candidates: Vec<&Card> = available_cards.iter().filter(|c| c.rank == r1).collect();
    if card1_candidates.is_empty() {
        return Err("No card found for first rank");
    }
    let card1 = *card1_candidates.choose(rng).ok_or("Failed to select first card")?;

    if is_suited {
        let card2 = available_cards.iter().find(|c| c.rank == r2 && c.suit == card1.suit);
        if card2.is_none() {
            error!("No card with rank {:?} and suit {:?} found for {}", r2, card1.suit, hand_str);
            return Err("No suited card found for second rank");
        }
        Ok(vec![*card1, *card2.unwrap()])
    } else {
        let card2 = available_cards.iter().find(|c| c.rank == r2 && c.suit != card1.suit);
        if card2.is_none() {
            error!("No card with rank {:?} and different suit from {:?} found for {}", r2, card1.suit, hand_str);
            return Err("No offsuit card found for second rank");
        }
        Ok(vec![*card1, *card2.unwrap()])
    }
}

fn char_to_rank(c: char) -> Result<Rank, &'static str> {
    match c {
        'A' => Ok(Rank::Ace),
        'K' => Ok(Rank::King),
        'Q' => Ok(Rank::Queen),
        'J' => Ok(Rank::Jack),
        'T' => Ok(Rank::Ten),
        '9' => Ok(Rank::Nine),
        '8' => Ok(Rank::Eight),
        '7' => Ok(Rank::Seven),
        '6' => Ok(Rank::Six),
        '5' => Ok(Rank::Five),
        '4' => Ok(Rank::Four),
        '3' => Ok(Rank::Three),
        '2' => Ok(Rank::Two),
        _ => Err("Invalid rank character"),
    }
}