use poker_eval::eval::seven::build_tables;

use backend::problems::king_of_the_hill::{generate, score, MAX_HANDS, MIN_HANDS};

#[test]
fn pairwise_scoring_matches_manual_counting() {
    let equities = [0.5, 0.3, 0.2];
    let expected = [
        ([0, 1, 2], 3),
        ([0, 2, 1], 2),
        ([1, 0, 2], 2),
        ([1, 2, 0], 1),
        ([2, 0, 1], 1),
        ([2, 1, 0], 0)
    ];
    for (guess, pairs) in expected {
        let s = score(&guess, &equities);
        assert_eq!(s.total_pairs, 3);
        assert_eq!(s.correct_pairs, pairs, "guess {guess:?}");
    }
}

#[test]
fn kendall_tau_puts_random_at_zero() {
    let equities = [0.4, 0.3, 0.2, 0.1];

    let perfect = score(&[0, 1, 2, 3], &equities);
    assert_eq!(perfect.kendall_tau, 1.0);
    assert_eq!(perfect.correct_pairs, perfect.total_pairs);

    let backwards = score(&[3, 2, 1, 0], &equities);
    assert_eq!(backwards.kendall_tau, -1.0);
    assert_eq!(backwards.correct_pairs, 0);

    let orders = [
        [0, 1, 2, 3], [0, 1, 3, 2], [0, 2, 1, 3], [0, 2, 3, 1], [0, 3, 1, 2], [0, 3, 2, 1],
        [1, 0, 2, 3], [1, 0, 3, 2], [1, 2, 0, 3], [1, 2, 3, 0], [1, 3, 0, 2], [1, 3, 2, 0],
        [2, 0, 1, 3], [2, 0, 3, 1], [2, 1, 0, 3], [2, 1, 3, 0], [2, 3, 0, 1], [2, 3, 1, 0],
        [3, 0, 1, 2], [3, 0, 2, 1], [3, 1, 0, 2], [3, 1, 2, 0], [3, 2, 0, 1], [3, 2, 1, 0]
    ];
    let n = orders.len() as f64;
    let mean_tau: f64 = orders.iter().map(|o| score(o, &equities).kendall_tau).sum::<f64>() / n;
    let mean_ratio: f64 = orders
        .iter()
        .map(|o| {
            let s = score(o, &equities);
            s.correct_pairs as f64 / s.total_pairs as f64
        })
        .sum::<f64>()
        / n;

    assert!(mean_tau.abs() < 1e-12, "random tau should be 0, got {mean_tau}");
    assert!((mean_ratio - 0.5).abs() < 1e-12, "random ratio is 0.5, got {mean_ratio}");
}

#[test]
fn tied_hands_accept_either_order() {
    let equities = [0.25, 0.25, 0.5];
    assert_eq!(score(&[2, 0, 1], &equities).correct_pairs, 3);
    assert_eq!(score(&[2, 1, 0], &equities).correct_pairs, 3);
}

#[test]
fn generated_problems_are_well_formed() {
    let tables = build_tables(false);
    for hands in MIN_HANDS..=MAX_HANDS {
        let p = generate(hands, vec!["flop".into()], &tables);

        assert_eq!(p.hands.len(), hands);
        assert_eq!(p.equities.len(), hands);
        assert_eq!(p.correct_order.len(), hands);
        assert_eq!(p.board.len(), 3);

        let total: f64 = p.equities.iter().sum();
        assert!((total - 1.0).abs() < 1e-9, "{hands} hands summed to {total}");

        let mut seen = p.correct_order.clone();
        seen.sort_unstable();
        assert_eq!(seen, (0..hands).collect::<Vec<_>>());
        for pair in p.correct_order.windows(2) {
            assert!(p.equities[pair[0]] >= p.equities[pair[1]], "order is not descending");
        }

        let s = score(&p.correct_order, &p.equities);
        assert_eq!(s.correct_pairs, s.total_pairs);
        assert_eq!(s.kendall_tau, 1.0);
        assert_eq!(s.total_pairs, hands * (hands - 1) / 2);
    }
}

#[test]
fn six_hands_deal_distinct_cards() {
    let tables = build_tables(false);
    let p = generate(6, vec!["turn".into()], &tables);
    let mut seen = std::collections::HashSet::new();
    for card in p.hands.iter().flatten().chain(p.board.iter()) {
        assert!(seen.insert(format!("{card:?}")), "duplicate card {card:?}");
    }
    assert_eq!(seen.len(), 6 * 2 + 4);
}

#[test]
fn the_river_is_never_dealt() {
    let tables = build_tables(false);
    for _ in 0..12 {
        let p = generate(5, vec!["river".into()], &tables);
        assert!(p.board.len() < 5, "dealt a complete board: {} cards", p.board.len());
    }
}

#[test]
fn preflop_rankings_have_no_board() {
    let tables = build_tables(false);
    let p = generate(5, vec!["pre-flop".into()], &tables);
    assert!(p.board.is_empty());
    assert_eq!(p.hands.len(), 5);
}

#[test]
fn near_identical_equities_count_as_a_tie() {
    let equities = vec![0.4, 0.3, 0.3 + 1e-12, 0.0];

    let one = score(&[0, 1, 2, 3], &equities);
    let other = score(&[0, 2, 1, 3], &equities);

    assert_eq!(one, other);
    assert_eq!(one.correct_pairs, one.total_pairs, "a tie must not cost a pair");
    assert_eq!(one.kendall_tau, 1.0);
}
