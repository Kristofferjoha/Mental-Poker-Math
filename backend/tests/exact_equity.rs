use poker_eval::eval::seven::{build_tables, TableSeven};

use backend::calculators::equity_calculator::calculate_equity;
use backend::poker_core::card::{parse_cards, parse_hand, Card};

fn tables() -> std::sync::Arc<TableSeven> {
    build_tables(false)
}

fn hand(s: &str) -> Vec<Card> {
    parse_hand(s).expect("valid hand")
}

fn board(s: &str) -> Vec<Card> {
    parse_cards(s).expect("valid board")
}

#[test]
fn aces_versus_kings_matches_independent_enumeration() {
    let t = tables();
    let (aa, kk) = (hand("AsAh"), hand("KsKh"));
    let result = calculate_equity(&[&aa, &kk], &[], &t);

    assert_eq!(result.boards, 1_712_304, "must enumerate C(48,5) boards");
    assert!(
        (result.hero() - 0.826366).abs() < 1e-5,
        "expected 0.826366, got {}",
        result.hero()
    );
}

#[test]
fn suited_ak_versus_queens_matches_independent_enumeration() {
    let t = tables();
    let (ak, qq) = (hand("AcKc"), hand("QdQh"));
    let result = calculate_equity(&[&ak, &qq], &[], &t);

    assert!(
        (result.hero() - 0.462145).abs() < 1e-5,
        "expected 0.462145, got {}",
        result.hero()
    );
}

#[test]
fn suit_overlap_changes_the_answer() {
    let t = tables();
    let (ak, shared) = (hand("AcKc"), hand("QcQd"));
    let overlapped = calculate_equity(&[&ak, &shared], &[], &t).hero();
    assert!(
        (overlapped - 0.458826).abs() < 1e-5,
        "expected 0.458826, got {overlapped}"
    );

    let disjoint = calculate_equity(&[&ak, &hand("QdQh")], &[], &t).hero();
    assert!(disjoint > overlapped, "disjoint suits should favour AKs");
}

#[test]
fn equities_sum_to_one_for_every_seat_count() {
    let t = tables();
    let all = [
        hand("AsAh"),
        hand("KsKh"),
        hand("QsQh"),
        hand("JsJh"),
        hand("TsTh"),
        hand("9s9h"),
    ];

    for seats in 2..=6 {
        let refs: Vec<&[Card]> = all[..seats].iter().map(|h| h.as_slice()).collect();
        let result = calculate_equity(&refs, &[], &t);

        let total: f64 = result.equities.iter().sum();
        assert!(
            (total - 1.0).abs() < 1e-9,
            "{seats} seats summed to {total}"
        );
        assert_eq!(result.equities.len(), seats);
        assert!(result.equities.iter().all(|e| (0.0..=1.0).contains(e)));
    }
}

#[test]
fn a_three_way_chop_splits_exactly() {
    let t = tables();
    let (a, b, c) = (hand("2c2d"), hand("3c3d"), hand("4c4d"));
    let result = calculate_equity(&[&a, &b, &c], &board("AsKsQsJsTs"), &t);

    assert_eq!(result.boards, 1, "a complete board is one showdown");
    for equity in &result.equities {
        assert!((equity - 1.0 / 3.0).abs() < 1e-12, "got {equity}");
    }
}

#[test]
fn a_complete_board_has_a_single_winner() {
    let t = tables();
    let (two_pair, queens) = (hand("AsKs"), hand("QhQd"));
    let result = calculate_equity(&[&two_pair, &queens], &board("AhKh2c7d9s"), &t);

    assert_eq!(result.equities, vec![1.0, 0.0]);
}

#[test]
fn partial_boards_enumerate_the_right_number_of_completions() {
    let t = tables();
    let (a, b) = (hand("AsKs"), hand("QhQd"));

    let flop = calculate_equity(&[&a, &b], &board("2c7d9s"), &t);
    assert_eq!(flop.boards, 45 * 44 / 2);

    let turn = calculate_equity(&[&a, &b], &board("2c7d9sTh"), &t);
    assert_eq!(turn.boards, 44);

    for r in [flop, turn] {
        let total: f64 = r.equities.iter().sum();
        assert!((total - 1.0).abs() < 1e-9);
    }
}

#[test]
fn seat_order_does_not_matter() {
    let t = tables();
    let (aa, kk) = (hand("AsAh"), hand("KsKh"));
    let forward = calculate_equity(&[&aa, &kk], &[], &t);
    let reversed = calculate_equity(&[&kk, &aa], &[], &t);

    assert!((forward.equities[0] - reversed.equities[1]).abs() < 1e-12);
    assert!((forward.equities[1] - reversed.equities[0]).abs() < 1e-12);
}

#[test]
#[should_panic(expected = "duplicate card")]
fn a_duplicated_card_is_a_bug_not_a_wrong_answer() {
    let t = tables();
    let (a, b) = (hand("AsAh"), hand("AsKh"));
    calculate_equity(&[&a, &b], &[], &t);
}