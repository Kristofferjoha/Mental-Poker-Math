use std::sync::Arc;
use uuid::Uuid;
use moka::sync::Cache; // Replaced HashMap and Mutex with moka::Cache
use poker_eval::eval::seven::TableSeven;

use crate::problems::pot_equity::PotEquityProblem;
use crate::problems::pure_equity::PureEqEquityProblem;
use crate::problems::pure_pot_odds::PurePotOddsProblem;
use crate::preflop_data::preflop_lookup::PreflopEquity;

/// Centralized, thread-safe application state for the Axum HTTP server.
/// `AppState` consolidates all shared mutable and read-only data required by request handlers.
///
/// The caches now use `moka::sync::Cache` to prevent memory leaks. Entries are
/// automatically evicted after a configured Time-To-Live (TTL), ensuring that
/// unanswered problems do not accumulate in memory indefinitely.
#[derive(Clone)]
pub struct AppState {
    pub pot_equity_cache: Arc<Cache<Uuid, PotEquityProblem>>,
    pub pure_equity_cache: Arc<Cache<Uuid, PureEqEquityProblem>>,
    pub pure_pot_odds_cache: Arc<Cache<Uuid, PurePotOddsProblem>>,
    pub preflop_equity_data: Arc<Vec<PreflopEquity>>,
    pub seven_card_tables: Arc<TableSeven>,
}