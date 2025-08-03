use crate::card::{Card, Suit};
use itertools::Itertools;
use serde::{Serialize, Deserialize};
use std::hash::Hash;


#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HandCategory {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HandRank {
    HighCard(Vec<u8>), // order by high card 
    OnePair(u8, Vec<u8>),
    TwoPair(u8, u8, u8),
    ThreeOfAKind(u8, Vec<u8>), //trips and kickers
    Straight(u8),
    Flush(Vec<u8>), // vector since it can be based on high card
    FullHouse(u8, u8),
    FourOfAKind(u8, u8),
    StraightFlush(u8),
    RoyalFlush,
}

impl HandRank {
    pub fn to_category(&self) -> HandCategory {
        match self {
            HandRank::HighCard(_) => HandCategory::HighCard,
            HandRank::OnePair(_, _) => HandCategory::OnePair,
            HandRank::TwoPair(_, _, _) => HandCategory::TwoPair,
            HandRank::ThreeOfAKind(_, _) => HandCategory::ThreeOfAKind,
            HandRank::Straight(_) => HandCategory::Straight,
            HandRank::Flush(_) => HandCategory::Flush,
            HandRank::FullHouse(_, _) => HandCategory::FullHouse,
            HandRank::FourOfAKind(_, _) => HandCategory::FourOfAKind,
            HandRank::StraightFlush(_) => HandCategory::StraightFlush,
            HandRank::RoyalFlush => HandCategory::RoyalFlush,
        }
    }
}

pub fn evaluate_hand(cards: &[Card]) -> HandRank {
    cards.iter()
        .combinations(5)
        .map(|combo| classify_hand(&combo.into_iter().cloned().collect::<Vec<_>>()))
        .max()
        .unwrap()
}

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
