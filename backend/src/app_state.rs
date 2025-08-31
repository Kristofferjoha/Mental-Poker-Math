use std::{collections::HashMap, sync::{Arc, Mutex}};
use uuid::Uuid;

use crate::poker_logic::problem_generator::PotEquityProblem;
use crate::poker_logic::pure_equity_gen::PureEqEquityProblem;
use crate::poker_logic::pure_pot_odds_gen::PurePotOddsProblem;
use crate::poker_logic::preflop_lookup::PreflopEquity;

#[derive(Clone)]
pub struct AppState {
    pub pot_eq_store: Arc<Mutex<HashMap<Uuid, PotEquityProblem>>>,
    pub pure_eq_store: Arc<Mutex<HashMap<Uuid, PureEqEquityProblem>>>,
    pub pure_pot_odds_store: Arc<Mutex<HashMap<Uuid, PurePotOddsProblem>>>,
    pub preflop_equity_data: Arc<Vec<PreflopEquity>>,
}