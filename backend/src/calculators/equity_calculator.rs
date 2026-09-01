use poker_eval::eval::seven::TableSeven;
use rayon::prelude::*;

use crate::calculators::calculator_helpers::card_to_poker_eval_id;
use crate::poker_core::card::Card;

pub const MAX_PLAYERS: usize = 6;

const BOARD_SIZE: usize = 5;

#[derive(Debug, Clone, PartialEq)]
pub struct Equity {
    pub equities: Vec<f64>,
    pub boards: u64,
}

impl Equity {
    pub fn hero(&self) -> f64 {
        self.equities[0]
    }
}

fn live_mask(hands: &[&[Card]], board: &[Card]) -> u64 {
    let mut dead: u64 = 0;
    for card in hands.iter().flat_map(|h| h.iter()).chain(board.iter()) {
        let bit = 1u64 << card_to_poker_eval_id(card);
        assert!(dead & bit == 0, "duplicate card {card:?} in hands or board");
        dead |= bit;
    }
    !dead & ((1u64 << 52) - 1)
}

fn cards_in(mask: u64) -> Vec<usize> {
    let mut out = Vec::with_capacity(mask.count_ones() as usize);
    let mut m = mask;
    while m != 0 {
        out.push(m.trailing_zeros() as usize);
        m &= m - 1;
    }
    out
}

struct RankContext<'a> {
    face_key: &'a [u32; 52],
    flush_key: &'a [u32; 52],
    card_suit: &'a [usize; 52],
    suit_mask: u32,
    suit_shift: u32,
    face_rank: &'a [u32],
    flush_rank: &'a [u32],
    flush_suit: &'a [i32]
}

impl<'a> RankContext<'a> {
    fn new(tables: &'a TableSeven) -> Self {
        let pk = &tables.t5.pk;
        Self {
            face_key: &pk.card_face_key,
            flush_key: &pk.card_flush_key,
            card_suit: &pk.card_suit,
            suit_mask: pk.suit_mask,
            suit_shift: pk.suit_bit_shift,
            face_rank: &tables.face_rank,
            flush_rank: &tables.flush_rank,
            flush_suit: &tables.flush_suit
        }
    }

    #[inline]
    fn rank(&self, hand_key: u32, hole: &[usize; 2], board: &[usize; BOARD_SIZE]) -> u32 {
        let suit = self.flush_suit[(hand_key & self.suit_mask) as usize];
        if suit < 0 {
            return self.face_rank[(hand_key >> self.suit_shift) as usize];
        }

        let suit = suit as usize;
        let mut flush_key = 0u32;
        for &card in hole.iter().chain(board.iter()) {
            if self.card_suit[card] == suit {
                flush_key += self.flush_key[card];
            }
        }
        self.flush_rank[flush_key as usize]
    }
}

fn showdown(
    ctx: &RankContext,
    hole: &[[usize; 2]],
    hole_keys: &[u32],
    board: &[usize; BOARD_SIZE],
    board_key: u32,
    acc: &mut [f64],
) {
    let mut ranks = [0u32; MAX_PLAYERS];
    for (seat, cards) in hole.iter().enumerate() {
        ranks[seat] = ctx.rank(hole_keys[seat] + board_key, cards, board);
    }

    let seats = &ranks[..hole.len()];
    let best = seats.iter().copied().max().expect("at least one seat");
    let share = 1.0 / seats.iter().filter(|&&r| r == best).count() as f64;

    for (seat, &rank) in seats.iter().enumerate() {
        if rank == best {
            acc[seat] += share;
        }
    }
}

fn binomial(n: usize, k: usize) -> u64 {
    if k > n {
        return 0;
    }
    (0..k as u64).fold(1u64, |acc, i| acc * (n as u64 - i) / (i + 1))
}

fn unrank_combination(mut rank: u64, m: usize, out: &mut [usize]) {
    let k = out.len();
    let mut candidate = 0usize;
    for slot in 0..k {
        loop {
            // how many combinations start with `candidate` in this slot
            let branch = binomial(m - candidate - 1, k - slot - 1);
            if rank < branch {
                out[slot] = candidate;
                candidate += 1;
                break;
            }
            rank -= branch;
            candidate += 1;
        }
    }
}

fn next_combination(combo: &mut [usize], m: usize) -> usize {
    let k = combo.len();
    for i in (0..k).rev() {
        if combo[i] != i + m - k {
            combo[i] += 1;
            for j in (i + 1)..k {
                combo[j] = combo[j - 1] + 1;
            }
            return i;
        }
    }
    k
}

