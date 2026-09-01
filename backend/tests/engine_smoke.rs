use poker_eval::eval::seven::build_tables;

use backend::calculators::calculator_helpers::card_to_poker_eval_id;
use backend::calculators::equity_calculator::calculate_equity;
use backend::poker_core::card::{Card, Rank, Suit};

fn card(rank: Rank, suit: Suit) -> Card {
    Card { rank, suit }
}

#[test]
fn card_ids_match_poker_eval_layout() {
    assert_eq!(card_to_poker_eval_id(&card(Rank::Two, Suit::Clubs)), 0);
    assert_eq!(card_to_poker_eval_id(&card(Rank::Two, Suit::Diamonds)), 1);
    assert_eq!(card_to_poker_eval_id(&card(Rank::Two, Suit::Hearts)), 2);
    assert_eq!(card_to_poker_eval_id(&card(Rank::Two, Suit::Spades)), 3);
    assert_eq!(card_to_poker_eval_id(&card(Rank::Ace, Suit::Clubs)), 48);
    assert_eq!(card_to_poker_eval_id(&card(Rank::Ace, Suit::Spades)), 51);
}

#[test]
fn complete_board_two_pair_beats_one_pair() {
    let tables = build_tables(false);

    let player = [card(Rank::Ace, Suit::Spades), card(Rank::King, Suit::Spades)];
    let opponent = [card(Rank::Queen, Suit::Hearts), card(Rank::Queen, Suit::Diamonds)];
    let board = [
        card(Rank::Ace, Suit::Hearts),
        card(Rank::King, Suit::Hearts),
        card(Rank::Two, Suit::Clubs),
        card(Rank::Seven, Suit::Diamonds),
        card(Rank::Nine, Suit::Spades),
    ];

    let equity = calculate_equity(&player, &opponent, &board, 25_000, &tables);
    assert_eq!(equity.wins, 1);
    assert_eq!(equity.ties, 0);
    assert_eq!(equity.equity(), 1.0);
}

#[test]
fn complete_board_identical_hands_tie() {
    let tables = build_tables(false);

    let player = [card(Rank::Ace, Suit::Spades), card(Rank::King, Suit::Spades)];
    let opponent = [card(Rank::Ace, Suit::Diamonds), card(Rank::King, Suit::Diamonds)];
    let board = [
        card(Rank::Two, Suit::Clubs),
        card(Rank::Seven, Suit::Diamonds),
        card(Rank::Nine, Suit::Spades),
        card(Rank::Ten, Suit::Hearts),
        card(Rank::Jack, Suit::Clubs),
    ];

    // both make A K J T 9
    let equity = calculate_equity(&player, &opponent, &board, 25_000, &tables);
    assert_eq!(equity.wins, 0);
    assert_eq!(equity.ties, 1);
    assert_eq!(equity.equity(), 0.5);
}
