use crate::poker_logic::{card::Card, deck::Deck, equity_calculator};
use rand::{thread_rng, Rng};

// Problem struct for pot-equity questions
pub struct PotEquityProblem {
    pub player_hand: Vec<Card>,
    pub opponent_hand: Vec<Card>,
    pub board: Vec<Card>,
    pub stage: u8,
    pub pot_size: u32,
    pub bet_to_call: u32,
    pub player_equity: f32,
    pub pot_odds: f32,
    pub correct_decision: bool,
}

pub fn generate_pot_eq_problem() -> PotEquityProblem {
    let mut rng = thread_rng();
    let mut deck = Deck::new();
    deck.shuffle(&mut rng);

    let player_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
    let opponent_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];

    let stage = rng.gen_range(0..=3);
    let board = draw_board(&mut deck, stage);

    // nearest 1000 for pot size and bet to call
    let pot_size = (rng.gen_range(10_000..100_000) / 1000) * 1000;
    let bet_to_call = (rng.gen_range(5_000..pot_size) / 1000) * 1000;


    let equity_result = equity_calculator::calculate_equity(
        &player_hand,
        &opponent_hand,
        &board,
        10_000,
    );
    let player_equity = equity_result.equity(); 

    let pot_odds = (bet_to_call as f32) / ((pot_size + bet_to_call) as f32);

    let correct_decision = player_equity > pot_odds;

    PotEquityProblem {
        player_hand,
        opponent_hand,
        board,
        stage,
        pot_size,
        bet_to_call,
        player_equity,
        pot_odds,
        correct_decision,
    }
}




fn draw_board(deck: &mut Deck, stage: u8) -> Vec<Card> {
    let board_card_counts = [0, 3, 4, 5];
    (0..board_card_counts[stage as usize])
        .map(|_| deck.cards.pop().unwrap())
        .collect()
}