pub fn calculate_equity(hands: &[&[Card]], board: &[Card], tables: &TableSeven) -> Equity {
    let seats = hands.len();
    assert!(
        (2..=MAX_PLAYERS).contains(&seats),
        "expected 2..={MAX_PLAYERS} hands, got {seats}"
    );
    assert!(
        hands.iter().all(|h| h.len() == 2),
        "every hand must be exactly two cards"
    );
    assert!(
        board.len() <= BOARD_SIZE,
        "board has {} cards, at most {BOARD_SIZE}",
        board.len()
    );

    let live = cards_in(live_mask(hands, board));
    let ctx = RankContext::new(tables);

    let hole: Vec<[usize; 2]> = hands
        .iter()
        .map(|h| [card_to_poker_eval_id(&h[0]), card_to_poker_eval_id(&h[1])])
        .collect();

    // Each seat's two hole cards are fixed for the whole enumeration.
    let hole_keys: Vec<u32> = hole
        .iter()
        .map(|c| ctx.face_key[c[0]] + ctx.face_key[c[1]])
        .collect();

    let mut known = [0usize; BOARD_SIZE];
    let mut known_key = 0u32;
    for (slot, card) in board.iter().enumerate() {
        let id = card_to_poker_eval_id(card);
        known[slot] = id;
        known_key += ctx.face_key[id];
    }
    let dealt = board.len();
    let needed = BOARD_SIZE - dealt;

    // A complete board is a single deterministic showdown.
    if needed == 0 {
        let mut acc = vec![0.0; seats];
        showdown(&ctx, &hole, &hole_keys, &known, known_key, &mut acc);
        return Equity {
            equities: acc,
            boards: 1,
        };
    }

    let live_count = live.len();
    let total_boards = binomial(live_count, needed);
    const MIN_CHUNK: u64 = 4096;
    let chunk = (total_boards / (rayon::current_num_threads() as u64 * 8)).max(MIN_CHUNK);
    let chunks = total_boards.div_ceil(chunk);

    let (totals, boards) = (0..chunks)
        .into_par_iter()
        .map(|c| {
            let lo = c * chunk;
            let hi = ((c + 1) * chunk).min(total_boards);

            let mut acc = vec![0.0; seats];
            let mut combo = [0usize; BOARD_SIZE];
            unrank_combination(lo, live_count, &mut combo[..needed]);

            let mut work = known;
            let mut running = [0u32; BOARD_SIZE];
            let mut stale_from = 0usize;

            for _ in lo..hi {
                for j in stale_from..needed {
                    let card = live[combo[j]];
                    work[dealt + j] = card;
                    let prefix = if j == 0 { known_key } else { running[j - 1] };
                    running[j] = prefix + ctx.face_key[card];
                }

                showdown(&ctx, &hole, &hole_keys, &work, running[needed - 1], &mut acc);
                stale_from = next_combination(&mut combo[..needed], live_count);
            }
            (acc, hi - lo)
        })
        .reduce(
            || (vec![0.0; seats], 0u64),
            |mut a, b| {
                for (slot, value) in a.0.iter_mut().zip(b.0) {
                    *slot += value;
                }
                (a.0, a.1 + b.1)
            },
        );

    Equity {
        equities: totals.iter().map(|w| w / boards as f64).collect(),
        boards,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poker_eval::eval::seven::{build_tables, get_rank};

    #[test]
    fn hoisted_rank_matches_poker_eval() {
        let tables = build_tables(false);
        let ctx = RankContext::new(&tables);

        // deterministic LCG: no rng dependency, same sample on every run
        let mut state = 0x2545_F491_4F6C_DD1Du64;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };

        let mut checked = 0u32;
        let mut flushes = 0u32;

        for _ in 0..200_000 {
            // draw seven distinct cards
            let mut deck: u64 = (1 << 52) - 1;
            let mut cards = [0usize; 7];
            for slot in cards.iter_mut() {
                let live = cards_in(deck);
                let card = live[(next() % live.len() as u64) as usize];
                *slot = card;
                deck &= !(1u64 << card);
            }

            let hole = [cards[0], cards[1]];
            let board = [cards[2], cards[3], cards[4], cards[5], cards[6]];

            let key: u32 = cards.iter().map(|&c| ctx.face_key[c]).sum();
            let hoisted = ctx.rank(key, &hole, &board);
            let reference = get_rank(&tables, cards);

            assert_eq!(
                hoisted, reference,
                "rank mismatch on {cards:?}: hoisted {hoisted}, poker_eval {reference}"
            );

            if ctx.flush_suit[(key & ctx.suit_mask) as usize] >= 0 {
                flushes += 1;
            }
            checked += 1;
        }

        assert_eq!(checked, 200_000);
        assert!(
            flushes > 1_000,
            "sample only hit the flush path {flushes} times; it is not being exercised"
        );
    }
}
