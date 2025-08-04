use crate::poker_logic::{
    card::Card,
    deck::Deck,
    hand_evaluator::evaluate_hand,
};
use rand::thread_rng;

// Equity struct, losses not used. Can be removed, keeping for now if needed later.
pub struct Equity {
    pub wins: u32,
    pub ties: u32,
    pub losses: u32,
    pub total_sims: u32,
}

// Calculating equity as a float value
impl Equity {
    pub fn equity(&self) -> f32 {
        (self.wins as f32 + self.ties as f32 / 2.0) / self.total_sims as f32
    }
}

// Calculates number of wins, ties, and losses for a given player hand against an opponent hand and board.
// Optimized using rayon parallelism later.
pub fn calculate_equity(
    player_hand: &[Card],
    opponent_hand: &[Card],
    board: &[Card],
    num_simulations: u32,
) -> Equity {
    let mut wins = 0;
    let mut ties = 0;

    for _ in 0..num_simulations {
        
        let mut deck = Deck::new();

        deck.remove_cards(player_hand);
        deck.remove_cards(opponent_hand);
        deck.remove_cards(board);

        deck.shuffle(&mut thread_rng());

        // Deals remaining cards needed
        let cards_to_deal = 5 - board.len();
        let mut simulated_board = board.to_vec();
        for _ in 0..cards_to_deal {
            // deal() should probably return Option<Card>, so unwrap or handle
            simulated_board.push(deck.cards.pop().unwrap()); 
        }

        // Evaluates hands
        // ectend_from_slice is used to add the simulated board to the player and opponent hands.
        let mut player_full_hand = player_hand.to_vec();
        player_full_hand.extend_from_slice(&simulated_board);

        let mut opponent_full_hand = opponent_hand.to_vec();
        opponent_full_hand.extend_from_slice(&simulated_board);

        let player_rank = evaluate_hand(&player_full_hand);
        let opponent_rank = evaluate_hand(&opponent_full_hand);

        // Compare the results and update counters.
        use std::cmp::Ordering;
        match player_rank.cmp(&opponent_rank) {
            Ordering::Greater => wins += 1,
            Ordering::Equal => ties += 1,
            Ordering::Less => (), 
        }
    }
    
    Equity {
        wins,
        ties,
        losses: num_simulations - wins - ties,
        total_sims: num_simulations,
    }
}
