use std::{collections::HashMap, sync::{Arc, Mutex}};
use uuid::Uuid;

use crate::poker_logic::problem_generator::PotEquityProblem;
use crate::poker_logic::pure_equity_gen::PureEqEquityProblem;

#[derive(Clone)]
pub struct AppState {
    pub pot_eq_store: Arc<Mutex<HashMap<Uuid, PotEquityProblem>>>,
    pub pure_eq_store: Arc<Mutex<HashMap<Uuid, PureEqEquityProblem>>>,
}