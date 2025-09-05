use std::{collections::HashMap, sync::{Arc, Mutex}};
use uuid::Uuid;

use crate::poker_logic::problem_generator::PotEquityProblem;
use crate::poker_logic::pure_equity_gen::PureEqEquityProblem;
use crate::poker_logic::pure_pot_odds_gen::PurePotOddsProblem;
use crate::poker_logic::preflop_lookup::PreflopEquity;

/// Centralized, thread-safe application state for the Axum HTTP server.
/// `AppState` consolidates all shared mutable and read-only data required by request handlers.
#[derive(Clone)]
pub struct AppState {
    pub pot_equity_cache: Arc<Mutex<HashMap<Uuid, PotEquityProblem>>>,
    pub pure_equity_cache: Arc<Mutex<HashMap<Uuid, PureEqEquityProblem>>>,
    pub pure_pot_odds_cache: Arc<Mutex<HashMap<Uuid, PurePotOddsProblem>>>,
    pub preflop_equity_data: Arc<Vec<PreflopEquity>>,
}