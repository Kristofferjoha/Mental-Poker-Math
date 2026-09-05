use std::sync::Arc;
use uuid::Uuid;
use moka::sync::Cache;
use poker_eval::eval::seven::TableSeven;

use crate::problems::pot_equity::PotEquityProblem;
use crate::problems::king_of_the_hill::KingOfHillProblem;
use crate::problems::whats_the_nuts::NutsProblem;
use crate::problems::pure_equity::PureEqEquityProblem;

#[derive(Clone)]
pub struct AppState {
    pub pot_equity_cache: Arc<Cache<Uuid, PotEquityProblem>>,
    pub pure_equity_cache: Arc<Cache<Uuid, PureEqEquityProblem>>,
    pub nuts_cache: Arc<Cache<Uuid, NutsProblem>>,
    pub koth_cache: Arc<Cache<Uuid, KingOfHillProblem>>,
    pub seven_card_tables: Arc<TableSeven>
}
