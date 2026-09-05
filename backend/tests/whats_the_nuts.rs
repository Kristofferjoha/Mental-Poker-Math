use poker_eval::eval::seven::{build_tables, get_rank};

use backend::calculators::calculator_helpers::card_to_poker_eval_id;
use backend::poker_core::card::Card;
use backend::problems::whats_the_nuts::{generate, Difficulty};

fn ids(cards: &[Card]) -> Vec<usize> {
    cards.iter().map(card_to_poker_eval_id).collect()
}

fn true_nut_rank(board: &[Card], tables: &poker_eval::eval::seven::TableSeven) -> u32 {
    let b = ids(board);
    let live: Vec<usize> = (0..52).filter(|c| !b.contains(c)).collect();
    let mut best = 0;
    for (i, &x) in live.iter().enumerate() {
        for &y in &live[i + 1..] {
            best = best.max(get_rank(tables, [x, y, b[0], b[1], b[2], b[3], b[4]]));
        }
    }
    best
}

fn rank_of(hand: &[Card], board: &[Card], tables: &poker_eval::eval::seven::TableSeven) -> u32 {
    let h = ids(hand);
    let b = ids(board);
    get_rank(tables, [h[0], h[1], b[0], b[1], b[2], b[3], b[4]])
}

#[test]
fn the_correct_candidate_is_actually_the_nuts() {
    let tables = build_tables(false);
    for difficulty in [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard] {
        for _ in 0..25 {
            let p = generate(difficulty, &tables);
            let best = true_nut_rank(&p.board, &tables);
            let chosen = rank_of(&p.candidates[p.correct_index], &p.board, &tables);
            assert_eq!(
                chosen, best,
                "{difficulty:?}: candidate {} is not the nuts",
                p.correct_index
            );
        }
    }
}

#[test]
fn no_distractor_ties_the_nuts() {
    let tables = build_tables(false);
    for difficulty in [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard] {
        for _ in 0..25 {
            let p = generate(difficulty, &tables);
            let best = rank_of(&p.candidates[p.correct_index], &p.board, &tables);
            for (i, candidate) in p.candidates.iter().enumerate() {
                if i == p.correct_index {
                    continue;
                }
                assert!(
                    rank_of(candidate, &p.board, &tables) < best,
                    "{difficulty:?}: candidate {i} ties or beats the nuts"
                );
            }
        }
    }
}

#[test]
fn candidates_use_only_undealt_cards() {
    let tables = build_tables(false);
    for _ in 0..30 {
        let p = generate(Difficulty::Hard, &tables);
        let board = ids(&p.board);
        assert_eq!(board.len(), 5);
        for candidate in &p.candidates {
            let c = ids(candidate);
            assert_eq!(c.len(), 2);
            assert_ne!(c[0], c[1], "a holding cannot repeat a card");
            for id in &c {
                assert!(!board.contains(id), "candidate reuses a board card");
            }
        }
    }
}

#[test]
fn each_difficulty_has_a_distinct_shape() {
    let tables = build_tables(false);
    let easy = generate(Difficulty::Easy, &tables);
    let hard = generate(Difficulty::Hard, &tables);

    assert!(hard.candidates.len() >= easy.candidates.len(), "harder offers more choices");
    for difficulty in [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard] {
        for _ in 0..40 {
            let n = generate(difficulty, &tables).candidates.len();
            assert!((4..=6).contains(&n), "{difficulty:?} produced {n} candidates");
        }
    }
}

#[test]
fn difficulty_changes_board_texture() {
    let tables = build_tables(false);
    let suited = |board: &[Card]| -> usize {
        (0..4)
            .map(|s| board.iter().filter(|c| card_to_poker_eval_id(c) % 4 == s).count())
            .max()
            .unwrap()
    };

    for _ in 0..20 {
        assert!(suited(&generate(Difficulty::Easy, &tables).board) <= 2);
        assert_eq!(suited(&generate(Difficulty::Medium, &tables).board), 3);
    }

    let rounds = 60;
    let four_suited = (0..rounds)
        .filter(|_| suited(&generate(Difficulty::Hard, &tables).board) == 4)
        .count();
    assert!(four_suited > 0, "hard never dealt a flush board in {rounds} spots");
    assert!(
        four_suited < rounds,
        "hard dealt nothing but flush boards in {rounds} spots"
    );
}

#[test]
fn difficulty_parsing_is_case_insensitive_and_rejects_junk() {
    assert_eq!(Difficulty::parse("HARD"), Some(Difficulty::Hard));
    assert_eq!(Difficulty::parse("easy"), Some(Difficulty::Easy));
    assert_eq!(Difficulty::parse("impossible"), None);
    assert_eq!(Difficulty::parse(""), None);
}
