use crate::poker_logic::{card::Card, deck::Deck, equity_calculator};
use rand::{thread_rng, Rng};

// Problem struct for pot-equity questions
#[derive(Clone, Debug)]
pub struct PureEqEquityProblem {
    pub player_hand: Vec<Card>,
    pub opponent_hand: Vec<Card>,
    pub board: Vec<Card>,
    pub player_equity: f32,
    pub lower_bound_equity: f32,
    pub upper_bound_equity: f32,
}

pub fn generate_pure_eq_problem() -> PureEqEquityProblem {
    let mut rng = thread_rng();
    let mut deck = Deck::new();
    deck.shuffle(&mut rng);

    let player_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];
    let opponent_hand = vec![deck.cards.pop().unwrap(), deck.cards.pop().unwrap()];

    let stage = rng.gen_range(0..=3);
    let board = draw_board(&mut deck, stage);



    let equity_result = equity_calculator::calculate_equity(
        &player_hand,
        &opponent_hand,
        &board,
        25_000,
    );

    let player_equity = equity_result.equity()*100.0; 
    let tolerance = 2.0; 
    let lower_bound_equity = player_equity-tolerance;
    let upper_bound_equity = player_equity+tolerance;


    PureEqEquityProblem {
        player_hand,
        opponent_hand,
        board,
        player_equity,
        lower_bound_equity,
        upper_bound_equity,
    }
}




fn draw_board(deck: &mut Deck, stage: u8) -> Vec<Card> {
    let board_card_counts = [0, 3, 4, 5];
    (0..board_card_counts[stage as usize])
        .map(|_| deck.cards.pop().unwrap())
        .collect()
}
