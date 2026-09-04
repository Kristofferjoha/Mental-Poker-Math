use poker_eval::eval::seven::{get_rank, TableSeven};
use rand::prelude::{IndexedRandom, SliceRandom};
use rand::{rng, Rng};

use crate::calculators::calculator_helpers::{card_from_poker_eval_id, card_to_poker_eval_id};
use crate::poker_core::{card::Card, deck::Deck};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

impl Difficulty {
    pub fn parse(s: &str) -> Option<Difficulty> {
        match s.to_ascii_lowercase().as_str() {
            "easy" => Some(Difficulty::Easy),
            "medium" => Some(Difficulty::Medium),
            "hard" => Some(Difficulty::Hard),
            _ => None
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Difficulty::Easy => "easy",
            Difficulty::Medium => "medium",
            Difficulty::Hard => "hard"
        }
    }

    fn board_suit_target(self) -> usize {
        match self {
            Difficulty::Easy => 2,
            Difficulty::Medium => 3,
            Difficulty::Hard => 4
        }
    }

    fn distractor_positions(self, available: usize, rng: &mut impl Rng) -> Vec<usize> {
        if available == 0 {
            return Vec::new();
        }
        let wanted: Vec<usize> = match self {
            Difficulty::Hard => {
                let mut pool: Vec<usize> = (2..=12).collect();
                pool.shuffle(rng);
                let mut chosen = vec![1];
                chosen.extend(pool.into_iter().take(4));
                chosen
            }
            Difficulty::Medium => [0.004, 0.02, 0.10, 0.40]
                .iter()
                .map(|f| ((available as f64 * f).round() as usize).max(1))
                .collect(),
            Difficulty::Easy => [0.02, 0.12, 0.60]
                .iter()
                .map(|f| ((available as f64 * f).round() as usize).max(1))
                .collect(),
        };

        let mut chosen: Vec<usize> = Vec::new();
        for position in wanted {
            let mut p = position.min(available);
            while chosen.contains(&p) && p > 1 {
                p -= 1;
            }
            while chosen.contains(&p) && p < available {
                p += 1;
            }
            if !chosen.contains(&p) {
                chosen.push(p);
            }
        }
        chosen
    }

}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Texture {
    Flush,
    Paired,
    Connected,
    Dry
}

const TEXTURES: [Texture; 4] = [Texture::Flush, Texture::Paired, Texture::Connected, Texture::Dry];

fn faces(board: &[Card]) -> Vec<i32> {
    board
        .iter()
        .map(|c| (card_to_poker_eval_id(c) / 4) as i32)
        .collect()
}

fn is_paired(board: &[Card]) -> bool {
    let mut f = faces(board);
    f.sort_unstable();
    f.windows(2).any(|w| w[0] == w[1])
}

fn is_connected(board: &[Card]) -> bool {
    let mut f = faces(board);
    if f.contains(&12) {
        f.push(-1);
    }
    f.sort_unstable();
    f.dedup();
    f.windows(3).any(|w| w[2] - w[0] <= 4)
}

fn has_texture(texture: Texture, board: &[Card]) -> bool {
    let suited = max_suited(board);
    match texture {
        Texture::Flush => suited == 4,
        Texture::Paired => suited <= 2 && is_paired(board),
        Texture::Connected => suited <= 2 && !is_paired(board) && is_connected(board),
        Texture::Dry => suited <= 2 && !is_paired(board) && !is_connected(board)
    }
}

#[derive(Clone, Debug)]
pub struct NutsProblem {
    pub board: Vec<Card>,
    pub candidates: Vec<Vec<Card>>,
    pub correct_index: usize,
    pub difficulty: Difficulty
}

fn max_suited(board: &[Card]) -> usize {
    (0..4)
        .map(|s| {
            board
                .iter()
                .filter(|c| card_to_poker_eval_id(c) % 4 == s)
                .count()
        })
        .max()
        .unwrap_or(0)
}

fn holdings_by_rank(board: &[Card], tables: &TableSeven) -> Vec<(u32, [usize; 2])> {
    let dealt: Vec<usize> = board.iter().map(card_to_poker_eval_id).collect();
    let live: Vec<usize> = (0..52).filter(|c| !dealt.contains(c)).collect();
    let b = [dealt[0], dealt[1], dealt[2], dealt[3], dealt[4]];

    let mut ranked: Vec<(u32, [usize; 2])> = Vec::with_capacity(live.len() * live.len() / 2);
    for (i, &first) in live.iter().enumerate() {
        for &second in &live[i + 1..] {
            ranked.push((
                get_rank(tables, [first, second, b[0], b[1], b[2], b[3], b[4]]),
                [first, second],
            ));
        }
    }
    ranked.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    ranked.dedup_by_key(|entry| entry.0);
    ranked
}

pub fn generate(difficulty: Difficulty, tables: &TableSeven) -> NutsProblem {
    let mut rng = rng();

    let target = difficulty.board_suit_target();
    let texture = (difficulty == Difficulty::Hard).then(|| *TEXTURES.choose(&mut rng).unwrap());

    const MIN_DISTINCT_RANKS: usize = 24;

    let mut board: Vec<Card> = Vec::new();
    let mut ranked: Vec<(u32, [usize; 2])> = Vec::new();
    for attempt in 0..500 {
        let mut deck = Deck::new();
        deck.shuffle(&mut rng);
        let candidate: Vec<Card> = (0..5).map(|_| deck.cards.pop().unwrap()).collect();

        let texture_ok = match texture {
            Some(t) => has_texture(t, &candidate),
            None if difficulty == Difficulty::Easy => max_suited(&candidate) <= target,
            None => max_suited(&candidate) == target,
        };

        if texture_ok || attempt == 499 {
            let by_rank = holdings_by_rank(&candidate, tables);
            if by_rank.len() >= MIN_DISTINCT_RANKS || attempt == 499 {
                board = candidate;
                ranked = by_rank;
                break;
            }
        }
    }

    let (nut_rank, nut_cards) = ranked[0];

    let mut picks: Vec<[usize; 2]> = vec![nut_cards];
    for position in difficulty.distractor_positions(ranked.len() - 1, &mut rng) {
        let (rank, cards) = ranked[position];
        debug_assert!(rank < nut_rank, "distractor must be strictly worse than the nuts");
        picks.push(cards);
    }

    picks.shuffle(&mut rng);
    let correct_index = picks
        .iter()
        .position(|p| *p == nut_cards)
        .expect("the nuts is always among the candidates");

    let candidates = picks
        .iter()
        .map(|p| p.iter().map(|&id| card_from_poker_eval_id(id)).collect())
        .collect();

    NutsProblem {
        board,
        candidates,
        correct_index,
        difficulty
    }
}
