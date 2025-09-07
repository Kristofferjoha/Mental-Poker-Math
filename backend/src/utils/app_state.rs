use std::{collections::HashMap, sync::{Arc, Mutex}};
use uuid::Uuid;

use crate::problems::pot_equity::PotEquityProblem;
use crate::problems::pure_equity::PureEqEquityProblem;
use crate::problems::pure_pot_odds::PurePotOddsProblem;
use crate::preflop_data::preflop_lookup::PreflopEquity;

/// Centralized, thread-safe application state for the Axum HTTP server.
/// `AppState` consolidates all shared mutable and read-only data required by request handlers.
#[derive(Clone)]
pub struct AppState {
    pub pot_equity_cache: Arc<Mutex<HashMap<Uuid, PotEquityProblem>>>,
    pub pure_equity_cache: Arc<Mutex<HashMap<Uuid, PureEqEquityProblem>>>,
    pub pure_pot_odds_cache: Arc<Mutex<HashMap<Uuid, PurePotOddsProblem>>>,
    pub preflop_equity_data: Arc<Vec<PreflopEquity>>,
}