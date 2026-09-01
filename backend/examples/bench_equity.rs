use std::time::Instant;

use poker_eval::eval::seven::{build_tables, get_rank, TableSeven};
use rand::seq::SliceRandom;
use rayon::prelude::*;

use backend::calculators::calculator_helpers::card_to_poker_eval_id;
use backend::calculators::equity_calculator::calculate_equity;
use backend::poker_core::card::{parse_cards, parse_hand, Card};
use backend::poker_core::deck::Deck;

fn old_monte_carlo(
    player: &[Card],
    opponent: &[Card],
    board: &[Card],
    sims: u32,
    tables: &TableSeven,
) -> f64 {
    if board.len() == 5 {
        let p: Vec<usize> = player.iter().chain(board).map(card_to_poker_eval_id).collect();
        let o: Vec<usize> = opponent.iter().chain(board).map(card_to_poker_eval_id).collect();
        let (p, o): ([usize; 7], [usize; 7]) = (p.try_into().unwrap(), o.try_into().unwrap());
        let (pr, or) = (get_rank(tables, p), get_rank(tables, o));
        return if pr > or { 1.0 } else if pr < or { 0.0 } else { 0.5 };
    }

    let remaining = {
        let mut d = Deck::new();
        d.remove_cards(player);
        d.remove_cards(opponent);
        d.remove_cards(board);
        d
    };

    let (wins, ties) = (0..sims)
        .into_par_iter()
        .map_init(rand::rng, |rng, _| {
            let mut deck_copy = remaining.clone();
            deck_copy.cards.shuffle(rng);

            let mut final_board = board.to_vec();
            final_board.extend(deck_copy.cards.iter().take(5 - board.len()));

            let pv: Vec<usize> = player.iter().chain(&final_board).map(card_to_poker_eval_id).collect();
            let ov: Vec<usize> = opponent.iter().chain(&final_board).map(card_to_poker_eval_id).collect();
            let p: [usize; 7] = pv.try_into().unwrap();
            let o: [usize; 7] = ov.try_into().unwrap();

            let (pr, or) = (get_rank(tables, p), get_rank(tables, o));
            if pr > or { (1u32, 0u32) } else if pr == or { (0, 1) } else { (0, 0) }
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    (wins as f64 + ties as f64 / 2.0) / sims as f64
}

fn bench(label: &str, mut f: impl FnMut() -> f64) -> (f64, f64) {
    f(); // warm
    let start = Instant::now();
    let runs = 20;
    let mut last = 0.0;
    for _ in 0..runs {
        last = f();
    }
    let ms = start.elapsed().as_secs_f64() * 1000.0 / runs as f64;
    println!("    {label:<26} {ms:>9.3} ms   equity {last:.6}");
    (ms, last)
}

fn main() {
    let t = build_tables(false);
    let hero = parse_hand("AsAh").unwrap();
    let villain = parse_hand("KsKh").unwrap();

    println!("threads: {}\n", rayon::current_num_threads());

    for (street, cards) in [
        ("pre-flop", ""),
        ("flop", "2c7d9s"),
        ("turn", "2c7d9sTh"),
        ("river", "2c7d9sThJc"),
    ] {
        let board = if cards.is_empty() { Vec::new() } else { parse_cards(cards).unwrap() };
        let completions = match board.len() {
            0 => 1_712_304u64,
            3 => 45 * 44 / 2,
            4 => 44,
            _ => 1,
        };
        println!("  {street}  ({completions} board completions)");
        let (old, old_eq) = bench("OLD  25k Monte Carlo", || {
            old_monte_carlo(&hero, &villain, &board, 25_000, &t)
        });
        let (new, new_eq) = bench("NEW  exact enumeration", || {
            calculate_equity(&[&hero, &villain], &board, &t).hero()
        });
        let ratio = old / new;
        let verdict = if ratio >= 1.0 {
            format!("{ratio:.1}x FASTER than the old MC")
        } else {
            format!("{:.1}x slower than the old MC", 1.0 / ratio)
        };
        println!("    -> exact is {verdict}");
        println!("    -> old MC error vs truth: {:.4} pts\n", (old_eq - new_eq).abs() * 100.0);
    }

    println!("  multiway exact (pre-flop, all seats):");
    let all: Vec<Vec<Card>> = ["AsAh", "KsKh", "QsQh", "JsJh", "TsTh", "9s9h"]
        .iter()
        .map(|h| parse_hand(h).unwrap())
        .collect();
    for seats in 2..=6 {
        let refs: Vec<&[Card]> = all[..seats].iter().map(|h| h.as_slice()).collect();
        let start = Instant::now();
        let r = calculate_equity(&refs, &[], &t);
        println!(
            "    {seats} seats  {:>8.3} ms   {:>9} boards   sum {:.9}",
            start.elapsed().as_secs_f64() * 1000.0,
            r.boards,
            r.equities.iter().sum::<f64>()
        );
    }
}
