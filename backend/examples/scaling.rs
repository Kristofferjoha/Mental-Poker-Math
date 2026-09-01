use std::time::Instant;
use poker_eval::eval::seven::build_tables;
use backend::calculators::equity_calculator::calculate_equity;
use backend::poker_core::card::parse_hand;

fn binom(n: u64, k: u64) -> u64 {
    if k > n { return 0 }
    (0..k).fold(1u64, |acc, i| acc * (n - i) / (i + 1))
}

fn main() {
    let t = build_tables(false);
    let (a, b) = (parse_hand("AsAh").unwrap(), parse_hand("KsKh").unwrap());

    let total: u64 = binom(48, 5);
    let tasks: Vec<u64> = (0..44).map(|i| binom(47 - i, 4)).collect();
    println!("pre-flop heads-up: {} boards", total);
    println!("  split-by-first-card (old): {} tasks, largest {} boards = {:.1}% of all work",
        tasks.len(), tasks[0], tasks[0] as f64 / total as f64 * 100.0);
    println!("    -> capped speedup at {:.1}x regardless of core count",
        total as f64 / tasks[0] as f64);
    println!("  split-by-index (current):  equal chunks, no such ceiling
");

    println!("measured scaling:");
    let mut baseline = 0.0;
    for threads in [1usize, 2, 4, 8, 16, 24] {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
        let ms = pool.install(|| {
            calculate_equity(&[&a, &b], &[], &t); // warm
            let start = Instant::now();
            for _ in 0..5 { calculate_equity(&[&a, &b], &[], &t); }
            start.elapsed().as_secs_f64() * 1000.0 / 5.0
        });
        if threads == 1 { baseline = ms }
        println!("  {threads:>2} threads  {ms:>8.3} ms   speedup {:>5.2}x   efficiency {:>5.1}%",
            baseline / ms, baseline / ms / threads as f64 * 100.0);
    }
}
