use crate::poker_logic::card::{Card, Suit};
use serde::{Serialize, Deserialize};
use std::hash::Hash;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HandRank {
    /// No combination; ranked by highest cards.
    HighCard(Vec<u8>),
    /// One pair plus three kickers.
    OnePair(u8, Vec<u8>),
    /// Two pairs and one kicker.
    TwoPair(u8, u8, u8),
    /// Three of a kind and two kickers.
    ThreeOfAKind(u8, Vec<u8>),
    /// Five sequential ranks. Stored as the high card of the straight.
    Straight(u8),
    /// Five cards of the same suit. Ranked by card values.
    Flush(Vec<u8>),
    /// Three of a kind plus a pair.
    FullHouse(u8, u8),
    /// Four of a kind and one kicker.
    FourOfAKind(u8, u8),
    /// Straight with all cards of the same suit. Stored as the high card.
    StraightFlush(u8),
    /// Ace-high straight flush (A-K-Q-J-T of same suit).
    RoyalFlush,
}

// Looks at 5 card combinations of the 7 cards (2 on hand, 5 community)
// returns strongest hand

const COMBINATIONS: [[usize; 5]; 21] = [
    [0, 1, 2, 3, 4],
    [0, 1, 2, 3, 5],
    [0, 1, 2, 3, 6],
    [0, 1, 2, 4, 5],
    [0, 1, 2, 4, 6],
    [0, 1, 2, 5, 6],
    [0, 1, 3, 4, 5],
    [0, 1, 3, 4, 6],
    [0, 1, 3, 5, 6],
    [0, 1, 4, 5, 6],
    [0, 2, 3, 4, 5],
    [0, 2, 3, 4, 6],
    [0, 2, 3, 5, 6],
    [0, 2, 4, 5, 6],
    [0, 3, 4, 5, 6],
    [1, 2, 3, 4, 5],
    [1, 2, 3, 4, 6],
    [1, 2, 3, 5, 6],
    [1, 2, 4, 5, 6],
    [1, 3, 4, 5, 6],
    [2, 3, 4, 5, 6],
];

pub fn evaluate_hand(cards: &[Card; 7]) -> HandRank {
    COMBINATIONS.iter()
        .map(|indices| {
            let mut hand = [Card::default(); 5];
            for i in 0..5 {
                hand[i] = cards[indices[i]];
            }
            classify_hand(&hand)
        })
        .max()
        .unwrap()
}

// 
fn classify_hand(hand: &[Card]) -> HandRank {
    let mut ranks: Vec<u8> = hand.iter().map(|c| c.rank.to_u8()).collect();
    let suits: Vec<Suit> = hand.iter().map(|c| c.suit).collect();

    ranks.sort_unstable_by(|a, b| b.cmp(a));
    let is_flush = suits.iter().all(|&s| s == suits[0]);
    let straight_high = detect_straight(&mut ranks);

    if is_flush {
        if let Some(high) = straight_high {
            if high == 14 {
                return HandRank::RoyalFlush;
            } else {
                return HandRank::StraightFlush(high);
            }
        }
    }

    let rank_counts = count_ranks(&ranks);
    let mut count_vec: Vec<(u8, usize)> = rank_counts.into_iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0))); // by count then rank

    match count_vec.as_slice() {
        [(quad, 4), (kicker, 1)] => HandRank::FourOfAKind(*quad, *kicker),
        [(three, 3), (two, 2)] => HandRank::FullHouse(*three, *two),
        _ if is_flush => HandRank::Flush(ranks.clone()),
        _ => {
            if let Some(high) = straight_high {
                return HandRank::Straight(high);
            }

            match count_vec.as_slice() {
                [(three, 3), (k1, 1), (k2, 1)] => HandRank::ThreeOfAKind(*three, vec![*k1, *k2]),
                [(p1, 2), (p2, 2), (k, 1)] => HandRank::TwoPair(*p1.max(p2), *p1.min(p2), *k),
                [(pair, 2), (k1, 1), (k2, 1), (k3, 1)] => {
                    HandRank::OnePair(*pair, vec![*k1, *k2, *k3])
                }
                _ => HandRank::HighCard(ranks.clone()),
            }
        }
    }
}

fn count_ranks(ranks: &[u8]) -> std::collections::HashMap<u8, usize> {
    let mut map = std::collections::HashMap::new();
    for &rank in ranks {
        *map.entry(rank).or_insert(0) += 1;
    }
    map
}

fn detect_straight(ranks: &mut Vec<u8>) -> Option<u8> {
    let mut dedup = ranks.clone();
    dedup.sort_unstable();
    dedup.dedup();

    if dedup.len() < 5 {
        return None;
    }

    for window in dedup.windows(5) {
        if window[4] - window[0] == 4 {
            return Some(window[4]);
        }
    }

    // Special case: A-2-3-4-5
    if dedup.contains(&14) && dedup.contains(&2) && dedup.contains(&3)
        && dedup.contains(&4) && dedup.contains(&5)
    {
        return Some(5);
    }

    None
}